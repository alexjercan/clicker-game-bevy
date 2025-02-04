//! The Game

#![allow(clippy::type_complexity)]

#[cfg(feature = "debug")]
mod debug;

#[cfg(feature = "debug")]
use debug::*;

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_asset_loader::prelude::*;
use hex::*;
use ortho_camera::*;
use skilltree::*;

use rand::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameStates {
    #[default]
    AssetLoading,
    Playing,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum ClickStates {
    /// The default click state where "click" action will just gather resources
    /// This increases the player's XP (only when clicking on a tile)
    #[default]
    Gather,
    /// When the player has skill points, they can spend them on the skill tree
    /// To make things easy for UX perspective, we use a different state for this
    /// if you have skill points, the next click anywhere will open the skill tree
    /// This means that you cannot gather resources while having skill points though
    LevelUp,
    /// When the player has skill points, they can spend them on the skill tree
    /// This state is used when the skill tree menu is open such that we know
    /// that clicks should not affect other systems.
    LevelUpMenu,
}

#[derive(AssetCollection, Resource)]
pub struct UIAssets {
    #[asset(path = "undefined.png")]
    pub skill_tree_points: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "gltf/tiles/base/hex_grass.gltf")]
    pub hex_base: Handle<Gltf>,
    #[asset(path = "gltf/decoration/nature/trees_A_large.gltf")]
    pub hex_tree: Handle<Gltf>,
    #[asset(path = "gltf/decoration/nature/hills_A.gltf")]
    pub hex_stone: Handle<Gltf>,
    #[asset(path = "gltf/buildings/neutral/building_dirt.gltf")]
    pub hex_dirt: Handle<Gltf>,
    #[asset(path = "gltf/buildings/neutral/building_grain.gltf")]
    pub hex_wheat: Handle<Gltf>,
}

#[derive(Resource, Default, Deref, DerefMut)]
struct HexMapRing(pub u32);

#[derive(Resource, Deref, DerefMut)]
struct HexMapRng(StdRng);

struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Clicker".to_string(),
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
        app.add_plugins(bevy_tweening::TweeningPlugin);

        app.add_plugins(CameraPlugin);
        app.add_plugins(HexPlugin);
        app.add_plugins(SkillTreePlugin);

        #[cfg(feature = "debug")]
        app.add_plugins(DebugPlugin);

        app.insert_resource(HexMapRing(0));
        app.insert_resource(HexMapRng(StdRng::from_os_rng()));

        app.init_state::<GameStates>();
        app.enable_state_scoped_entities::<GameStates>();
        app.init_state::<ClickStates>();
        app.enable_state_scoped_entities::<ClickStates>();

        app.add_loading_state(
            LoadingState::new(GameStates::AssetLoading)
                .continue_to_state(GameStates::Playing)
                .load_collection::<UIAssets>()
                .load_collection::<GameAssets>(),
        );

        app.add_systems(OnEnter(GameStates::AssetLoading), setup_asset_loading);
        app.add_systems(OnEnter(GameStates::Playing), setup_playing);

        app.add_systems(
            Update,
            (
                update_click_state,
                update_selected_hex,
                update_camera_zoom,
            )
                .run_if(in_state(GameStates::Playing)),
        );
        app.add_systems(
            Update,
            (handle_clicked_selected, handle_click_tile)
                .run_if(in_state(GameStates::Playing))
                .run_if(in_state(ClickStates::Gather)),
        );
        app.add_systems(
            Update,
            (handle_click_level_up,)
                .run_if(in_state(GameStates::Playing))
                .run_if(in_state(ClickStates::LevelUp)),
        );

        app.configure_sets(
            Update,
            CameraPluginSet.run_if(in_state(GameStates::Playing)),
        );
        app.configure_sets(Update, HexPluginSet.run_if(in_state(GameStates::Playing)));
        app.configure_sets(
            PostUpdate,
            HexPluginSet.run_if(in_state(GameStates::Playing)),
        );
        app.configure_sets(
            Update,
            SkillTreePluginSet.run_if(in_state(GameStates::Playing)),
        );
    }
}

