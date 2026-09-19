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

pub struct ClickerAnimationPlugin;

impl Plugin for ClickerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweeningPlugin);
    }
}

pub fn select_tile() -> TweenAnim {
    TweenAnim::new(Tween::new(
        EaseFunction::QuadraticOut,
        std::time::Duration::from_millis(100),
        TransformScaleLens {
            start: Vec3::ONE,
            end: Vec3::splat(HEX_UP_SCALE),
        },
    ))
}

pub fn deselect_tile() -> TweenAnim {
    TweenAnim::new(Tween::new(
        EaseFunction::QuadraticOut,
        std::time::Duration::from_millis(100),
        TransformScaleLens {
            start: Vec3::splat(HEX_UP_SCALE),
            end: Vec3::ONE,
        },
    ))
}

pub fn click_tile() -> TweenAnim {
    TweenAnim::new(
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
    )
}

pub fn spawn_tile() -> TweenAnim {
    TweenAnim::new(Tween::new(
        EaseFunction::QuadraticOut,
        std::time::Duration::from_millis(500),
        TransformPositionScaleLens {
            start_translation: Vec3::new(0.0, -5.0, 0.0),
            end_translation: Vec3::ZERO,
            start_scale: Vec3::splat(0.5),
            end_scale: Vec3::ONE,
        },
    ))
}
