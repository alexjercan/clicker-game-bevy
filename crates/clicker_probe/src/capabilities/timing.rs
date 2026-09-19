use std::{path::PathBuf, time::Instant};

use bevy::prelude::*;
use clicker_state::GameState;

pub const TIMING_ENV: &str = "CLICKER_PROBE_TIMING";
pub const TIMING_WARMUP_ENV: &str = "CLICKER_PROBE_TIMING_WARMUP";
pub const DEFAULT_WARMUP_UPDATES: u32 = 120;
pub const DEFAULT_CAPTURE_UPDATES: u32 = 600;

#[derive(Message, Default)]
pub struct BeginTimingCapture;

#[derive(Resource, Default)]
pub struct TimingStatus {
    pub complete: bool,
    pub captured_updates: u32,
}

pub struct TimingPlugin {
    out: Option<PathBuf>,
    warmup_updates: u32,
    capture_updates: u32,
}

impl TimingPlugin {
    pub fn out(mut self, path: impl Into<PathBuf>) -> Self {
        self.out = Some(path.into());
        self
    }

    pub fn window(mut self, warmup_updates: u32, capture_updates: u32) -> Self {
        assert!(
            capture_updates > 0,
            "timing capture needs at least one update"
        );
        self.warmup_updates = warmup_updates;
        self.capture_updates = capture_updates;
        self
    }
}