fn setup_asset_loading(mut commands: Commands) {
    commands.spawn((
        Name::new("CameraUI"),
        Camera2d::default(),
        StateScoped(GameStates::AssetLoading),
    ));
}

fn setup_playing(mut commands: Commands, game_assets: Res<GameAssets>, ui_assets: Res<UIAssets>) {
    commands.spawn((
        Name::new("Camera3D"),
        Camera3d::default(),
        Transform::from_xyz(-15.0, 15.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraOrtho::default(),
        StateScoped(GameStates::Playing),
    ));

    commands.spawn((
        Name::new("DirectionalLight"),
        DirectionalLight::default(),
        Transform::from_xyz(-1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        StateScoped(GameStates::Playing),
    ));

    commands
        .spawn((
            Name::new("UIRoot"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Top bar for XP and Skill points alerts
            parent.spawn((
                Name::new("TopBar"),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
                SkillTreeUIRoot,
            ));
        });

    commands.insert_resource(HexRenderAssets {
        base: game_assets.hex_base.clone(),
        tree: game_assets.hex_tree.clone(),
        stone: game_assets.hex_stone.clone(),
        dirt: game_assets.hex_dirt.clone(),
        wheat: game_assets.hex_wheat.clone(),
    });

    commands.insert_resource(SkillTreeUIAssets {
        skill_tree_points: ui_assets.skill_tree_points.clone(),
    });

    // TODO: Maybe we should load the settings from a file + save/load mechanics
    commands.spawn((
        Name::new("TestingLevelXP"),
        LevelXP::default(),
        NextLevelXP(10),
        SkillTreePoints(1),
        StateScoped(GameStates::Playing),
    ));
}

fn update_click_state(
    state: Res<State<ClickStates>>,
    mut next_state: ResMut<NextState<ClickStates>>,
    q_points: Query<&SkillTreePoints>,
) {
    // If we have skill points, we should go to the level up state

    let Ok(points) = q_points.get_single() else {
        return;
    };

    if **points > 0 {
        match **state {
            ClickStates::Gather => next_state.set(ClickStates::LevelUp),
            _ => {}
        }
    } else {
        next_state.set(ClickStates::Gather);
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

fn handle_click_level_up(
    mut commands: Commands,
    hexmap: Res<HexMapResource>,
    mut ring: ResMut<HexMapRing>,
    mut rng: ResMut<HexMapRng>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut q_points: Query<&mut SkillTreePoints>,
) {
    let Ok(mut points) = q_points.get_single_mut() else {
        return;
    };

    if buttons.just_pressed(MouseButton::Left) {
        if **ring == 0 {
            let coord = hexmap.axial_to_pixel(IVec2::ZERO);
            let translation = coord.extend(0.0).xzy();

            commands.spawn((
                Name::new("HexTile"),
                HexTile,
                HexTileKind::random(&mut *rng),
                Visibility::default(),
                Transform::from_translation(translation),
                StateScoped(GameStates::Playing),
            ));
        } else {
            for hex in hexmap.axial_ring(IVec2::ZERO, **ring) {
                let coord = hexmap.axial_to_pixel(hex);
                let translation = coord.extend(0.0).xzy();

                commands.spawn((
                    Name::new("HexTile"),
                    HexTile,
                    HexTileKind::random(&mut *rng),
                    Visibility::default(),
                    Transform::from_translation(translation),
                    StateScoped(GameStates::Playing),
                ));
            }
        }

        **ring += 1;
        **points -= 1;
    }
}

fn update_camera_zoom(ring: Res<HexMapRing>, mut viewport_height: ResMut<ViewportHeight>) {
    **viewport_height = 6.0 + 2.0 * **ring as f32;
}

fn main() {
    let mut app = App::new();
    app.add_plugins(MainPlugin);
    app.run();
}
