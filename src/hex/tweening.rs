//! Tweening for hex tiles. Pop in the tiles with a nice animation.

use bevy::prelude::*;
use bevy_tweening::lens::*;
use bevy_tweening::*;

use super::{HexTile, HexTileClickSelected, HexTileSelected};

#[derive(Component, Debug, Default)]
struct HexTileSpawnTweening;

#[derive(Component, Debug, Default)]
struct HexTileSpawnTweeningDone;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexTweeningPluginSet;

#[derive(Debug)]
pub struct HexTweeningPlugin;

impl Plugin for HexTweeningPlugin {
    fn build(&self, app: &mut App) {
        // NOTE: Tweening runs in post update because we want to add the animation before we add
        // the render mesh
        app.add_systems(
            PostUpdate,
            (add_hex_tweening_spawn, update_hex_tweening_spawn).in_set(HexTweeningPluginSet),
        );
        app.add_systems(
            PostUpdate,
            (add_hex_tweening_selected, add_hex_tweening_deselected).in_set(HexTweeningPluginSet),
        );
        app.add_systems(
            PostUpdate,
            add_hex_tweening_clicked.in_set(HexTweeningPluginSet),
        );
    }
}

fn add_hex_tweening_spawn(
    mut commands: Commands,
    q_hex: Query<
        (Entity, &Transform),
        (
            With<HexTile>,
            Without<HexTileSpawnTweening>,
            Without<HexTileSpawnTweeningDone>,
        ),
    >,
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

        commands
            .entity(entity)
            .insert((HexTileSpawnTweening, Animator::new(track)));
    }
}

fn update_hex_tweening_spawn(
    mut commands: Commands,
    q_hex: Query<(Entity, &Animator<Transform>), (With<HexTile>, With<HexTileSpawnTweening>)>,
) {
    for (entity, animator) in &q_hex {
        if animator.tweenable().times_completed() > 0 {
            commands.entity(entity).remove::<HexTileSpawnTweening>();
            commands.entity(entity).insert(HexTileSpawnTweeningDone);
        }
    }
}

fn add_hex_tweening_selected(
    mut commands: Commands,
    q_hex: Query<
        Entity,
        (
            With<HexTile>,
            With<HexTileSpawnTweeningDone>,
            Added<HexTileSelected>,
        ),
    >,
) {
    for entity in &q_hex {
        let tween_scale = Tween::new(
            EaseFunction::QuadraticOut,
            std::time::Duration::from_millis(250),
            TransformScaleLens {
                start: Vec3::new(1.0, 1.0, 1.0),
                end: Vec3::new(1.1, 1.1, 1.1),
            },
        );

        let track = Tracks::new([tween_scale]);

        commands.entity(entity).insert(Animator::new(track));
    }
}

fn add_hex_tweening_deselected(
    mut commands: Commands,
    q_hex: Query<Entity, (With<HexTile>, With<HexTileSpawnTweeningDone>)>,
    mut ev_removed: RemovedComponents<HexTileSelected>,
) {
    for entity in ev_removed.read() {
        if let Ok(entity) = q_hex.get(entity) {
            let tween_scale = Tween::new(
                EaseFunction::QuadraticOut,
                std::time::Duration::from_millis(250),
                TransformScaleLens {
                    start: Vec3::new(1.1, 1.1, 1.1),
                    end: Vec3::new(1.0, 1.0, 1.0),
                },
            );

            let track = Tracks::new([tween_scale]);

            commands.entity(entity).insert(Animator::new(track));
        }
    }
}

fn add_hex_tweening_clicked(
    mut commands: Commands,
    q_hex: Query<Entity, (With<HexTile>, With<HexTileSpawnTweeningDone>, With<HexTileSelected>)>,
    mut ev_click: EventReader<HexTileClickSelected>,
) {
    for _ in ev_click.read() {
        for entity in &q_hex {
            let tween_scale = Tween::new(
                EaseFunction::QuadraticOut,
                std::time::Duration::from_millis(100),
                TransformScaleLens {
                    start: Vec3::new(1.0, 1.0, 1.0),
                    end: Vec3::new(1.2, 1.2, 1.2),
                },
            )
                .with_repeat_count(RepeatCount::Finite(2))
                .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

            let track = Tracks::new([tween_scale]);

            commands.entity(entity).insert(Animator::new(track));
        }
    }
}
