use bevy::prelude::*;
use bevy_rand::prelude::*;
use hexx::*;
use rand_core::Rng;

use crate::{
    InitializeTileWorld, PlaceTile, TileCoord, TileKind, TilePlaced, TileSettled, TileSystems,
};

const WORLD_HALF_RADIUS: u32 = 3;

#[derive(Component, Debug, Clone)]
pub struct HexTile;

#[derive(Component, Debug, Clone)]
pub struct HexGhost;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct HexCoord(pub(crate) Hex);

impl HexCoord {
    pub fn tile_coord(&self) -> TileCoord {
        TileCoord::from(self.0)
    }
}

impl From<Hex> for TileCoord {
    fn from(value: Hex) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<TileCoord> for Hex {
    fn from(value: TileCoord) -> Self {
        Self::new(value.q(), value.r())
    }
}

impl TileKind {
    fn random(rng: &mut impl Rng) -> Self {
        match rng.next_u32() % 4 {
            0 => Self::Empty,
            1 => Self::Tree,
            2 => Self::Stone,
            _ => Self::Wheat,
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct HexMap(HexLayout);

impl Default for HexMap {
    fn default() -> Self {
        Self(HexLayout {
            orientation: HexOrientation::Flat,
            scale: Vec2::splat(2.0 / 3.0f32.sqrt()),
            ..default()
        })
    }
}

impl HexMap {
    pub fn tile_at(&self, world_position: Vec2) -> TileCoord {
        self.world_pos_to_hex(world_position).into()
    }

    pub fn world_position(&self, coord: TileCoord) -> Vec2 {
        self.hex_to_world_pos(coord.into())
    }

    pub fn layout(&self) -> &HexLayout {
        &self.0
    }
}

type TileEntity = Or<(With<HexGhost>, With<HexTile>)>;

pub(crate) struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<InitializeTileWorld>()
            .add_message::<PlaceTile>()
            .add_message::<TilePlaced>()
            .add_message::<TileSettled>()
            .init_resource::<HexMap>()
            .add_systems(
                Update,
                (initialize_world, place_tile)
                    .chain()
                    .in_set(TileSystems::Mutation),
            );
    }
}

fn initialize_world(
    mut commands: Commands,
    mut events: MessageReader<InitializeTileWorld>,
    existing: Query<Entity, TileEntity>,
) {
    for _ in events.read() {
        for entity in &existing {
            commands.entity(entity).despawn();
        }
        spawn_ghost(&mut commands, Hex::ZERO);
    }
}

#[derive(bevy::ecs::system::SystemParam)]
struct WorldQueries<'w, 's> {
    ghosts: Query<'w, 's, (Entity, &'static HexCoord), With<HexGhost>>,
    tiles: Query<'w, 's, (Entity, &'static HexCoord), With<HexTile>>,
}

fn place_tile(
    mut commands: Commands,
    mut events: MessageReader<PlaceTile>,
    mut placed: MessageWriter<TilePlaced>,
    queries: WorldQueries,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    for event in events.read() {
        let Ok((entity, hex_coord)) = queries.ghosts.get(event.0) else {
            continue;
        };
        commands.entity(entity).despawn();
        let kind = TileKind::random(&mut rng);
        spawn_tile(&mut commands, **hex_coord, kind);
        placed.write(TilePlaced {
            coord: TileCoord::from(**hex_coord),
            kind,
        });
        for coord in open_neighbors(hex_coord, &queries) {
            spawn_ghost(&mut commands, coord);
        }
    }
}

fn open_neighbors<'a>(
    hex_coord: &'a HexCoord,
    queries: &'a WorldQueries,
) -> impl Iterator<Item = Hex> + 'a {
    hex_coord
        .ring(1)
        .filter(|coord| {
            queries
                .ghosts
                .iter()
                .all(|(_, HexCoord(ghost_coord))| ghost_coord != coord)
                && queries
                    .tiles
                    .iter()
                    .all(|(_, HexCoord(tile_coord))| tile_coord != coord)
        })
        .filter(|coord| is_in_world(*coord))
}

fn spawn_tile(commands: &mut Commands, coord: Hex, kind: TileKind) {
    commands.spawn((Name::new("HexTile"), HexTile, HexCoord(coord), kind));
}

fn spawn_ghost(commands: &mut Commands, coord: Hex) {
    commands.spawn((Name::new("HexGhost"), HexGhost, HexCoord(coord)));
}

pub fn tile_is_in_world(coord: TileCoord) -> bool {
    is_in_world(coord.into())
}

fn is_in_world(coord: Hex) -> bool {
    HexBounds::new(Hex::ZERO, WORLD_HALF_RADIUS).is_in_bounds(coord)
}

#[cfg(test)]
mod tests {
    use rand_core::SeedableRng;

    use super::*;

    #[test]
    fn world_bounds_include_radius_three_and_exclude_radius_four() {
        assert!(is_in_world(Hex::new(3, 0)));
        assert!(is_in_world(Hex::new(-3, 3)));
        assert!(!is_in_world(Hex::new(4, 0)));
        assert!(!is_in_world(Hex::new(-4, 4)));
    }

    #[test]
    fn tile_kinds_follow_the_seeded_wyrand_sequence() {
        let mut rng = WyRand::from_seed(7u64.to_le_bytes());
        let kinds = (0..8)
            .map(|_| TileKind::random(&mut rng))
            .collect::<Vec<_>>();

        assert_eq!(
            kinds,
            vec![
                TileKind::Tree,
                TileKind::Empty,
                TileKind::Wheat,
                TileKind::Tree,
                TileKind::Empty,
                TileKind::Tree,
                TileKind::Tree,
                TileKind::Empty,
            ]
        );
    }
}
