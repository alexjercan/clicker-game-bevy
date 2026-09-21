use bevy::prelude::*;
use clicker_animation::AnimationsEnabled;
use clicker_state::GameState;
use clicker_tile::{HexGhost, HexTile, InitializeTileWorld, TilePlaced, TileSettled, TileSystems};

type TileEntity = Or<(With<HexGhost>, With<HexTile>)>;

pub(crate) struct TileGluePlugin;

impl Plugin for TileGluePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), initialize_tile_world)
            .add_systems(OnExit(GameState::Playing), cleanup_tile_world);
        // NOTE: Animation enablement is read once here, so the rendered path always
        // NOTE: settles tiles through the spawn animation. A runtime animation setting
        // NOTE: would need a run condition or a runtime-gated settlement path instead.
        if !app.world().contains_resource::<AnimationsEnabled>() {
            app.add_systems(Update, settle_placed_tiles.after(TileSystems::Mutation));
        }
    }
}

fn initialize_tile_world(mut events: MessageWriter<InitializeTileWorld>) {
    events.write(InitializeTileWorld);
}

fn cleanup_tile_world(mut commands: Commands, entities: Query<Entity, TileEntity>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn settle_placed_tiles(
    mut placed: MessageReader<TilePlaced>,
    mut settled: MessageWriter<TileSettled>,
) {
    for event in placed.read() {
        settled.write(TileSettled { coord: event.coord });
    }
}
