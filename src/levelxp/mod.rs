//! Level XP Plugin for the game

use bevy::prelude::*;

mod ui;

pub use ui::*;

/// The player's and experience points.
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct LevelXP(pub u32);

/// The player's and next level experience points.
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct NextLevelXP(pub u32);

/// Level up event.
#[derive(Event, Debug)]
pub struct LevelUpEvent(pub Entity);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelXPPluginSet;

pub struct LevelXPPlugin;

impl Plugin for LevelXPPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelUpEvent>();

        app.add_systems(Update, player_level_up.in_set(LevelXPPluginSet));
    }
}

fn player_level_up(
    mut ev_level_up: EventWriter<LevelUpEvent>,
    q_player: Query<(Entity, &LevelXP, &NextLevelXP)>,
) {
    for (entity, level, next_level) in &q_player {
        if **level >= **next_level {
            ev_level_up.send(LevelUpEvent(entity));
        }
    }
}
