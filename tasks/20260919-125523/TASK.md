# Add a deterministic autopilot and probe harness

- STATUS: OPEN
- PRIORITY: 60
- TAGS: tooling,bevy,iteration-2

## User facts

- Add better and cooler autopilot, debug, probe, and --norender modes similar to Nova Protocol.
- This can follow the first foundation iteration.
- Do not turn the project into Nova or add unrelated features.

## Agent findings

- The current optional debug crate provides inspector/performance tooling, but the game has no CLI, bounded deterministic runner, probe contract, evidence output, or no-render assembly.
- Nova separates in-game autopilot/probe capabilities from the native host CLI and keeps all of it behind debug features and environment gates.

## Delivery

- First write a small contract for commands, environment variables, seed precedence, frame limits, exit codes, and evidence files. Prefix all process variables for this game and keep one owner per variable.
- Add a Clap-based native CLI with --debug, --norender, deterministic seed, bounded frame count, and a probe subcommand. Keep wasm free of native CLI dependencies.
- Build a true no-render app with no GPU device, window, or visual plugins while retaining the simulation systems required by probes.
- Add an autopilot that drives named game actions through the same gameplay interfaces as input, advances deterministic flows, fails loudly on unmet predicates, and always has a frame/time budget.
- Add probes for boot/loading, tile placement, XP gain and rollover, skill-point spending, world bounds, and deterministic seeded tile kinds.
- Emit machine-readable results plus concise human output. Capture rendered screenshots only in a separate rendered mode; no-render evidence must never claim visual correctness.
- Keep debug/probe/autopilot code out of release builds and preserve the ordinary launch path.

## Verification

- Run each probe repeatedly with the same seed and compare facts.
- Prove --norender creates no window/GPU renderer and exits at the configured bound.
- Prove an impossible predicate and unknown probe fail nonzero with useful evidence.
- Build release and wasm artifacts and confirm the debug host is absent.

## Done when

- One command can run a bounded deterministic gameplay flow locally and in CI.
- Probe results identify the failed invariant, seed, frame, and relevant world facts.
- Rendered and no-render modes have explicit, tested evidence limits.
