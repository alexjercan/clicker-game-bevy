mod interaction;
mod progression;
mod world;

use bevy::prelude::*;
use bevy_rand::prelude::*;
use clicker_assets::GameState;

pub use progression::{SkillPoints, XpMax, XpValue};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameplaySystems {
    Selection,
    SelectionFeedback,
    Click,
    ClickFeedback,
    Progression,
}

pub struct ClickerGameplayPlugin;

impl Plugin for ClickerGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EntropyPlugin::<WyRand>::default())
            .configure_sets(
                Update,
                (
                    GameplaySystems::Selection,
                    GameplaySystems::SelectionFeedback,
                    GameplaySystems::Click,
                    GameplaySystems::ClickFeedback,
                    GameplaySystems::Progression,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_plugins((
                world::WorldPlugin,
                interaction::InteractionPlugin,
                progression::ProgressionPlugin,
            ));
    }
}
