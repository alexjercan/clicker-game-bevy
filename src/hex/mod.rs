use bevy::prelude::*;

pub mod render;

pub use render::*;

#[derive(Component, Debug, Default)]
pub struct HexTile;

#[derive(Component, Debug)]
pub enum HexTileKind {
    Tree,
    Stone,
}