pub fn timing() -> TimingPlugin {
    TimingPlugin {
        out: None,
        warmup_updates: DEFAULT_WARMUP_UPDATES,
        capture_updates: DEFAULT_CAPTURE_UPDATES,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CapturePhase {
    Waiting,
    Warmup,
    Capture,
    Complete,
}

#[derive(Resource)]
struct TimingCapture {
    path: Option<PathBuf>,
    phase: CapturePhase,
    warmup_updates: u32,
    warmed: u32,
    capture_updates: u32,
    update_start: Option<Instant>,
    frame_ms: Vec<f64>,
    update_work_ms: Vec<f64>,
    written: bool,
}

impl Plugin for TimingPlugin {
    fn build(&self, app: &mut App) {
        let path = self
            .out
            .clone()
            .or_else(|| std::env::var_os(TIMING_ENV).map(PathBuf::from));
        if let Some(parent) = path.as_deref().and_then(std::path::Path::parent) {
            std::fs::create_dir_all(parent).unwrap_or_else(|error| {
                panic!("cannot create timing directory {parent:?}: {error}")
            });
        }
        let warmup_updates = std::env::var(TIMING_WARMUP_ENV)
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(self.warmup_updates);
        app.add_message::<BeginTimingCapture>()
            .insert_resource(TimingStatus {
                complete: false,
                captured_updates: 0,
            })
            .insert_resource(TimingCapture {
                path,
                phase: CapturePhase::Waiting,
                warmup_updates,
                warmed: 0,
                capture_updates: self.capture_updates,
                update_start: None,
                frame_ms: Vec::with_capacity(self.capture_updates as usize),
                update_work_ms: Vec::with_capacity(self.capture_updates as usize),
                written: false,
            })
            .add_systems(
                Update,
                begin_requested_capture.run_if(in_state(GameState::Playing)),
            )
            .add_systems(First, begin_update.run_if(in_state(GameState::Playing)))
            .add_systems(Last, finish_update.run_if(in_state(GameState::Playing)));
    }
}

fn begin_requested_capture(
    mut requests: MessageReader<BeginTimingCapture>,
    mut capture: ResMut<TimingCapture>,
    mut status: ResMut<TimingStatus>,
) {
    if requests.read().next().is_none() {
        return;
    }
    capture.phase = if capture.warmup_updates == 0 {
        CapturePhase::Capture
    } else {
        CapturePhase::Warmup
    };
    capture.warmed = 0;
    capture.frame_ms.clear();
    capture.update_work_ms.clear();
    capture.written = false;
    status.complete = false;
    status.captured_updates = 0;
}

fn begin_update(mut capture: ResMut<TimingCapture>) {
    if capture.phase == CapturePhase::Waiting {
        capture.phase = if capture.warmup_updates == 0 {
            CapturePhase::Capture
        } else {
            CapturePhase::Warmup
        };
    }
    capture.update_start = Some(Instant::now());
}

fn finish_update(
    time: Res<Time<Real>>,
    mut exits: MessageReader<AppExit>,
    mut capture: ResMut<TimingCapture>,
    mut status: ResMut<TimingStatus>,
) {
    let update_work_ms = capture
        .update_start
        .take()
        .map(|start| milliseconds(start.elapsed()));
    match capture.phase {
        CapturePhase::Waiting => {}
        CapturePhase::Warmup => {
            capture.warmed += 1;
            if capture.warmed >= capture.warmup_updates {
                capture.phase = CapturePhase::Capture;
            }
        }
        CapturePhase::Capture => {
            capture.frame_ms.push(milliseconds(time.delta()));
            if let Some(update_work_ms) = update_work_ms {
                capture.update_work_ms.push(update_work_ms);
            }
            status.captured_updates = capture.frame_ms.len() as u32;
            if status.captured_updates >= capture.capture_updates {
                capture.phase = CapturePhase::Complete;
                status.complete = true;
                write_capture(&mut capture, true);
            }
        }
        CapturePhase::Complete => {}
    }
    if exits.read().next().is_some() && !capture.written {
        let complete = capture.phase == CapturePhase::Complete;
        write_capture(&mut capture, complete);
    }
}

fn write_capture(capture: &mut TimingCapture, complete: bool) {
    let output = serde_json::json!({
        "mode": if std::env::var_os("CLICKER_NORENDER").is_some() { "no-render" } else { "rendered" },
        "complete": complete,
        "warmup_updates": capture.warmup_updates,
        "capture_updates": capture.capture_updates,
        "captured_updates": capture.frame_ms.len(),
        "source_runs": 1,
        "frame_intervals": statistics(&capture.frame_ms),
        "update_work": statistics(&capture.update_work_ms),
        "frame_ms": capture.frame_ms,
        "update_work_ms": capture.update_work_ms,
    });
    if let Some(path) = &capture.path {
        std::fs::write(path, output.to_string())
            .unwrap_or_else(|error| panic!("cannot write timing {path:?}: {error}"));
    }
    capture.written = true;
}

fn milliseconds(duration: std::time::Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn aggregate_timing(
    reports: &[serde_json::Value],
    target_updates: usize,
) -> serde_json::Value {
    let mut frame_ms = Vec::new();
    let mut update_work_ms = Vec::new();
    for report in reports {
        let frames = report["frame_ms"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(serde_json::Value::as_f64)
            .skip(1);
        let work = report["update_work_ms"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(serde_json::Value::as_f64)
            .skip(1);
        frame_ms.extend(frames);
        update_work_ms.extend(work);
    }
    frame_ms.truncate(target_updates);
    update_work_ms.truncate(target_updates);
    let complete = frame_ms.len() == target_updates;
    serde_json::json!({
        "mode": reports.first().and_then(|report| report["mode"].as_str()).unwrap_or("unknown"),
        "complete": complete,
        "source_runs": reports.len(),
        "warmup_updates_per_run": 1,
        "capture_updates": target_updates,
        "captured_updates": frame_ms.len(),
        "frame_intervals": statistics(&frame_ms),
        "update_work": statistics(&update_work_ms),
        "frame_ms": frame_ms,
        "update_work_ms": update_work_ms,
    })
}

fn statistics(samples: &[f64]) -> serde_json::Value {
    if samples.is_empty() {
        return serde_json::Value::Null;
    }
    let mut sorted = samples.to_vec();
    sorted.sort_by(f64::total_cmp);
    let mean_ms = sorted.iter().sum::<f64>() / sorted.len() as f64;
    serde_json::json!({
        "samples": sorted.len(),
        "mean_ms": mean_ms,
        "p50_ms": percentile(&sorted, 0.50),
        "p95_ms": percentile(&sorted, 0.95),
        "p99_ms": percentile(&sorted, 0.99),
        "max_ms": sorted[sorted.len() - 1],
        "updates_per_second": 1000.0 / mean_ms,
    })
}

fn percentile(sorted: &[f64], percentile: f64) -> f64 {
    let index = ((sorted.len() - 1) as f64 * percentile).ceil() as usize;
    sorted[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statistics_report_updates_and_tail_latency() {
        let stats = statistics(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(stats["samples"], 4);
        assert_eq!(stats["mean_ms"], 2.5);
        assert_eq!(stats["p50_ms"], 3.0);
        assert_eq!(stats["p95_ms"], 4.0);
        assert_eq!(stats["p99_ms"], 4.0);
        assert_eq!(stats["updates_per_second"], 400.0);
    }

    #[test]
    fn aggregation_discards_each_run_start_and_stops_at_the_target() {
        let reports = [
            serde_json::json!({"mode": "no-render", "frame_ms": [99.0, 1.0, 2.0], "update_work_ms": [99.0, 0.5, 1.0]}),
            serde_json::json!({"mode": "no-render", "frame_ms": [88.0, 3.0, 4.0], "update_work_ms": [88.0, 1.5, 2.0]}),
        ];
        let aggregate = aggregate_timing(&reports, 3);
        assert_eq!(aggregate["complete"], true);
        assert_eq!(aggregate["source_runs"], 2);
        assert_eq!(aggregate["frame_ms"], serde_json::json!([1.0, 2.0, 3.0]));
    }
}
