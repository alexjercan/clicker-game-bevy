use bevy::prelude::*;
use bevy_tweening::lens::*;
use bevy_tweening::*;

use super::HexTile;

#[derive(Component, Debug, Default)]
struct HexTileTweening;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexTweeningPluginSet;

#[derive(Debug)]
pub struct HexTweeningPlugin;

impl Plugin for HexTweeningPlugin {
    fn build(&self, app: &mut App) {
        // NOTE: Tweening runs in post update because we want to add the animation before we add
        // the render mesh
        app.add_systems(PostUpdate, add_hex_tweening.in_set(HexTweeningPluginSet));
    }
}

fn add_hex_tweening(
    mut commands: Commands,
    q_hex: Query<(Entity, &Transform), (With<HexTile>, Without<HexTileTweening>)>,
) {
    for (entity, transform) in &q_hex {
        let tween_move = Tween::new(
            EaseFunction::QuadraticOut,
            std::time::Duration::from_millis(500),
            TransformPositionLens {
                start: transform.translation + Vec3::new(0.0, -5.0, 0.0),
                end: transform.translation,
            },
        );

        let tween_scale = Tween::new(
            EaseFunction::QuadraticOut,
            std::time::Duration::from_millis(500),
            TransformScaleLens {
                start: Vec3::new(0.5, 0.5, 0.5),
                end: Vec3::new(1.0, 1.0, 1.0),
            },
        );

        let track = Tracks::new([tween_move, tween_scale]);

        commands.entity(entity).insert((HexTileTweening, Animator::new(track)));
    }
}
