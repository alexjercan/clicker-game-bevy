use bevy::prelude::*;
use clicker_assets::{GameState, UiAssets, BACKGROUND_DARK_COLOR};
use clicker_gameplay::{GameplaySystems, SkillPoints, XpMax, XpValue};

const HIGHLIGHT_COLOR: Color = Color::srgb(0.0, 0.5, 0.0);

#[derive(Component, Debug, Clone)]
struct XpBarFill;

#[derive(Component, Debug, Clone)]
struct SkillPointsNotification;

pub struct ClickerUiPlugin;

impl Plugin for ClickerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_ui)
            .add_systems(
                Update,
                (
                    update_xp_bar_fill.run_if(resource_changed::<XpValue>),
                    update_skill_points_notification.run_if(resource_changed::<SkillPoints>),
                )
                    .after(GameplaySystems::Progression)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup_ui(mut commands: Commands, ui_assets: Res<UiAssets>) {
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
                                        XpBarFill,
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

fn update_xp_bar_fill(
    xp_value: Res<XpValue>,
    xp_max: Res<XpMax>,
    mut xp_bar_fill: Query<&mut Node, With<XpBarFill>>,
) {
    if let Ok(mut xp_bar_fill) = xp_bar_fill.single_mut() {
        let xp_percent = **xp_value as f32 / **xp_max as f32;
        xp_bar_fill.width = Val::Percent(xp_percent * 100.0);
    }
}

fn update_skill_points_notification(
    skill_points: Res<SkillPoints>,
    mut notifications: Query<&mut Visibility, With<SkillPointsNotification>>,
) {
    if let Ok(mut visibility) = notifications.single_mut() {
        *visibility = if skill_points.0 > 0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
