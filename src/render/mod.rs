use bevy::prelude::*;

use crate::{core::*, hex::*};

pub(crate) struct RenderPlugin;

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
        hex_grass: game_assets.hex_grass.clone(),
        trees_a_large: game_assets.trees_a_large.clone(),
    });
}
