//! SkillTree XP Plugin for the game

use bevy::prelude::*;

mod ui_skill;
mod ui_xp;

pub use ui_skill::*;
pub use ui_xp::*;

/// The player's experience points.
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct LevelXP(pub u32);

/// The player's next level experience points.
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct NextLevelXP(pub u32);

/// The player's number of skill points. One skill point is added on level up
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct SkillTreePoints(pub u32);

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Skill<T> {
    pub name: String,
    pub description: String,
    pub icon: Handle<Image>,
    pub component: T,
}

/// This resource is used to store the available skills that the player can learn from leveling up.
#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct AvailableSkills<T>(pub Vec<Skill<T>>);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SkillTreePluginSet;

#[derive(Debug, Default)]
pub struct SkillTreePlugin<T>
where
    T: Component + Clone + Copy + Default,
{
    _marker: std::marker::PhantomData<T>,
}

impl<T> Plugin for SkillTreePlugin<T>
where
    T: Component + Clone + Copy + Default,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(LevelXPBarUIPlugin);
        app.add_plugins(SkillTreeUIPlugin::<T>::default());

        app.init_resource::<AvailableSkills<T>>();

        app.add_systems(Update, handle_level_up_player.in_set(SkillTreePluginSet));

        app.configure_sets(Update, SkillTreeUIPluginSet.in_set(SkillTreePluginSet));
        app.configure_sets(Update, LevelXPBarUIPluginSet.in_set(SkillTreePluginSet));
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
