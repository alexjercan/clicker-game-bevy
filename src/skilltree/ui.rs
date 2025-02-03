//! UI Plugin for the Skill tree in the game.

use bevy::prelude::*;

use crate::core::*;

use super::SkillTreePoints;

/// Component used to indicate where the skilltree UI will spawn as a child.
/// This UI component will tell you if you have skill points available to use
#[derive(Component, Debug, Default)]
pub struct SkillTreePointsUI;

/// Component used to indicate the number of skill points available to use.
#[derive(Component, Debug, Default)]
pub struct SkillTreePointsAlertUI;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SkillTreeUIPluginSet;

#[derive(Debug)]
pub struct SkillTreeUIPlugin;

impl Plugin for SkillTreeUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            add_skill_tree_alerts_ui.in_set(SkillTreeUIPluginSet),
        );

        app.add_systems(
            Update,
            update_skill_tree_alerts_ui.in_set(SkillTreeUIPluginSet),
        );
    }
}

fn update_skill_tree_alerts_ui(
    q_points: Query<&SkillTreePoints>,
    mut q_alerts: Query<&mut Visibility, With<SkillTreePointsAlertUI>>,
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

fn add_skill_tree_alerts_ui(
    mut commands: Commands,
    q_root: Query<Entity, With<SkillTreePointsUI>>,
    ui_assets: Res<UIAssets>,
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
                Name::new("SkillTreePointsUI_Root"),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_items: JustifyItems::Center,
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Name::new("SkillTreePointsAlertUI"),
                    Visibility::Hidden,
                    Node {
                        width: Val::Px(24.0),
                        height: Val::Px(24.0),
                        ..default()
                    },
                    ImageNode::new(ui_assets.skill_tree_points_alert.clone()),
                    SkillTreePointsAlertUI,
                ));
            });
    });
}
