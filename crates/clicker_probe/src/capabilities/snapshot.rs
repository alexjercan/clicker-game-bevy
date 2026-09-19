use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use bevy::prelude::*;
use clicker_gameplay::{SkillPoints, XpMax, XpValue};
use clicker_state::GameState;

use clicker_tile::{HexCoord, HexGhost, HexTile, TileCoord, TileKind};

use crate::ProbeFrame;

pub const SNAPSHOT_ENV: &str = "CLICKER_PROBE_SNAPSHOT";
pub const SNAPSHOT_SCHEMA: u32 = 1;

pub(crate) struct TileSnapshot {
    pub(crate) coord: TileCoord,
    pub(crate) kind: TileKind,
}

pub(crate) struct GameSnapshot {
    pub(crate) xp: u32,
    pub(crate) xp_max: u32,
    pub(crate) skill_points: u32,
    pub(crate) tiles: Vec<TileSnapshot>,
    pub(crate) ghosts: Vec<TileCoord>,
}

pub struct SnapshotPlugin {
    out: Option<PathBuf>,
}

impl SnapshotPlugin {
    pub fn out(mut self, path: impl Into<PathBuf>) -> Self {
        self.out = Some(path.into());
        self
    }
}

pub fn snapshots() -> SnapshotPlugin {
    SnapshotPlugin { out: None }
}

#[derive(Resource)]
pub struct ProbeSnapshots(BufWriter<File>);

impl Plugin for SnapshotPlugin {
    fn build(&self, app: &mut App) {
        let path = self
            .out
            .clone()
            .or_else(|| std::env::var_os(SNAPSHOT_ENV).map(PathBuf::from));
        let Some(path) = path else {
            return;
        };
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap_or_else(|error| {
                panic!("cannot create snapshot directory {parent:?}: {error}")
            });
        }
        let file = File::create(&path)
            .unwrap_or_else(|error| panic!("cannot create snapshot {path:?}: {error}"));
        crate::ensure_clock(app);
        app.insert_resource(ProbeSnapshots(BufWriter::new(file)));
    }
}

pub(crate) fn game_snapshot(world: &mut World) -> GameSnapshot {
    let xp = world.resource::<XpValue>().0;
    let xp_max = world.resource::<XpMax>().0;
    let skill_points = world.resource::<SkillPoints>().0;
    let mut tile_query = world.query_filtered::<(&HexCoord, &TileKind), With<HexTile>>();
    let mut tiles = tile_query
        .iter(world)
        .map(|(coord, kind)| TileSnapshot {
            coord: coord.tile_coord(),
            kind: *kind,
        })
        .collect::<Vec<_>>();
    let mut ghost_query = world.query_filtered::<&HexCoord, With<HexGhost>>();
    let mut ghosts = ghost_query
        .iter(world)
        .map(HexCoord::tile_coord)
        .collect::<Vec<_>>();
    tiles.sort_by_key(|tile| (tile.coord.q(), tile.coord.r()));
    ghosts.sort_by_key(|coord| (coord.q(), coord.r()));
    GameSnapshot {
        xp,
        xp_max,
        skill_points,
        tiles,
        ghosts,
    }
}

pub fn capture_snapshot(world: &mut World, reason: &str) -> serde_json::Value {
    let frame = world
        .get_resource::<ProbeFrame>()
        .map_or(0, |frame| frame.0);
    let state = world
        .get_resource::<State<GameState>>()
        .map(|state| format!("{:?}", state.get()));
    let snapshot = game_snapshot(world);
    serde_json::json!({
        "schema": SNAPSHOT_SCHEMA,
        "reason": reason,
        "frame": frame,
        "game_state": state,
        "seed": std::env::var(clicker_tile::SEED_ENV).ok(),
        "xp": snapshot.xp,
        "xp_max": snapshot.xp_max,
        "skill_points": snapshot.skill_points,
        "tiles": snapshot.tiles.iter().map(|tile| serde_json::json!({
            "q": tile.coord.q(),
            "r": tile.coord.r(),
            "kind": format!("{:?}", tile.kind).to_lowercase(),
        })).collect::<Vec<_>>(),
        "ghosts": snapshot.ghosts.iter().map(|coord| serde_json::json!({
            "q": coord.q(),
            "r": coord.r(),
        })).collect::<Vec<_>>(),
    })
}

pub fn probe_snapshot(world: &mut World, reason: &str) {
    if !world.contains_resource::<ProbeSnapshots>() {
        return;
    }
    let snapshot = capture_snapshot(world, reason);
    let mut sink = world.resource_mut::<ProbeSnapshots>();
    writeln!(sink.0, "{snapshot}")
        .and_then(|()| sink.0.flush())
        .unwrap_or_else(|error| panic!("cannot write probe snapshot: {error}"));
}
