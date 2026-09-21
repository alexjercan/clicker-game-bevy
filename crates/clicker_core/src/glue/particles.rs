use bevy::prelude::*;
use clicker_gameplay::{HexMap, TileClicked, TileKind, TilePlaced, TilePointer, TileSystems};
use clicker_particles::{ParticleBurst, ParticleBurstKind, ParticlePalette, ParticleSystems};
use clicker_state::GameState;

const PARTICLE_HEIGHT: f32 = 0.35;

pub(super) struct ParticleGluePlugin;

impl Plugin for ParticleGluePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, ParticleSystems::Spawn.after(TileSystems::Feedback))
            .add_systems(
                Update,
                (request_click_particles, request_spawn_particles)
                    .in_set(TileSystems::Feedback)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn request_click_particles(
    hexmap: Res<HexMap>,
    pointer: Res<TilePointer>,
    kinds: Query<&TileKind>,
    mut clicks: MessageReader<TileClicked>,
    mut bursts: MessageWriter<ParticleBurst>,
) {
    for click in clicks.read() {
        let Ok(kind) = kinds.get(click.entity) else {
            continue;
        };
        bursts.write(ParticleBurst {
            kind: ParticleBurstKind::TileClick(palette(*kind)),
            position: click_particle_position(&hexmap, &pointer, click.coord),
        });
    }
}

fn request_spawn_particles(
    hexmap: Res<HexMap>,
    mut placed: MessageReader<TilePlaced>,
    mut bursts: MessageWriter<ParticleBurst>,
) {
    for tile in placed.read() {
        bursts.write(ParticleBurst {
            kind: ParticleBurstKind::TileSpawn,
            position: particle_position(&hexmap, tile.coord),
        });
    }
}

fn click_particle_position(
    hexmap: &HexMap,
    pointer: &TilePointer,
    coord: clicker_gameplay::TileCoord,
) -> Vec3 {
    pointer
        .world_position
        .filter(|position| hexmap.tile_at(*position) == coord)
        .unwrap_or_else(|| hexmap.world_position(coord))
        .extend(0.0)
        .xzy()
        + Vec3::Y * PARTICLE_HEIGHT
}

fn particle_position(hexmap: &HexMap, coord: clicker_gameplay::TileCoord) -> Vec3 {
    hexmap.world_position(coord).extend(0.0).xzy() + Vec3::Y * PARTICLE_HEIGHT
}

fn palette(kind: TileKind) -> ParticlePalette {
    match kind {
        TileKind::Empty => ParticlePalette::Grass,
        TileKind::Tree => ParticlePalette::Leaves,
        TileKind::Stone => ParticlePalette::Stone,
        TileKind::Wheat => ParticlePalette::Wheat,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clicker_gameplay::TileCoord;

    #[derive(Resource, Default)]
    struct CollectedBursts(Vec<ParticleBurst>);

    fn collect_bursts(
        mut bursts: MessageReader<ParticleBurst>,
        mut collected: ResMut<CollectedBursts>,
    ) {
        collected.0.extend(bursts.read());
    }

    #[test]
    fn click_and_placement_request_distinct_bursts() {
        let mut app = App::new();
        app.init_resource::<HexMap>()
            .init_resource::<TilePointer>()
            .init_resource::<CollectedBursts>()
            .add_message::<TileClicked>()
            .add_message::<TilePlaced>()
            .add_message::<ParticleBurst>()
            .add_systems(
                Update,
                (
                    request_click_particles,
                    request_spawn_particles,
                    collect_bursts,
                )
                    .chain(),
            );
        let entity = app.world_mut().spawn(TileKind::Stone).id();
        app.world_mut().resource_mut::<TilePointer>().world_position = Some(Vec2::new(0.2, 0.1));
        app.world_mut().write_message(TileClicked {
            entity,
            coord: TileCoord::ZERO,
        });
        app.world_mut().write_message(TilePlaced {
            coord: TileCoord::new(1, 0),
            kind: TileKind::Tree,
        });

        app.update();

        let bursts = &app.world().resource::<CollectedBursts>().0;
        assert_eq!(bursts.len(), 2);
        assert_eq!(
            bursts[0].kind,
            ParticleBurstKind::TileClick(ParticlePalette::Stone)
        );
        assert_eq!(bursts[1].kind, ParticleBurstKind::TileSpawn);
        assert_eq!(bursts[0].position, Vec3::new(0.2, PARTICLE_HEIGHT, 0.1));
        assert_eq!(bursts[1].position.y, PARTICLE_HEIGHT);
    }
}
