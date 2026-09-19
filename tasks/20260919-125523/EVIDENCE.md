# Evidence

## Automated checks

- `nix develop --command cargo test --workspace --all-features`
  - Passed workspace unit, policy, and documentation tests. Autopilot and probe examples are intentionally not wrapped in the Rust test suite.
- `nix develop --command cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - Passed.
- `nix develop --command cargo fmt --all -- --check`
  - Passed.
- `nix develop --command cargo build --release --no-default-features`
  - Passed after the tile/core glue and shared-state extraction.
- `nix develop --command trunk build --release`
  - Passed after the tile/core glue and shared-state extraction.
- `cargo tree` checks show no Clicker crate dependency under `clicker_tile` and no `clicker_assets` dependency under `clicker_gameplay`.

## Autopilot behavior

The autopilot now owns completion and the run-level backstop directly. There is no completion module or registration protocol. `until(predicate, deadline_secs)` owns its wait deadline; `expect` remains an immediate assertion. The unused per-update action API was removed.

All three autopilot examples exit successfully. `autopilot_expectation` demonstrates a passing immediate assertion. `autopilot_deadline` demonstrates a condition reached within its deadline. The tile scenario completes its full progression flow.

## Tile abstraction and glue

`clicker_tile` now depends only on Bevy, `bevy_rand`, `hexx`, and `rand_core`. It owns logical coordinates, entities, selection, clicks, placement, RNG, world bounds, messages, and the injectable `TilePointer`. It has no window, mouse, camera, rendering, animation, asset, or lifecycle-state dependency. Core sends `InitializeTileWorld` on entry to playing and cleans logical tile entities on exit.

`clicker_core::glue`:

- converts native mouse/camera input into `TilePointer` world position and activation;
- attaches visual components and render children to newly added logical ghosts and tiles;
- applies selection, deselection, click, and spawn animation bundles;
- translates `SpawnAnimationFinished` to `TileSettled` in rendered mode;
- translates `TilePlaced` directly to `TileSettled` in headless mode.

The rendered autopilot completed successfully after this split. Its timeline recorded `tile_settled` at frames 65 and 81 and successful run end at frame 85. The final rendered launch was inspected at `/tmp/clicker-final-render.png`; the scene, central ghost, and HUD loaded with debug presentation hidden.

## Shared state

Added `clicker_state` as the owner of `GameState`. Assets drive loading transitions. Gameplay, UI, core, and probes consume the shared type without depending on the asset crate for lifecycle state.

## Probe ownership

Removed `GameSnapshot`, `TileSnapshot`, and `game_snapshot` from `clicker_gameplay` and the core re-exports. Probe-private snapshot projection now serves snapshot capture and invariant checks. Removed the `Capability` enum, `ProbeContract`, declaration calls, contract environment variable, and `contract.json`. `ClickerProbePlugin` directly adds all three plain capability plugins.

`probe all --norender --seed 7 --out target/probe-all-refactor` exited zero. It ran all three passing examples and wrote each run's timeline, snapshot, logs, metadata, and `report.html`, plus aggregate `probe-all.json` and `report.html` at the root. Pure HTML string rendering and escaping live in the unconditional sibling module `clicker_probe/src/html.rs`. It returns `String`, performs no file I/O, and compiles for wasm. Command orchestration and all artifact reads and writes remain in native-only `native.rs`. The report now uses a responsive verdict header, run summary cards, structured metadata, and expandable evidence and log sections.

`probe all --norender --seed 7 --out target/probe-html-test` exited zero after the report redesign. Chromium rendered `autopilot_tiles/report.html` at 1280x900; `/tmp/clicker-probe-report.png` shows the report structure, spacing, typography, verdict, and expanded snapshot without clipping or overlap.

`probe run autopilot_tiles --norender --seed 7 --out target/probe-external-tiles` exited zero. The host ran 24 independent single-cycle processes, discarded the first frame sample from each repeated run, and aggregated exactly 600 valid `Playing` updates. `timing.json` records `complete: true`, `source_runs: 24`, and `captured_updates: 600`.

`probe run autopilot_deadline --norender --seed 7 --out target/probe-external-deadline` exited zero. The short scenario produced no sample after its per-process startup sample, so the host stopped after two runs and wrote `complete: false`, `captured_updates: 0`. Its HTML performance values render as N/A instead of presenting an invalid statistic.

`probe run autopilot_performance --seed 7 --out target/probe-rendered-perf` exited zero with 600 rendered samples after 120 `Playing` warmup updates. The observed run reported 165.0 FPS mean, 6.061 ms mean, 6.878 ms p95, 7.375 ms p99, and 9.646 ms maximum. `/tmp/clicker-perf-report.png` shows these values in the responsive report. This is one host-specific observation, not a baseline.

`target/probe-trace-names/trace.json` loaded as Chrome trace JSON and contains named nested Bevy spans such as `system: name="bevy_state::state::transitions::last_transition<clicker_state::GameState>"` and schedule spans. The env-gated custom trace layer produced no default `trace-*.json` during ordinary all-feature compilation.

The feature matrix passed for native `--no-default-features`, `debug`, and `dev`, plus wasm `--no-default-features`, `debug`, and `dev`. Root defaults are empty. The probe CLI is owned by `dev`; `debug` only enables game debug presentation. `cargo run --quiet --features dev -- probe run autopilot_deadline --norender --seed 7 --out target/probe-dev-feature` exited zero through the final feature wiring.

`probe run autopilot_performance --norender --seed 7 --out target/probe-html-tables` exited zero after the table redesign. Chromium rendered the report at 1280x1100; `/tmp/clicker-probe-tables.png` shows square panels, a separate run table, capture table, frame-time/UPS table, and main-world update-work table without clipping or overlap. `probe run autopilot_deadline --norender --seed 7 --out target/probe-html-na` also exited zero and rendered incomplete timing as N/A.

## Animation and debug presentation

All animation constructors return component bundles. Spawn bundles include `SpawnAnimation`; `clicker_animation` emits `SpawnAnimationFinished` only for completed animations carrying that marker.

Debug presentation starts hidden. `F11` toggles the FPS overlay, world inspector, and cursor gizmo together. The frame-time graph is disabled to avoid its persistent gray graph box.

## Future tasks

- `20260919-195719`: Improve the in-game UI foundation.
- `20260919-201427`: Expand the core gameplay and progression loop.
- `20260919-201418`: Add particles, audio, camera feedback, and animation juice.
