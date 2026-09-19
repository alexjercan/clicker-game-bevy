# Migrate the Nix flake to flake-parts and rust-flake

- STATUS: CLOSED
- PRIORITY: 95
- TAGS: foundation, nix

## User facts

- flake.nix must use the same flake-parts and rust-flake approach as Nova Protocol.
- The resulting package and development shell must keep working.

## Agent findings

- The current flake uses flake-utils, rust-overlay, and nixos-24.11.
- Nova imports rust-flake's default and nixpkgs modules, declares four systems, defines one pinned Rust toolchain, and wraps the native package with assets and runtime libraries.
- This game loads assets at runtime and needs Linux graphics, window, audio, and input libraries.

## Delivery

- Replace flake-utils wiring with flake-parts plus rust-flake.
- Pin nixpkgs and one Rust toolchain. Keep flake.nix, rust-toolchain.toml, CI, and the wasm target aligned.
- Build the root game crate through rust-flake and expose a Linux wrapped package, default app, and default development shell.
- Package assets and credits beside the binary. Set BEVY_ASSET_ROOT and runtime library paths in the wrapper.
- Include only tools this repository uses. Do not copy Nova's website, mdBook, modding, profiling, or release-only tools without an owner.
- Refresh flake.lock and delete obsolete flake inputs.

## Verification

- Run nix flake check.
- Run nix build and launch a bounded smoke path from the result once the runtime harness exists; before that, verify package layout and --help or an equivalent noninteractive command.
- Run nix develop --command cargo check --locked.
- Confirm the package contains assets and credits and does not depend on the working tree.

## Done when

- flake show exposes the intended package, app, check, and shell on supported systems.
- The shell can compile native and wasm targets.
- The packaged game resolves its runtime assets and shared libraries outside the repository.

## Completion

Replaced flake-utils and the direct rust-overlay wiring with flake-parts and
rust-flake. Pinned nightly 2026-07-03 in the flake, rust-toolchain.toml, CI,
deployment, and release workflows. The development shell includes native and
wasm Rust targets, Cargo tools, Trunk, and the Linux runtime libraries.

The Linux package builds the dist profile without default development features,
wraps the executable with its runtime libraries and BEVY_ASSET_ROOT, and links
store-owned assets and credits under share/clicker. The flake exposes the
wrapped and unwrapped packages, default app, package check, and development
shells. The lock now contains only the root flake-parts, nixpkgs, and rust-flake
inputs.

Verification completed:

- `nix flake show --all-systems`
- `nix flake check --print-build-logs`
- `nix build --print-build-logs`
- `nix develop --command cargo check --locked`
- `nix develop --command cargo check --locked --target wasm32-unknown-unknown --no-default-features`
- An eight-second packaged launch from `/tmp` reached the completed asset-loading
  state before the bounded timeout.
- Package inspection found the expected asset and credit files, all package
  references point into the Nix store, and no working-tree path is embedded.

The first package smoke attempt exposed a missing Bevy dynamic library because
the package inherited the default development feature. The package now builds
with `--no-default-features`; the repeated smoke launch succeeded.
