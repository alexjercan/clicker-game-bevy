#[cfg(feature = "debug")]
use debug::*;

use bevy::{asset::AssetMetaCheck, prelude::*};
use fill_bar::*;
use select_menu::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum ClickStates {
    /// The default state where the player clicks to increase the XP
    #[default]
    Click,
    /// Once the player reaches max XP (when the event is triggered) we switch to this state to
    /// indicate that click will enable the level up menu
    LevelUp,
    /// When the level up menu is open we want clicks to only iterract with that
    LevelUpMenu,
}

#[derive(Component, Debug, Default)]
struct UIRoot;

struct DemoPlugin;

impl Plugin for DemoPlugin {
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

        app.add_plugins(SelectMenuPlugin);
        app.add_plugins(FillBarPlugin);

        #[cfg(feature = "debug")]
        app.add_plugins(DebugPlugin);

        app.init_state::<ClickStates>();
        app.enable_state_scoped_entities::<ClickStates>();

        app.add_systems(Startup, setup);

        app.add_systems(
            Update,
            (handle_clicked_increase_xp, handle_xp_bar_reached_max)
                .run_if(in_state(ClickStates::Click)),
        );
        app.add_systems(
            Update,
            handle_clicked_level_up.run_if(in_state(ClickStates::LevelUp)),
        );

        app.add_systems(OnEnter(ClickStates::LevelUpMenu), setup_level_up_menu_ui);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera3D"),
        Camera3d::default(),
        Transform::from_xyz(-15.0, 15.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let xp = commands
        .spawn((
            Name::new("TestingLevelXP"),
            FillBarValue(0),
            FillBarMaxValue(10),
        ))
        .id();

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
            UIRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("XPBar"),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
                FillBarUI {
                    fill_color: Color::srgb(0.0, 0.5, 0.0),
                    ..default()
                },
                FillBarUITrack(xp),
            ));
        });
}

fn handle_clicked_increase_xp(
    mut q_xp: Query<&mut FillBarValue, With<FillBarMaxValue>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for mut xp in q_xp.iter_mut() {
            **xp += 1;
            println!("XP increased to {}", **xp);
        }
    }
}

fn handle_xp_bar_reached_max(
    mut next_state: ResMut<NextState<ClickStates>>,
    mut ev_xp_bar_reached_max: EventReader<FillBarValueReachedMax>,
) {
    for FillBarValueReachedMax(_) in ev_xp_bar_reached_max.read() {
        next_state.set(ClickStates::LevelUp);
        println!("XP bar reached max");
    }
}

fn handle_clicked_level_up(
    mut next_state: ResMut<NextState<ClickStates>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        println!("Level up clicked");
        next_state.set(ClickStates::LevelUpMenu);
    }
}

fn setup_level_up_menu_ui(mut commands: Commands, q_root: Query<Entity, With<UIRoot>>) {
    let Ok(root) = q_root.get_single() else {
        return;
    };

    commands.entity(root).with_children(|parent| {
        parent
            .spawn((
                Name::new("LevelUpMenuUI_Root"),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                StateScoped(ClickStates::LevelUpMenu),
            ))
            .with_children(|parent| {
                // Text with "Level Up"
                parent
                    .spawn((
                        Name::new("LevelUpMenuUI_Title"),
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(15.0),
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                    ))
                    .with_child((
                        Name::new("LevelUpMenuUI_TitleText"),
                        Text::new("Level Up"),
                        TextFont {
                            font_size: 35.0,
                            ..default()
                        },
                        Label,
                    ));

                // Description with the level count
                parent
                    .spawn((
                        Name::new("LevelUpMenuUI_Description"),
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(25.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::FlexStart,
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(10.0),
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Name::new("LevelUpMenuUI_DescriptionIcon"),
                            Node {
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            // ImageNode::new(ui_assets.skill_tree_points.clone()),
                        ));
                        parent.spawn((
                            Name::new("LevelUpMenuUI_DescriptionText"),
                            Node {
                                height: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::FlexStart,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            Text::new("This is the description of the level"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            Label,
                        ));
                    });

                // Level Up Grid 2x2 with 4 available skills to improve
                parent
                    .spawn((
                        Name::new("LevelUpMenuUI_Grid"),
                        Node {
                            display: Display::Grid,
                            width: Val::Percent(100.0),
                            flex_grow: 1.0,
                            grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
                            grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
                            row_gap: Val::Px(12.0),
                            column_gap: Val::Px(12.0),
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                        // SkillTreeUIGrid,
                    ))
                    .with_children(|parent| {
                        for (_, skill) in q_skills.iter().take(4).enumerate() {
                            parent
                                .spawn((
                                    Name::new("LevelUpMenuUI_Skill"),
                                    Node {
                                        display: Display::Grid,
                                        padding: UiRect::all(Val::Px(3.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::BLACK.into()),
                                ))
                                .with_children(|parent| {
                                    parent
                                        .spawn((
                                            Name::new("LevelUpMenuUI_SkillButton"),
                                            Button,
                                            Node {
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(100.0),
                                                flex_direction: FlexDirection::Row,
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Center,
                                                ..default()
                                            },
                                            BackgroundColor(NORMAL_BUTTON),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                Name::new("LevelUpMenuUI_SkillImage"),
                                                Node {
                                                    width: Val::Px(50.0),
                                                    height: Val::Px(50.0),
                                                    ..default()
                                                },
                                                ImageNode::new(skill.icon.clone()),
                                            ));
                                            parent.spawn((
                                                Name::new("LevelUpMenuUI_SkillText"),
                                                Text::new(skill.name.clone()),
                                                TextFont {
                                                    font_size: 20.0,
                                                    ..default()
                                                },
                                                Label,
                                            ));
                                        });
                                });
                        }
                    });
            });
    });
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DemoPlugin);
    app.run();
}
