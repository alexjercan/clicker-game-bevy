//! This examples showcases spawning tiles on level up

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

        app.insert_resource(HexMapRing(2));
        app.insert_resource(HexMapRng(StdRng::from_os_rng()));

        app.add_systems(OnEnter(GameStates::Playing), setup_game);
        app.add_systems(
            Update,
            update_selected_hex.run_if(in_state(GameStates::Playing)),
        );
        app.add_systems(
            Update,
            (
                handle_click_tile,
                handle_clicked_selected,
                handle_level_up_player,
                update_camera_zoom,
            )
                .run_if(in_state(GameStates::Playing)),
        );
    }
}

fn setup_game(
    mut commands: Commands,
    hexmap: Res<HexMapResource>,
    ring: Res<HexMapRing>,
    mut rng: ResMut<HexMapRng>,
) {
    commands.spawn((
        Name::new("TestingLevelXP"),
        LevelXP::default(),
        NextLevelXP(10),
        SkillTreePoints::default(),
        StateScoped(GameStates::Playing),
    ));

    commands.spawn((
        Name::new("TestingRootUI"),
        RootUI,
        StateScoped(GameStates::Playing),
    ));

    for hex in hexmap.axial_spiral(IVec2::ZERO, **ring) {
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

fn update_selected_hex(
    mut commands: Commands,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    hexmap: Res<HexMapResource>,
    q_hex: Query<(Entity, &HexTileAxial), (With<HexTile>, Without<HexTileSelected>)>,
    q_selected: Query<(Entity, &HexTileAxial), (With<HexTile>, With<HexTileSelected>)>,
) {
    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        return;
    };

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) else {
        return;
    };
    let point = ray.get_point(distance);

    let coord = point.xz();

    let axial = hexmap.pixel_to_axial(coord);

    for (entity, hex) in q_hex.iter() {
        if axial == **hex {
            commands.entity(entity).insert(HexTileSelected);
        }
    }

    for (entity, hex) in q_selected.iter() {
        if axial != **hex {
            commands.entity(entity).remove::<HexTileSelected>();
        }
    }
}

fn handle_click_tile(
    buttons: Res<ButtonInput<MouseButton>>,
    mut ev_click: EventWriter<HexTileClickSelected>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        ev_click.send(HexTileClickSelected);
    }
}

fn handle_clicked_selected(
    mut q_player: Query<&mut LevelXP>,
    q_hex: Query<Entity, (With<HexTile>, With<HexTileSelected>)>,
    mut ev_click: EventReader<HexTileClickSelected>,
) {
    for _ in &q_hex {
        for HexTileClickSelected in ev_click.read() {
            for mut level_xp in q_player.iter_mut() {
                **level_xp += 1;
            }
        }
    }
}

fn handle_level_up_player(
    mut ev_level_up: EventReader<LevelUpEvent>,
    mut q_player: Query<&mut LevelXP>,
) {
    for LevelUpEvent(entity) in ev_level_up.read() {
        if let Ok(mut level_xp) = q_player.get_mut(*entity) {
            **level_xp = 0;
        }
    }
}

fn update_camera_zoom(ring: Res<HexMapRing>, mut viewport_height: ResMut<ViewportHeight>) {
    **viewport_height = 6.0 + 2.0 * **ring as f32;
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DemoPlugin);
    app.run();
}
