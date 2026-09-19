mod progression;

use bevy::prelude::*;
use clicker_state::GameState;
use clicker_tile::{ClickerTilePlugin, TileSystems};

pub use clicker_tile::{
    tile_is_in_world, ClickTile, GhostClicked, HexCoord, HexGhost, HexMap, HexTile,
    InitializeTileWorld, PlaceTile, TileClicked, TileCoord, TileDeselected, TileKind, TilePlaced,
    TilePointer, TileSelected, TileSettled, SEED_ENV,
};
pub use progression::{SkillPoints, XpMax, XpValue};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameplaySystems {
    Progression,
}

pub struct ClickerGameplayPlugin;

impl Plugin for ClickerGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ClickerTilePlugin)
            .configure_sets(
                Update,
                GameplaySystems::Progression
                    .after(TileSystems::Action)
                    .before(TileSystems::Mutation)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_plugins(progression::ProgressionPlugin);
    }
}
