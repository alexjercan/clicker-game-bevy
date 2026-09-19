pub mod capabilities;
pub mod html;
#[cfg(not(target_arch = "wasm32"))]
pub mod native;

use bevy::prelude::*;

pub use capabilities::*;
#[derive(Resource, Default)]
struct ProbeFrame(u64);

fn ensure_clock(app: &mut App) {
    if app.world().contains_resource::<ProbeFrame>() {
        return;
    }
    app.init_resource::<ProbeFrame>()
        .add_systems(First, |mut frame: ResMut<ProbeFrame>| frame.0 += 1);
}

pub struct ClickerProbePlugin;

impl Plugin for ClickerProbePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((timeline(), snapshots(), invariants(), timing()));
    }
}

pub mod prelude {
    pub use crate::{
        capture_snapshot, invariants, marker, probe_snapshot, snapshots, timeline, timing,
        BeginTimingCapture, ClickerProbePlugin, InvariantsPlugin, SnapshotPlugin, TimelinePlugin,
        TimingPlugin, TimingStatus, DEFAULT_CAPTURE_UPDATES, DEFAULT_WARMUP_UPDATES, SNAPSHOT_ENV,
        TIMELINE_ENV, TIMING_ENV, TIMING_WARMUP_ENV,
    };
}
