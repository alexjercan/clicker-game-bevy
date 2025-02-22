#[cfg(feature = "debug")]
use debug::*;

use bevy::{asset::AssetMetaCheck, prelude::*, render::camera::ScalingMode, window::WindowMode};
use bevy_asset_loader::prelude::*;
use bevy_tweening::lens::*;
use bevy_tweening::*;
use hexmap::*;
use rand::prelude::*;

const BACKGROUND_DARK_COLOR: Color = Color::srgb(0.65, 0.65, 0.65);
const HIGHLIGHT_COLOR: Color = Color::srgb(0.0, 0.5, 0.0);

const WORLD_HALF_WIDTH: i32 = 4;
const WORLD_HALF_HEIGHT: i32 = 1;
const HEX_UP_SCALE: f32 = 1.1;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameStates {
    #[default]
    AssetLoading,
    Playing,
}

#[derive(AssetCollection, Resource)]
struct UIAssets {
    #[asset(path = "undefined.png")]
    skill_point: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
struct GameAssets {
    #[asset(path = "gltf/tiles/base/hex_grass.gltf")]
    hex_base: Handle<Gltf>,
    #[asset(path = "gltf/decoration/nature/trees_A_large.gltf")]
    hex_tree: Handle<Gltf>,
    #[asset(path = "gltf/decoration/nature/hills_A.gltf")]
    hex_stone: Handle<Gltf>,
    #[asset(path = "gltf/buildings/neutral/building_dirt.gltf")]
    hex_dirt: Handle<Gltf>,
    #[asset(path = "gltf/buildings/neutral/building_grain.gltf")]
    hex_wheat: Handle<Gltf>,
}

#[derive(Component, Debug, Clone)]
struct XPBarFill;

#[derive(Component, Debug, Clone)]
struct SkillPointsNotification;

#[derive(Component, Debug, Clone)]
struct HexTile;

#[derive(Component, Debug, Clone)]
struct HexGhost;

// TODO: Rename to HexTileCoord
#[derive(Component, Debug, Default, Deref, DerefMut)]
struct HexTileAxial(IVec2);

#[derive(Component, Debug)]
enum HexTileKind {
    Empty,
    Tree,
    Stone,
    Wheat,
}

impl HexTileKind {
    fn random(rng: &mut impl Rng) -> Self {
        match rng.random_range(0..4) {
            0 => HexTileKind::Empty,
            1 => HexTileKind::Tree,
            2 => HexTileKind::Stone,
            _ => HexTileKind::Wheat,
        }
    }
}

#[derive(Resource, Debug, Default, Deref, DerefMut)]
struct XPValue(u32);

#[derive(Resource, Debug, Default, Deref, DerefMut)]
struct XPMax(u32);

#[derive(Resource, Debug, Default, Deref, DerefMut)]
struct SkillPoints(u32);

#[derive(Resource, Deref, DerefMut)]
struct HexMapRng(StdRng);

#[derive(Resource, Default, Deref, DerefMut)]
struct HexMapResource(HexMap);

#[derive(Component, Debug, Clone)]
struct Selected;

#[derive(Event)]
struct SelectedEvent<T: Component> {
    entity: Entity,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Component> SelectedEvent<T> {
    fn new(entity: Entity) -> Self {
        Self {
            entity,
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Event)]
struct DeselectedEvent<T: Component> {
    entity: Entity,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Component> DeselectedEvent<T> {
    fn new(entity: Entity) -> Self {
        Self {
            entity,
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Event)]
struct ClickedSelectedEvent<T: Component> {
    entity: Entity,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Component> ClickedSelectedEvent<T> {
    fn new(entity: Entity) -> Self {
        Self {
            entity,
            _marker: std::marker::PhantomData,
        }
    }
}

fn main() {
    let mut app = App::new();

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
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    #[cfg(feature = "debug")]
    app.add_plugins(DebugPlugin);

    app.add_plugins(bevy_tweening::TweeningPlugin);

    app.add_event::<ClickedSelectedEvent<HexGhost>>();
    app.add_event::<ClickedSelectedEvent<HexTile>>();
    app.add_event::<SelectedEvent<HexTile>>();
    app.add_event::<DeselectedEvent<HexTile>>();

    app.init_state::<GameStates>();
    app.enable_state_scoped_entities::<GameStates>();

    app.add_loading_state(
        LoadingState::new(GameStates::AssetLoading)
            .continue_to_state(GameStates::Playing)
            .load_collection::<UIAssets>()
            .load_collection::<GameAssets>(),
    );

    app.init_resource::<XPValue>();
    app.insert_resource(SkillPoints(1));
    app.insert_resource(XPMax(3));
    app.insert_resource(HexMapRng(StdRng::from_os_rng()));
    app.insert_resource(HexMapResource(HexMap::new(2.0 / 3.0f32.sqrt())));

    app.add_systems(OnEnter(GameStates::AssetLoading), setup_asset_loading);
    app.add_systems(OnEnter(GameStates::Playing), setup_playing);
    app.add_systems(
        Update,
        (
            update_selected,
            selected_tile_tween,
            deselect_tile_tween,
            update_click_selected_hex::<HexTile>,
            update_click_selected_hex::<HexGhost>,
            clicked_tile_increase_xp,
            clicked_tile_tween,
            clicked_ghost_spawn,
            (update_xp_value, update_xp_bar_fill)
                .chain()
                .run_if(resource_changed::<XPValue>),
            update_skill_points_notification.run_if(resource_changed::<SkillPoints>),
        )
            .run_if(in_state(GameStates::Playing)),
    );

    app.run();
}

fn setup_asset_loading(mut commands: Commands) {
    commands.spawn((
        Name::new("CameraUI"),
        Camera2d::default(),
        StateScoped(GameStates::AssetLoading),
    ));
}

fn setup_playing(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    ui_assets: Res<UIAssets>,
    hexmap: Res<HexMapResource>,
) {
    commands.spawn((
        Name::new("Camera3D"),
        Camera3d::default(),
        Transform::from_xyz(-15.0, 15.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 6.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        StateScoped(GameStates::Playing),
    ));

    commands.spawn((
        Name::new("DirectionalLight"),
        DirectionalLight::default(),
        Transform::from_xyz(-1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        StateScoped(GameStates::Playing),
    ));

    let coord = hexmap.axial_to_pixel(IVec2::ZERO);
    let translation = coord.extend(0.0).xzy();

    commands.spawn((
        Name::new("HexGhost"),
        HexGhost,
        Visibility::default(),
        Transform::from_translation(translation),
        HexTileAxial(hexmap.pixel_to_axial(translation.xz())),
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
                justify_content: JustifyContent::Start,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("TopBar"),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(64.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(10.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Name::new("TopBar_XP"),
                            Node {
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Name::new("TopBar_XP_BG"),
                                    Node {
                                        width: Val::Px(200.0),
                                        height: Val::Px(24.0),
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::FlexStart,
                                        ..default()
                                    },
                                    BackgroundColor(BACKGROUND_DARK_COLOR),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Name::new("TopBar_XP_Fill"),
                                        Node {
                                            width: Val::Percent(0.0),
                                            height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        BackgroundColor(HIGHLIGHT_COLOR),
                                        XPBarFill,
                                    ));
                                });
                        });

                    parent
                        .spawn((
                            Name::new("TopBar_SkillPoints"),
                            Node {
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            Visibility::Visible,
                            SkillPointsNotification,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Name::new("TopBar_SkillPoints_Icon"),
                                Node {
                                    width: Val::Px(24.0),
                                    height: Val::Px(24.0),
                                    ..default()
                                },
                                ImageNode::new(ui_assets.skill_point.clone()),
                            ));
                        });
                });
        });
}

fn update_selected(
    mut commands: Commands,
    hexmap: Res<HexMapResource>,
    windows: Query<&Window>,
    q_hex: Query<(Entity, &HexTileAxial), Without<Selected>>,
    q_selected: Query<(Entity, &HexTileAxial), With<Selected>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut ev_selected: EventWriter<SelectedEvent<HexTile>>,
    mut ev_deselected: EventWriter<DeselectedEvent<HexTile>>,
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

    let position = point.xz();

    let cursor_coord = hexmap.pixel_to_axial(position);

    if let Ok((entity, hex_coord)) = q_selected.get_single() {
        if **hex_coord != cursor_coord {
            commands.entity(entity).remove::<Selected>();
            ev_deselected.send(DeselectedEvent::new(entity));
        } else {
            return;
        }
    }

    if let Some((entity, _)) = q_hex
        .iter()
        .find(|(_, HexTileAxial(hex_coord))| *hex_coord == cursor_coord)
    {
        commands.entity(entity).insert(Selected);
        ev_selected.send(SelectedEvent::new(entity));
    }
}

fn selected_tile_tween(
    mut commands: Commands,
    mut ev_selected: EventReader<SelectedEvent<HexTile>>,
) {
    for event in ev_selected.read() {
        let tween_scale = Tween::new(
            EaseFunction::QuadraticOut,
            std::time::Duration::from_millis(100),
            TransformScaleLens {
                start: Vec3::splat(1.0),
                end: Vec3::splat(HEX_UP_SCALE),
            },
        );

        let track = Tracks::new([tween_scale]);

        commands.entity(event.entity).insert(Animator::new(track));
    }
}

fn deselect_tile_tween(
    mut commands: Commands,
    mut ev_deselected: EventReader<DeselectedEvent<HexTile>>,
) {
    for event in ev_deselected.read() {
        let tween_scale = Tween::new(
            EaseFunction::QuadraticOut,
            std::time::Duration::from_millis(100),
            TransformScaleLens {
                start: Vec3::splat(HEX_UP_SCALE),
                end: Vec3::splat(1.0),
            },
        );

        let track = Tracks::new([tween_scale]);

        commands.entity(event.entity).insert(Animator::new(track));
    }
}

fn update_click_selected_hex<T: Component>(
    buttons: Res<ButtonInput<MouseButton>>,
    q_selected: Query<Entity, (With<Selected>, With<T>)>,
    mut ev_clicked: EventWriter<ClickedSelectedEvent<T>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Ok(entity) = q_selected.get_single() {
            ev_clicked.send(ClickedSelectedEvent::new(entity));
        }
    }
}

fn clicked_tile_increase_xp(
    mut ev_clicked: EventReader<ClickedSelectedEvent<HexTile>>,
    mut xp_value: ResMut<XPValue>,
) {
    for _ in ev_clicked.read() {
        **xp_value += 1;
    }
}

fn clicked_tile_tween(
    mut commands: Commands,
    mut ev_clicked: EventReader<ClickedSelectedEvent<HexTile>>,
) {
    for event in ev_clicked.read() {
        let tween_scale = Tween::new(
            EaseFunction::QuadraticOut,
            std::time::Duration::from_millis(100),
            TransformScaleLens {
                start: Vec3::splat(HEX_UP_SCALE),
                end: Vec3::splat(1.0),
            },
        )
        .with_repeat_count(RepeatCount::Finite(2))
        .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

        let track = Tracks::new([tween_scale]);

        commands.entity(event.entity).insert(Animator::new(track));
    }
}

fn clicked_ghost_spawn(
    hexmap: Res<HexMapResource>,
    mut commands: Commands,
    mut skill_points: ResMut<SkillPoints>,
    mut ev_clicked: EventReader<ClickedSelectedEvent<HexGhost>>,
    q_ghost: Query<(Entity, &HexTileAxial), With<HexGhost>>,
    q_tiles: Query<(Entity, &HexTileAxial), With<HexTile>>,
    game_assets: Res<GameAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    mut rng: ResMut<HexMapRng>,
) {
    for event in ev_clicked.read() {
        if let Ok((entity, hex_coord)) = q_ghost.get(event.entity) {
            if **skill_points > 0 {
                commands.entity(entity).despawn_recursive();

                let translation = hexmap.axial_to_pixel(**hex_coord).extend(0.0).xzy();

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

                let track = Tracks::new([tween_move, tween_scale]);

                commands
                    .spawn((
                        Name::new("HexTile"),
                        HexTile,
                        Visibility::default(),
                        Transform::from_translation(translation),
                        HexTileAxial(**hex_coord),
                        StateScoped(GameStates::Playing),
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn((
                                Name::new("HexTileRender"),
                                Transform::from_xyz(0.0, -5.0, 0.0),
                                Animator::new(track),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Name::new("HexTileMesh"),
                                    Transform::from_xyz(0.0, 0.0, 0.0),
                                    GlobalTransform::default(),
                                    SceneRoot(
                                        gltf_assets.get(&game_assets.hex_base).unwrap().scenes[0]
                                            .clone(),
                                    ),
                                ));

                                match HexTileKind::random(&mut *rng) {
                                    HexTileKind::Empty => {}
                                    HexTileKind::Tree => {
                                        parent.spawn((
                                            Name::new("HexTreeMesh"),
                                            Transform::from_xyz(0.0, 0.0, 0.0),
                                            GlobalTransform::default(),
                                            SceneRoot(
                                                gltf_assets
                                                    .get(&game_assets.hex_tree)
                                                    .unwrap()
                                                    .scenes[0]
                                                    .clone(),
                                            ),
                                        ));
                                    }
                                    HexTileKind::Stone => {
                                        parent.spawn((
                                            Name::new("HexStoneMesh"),
                                            Transform::from_xyz(0.0, 0.0, 0.0),
                                            GlobalTransform::default(),
                                            SceneRoot(
                                                gltf_assets
                                                    .get(&game_assets.hex_stone)
                                                    .unwrap()
                                                    .scenes[0]
                                                    .clone(),
                                            ),
                                        ));
                                    }
                                    HexTileKind::Wheat => {
                                        parent.spawn((
                                            Name::new("HexDirtMesh"),
                                            Transform::from_xyz(0.0, 0.0, 0.0),
                                            GlobalTransform::default(),
                                            SceneRoot(
                                                gltf_assets
                                                    .get(&game_assets.hex_dirt)
                                                    .unwrap()
                                                    .scenes[0]
                                                    .clone(),
                                            ),
                                        ));
                                        parent.spawn((
                                            Name::new("HexWheatMesh"),
                                            Transform::from_xyz(0.0, 0.0, 0.0),
                                            GlobalTransform::default(),
                                            SceneRoot(
                                                gltf_assets
                                                    .get(&game_assets.hex_wheat)
                                                    .unwrap()
                                                    .scenes[0]
                                                    .clone(),
                                            ),
                                        ));
                                    }
                                }
                            });
                    });

                hexmap
                    .axial_ring(**hex_coord, 1)
                    .iter()
                    .filter(|hex_coord| {
                        q_ghost
                            .iter()
                            .all(|(_, HexTileAxial(ghost_coord))| *ghost_coord != **hex_coord)
                            && q_tiles
                                .iter()
                                .all(|(_, HexTileAxial(tile_coord))| *tile_coord != **hex_coord)
                    })
                    // .filter(|coord| {
                    //     coord.x.abs() <= WORLD_HALF_WIDTH && coord.y.abs() <= WORLD_HALF_HEIGHT
                    // })
                    .for_each(|coord| {
                        let translation = hexmap.axial_to_pixel(*coord).extend(0.0).xzy();

                        commands.spawn((
                            Name::new("HexGhost"),
                            HexGhost,
                            Visibility::default(),
                            Transform::from_translation(translation),
                            HexTileAxial(*coord),
                            StateScoped(GameStates::Playing),
                        ));
                    });

                **skill_points -= 1;
            }
        }
    }
}

fn update_xp_value(
    mut xp_value: ResMut<XPValue>,
    mut xp_max: ResMut<XPMax>,
    mut skill_points: ResMut<SkillPoints>,
) {
    if **xp_value >= **xp_max {
        **xp_value = **xp_value - **xp_max;
        **skill_points = **skill_points + 1;
        **xp_max = **xp_max;// + 10;
    }
}

fn update_xp_bar_fill(
    xp_value: Res<XPValue>,
    xp_max: Res<XPMax>,
    mut q_xp_bar_fill: Query<&mut Node, With<XPBarFill>>,
) {
    if let Ok(mut xp_bar_fill) = q_xp_bar_fill.get_single_mut() {
        let xp_percent = **xp_value as f32 / **xp_max as f32;
        xp_bar_fill.width = Val::Percent(xp_percent * 100.0);
    }
}

fn update_skill_points_notification(
    skill_points: Res<SkillPoints>,
    mut q_skill_points_notification: Query<&mut Visibility, With<SkillPointsNotification>>,
) {
    if let Ok(mut visibility) = q_skill_points_notification.get_single_mut() {
        if skill_points.0 > 0 {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
