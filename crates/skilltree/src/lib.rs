//! SkillTree XP Plugin for the game

use bevy::prelude::*;

mod ui;

pub use ui::*;

/// The player's experience points.
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct LevelXP(pub u32);

/// The player's next level experience points.
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct NextLevelXP(pub u32);

/// The player's number of skill points. One skill point is added on level up
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct SkillTreePoints(pub u32);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SkillTreePluginSet;

pub struct SkillTreePlugin;

impl Plugin for SkillTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SkillTreeUIPlugin);

        app.add_systems(Update, handle_level_up_player.in_set(SkillTreePluginSet));

        app.configure_sets(Update, SkillTreeUIPluginSet.in_set(SkillTreePluginSet));
    }
}

fn handle_level_up_player(
    mut q_skill_tree: Query<(&mut SkillTreePoints, &mut LevelXP, &mut NextLevelXP)>,
) {
    for (mut skill, mut level, mut next_level) in &mut q_skill_tree {
        if **level >= **next_level {
            **level = **level - **next_level;
            **skill += 1;
            **next_level = **next_level + 10;
        }
    }
}
