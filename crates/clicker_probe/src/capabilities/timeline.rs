use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use bevy::{prelude::*, state::state::StateTransitionEvent};
use clicker_gameplay::{SkillPoints, XpMax, XpValue};
use clicker_state::GameState;
use clicker_tile::{TileClicked, TilePlaced, TileSettled};

use crate::ProbeFrame;

pub const TIMELINE_ENV: &str = "CLICKER_PROBE_TIMELINE";

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProbeSystems {
    RunEnd,
}

pub struct TimelinePlugin {
    out: Option<PathBuf>,
}

impl TimelinePlugin {
    pub fn out(mut self, path: impl Into<PathBuf>) -> Self {
        self.out = Some(path.into());
        self
    }
}

pub fn timeline() -> TimelinePlugin {
    TimelinePlugin { out: None }
}

#[derive(Resource)]
pub struct ProbeTimeline {
    sink: BufWriter<File>,
    entries: u64,
}

impl ProbeTimeline {
    fn create(path: PathBuf) -> Self {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap_or_else(|error| {
                panic!("cannot create timeline directory {parent:?}: {error}")
            });
        }
        let file = File::create(&path)
            .unwrap_or_else(|error| panic!("cannot create timeline {path:?}: {error}"));
        Self {
            sink: BufWriter::new(file),
            entries: 0,
        }
    }

    fn record(&mut self, frame: u64, kind: &str, name: &str, data: serde_json::Value) {
        let line = serde_json::json!({
            "frame": frame,
            "kind": kind,
            "name": name,
            "data": data,
        });
        writeln!(self.sink, "{line}")
            .and_then(|()| self.sink.flush())
            .unwrap_or_else(|error| panic!("cannot write probe timeline: {error}"));
        self.entries += 1;
    }
}

impl Plugin for TimelinePlugin {
    fn build(&self, app: &mut App) {
        let path = self
            .out
            .clone()
            .or_else(|| std::env::var_os(TIMELINE_ENV).map(PathBuf::from));
        let Some(path) = path else {
            return;
        };
        crate::ensure_clock(app);
        app.insert_resource(ProbeTimeline::create(path))
            .add_systems(Startup, record_start)
            .add_systems(Update, record_states)
            .add_systems(
                Last,
                (record_tile_events, record_progression, record_end)
                    .chain()
                    .in_set(ProbeSystems::RunEnd),
            );
    }
}

fn record_start(frame: Res<ProbeFrame>, mut timeline: ResMut<ProbeTimeline>) {
    timeline.record(frame.0, "run_start", "run", serde_json::json!({}));
}

fn record_states(
    mut transitions: MessageReader<StateTransitionEvent<GameState>>,
    frame: Res<ProbeFrame>,
    mut timeline: ResMut<ProbeTimeline>,
) {
    for transition in transitions.read() {
        timeline.record(
            frame.0,
            "state",
            "GameState",
            serde_json::json!({
                "exited": transition.exited.as_ref().map(|state| format!("{state:?}")),
                "entered": transition.entered.as_ref().map(|state| format!("{state:?}")),
            }),
        );
    }
}

fn record_tile_events(
    mut clicks: MessageReader<TileClicked>,
    mut placements: MessageReader<TilePlaced>,
    mut settled: MessageReader<TileSettled>,
    frame: Res<ProbeFrame>,
    mut timeline: ResMut<ProbeTimeline>,
) {
    for click in clicks.read() {
        timeline.record(
            frame.0,
            "action",
            "tile_clicked",
            serde_json::json!({"q": click.coord.q(), "r": click.coord.r()}),
        );
    }
    for placed in placements.read() {
        timeline.record(
            frame.0,
            "action",
            "tile_placed",
            serde_json::json!({
                "q": placed.coord.q(),
                "r": placed.coord.r(),
                "kind": format!("{:?}", placed.kind).to_lowercase(),
            }),
        );
    }
    for event in settled.read() {
        timeline.record(
            frame.0,
            "animation",
            "tile_settled",
            serde_json::json!({"q": event.coord.q(), "r": event.coord.r()}),
        );
    }
}

fn record_progression(
    xp: Res<XpValue>,
    max: Res<XpMax>,
    points: Res<SkillPoints>,
    frame: Res<ProbeFrame>,
    mut previous: Local<Option<(u32, u32, u32)>>,
    mut timeline: ResMut<ProbeTimeline>,
) {
    let current = (xp.0, max.0, points.0);
    if previous.as_ref() == Some(&current) {
        return;
    }
    timeline.record(
        frame.0,
        "progression",
        "resources",
        serde_json::json!({"xp": current.0, "xp_max": current.1, "skill_points": current.2}),
    );
    *previous = Some(current);
}

fn record_end(
    mut exits: MessageReader<AppExit>,
    frame: Res<ProbeFrame>,
    mut timeline: ResMut<ProbeTimeline>,
) {
    let Some(exit) = exits.read().next() else {
        return;
    };
    let entries = timeline.entries;
    timeline.record(
        frame.0,
        "run_end",
        "run",
        serde_json::json!({"exit": format!("{exit:?}"), "entries": entries}),
    );
}

pub fn marker(world: &mut World, name: &str, data: serde_json::Value) {
    if !world.contains_resource::<ProbeTimeline>() {
        return;
    }
    let frame = world.resource::<ProbeFrame>().0;
    world
        .resource_mut::<ProbeTimeline>()
        .record(frame, "marker", name, data);
}
