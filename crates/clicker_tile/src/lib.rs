mod interaction;
mod world;

use bevy::prelude::*;
use bevy_rand::prelude::*;

pub use world::{tile_is_in_world, HexCoord, HexGhost, HexMap, HexTile};

pub const SEED_ENV: &str = "CLICKER_SEED";

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileSystems {
    Selection,
    Action,
    Mutation,
    Feedback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileCoord {
    q: i32,
    r: i32,
}

impl TileCoord {
    pub const ZERO: Self = Self::new(0, 0);

    pub const fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    pub const fn q(self) -> i32 {
        self.q
    }

    pub const fn r(self) -> i32 {
        self.r
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    Empty,
    Tree,
    Stone,
    Wheat,
}

#[derive(Resource, Debug, Default, Clone, Copy, PartialEq)]
pub struct TilePointer {
    pub world_position: Option<Vec2>,
    pub activate: bool,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitializeTileWorld;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClickTile(pub TileCoord);

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileSelected {
    pub entity: Entity,
    pub coord: TileCoord,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileDeselected {
    pub entity: Entity,
    pub coord: TileCoord,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileClicked {
    pub entity: Entity,
    pub coord: TileCoord,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GhostClicked {
    pub entity: Entity,
    pub coord: TileCoord,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaceTile(pub Entity);

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TilePlaced {
    pub coord: TileCoord,
    pub kind: TileKind,
}

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileSettled {
    pub coord: TileCoord,
}

pub struct ClickerTilePlugin;

impl Plugin for ClickerTilePlugin {
    fn build(&self, app: &mut App) {
        match seed_from_env() {
            Some(seed) => app.add_plugins(EntropyPlugin::<WyRand>::with_seed(seed.to_le_bytes())),
            None => app.add_plugins(EntropyPlugin::<WyRand>::default()),
        };

        app.configure_sets(
            Update,
            (
                TileSystems::Selection,
                TileSystems::Action,
                TileSystems::Mutation,
                TileSystems::Feedback,
            )
                .chain(),
        )
        .add_plugins((world::WorldPlugin, interaction::InteractionPlugin));
    }
}

pub fn seed_from_env() -> Option<u64> {
    let raw = std::env::var(SEED_ENV).ok()?;
    Some(
        raw.parse()
            .unwrap_or_else(|error| panic!("{SEED_ENV}={raw:?} is not a u64 seed ({error})")),
    )
}
