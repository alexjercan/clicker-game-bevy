use bevy::prelude::*;

pub mod render;
pub mod tweening;

pub use render::*;
pub use tweening::*;

use rand::Rng;

#[derive(Component, Debug, Default)]
pub struct HexTile;

#[derive(Component, Debug)]
pub enum HexTileKind {
    Empty,
    Tree,
    Stone,
    Wheat,
}

impl HexTileKind {
    pub fn random(rng: &mut impl Rng) -> Self {
        match rng.random_range(0..4) {
            0 => HexTileKind::Empty,
            1 => HexTileKind::Tree,
            2 => HexTileKind::Stone,
            _ => HexTileKind::Wheat,
        }
    }
}
