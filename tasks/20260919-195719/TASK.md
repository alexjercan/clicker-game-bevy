# Improve the in-game UI foundation

- STATUS: OPEN
- PRIORITY: 60
- TAGS: ui, iteration-3

## User facts

- The current `clicker_ui` implementation needs a substantial improvement pass.
- Keep this work separate from the autopilot, probe, tile ownership, and scene ownership change.

## Agent findings

- The UI is one large setup function with deeply nested inline styles.
- The XP bar has no text value, label, or level context.
- The skill-point indicator is only an icon and does not show its count or explain the available action.
- The root UI entity is not explicitly scoped to `GameState::Playing`.
- Layout values are fixed and have not been checked across native window sizes or the wasm canvas.
- UI behavior currently reads progression resources directly and has no focused UI-state tests.

## Decisions needed

- Confirm the desired visual direction, typography, and whether the HUD should remain a top bar.
- Confirm which gameplay facts must be visible: XP value and threshold, skill-point count, placement availability, selected tile, and any controls hint.
- Confirm the minimum supported viewport and scaling policy.

## Delivery

- Give the UI a small internal structure instead of one monolithic spawn function.
- Define named style constants and focused bundles or spawn helpers for the HUD sections.
- Make the XP display readable as both a bar and text without changing progression rules.
- Show the skill-point count and a clear placement affordance without adding new gameplay.
- Scope all gameplay UI entities to the playing state and verify clean state transitions.
- Make the HUD responsive at the agreed native and wasm viewport sizes.
- Keep `clicker_ui` as the sole owner of presentation. Consume public gameplay state without moving gameplay rules into UI systems.
- Update visual assets and credits only if an approved design requires them.

## Verification

- Add focused Bevy app tests for resource-to-UI state changes and playing-state cleanup.
- Inspect rendered screenshots at the agreed small, standard, and wide viewports.
- Verify native and wasm interaction with the HUD visible.
- Confirm zero XP, partial XP, rollover, zero skill points, and multiple skill points.
- Run formatting, tests, clippy, native release, and Trunk release checks.

## Done when

- The HUD clearly communicates progression and available skill points.
- It remains usable at every agreed viewport.
- UI entities enter and leave with the playing state.
- Automated checks prove state binding; rendered evidence proves layout and appearance.
