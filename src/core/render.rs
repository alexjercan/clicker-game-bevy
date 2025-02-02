//! Rendering plugin for the game.

use bevy::prelude::*;

use crate::core::*;

pub(super) struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HexRenderPlugin);

        app.configure_sets(
            Update,
            HexRenderPluginSet.run_if(in_state(GameStates::Playing)),
        );

        app.add_systems(OnEnter(GameStates::Playing), setup_hex_render);
    }
}

fn setup_hex_render(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.insert_resource(HexRenderAssets {
        base: game_assets.hex_base.clone(),
        tree: game_assets.hex_tree.clone(),
        stone: game_assets.hex_stone.clone(),
        dirt: game_assets.hex_dirt.clone(),
        wheat: game_assets.hex_wheat.clone(),
    });
}
