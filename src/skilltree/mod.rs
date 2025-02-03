//! Skilltree plugin for the game. This plugin will contain the game's skilltree logic and systems.

use bevy::prelude::*;

mod ui;

pub use ui::*;

use crate::levelxp::*;

/// The player's number of skill points.
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct SkillTreePoints(pub u32);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SkillTreePluginSet;

pub struct SkillTreePlugin;

impl Plugin for SkillTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_level_up_player.in_set(SkillTreePluginSet));
    }
}

fn handle_level_up_player(
    mut ev_level_up: EventReader<LevelUpEvent>,
    mut q_skill_tree: Query<&mut SkillTreePoints>,
) {
    for _ in ev_level_up.read() {
        for mut skill_tree_points in q_skill_tree.iter_mut() {
            **skill_tree_points += 1;
        }
    }
}
