//! UI Plugin for the Level XP in the game.

use bevy::prelude::*;

use super::{LevelXP, NextLevelXP};

/// Component used to indicate where the XP UI will spawn as a child.
#[derive(Component, Debug, Default)]
pub struct LevelXPUI;

/// Component used to indicate the fill of the XP bar.
#[derive(Component, Debug, Default)]
pub struct LevelXPBarFill;

const LEVEL_XP_UI_BAR_WIDTH: f32 = 200.0;
const LEVEL_XP_UI_BAR_HEIGHT: f32 = 24.0;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelXPUIPluginSet;

#[derive(Debug)]
pub struct LevelXPUIPlugin;

impl Plugin for LevelXPUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_level_xp_ui.in_set(LevelXPUIPluginSet));

        app.add_systems(Update, update_level_xp_ui.in_set(LevelXPUIPluginSet));
    }
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

fn add_level_xp_ui(
    mut commands: Commands,
    q_root: Query<Entity, With<LevelXPUI>>,
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
                Name::new("LevelXPUI_BarRoot"),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_items: JustifyItems::Center,
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
    });
}
