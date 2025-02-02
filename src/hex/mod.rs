//! The hex module. This module will contain the game's hex logic and systems.

use bevy::prelude::*;

pub mod render;
pub mod tweening;

use hexmap::HexMap;
pub use render::*;
pub use tweening::*;

use rand::Rng;

/// The hex tile component. Used to indicate that this entity is a hex tile.
#[derive(Component, Debug, Default)]
pub struct HexTile;

/// The hex tile axial component. Used to indicate the axial position of this hex tile.
#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct HexTileAxial(pub IVec2);

/// The hex tile selected component. Used to indicate that this entity is a selected hex tile.
#[derive(Component, Debug, Default)]
pub struct HexTileSelected;

#[derive(Event, Debug)]
pub struct HexTileClickSelected;

/// The hex tile kind component. Used to indicate the kind of hex tile.
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

#[derive(Resource, Default, Deref, DerefMut)]
pub struct HexMapResource(HexMap);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexPluginSet;

pub struct HexPlugin;

impl Plugin for HexPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HexTileClickSelected>();

        app.insert_resource(HexMapResource(HexMap::new(2.0 / 3.0f32.sqrt())));

        app.add_systems(Update, setup_hex.in_set(HexPluginSet));
    }
}

fn setup_hex(
    mut commands: Commands,
    q_hex: Query<(Entity, &Transform), (With<HexTile>, Without<HexTileAxial>)>,
    hexmap: Res<HexMapResource>,
) {
    for (entity, transform) in &q_hex {
        let axial = hexmap.pixel_to_axial(transform.translation.xz());
        commands.entity(entity).insert(HexTileAxial(axial));
    }
}
