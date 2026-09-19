use std::sync::Arc;

use bevy::{input::InputSystems, prelude::*};

use crate::predicate::Predicate;

pub const AUTOPILOT_ENV: &str = "CLICKER_AUTOPILOT";
pub const DEADLINE_ENV: &str = "CLICKER_AUTOPILOT_DEADLINE";
pub const DEFAULT_DEADLINE_SECS: f32 = 120.0;

type ActFn = dyn Fn(&mut World) + Send + Sync;
type DiagnoseFn = dyn Fn(&World) -> String + Send + Sync;

#[derive(Clone)]
struct Wait {
    predicate: Arc<Predicate>,
    deadline_secs: f32,
}

#[derive(Clone)]
struct Step {
    name: String,
    acts: Vec<Arc<ActFn>>,
    wait: Option<Wait>,
    expectation: Arc<Predicate>,
    diagnose: Option<Arc<DiagnoseFn>>,
}

fn immediately() -> Arc<Predicate> {
    Arc::new(|_| true)
}

#[derive(Default)]
pub struct AutopilotPlugin {
    steps: Vec<Step>,
}

impl AutopilotPlugin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(self, name: impl Into<String>) -> StepBuilder {
        StepBuilder {
            plugin: self,
            step: Step {
                name: name.into(),
                acts: Vec::new(),
                wait: None,
                expectation: immediately(),
                diagnose: None,
            },
        }
    }
}

pub struct StepBuilder {
    plugin: AutopilotPlugin,
    step: Step,
}

impl StepBuilder {
    pub fn act(mut self, action: impl Fn(&mut World) + Send + Sync + 'static) -> Self {
        self.step.acts.push(Arc::new(action));
        self
    }

    pub fn until(mut self, predicate: Arc<Predicate>, deadline_secs: f32) -> Self {
        assert!(
            deadline_secs.is_finite() && deadline_secs > 0.0,
            "autopilot step deadline must be finite and positive"
        );
        self.step.wait = Some(Wait {
            predicate,
            deadline_secs,
        });
        self
    }

    pub fn expect(mut self, predicate: Arc<Predicate>) -> Self {
        self.step.expectation = predicate;
        self
    }

    pub fn diagnose(mut self, diagnose: impl Fn(&World) -> String + Send + Sync + 'static) -> Self {
        self.step.diagnose = Some(Arc::new(diagnose));
        self
    }

    pub fn add(mut self) -> AutopilotPlugin {
        self.plugin.steps.push(self.step);
        self.plugin
    }
}

#[derive(Resource, Default)]
pub(crate) struct AutopilotClock {
    pub(crate) step_elapsed: f32,
    step_real: f32,
    run_real: f32,
}

#[derive(Resource)]
struct AutopilotState {
    steps: Arc<[Step]>,
    index: usize,
    acted: bool,
    done: bool,
    run_deadline_secs: f32,
}

impl Plugin for AutopilotPlugin {
    fn build(&self, app: &mut App) {
        if std::env::var_os(AUTOPILOT_ENV).is_none() {
            return;
        }
        assert!(!self.steps.is_empty(), "armed autopilot has no steps");
        let run_deadline_secs = std::env::var(DEADLINE_ENV)
            .ok()
            .map(|raw| {
                raw.parse().unwrap_or_else(|error| {
                    panic!("{DEADLINE_ENV}={raw:?} is not a deadline in seconds ({error})")
                })
            })
            .unwrap_or(DEFAULT_DEADLINE_SECS);
        assert!(
            run_deadline_secs.is_finite() && run_deadline_secs > 0.0,
            "autopilot run deadline must be finite and positive"
        );
        app.insert_resource(AutopilotState {
            steps: self.steps.clone().into(),
            index: 0,
            acted: false,
            done: false,
            run_deadline_secs,
        })
        .init_resource::<AutopilotClock>()
        .add_systems(PreUpdate, drive.after(InputSystems));
    }
}

fn drive(world: &mut World) {
    let mut state = world
        .remove_resource::<AutopilotState>()
        .expect("autopilot state exists while its system runs");
    if state.done {
        world.insert_resource(state);
        return;
    }

    let real_delta = world.resource::<Time<Real>>().delta_secs();
    {
        let mut clock = world.resource_mut::<AutopilotClock>();
        clock.run_real += real_delta;
    }
    if world.resource::<AutopilotClock>().run_real >= state.run_deadline_secs {
        error!(
            "autopilot: run exceeded its {:.1}s deadline at step `{}`",
            state.run_deadline_secs, state.steps[state.index].name
        );
        world.write_message(AppExit::error());
        state.done = true;
        world.insert_resource(state);
        return;
    }

    let steps = Arc::clone(&state.steps);
    let step = &steps[state.index];
    if !state.acted {
        info!("autopilot: step `{}` begins", step.name);
        for action in &step.acts {
            action(world);
        }
        state.acted = true;
        let mut clock = world.resource_mut::<AutopilotClock>();
        clock.step_elapsed = 0.0;
        clock.step_real = 0.0;
        world.insert_resource(state);
        return;
    }

    let delta = world.resource::<Time>().delta_secs();
    let step_real = {
        let mut clock = world.resource_mut::<AutopilotClock>();
        clock.step_elapsed += delta;
        clock.step_real += real_delta;
        clock.step_real
    };
    if let Some(wait) = &step.wait {
        if !(wait.predicate)(world) {
            if step_real >= wait.deadline_secs {
                fail(
                    world,
                    format!(
                        "autopilot: step `{}` exceeded its {:.1}s deadline{}",
                        step.name,
                        wait.deadline_secs,
                        diagnosis(step, world)
                    ),
                );
                state.done = true;
                world.insert_resource(state);
            } else {
                world.insert_resource(state);
            }
            return;
        }
    }
    if !(step.expectation)(world) {
        fail(
            world,
            format!(
                "autopilot: step `{}` failed its expectation{}",
                step.name,
                diagnosis(step, world)
            ),
        );
        state.done = true;
        world.insert_resource(state);
        return;
    }

    state.index += 1;
    if state.index == state.steps.len() {
        info!("autopilot: cycle complete, no panic");
        world.write_message(AppExit::Success);
        state.done = true;
    } else {
        state.acted = false;
    }
    world.insert_resource(state);
}

fn diagnosis(step: &Step, world: &World) -> String {
    step.diagnose
        .as_ref()
        .map(|diagnose| format!(" - {}", diagnose(world)))
        .unwrap_or_default()
}

fn fail(world: &mut World, message: String) {
    error!("{message}");
    world.write_message(AppExit::error());
}
