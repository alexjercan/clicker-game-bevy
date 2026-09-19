use bevy::prelude::*;
use bevy_tweening::{lens::TransformScaleLens, *};

const HEX_UP_SCALE: f32 = 1.1;

#[derive(Debug, Copy, Clone, PartialEq)]
struct TransformPositionScaleLens {
    start_translation: Vec3,
    end_translation: Vec3,
    start_scale: Vec3,
    end_scale: Vec3,
}

impl Lens<Transform> for TransformPositionScaleLens {
    fn lerp(&mut self, mut target: Mut<Transform>, ratio: f32) {
        target.translation = self.start_translation.lerp(self.end_translation, ratio);
        target.scale = self.start_scale.lerp(self.end_scale, ratio);
    }
}

#[derive(Resource)]
pub struct AnimationsEnabled;

#[derive(Component)]
pub struct SpawnAnimation;

#[derive(Message, Debug, Clone, Copy)]
pub struct SpawnAnimationFinished {
    pub entity: Entity,
}

pub struct ClickerAnimationPlugin;

impl Plugin for ClickerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimationsEnabled)
            .add_message::<SpawnAnimationFinished>()
            .add_plugins(TweeningPlugin)
            .add_systems(Update, report_finished_animations);
    }
}

fn report_finished_animations(
    mut completed: MessageReader<AnimCompletedEvent>,
    spawning: Query<(), With<SpawnAnimation>>,
    mut finished: MessageWriter<SpawnAnimationFinished>,
) {
    for event in completed.read() {
        if spawning.contains(event.anim_entity) {
            finished.write(SpawnAnimationFinished {
                entity: event.anim_entity,
            });
        }
    }
}

pub fn tile_select_animation() -> impl Bundle {
    (TweenAnim::new(Tween::new(
        EaseFunction::QuadraticOut,
        std::time::Duration::from_millis(100),
        TransformScaleLens {
            start: Vec3::ONE,
            end: Vec3::splat(HEX_UP_SCALE),
        },
    )),)
}

pub fn tile_deselect_animation() -> impl Bundle {
    (TweenAnim::new(Tween::new(
        EaseFunction::QuadraticOut,
        std::time::Duration::from_millis(100),
        TransformScaleLens {
            start: Vec3::splat(HEX_UP_SCALE),
            end: Vec3::ONE,
        },
    )),)
}

pub fn tile_click_animation() -> impl Bundle {
    (TweenAnim::new(
        Tween::new(
            EaseFunction::QuadraticOut,
            std::time::Duration::from_millis(100),
            TransformScaleLens {
                start: Vec3::splat(HEX_UP_SCALE),
                end: Vec3::ONE,
            },
        )
        .with_repeat_count(RepeatCount::Finite(2))
        .with_repeat_strategy(RepeatStrategy::MirroredRepeat),
    ),)
}

pub fn tile_spawn_animation() -> impl Bundle {
    (
        SpawnAnimation,
        TweenAnim::new(Tween::new(
            EaseFunction::QuadraticOut,
            std::time::Duration::from_millis(500),
            TransformPositionScaleLens {
                start_translation: Vec3::new(0.0, -5.0, 0.0),
                end_translation: Vec3::ZERO,
                start_scale: Vec3::splat(0.5),
                end_scale: Vec3::ONE,
            },
        )),
    )
}
