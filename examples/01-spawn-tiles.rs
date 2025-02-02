//! This examples showcases how to spawn tiles in a hexagonal grid using the game's library.

#![allow(clippy::type_complexity)]

use bevy::{asset::AssetMetaCheck, prelude::*};

use game::prelude::*;
use rand::prelude::*;

#[derive(Resource, Default, Deref, DerefMut)]
struct HexMapRing(pub u32);

#[derive(Resource, Deref, DerefMut)]
struct HexMapRng(StdRng);

struct DemoPlugin;

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

        app.insert_resource(HexMapRing(0));
        app.insert_resource(HexMapRng(StdRng::from_os_rng()));

        app.add_systems(OnEnter(GameStates::Playing), setup_game);
        app.add_systems(Update, (update_ring, update_camera_zoom).run_if(in_state(GameStates::Playing)));
    }
}

fn setup_game(mut commands: Commands, hexmap: Res<HexMapResource>, ring: Res<HexMapRing>) {
    for hex in hexmap.axial_spiral(IVec2::ZERO, **ring) {
        let coord = hexmap.axial_to_pixel(hex);
        let translation = coord.extend(0.0).xzy();

        commands.spawn((
            Name::new("TestingTile"),
            HexTile,
            HexTileKind::Empty,
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
    mut rng: ResMut<HexMapRng>,
) {
    if keys.just_pressed(KeyCode::Space) {
        **ring += 1;

        for hex in hexmap.axial_ring(IVec2::ZERO, **ring) {
            let coord = hexmap.axial_to_pixel(hex);
            let translation = coord.extend(0.0).xzy();

            commands.spawn((
                Name::new("TestingTile"),
                HexTile,
                HexTileKind::random(&mut *rng),
                Visibility::default(),
                Transform::from_translation(translation),
                StateScoped(GameStates::Playing),
            ));
        }
    }
}

fn update_camera_zoom(
    ring: Res<HexMapRing>,
    mut viewport_height: ResMut<ViewportHeight>,
) {
    **viewport_height = 6.0 + 2.0 * **ring as f32;
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DemoPlugin);
    app.run();
}
