# Refactor the game into focused Bevy modules

- STATUS: OPEN
- PRIORITY: 80
- TAGS: architecture,bevy

## User facts

- Split the codebase into nicer modules without adding or changing features.
- Keep the game basic.

## Agent findings

- Nearly all game behavior, state, resources, events, UI construction, selection, progression, world spawning, and app assembly are in src/main.rs.
- src/lib.rs only exports materials.
- The debug crate exists separately, but its role and API must be audited.

## Delivery

- Capture the current behavior and startup/plugin ordering before moving code.
- Make main.rs a thin CLI/app launch entry point.
- Move app construction into the library and split ownership into focused modules such as app/state, assets/loading, world/hex map, interaction/selection, progression, UI, and materials. Final names must follow actual ownership found during implementation.
- Give each subsystem a plugin and named system sets where ordering matters. Keep shared types in the lowest owning module and keep public surface minimal.
- Replace the crate-wide too_many_arguments allowance with bundles, system params, or narrow justified exceptions that comply with the marked-comment policy.
- Preserve asset paths, state transitions, random tile distribution, world bounds, XP thresholds, skill-point spending, tweens, camera, UI, and click behavior.
- Delete obsolete re-exports and the monolithic implementations after callers move.

## Verification

- Add unit tests only for stable pure rules such as XP rollover, world bounds, and tile-kind selection under a fixed seed.
- Use the deterministic runtime harness to prove one complete player flow: load, place the first tile, click it to earn XP, cross a threshold, and place another tile.
- Compare captured world facts and a rendered frame with the pre-refactor baseline.

## Done when

- main.rs only parses/dispatches and launches the app.
- Every system and type has one clear module owner.
- No duplicate compatibility path remains.
- The baseline flow and visible result are unchanged.
