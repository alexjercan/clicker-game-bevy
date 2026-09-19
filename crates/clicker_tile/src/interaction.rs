use bevy::prelude::*;

use crate::world::HexMap;
use crate::{
    ClickTile, GhostClicked, HexCoord, HexGhost, HexTile, TileClicked, TileDeselected, TilePointer,
    TileSelected, TileSystems,
};

#[derive(Component)]
struct Selected;

pub(crate) struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TilePointer>()
            .add_message::<ClickTile>()
            .add_message::<TileClicked>()
            .add_message::<GhostClicked>()
            .add_message::<TileSelected>()
            .add_message::<TileDeselected>()
            .add_systems(Update, update_selected.in_set(TileSystems::Selection))
            .add_systems(
                Update,
                (click_selected, resolve_click)
                    .chain()
                    .in_set(TileSystems::Action),
            );
    }
}

#[derive(bevy::ecs::system::SystemParam)]
struct SelectionEvents<'w> {
    selected: MessageWriter<'w, TileSelected>,
    deselected: MessageWriter<'w, TileDeselected>,
}

fn update_selected(
    mut commands: Commands,
    hexmap: Res<HexMap>,
    pointer: Res<TilePointer>,
    hexes: Query<(Entity, &HexCoord), Without<Selected>>,
    selected: Query<(Entity, &HexCoord), With<Selected>>,
    mut events: SelectionEvents,
) {
    let pointer_coord = pointer
        .world_position
        .map(|world_position| hexmap.tile_at(world_position));

    if let Ok((entity, coord)) = selected.single() {
        if Some(coord.tile_coord()) == pointer_coord {
            return;
        }
        commands.entity(entity).remove::<Selected>();
        events.deselected.write(TileDeselected {
            entity,
            coord: coord.tile_coord(),
        });
    }

    let Some(pointer_coord) = pointer_coord else {
        return;
    };
    if let Some((entity, coord)) = hexes
        .iter()
        .find(|(_, coord)| coord.tile_coord() == pointer_coord)
    {
        commands.entity(entity).insert(Selected);
        events.selected.write(TileSelected {
            entity,
            coord: coord.tile_coord(),
        });
    }
}

fn click_selected(
    pointer: Res<TilePointer>,
    selected: Query<&HexCoord, With<Selected>>,
    mut clicks: MessageWriter<ClickTile>,
) {
    if pointer.activate {
        if let Ok(coord) = selected.single() {
            clicks.write(ClickTile(coord.tile_coord()));
        }
    }
}

fn resolve_click(
    mut clicks: MessageReader<ClickTile>,
    ghosts: Query<(Entity, &HexCoord), With<HexGhost>>,
    tiles: Query<(Entity, &HexCoord), With<HexTile>>,
    mut ghost_clicks: MessageWriter<GhostClicked>,
    mut tile_clicks: MessageWriter<TileClicked>,
) {
    for click in clicks.read() {
        if let Some((entity, _)) = ghosts
            .iter()
            .find(|(_, coord)| coord.tile_coord() == click.0)
        {
            ghost_clicks.write(GhostClicked {
                entity,
                coord: click.0,
            });
        } else if let Some((entity, _)) = tiles
            .iter()
            .find(|(_, coord)| coord.tile_coord() == click.0)
        {
            tile_clicks.write(TileClicked {
                entity,
                coord: click.0,
            });
        }
    }
}
