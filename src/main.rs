#[cfg(feature = "debug")]
use debug::*;

use bevy::{asset::AssetMetaCheck, prelude::*, render::camera::ScalingMode, window::WindowMode};
use bevy_asset_loader::prelude::*;

const BACKGROUND_DARK_COLOR: Color = Color::srgb(0.65, 0.65, 0.65);
const HIGHLIGHT_COLOR: Color = Color::srgb(0.0, 0.5, 0.0);

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameStates {
    #[default]
    AssetLoading,
    Playing,
}

#[derive(AssetCollection, Resource)]
struct UIAssets {
    #[asset(path = "undefined.png")]
    pub skill_point: Handle<Image>,
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

#[derive(Resource, Debug, Default, Deref, DerefMut)]
struct XPValue(pub u32);

#[derive(Resource, Debug, Default, Deref, DerefMut)]
struct XPMax(pub u32);

#[derive(Resource, Debug, Default, Deref, DerefMut)]
struct SkillPoints(pub u32);

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

    app.init_state::<GameStates>();
    app.enable_state_scoped_entities::<GameStates>();

    app.add_loading_state(
        LoadingState::new(GameStates::AssetLoading)
            .continue_to_state(GameStates::Playing)
            .load_collection::<UIAssets>()
            .load_collection::<GameAssets>(),
    );

    app.init_resource::<SkillPoints>();
    app.init_resource::<XPValue>();
    app.insert_resource(XPMax(10));

    app.add_systems(OnEnter(GameStates::AssetLoading), setup_asset_loading);
    app.add_systems(OnEnter(GameStates::Playing), setup_playing);
    app.add_systems(Update, update.run_if(in_state(GameStates::Playing)));

    app.run();
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

fn update(
    mut commands: Commands,
    mut skill_points: ResMut<SkillPoints>,
    mut xp_value: ResMut<XPValue>,
    mut xp_max: ResMut<XPMax>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut q_xp_bar_fill: Query<&mut Node, With<XPBarFill>>,
    mut q_skill_points_notification: Query<&mut Visibility, With<SkillPointsNotification>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        **xp_value += 1;
    }

    if **xp_value >= **xp_max {
        **xp_value = **xp_value - **xp_max;
        **skill_points = **skill_points + 1;
        **xp_max = **xp_max + 10;
    }

    if let Ok(mut xp_bar_fill) = q_xp_bar_fill.get_single_mut() {
        let xp_percent = **xp_value as f32 / **xp_max as f32;
        xp_bar_fill.width = Val::Percent(xp_percent * 100.0);
    }

    if let Ok(mut visibility) = q_skill_points_notification.get_single_mut() {
        if skill_points.0 > 0 {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
