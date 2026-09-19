# Add a deterministic autopilot and probe harness

- STATUS: CLOSED
- PRIORITY: 60
- TAGS: tooling, bevy, iteration-2

## User facts

- Add better and cooler autopilot, debug, probe, and --norender modes similar to Nova Protocol.
- This can follow the first foundation iteration.
- Do not turn the project into Nova or add unrelated features.
- The autopilot harness must provide generic named steps, actions, predicates, and deadlines. Examples own game-specific scripts.
- `headless_app` is a production app constructor and must not depend on the debug feature.
- Probe code records capabilities and evidence. It does not drive probe scenarios.
- Tile behavior belongs in `clicker_tile`; progression consumes its events.
- Skip the agent bench and agent CLI in this task.
- Keep camera and light setup out of the tile subsystem.
- Keep tweening details in `clicker_animation`; tile consumers should only observe tile-level messages.
- Track the larger UI improvement separately.
- Keep completion and deadline handling inside the autopilot driver instead of a shared completion layer.
- Bind each wait deadline directly to `until` and keep `expect` immediate.
- Use purpose-specific animation completion messages instead of forwarding every tween completion.
- Make debug presentation one `F11` toggle and replace the hard-coded performance text with Bevy's maintained overlay.
- Track expanded gameplay and game-feel work separately.
- Keep `clicker_tile` independent of windows, rendering, animation, assets, and app lifecycle state. Inject abstract pointer intent and compose presentation in core glue.
- Give shared app lifecycle state a common crate instead of making gameplay depend on assets.
- Keep snapshot construction in `clicker_probe`; gameplay must not own probe-only projections.
- Probe capabilities are plain plugins added by `ClickerProbePlugin`. Do not maintain a capability enum or declaration contract.
- Every checked-in example must exit successfully under its documented invocation.
- Run autopilot and probe examples explicitly when wanted; do not wrap them in the normal Rust test suite.
- Generate a self-contained HTML report for one probe run and an aggregate HTML report for `probe all`.
- Measure frame intervals and main-world update work only while the game is in `Playing`.
- Report rendered update rate as FPS and no-render update rate as UPS. Do not add baseline comparison or Samply.
- Keep examples single-cycle and independent of timing state. The host may repeat complete example processes externally and aggregate valid samples. Insufficient samples report N/A.
- Export named Bevy schedule and system spans with durations as a Chrome trace.

## Agent findings

- The current optional debug crate provides inspector/performance tooling, but the game has no CLI, bounded deterministic runner, probe contract, evidence output, or no-render assembly.
- Nova separates in-game autopilot/probe capabilities from the native host CLI and keeps all of it behind debug features and environment gates.

## Decisions

- `clicker_autopilot` depends on Bevy only. `CLICKER_AUTOPILOT` arms named predicate-driven steps. Each step can act once, wait with `until(predicate, deadline_secs)`, and assert immediately with `expect`. A false expectation, an expired wait, and the run-level `CLICKER_AUTOPILOT_DEADLINE` backstop are separate failures. The driver writes `AppExit` directly.
- Root examples depend on autopilot and probe crates as dev dependencies. They own all Clicker-specific actions, waits, and assertions.
- `clicker_probe` adds timeline, snapshot, and invariant plugins directly. Timeline and snapshot paths are armed by `CLICKER_PROBE_TIMELINE` and `CLICKER_PROBE_SNAPSHOT`; invariants always run with the probe plugin.
- `ClickTile` is the public action. `clicker_tile` owns coordinates, kinds, logical tile and ghost state, abstract `TilePointer` interpretation, placement mutation, world bounds, and the RNG. It has no Clicker crate dependency.
- `clicker_animation` owns tweening. Animation constructors return component bundles. Spawn bundles carry `SpawnAnimation`, and the plugin emits `SpawnAnimationFinished` only for that animation kind.
- `clicker_core` owns the main camera and directional light through `MainScenePlugin`. Its glue injects native pointer state, attaches visual components and children to logical tile entities, applies animation bundles, translates rendered settlement, and provides immediate headless settlement.
- `clicker_state` owns `GameState`. Assets drive its loading transition; gameplay, UI, core, and probes consume the shared state without depending on assets for lifecycle types.
- Probe snapshot types and world projection are private to `clicker_probe`; gameplay exposes only gameplay state and actions.
- Timing discards 120 `Playing` updates for a dedicated long-running capture. For short examples, the native host repeats fresh single-cycle processes, discards the first sample from each process, and aggregates up to 600 valid samples. A run that cannot contribute a post-startup sample remains successful but reports timing as N/A.
- Root default features are empty. `debug` enables only in-game debug presentation. `dev` enables debug presentation, dynamic linking, the native probe host, Bevy tracing, and the env-gated Chrome layer. The layer writes only when `TRACE_CHROME` names an output file, so ordinary all-feature checks create no trace files.
- Progression defaults use named constants and resource defaults. XP gain and skill spending are focused systems without internal progression system sets.
- Debug presentation starts hidden. `F11` toggles the FPS text overlay, world inspector, and cursor gizmo together. The frame-time graph stays disabled because its retained graph surface is visually distracting.
- `CLICKER_SEED` is owned by `clicker_tile`, uses `u64::to_le_bytes`, applies to rendered and no-render apps, uses OS entropy when absent, and refuses malformed values.
- `headless_app()` is always available. It uses Bevy minimal, state, and log plugins plus simulation plugins. It has no window, renderer, asset loading, animation, UI, or visual debug plugin.
- The game CLI selects `--norender`. The debug-only probe host supports `run <example>` and `all`, captures artifacts and logs, writes per-run and aggregate HTML reports, and may pass a seed through `CLICKER_SEED`. There is no agent command, frame runner, or game-level seed flag.

## Delivery

- Extract tile ownership into `clicker_tile` and keep XP and skill spending in `clicker_gameplay`.
- Make rendered and no-render app construction production interfaces. Keep `--norender` and `CLICKER_NORENDER` as selectors.
- Add the game-independent `clicker_autopilot` step driver with direct bounded completion.
- Add `clicker_probe` capability contracts, structured timelines, deterministic snapshots, continuous invariants, and a host command that runs examples and records artifacts and logs.
- Add a tile gameplay example that waits on observable state, uses `ClickTile`, asserts progression and placement, and captures probe evidence.
- Delete the replaced game-specific harness, agent channel, frame runner, and hard-coded probe scenarios.

## Verification

- Run the example twice with one seed and compare snapshots.
- Prove the timeline contains state, tile action, progression, marker, and run-end entries.
- Prove an impossible predicate error-exits and names its step and diagnosis.
- Prove no-render has no window or render sub-app.
- Run a rendered load and inspect the initial world.
- Build native release and wasm artifacts.

## Done when

- Examples compose generic harness primitives into bounded gameplay flows.
- Probe capabilities record what happened and what the world contains without driving it.
- One seed reaches the same gameplay RNG stream in rendered and no-render apps.
- The ordinary game does not link the autopilot crate; only examples depend on it. Probe host code remains debug-only.
