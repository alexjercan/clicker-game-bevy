//! UI Plugin for the Skill Tree XP

use bevy::prelude::*;

use super::{LevelXP, NextLevelXP, SkillTreePoints};

/// The assets used for showing UI
#[derive(Resource, Clone, Debug, Default)]
pub struct SkillTreeUIAssets {
    pub skill_tree_points: Handle<Image>,
}

/// Component used to indicate where the XP UI will spawn as a child.
#[derive(Component, Debug, Default)]
pub struct SkillTreeUIRoot;

/// Component used to indicate the fill of the XP bar.
#[derive(Component, Debug, Default)]
pub struct LevelXPBarFill;

/// Component used to indicate the number of skill points available to use.
#[derive(Component, Debug, Default)]
pub struct SkillTreePointsUI;

const LEVEL_XP_UI_BAR_WIDTH: f32 = 200.0;
const LEVEL_XP_UI_BAR_HEIGHT: f32 = 24.0;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SkillTreeUIPluginSet;

#[derive(Debug)]
pub struct SkillTreeUIPlugin;

impl Plugin for SkillTreeUIPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Can we have a better way to add assets?
        app.init_resource::<SkillTreeUIAssets>();

        app.add_systems(Update, setup_ui.in_set(SkillTreeUIPluginSet));

        app.add_systems(
            Update,
            (update_level_xp_ui, update_skill_tree_points_ui).in_set(SkillTreeUIPluginSet),
        );
    }
}

fn setup_ui(
    mut commands: Commands,
    q_root: Query<Entity, With<SkillTreeUIRoot>>,
    ui_assets: Res<SkillTreeUIAssets>,
    mut has_run: Local<bool>,
) {
    let Ok(root) = q_root.get_single() else {
        return;
    };

    if !*has_run {
        *has_run = true;
    } else {
        return;
    }

    commands.entity(root).with_children(|parent| {
        parent
            .spawn((
                Name::new("LevelXPUI_Root"),
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Name::new("LevelXPUI_BarBackground"),
                        Node {
                            width: Val::Px(LEVEL_XP_UI_BAR_WIDTH),
                            height: Val::Px(LEVEL_XP_UI_BAR_HEIGHT),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Name::new("LevelXPUI_BarFill"),
                            Node {
                                width: Val::Percent(0.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.0, 0.5, 0.0)),
                            LevelXPBarFill,
                        ));
                    });
            });

        parent
            .spawn((
                Name::new("SkillTreePointsUI_Root"),
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Name::new("SkillTreePointsUI"),
                    Visibility::Hidden,
                    Node {
                        width: Val::Px(24.0),
                        height: Val::Px(24.0),
                        ..default()
                    },
                    ImageNode::new(ui_assets.skill_tree_points.clone()),
                    SkillTreePointsUI,
                ));
            });
    });
}

fn update_level_xp_ui(
    q_xp: Query<(&LevelXP, &NextLevelXP), Changed<LevelXP>>,
    mut q_fill: Query<(&mut Node, &mut BackgroundColor), With<LevelXPBarFill>>,
) {
    for (level, next_level) in &q_xp {
        let frac = **level as f32 / **next_level as f32;

        for (mut node, mut color) in &mut q_fill {
            node.width = Val::Px(LEVEL_XP_UI_BAR_WIDTH * frac);
            color.0 = Color::srgb(0.0, 0.5, 0.0);
        }
    }
}

fn update_skill_tree_points_ui(
    q_points: Query<&SkillTreePoints>,
    mut q_alerts: Query<&mut Visibility, With<SkillTreePointsUI>>,
) {
    let Ok(points) = q_points.get_single() else {
        return;
    };

    let new_visibility = if **points == 0 {
        Visibility::Hidden
    } else {
        Visibility::Visible
    };

    for mut visibility in &mut q_alerts {
        *visibility = new_visibility;
    }
}
