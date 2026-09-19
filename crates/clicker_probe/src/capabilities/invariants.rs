use std::collections::HashSet;

use bevy::prelude::*;
use clicker_gameplay::tile_is_in_world;

use super::snapshot::game_snapshot;

pub struct InvariantsPlugin;

pub fn invariants() -> InvariantsPlugin {
    InvariantsPlugin
}

impl Plugin for InvariantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, check_invariants);
    }
}

fn check_invariants(world: &mut World) {
    let snapshot = game_snapshot(world);
    assert!(snapshot.xp < snapshot.xp_max);
    let mut occupied = HashSet::new();
    for coord in snapshot
        .tiles
        .iter()
        .map(|tile| tile.coord)
        .chain(snapshot.ghosts.iter().copied())
    {
        assert!(tile_is_in_world(coord));
        assert!(occupied.insert(coord));
    }
}
