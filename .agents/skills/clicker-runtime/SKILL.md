---
name: clicker-runtime
description: Verify Clicker game loading, ECS behavior, input flows, and rendered output.
---

# Clicker Runtime

Follow [AGENTS.md](../../../AGENTS.md). Use this skill for Bevy runtime,
loading, interaction, state, UI, material, animation, and performance changes.

## Choose evidence

- Pure progression or hex rules: unit test.
- Components, resources, events, schedules, or state transitions: focused Bevy
  app test.
- Asset loading or player interaction: run the game and exercise the exact flow.
- Layout, mesh, material, tween, camera, or lighting: inspect rendered output.
- Performance: compare the same bounded flow and build profile before and after.

Do not use a successful launch as proof of a player outcome. Do not use
headless evidence for a visual claim. Record the seed and inputs for any random
flow when the current code permits it.

Run only the affected commands through Nix:

```bash
nix develop --command cargo test --all-features
nix develop --command cargo run
nix develop --command trunk build --release
```

For interactive checks, record the start state, actions, expected facts, actual
facts, and visual observations. Stop after the affected flow. Use an armed
`autopilot_*` example for a bounded gameplay flow. Use `CLICKER_PROBE_*`
artifacts for simulation facts. No-render artifacts cannot prove visuals.
