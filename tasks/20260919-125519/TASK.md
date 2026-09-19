# Upgrade Bevy and the dependency stack

- STATUS: CLOSED
- PRIORITY: 90
- TAGS: foundation, rust

## User facts

- Update to the latest Bevy release and use relevant packages similar to Nova Protocol.
- Keep the current game features and player behavior unchanged.

## Agent findings

- The game is on Bevy 0.15.1 with bevy_asset_loader 0.22, bevy_tweening 0.12, leafwing-input-manager 0.16, hexx 0.20, rand 0.9, and a separate debug crate on the same old Bevy.
- Nova currently uses Bevy 0.19, Clap, bevy_rand, feature-gated debug tooling, and a pinned nightly.
- leafwing-input-manager appears declared but unused. All dependency owners must be confirmed before removal or replacement.

## Delivery

- Resolve the newest mutually compatible Bevy ecosystem versions at implementation time and record the chosen versions in task notes.
- Upgrade the root crate and debug crate together. Apply API migrations without changing gameplay, visuals, controls, asset paths, or startup behavior.
- Audit every direct dependency. Remove unused dependencies. Prefer Nova-aligned packages only where they have a concrete owner, such as Clap for native CLI parsing and bevy_rand for seeded ECS randomness.
- Keep release dependencies separate from debug and test dependencies. Keep native-only tooling out of wasm.
- Refresh Cargo.lock and add a pinned Rust toolchain if required by the chosen Bevy release.

## Implementation notes

- Selected Bevy 0.19.1, the latest stable release at implementation time. Bevy 0.20.0 was still a release candidate.
- Selected bevy_asset_loader 0.27.0, bevy_tweening 0.16.0, bevy_rand 0.15.2, hexx 0.25.0, and bevy-inspector-egui 0.37.0 as the compatible ecosystem set.
- Centralized Bevy in the workspace and migrated both crates together.
- Replaced rand and getrandom with bevy_rand using WyRand. rand_core remains as the direct owner of the RNG trait API.
- Removed unused leafwing-input-manager. Removed iyes_perf_ui because its stable release does not support Bevy 0.19, and retained the F12 performance display with a small Bevy diagnostics overlay.
- Updated embed-resource for the Windows icon build. Every remaining direct dependency has a source, asset-loading, gameplay, debug, wasm, or build-script owner.
- Cargo.lock pins encase, encase_derive, and encase_derive_impl to 0.12.0 because encase 0.12.2 is incompatible with the current syn 3 API.

## Verification

- Run cargo check --locked for default and release feature sets.
- Run cargo test --locked for affected crates.
- Build wasm32-unknown-unknown without native debug tools.
- Compare a deterministic baseline flow before and after the migration once the harness task is available.

## Verification results

- Passed locked default and release checks.
- Passed workspace tests and rustfmt.
- Passed the locked wasm32 check without default features.
- Passed nix build and nix flake check.
- Native debug and packaged release smoke runs reached the Playing state with assets loaded and no rendering errors.
- Deterministic before-and-after flow comparison remains deferred until the harness task is implemented.

## Done when

- One Bevy version is used across the workspace.
- Cargo reports no stale direct dependency that lacks a code owner.
- Native and wasm builds compile and the basic click, XP, skill-point, tile-spawn, loading, and material behavior remains unchanged.
