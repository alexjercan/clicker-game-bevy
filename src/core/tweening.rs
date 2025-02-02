//! Tweening plugin for the game.

use bevy::prelude::*;

use crate::core::*;

pub(super) struct TweeningPlugin;

impl Plugin for TweeningPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_tweening::TweeningPlugin);
        app.add_plugins(HexTweeningPlugin);

        app.configure_sets(
            PostUpdate,
            HexTweeningPluginSet.run_if(in_state(GameStates::Playing)),
        );
    }
}
