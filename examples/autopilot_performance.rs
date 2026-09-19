use bevy::prelude::*;
use clicker_autopilot::prelude::*;
use clicker_probe::prelude::*;
use game::{ClickTile, GameState, HexGhost, HexTile, TileCoord, TileSettled};

const STEP_DEADLINE: f32 = 120.0;

#[derive(Resource, Default)]
struct PerformanceLoad(bool);

#[derive(Resource, Default)]
struct SettledTiles(u32);

fn main() -> AppExit {
    let mut app = game::app();
    app.init_resource::<PerformanceLoad>()
        .init_resource::<SettledTiles>()
        .add_systems(Update, (drive_performance_load, track_settled_tiles))
        .add_plugins((script(), ClickerProbePlugin));
    app.run()
}

fn drive_performance_load(load: Res<PerformanceLoad>, mut clicks: MessageWriter<ClickTile>) {
    if load.0 {
        clicks.write(ClickTile(TileCoord::ZERO));
    }
}

fn track_settled_tiles(mut events: MessageReader<TileSettled>, mut settled: ResMut<SettledTiles>) {
    settled.0 += events.read().count() as u32;
}

fn script() -> AutopilotPlugin {
    AutopilotPlugin::new()
        .step("the playable tile world is ready")
        .until(
            and(state_is(GameState::Playing), entity_count::<HexGhost>(1)),
            STEP_DEADLINE,
        )
        .add()
        .step("the measured tile is placed")
        .act(|world| {
            world.write_message(ClickTile(TileCoord::ZERO));
        })
        .until(
            and(
                entity_count::<HexTile>(1),
                resource_where::<SettledTiles>(|settled| settled.0 == 1),
            ),
            STEP_DEADLINE,
        )
        .add()
        .step("the playing workload fills the timing window")
        .act(|world| {
            world.resource_mut::<PerformanceLoad>().0 = true;
            world.write_message(BeginTimingCapture);
        })
        .until(
            resource_where::<TimingStatus>(|status| status.complete),
            STEP_DEADLINE,
        )
        .add()
        .step("the timing capture completed")
        .act(|world| {
            world.resource_mut::<PerformanceLoad>().0 = false;
            marker(
                world,
                "timing capture complete",
                serde_json::json!({"tile": {"q": 0, "r": 0}}),
            );
            probe_snapshot(world, "timing capture complete");
        })
        .expect(resource_where::<TimingStatus>(|status| {
            status.complete && status.captured_updates == DEFAULT_CAPTURE_UPDATES
        }))
        .add()
}
