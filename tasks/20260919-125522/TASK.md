# Refactor the game into focused Bevy modules

- STATUS: CLOSED
- PRIORITY: 80
- TAGS: architecture, bevy

## User facts

- Split the codebase into nicer modules without adding or changing features.
- Keep the game basic.
- Use Nova Protocol's crate and composition-root structure as a reference.
- Add unit tests and do not add features.
- Prefix every crate package and directory with `clicker_`.
- Split asset loading, gameplay, tween animation, and UI into separate crates.
- Inherit version, edition, and license from workspace package metadata in every package.

## Decisions

- `clicker_assets` owns loading state, asset collections, the fading material, and the loading transition.
- `clicker_animation` owns the tween plugin and transform tween constructors.
- `clicker_gameplay` owns world, interaction, and progression behavior and depends on assets and animation.
- `clicker_ui` owns the top bar and depends on assets and gameplay.
- `clicker_core` only composes the app and subsystem plugins.
- `clicker_debug` remains separate and optional.
- The root library remains a re-export facade and `main.rs` only launches the app.

## Agent findings

- The original `src/main.rs` owned 732 lines of app assembly, state, assets, world spawning, interaction, progression, UI, and mesh conversion.
- The original library only exported the fading material.
- The later deterministic harness task is `20260919-125523`; this task uses direct X11 input and rendered frame evidence without adding harness code.

## Delivery

- Add `crates/clicker_animation`, `crates/clicker_assets`, `crates/clicker_core`, `crates/clicker_gameplay`, and `crates/clicker_ui`.
- Rename the existing debug package and directory to `clicker_debug` and `crates/clicker_debug`.
- Keep dependencies directed from core to subsystems, UI to gameplay and assets, and gameplay to animation and assets.
- Use explicit gameplay system sets for selection, click reactions, progression, and UI ordering.
- Use focused system parameters instead of the crate-wide `too_many_arguments` allowance.
- Delete the old monolithic implementation and old root materials module.
- Preserve asset paths, state transitions, world bounds, tile distribution, XP thresholds, skill-point spending, tweens, camera, UI, and click behavior.

## Verification

- Unit tests pin XP rollover, radius-three world bounds, and tile-kind selection with `WyRand` seed `[7; 8]`.
- `evidence/baseline.png` and `evidence/post-initial.png` record the initial rendered result before and after the refactor.
- `evidence/post-first-placement.png` records first placement and neighboring ghosts.
- `evidence/post-flow.png` records two placed tiles after ten clicks granted and spent the next skill point.
- `evidence/post-split-flow.png` repeats the complete flow after the subsystem crate split.
- `evidence/REPORT.md` records the exact checks, actions, limitations, and observations.

## Done when

- `main.rs` only launches the app.
- Every system and type has one clear crate and module owner.
- Every package inherits workspace version, edition, and license metadata.
- No duplicate compatibility path remains.
- Pure gameplay rules have focused unit tests.
- Native tests, Clippy, formatting, wasm release build, and the rendered player flow pass.
