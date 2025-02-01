//! This examples showcases how to spawn tiles in a hexagonal grid using the game's library.

#![allow(clippy::type_complexity)]

use bevy::{asset::AssetMetaCheck, prelude::*};

use game::prelude::*;
use hexmap::HexMap;

#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct HexMapRing(pub u32);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct HexMapResource(HexMap);

pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Example Spawn Tiles".to_string(),
                        // Bind to canvas included in `index.html`
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5 and Ctrl+R
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        );

        app.add_plugins(CorePlugin);

        app.insert_resource(HexMapResource(HexMap::new(2.0 / 3.0f32.sqrt())));
        app.insert_resource(HexMapRing(0));

        app.add_systems(OnEnter(GameStates::Playing), setup_game);
        app.add_systems(Update, update_ring.run_if(in_state(GameStates::Playing)));
    }
}

fn setup_game(mut commands: Commands, hexmap: Res<HexMapResource>, ring: Res<HexMapRing>) {
    for hex in hexmap.axial_spiral(IVec2::ZERO, **ring) {
        let coord = hexmap.axial_to_pixel(hex);
        let translation = coord.extend(0.0).xzy();

        commands.spawn((
            Name::new("TestingTile"),
            HexTile,
            Visibility::default(),
            Transform::from_translation(translation),
            StateScoped(GameStates::Playing),
        ));
    }
}

fn update_ring(
    mut commands: Commands,
    hexmap: Res<HexMapResource>,
    mut ring: ResMut<HexMapRing>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        **ring += 1;

        for hex in hexmap.axial_ring(IVec2::ZERO, **ring) {
            let coord = hexmap.axial_to_pixel(hex);
            let translation = coord.extend(0.0).xzy();

            /*
            let tween_move = Tween::new(
                EaseFunction::QuadraticOut,
                std::time::Duration::from_millis(500),
                TransformPositionLens {
                    start: Vec3::new(0.0, -5.0, 0.0),
                    end: Vec3::new(0.0, 0.0, 0.0),
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

            let _track = Tracks::new([tween_move, tween_scale]);
            */

            commands.spawn((
                Name::new("TestingTile"),
                HexTile,
                Visibility::default(),
                Transform::from_translation(translation),
                StateScoped(GameStates::Playing),
            ));
        }
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DemoPlugin);
    app.run();
}
