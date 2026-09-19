# Clicker Game Bevy

A small hex-grid clicker game built with Bevy. Place tiles, click them to earn XP, and spend skill points to expand the world.

## Quickstart

```console
nix run
```

## Controls

- Move the pointer over a tile or an open hex to select it.
- Left-click an open hex to spend a skill point and place a tile.
- Left-click a placed tile to earn XP.
- In a debug build, press `F11` to toggle the FPS text, world inspector, and cursor gizmo.

## Development

Enter the Nix development shell and run the game:

```console
nix develop
cargo run
```

Run with debug presentation:

```console
cargo run --features debug
```

Build the WebAssembly launcher:

```console
trunk build --release
```

This repository contains only the game and its WebAssembly launcher. It does not contain a separate website or landing page.

### Without Nix

Install the Rust toolchain declared in [`rust-toolchain.toml`](rust-toolchain.toml). Native Linux builds also need `pkg-config`, ALSA and udev development files, a supported X11 or Wayland environment, and Vulkan loader and driver support. Web builds need [Trunk](https://trunkrs.dev/) and the `wasm32-unknown-unknown` Rust target.

Then run:

```console
cargo run
```

## Checks

Run project commands from the repository root:

```console
nix develop --command cargo fmt --all -- --check
nix develop --command cargo test --locked --workspace --all-features
nix develop --command cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
nix develop --command cargo build --locked --release --no-default-features
nix develop --command trunk build --release --locked
```

## Probe reports

The `dev` feature provides the native probe host. This command runs every probe example without rendering and writes an aggregate HTML report:

```console
nix develop --command cargo run --features dev -- \
  probe all --norender --seed 7 --out target/probe-all
```

Open `target/probe-all/report.html`. Omit `--norender` to collect rendered frame timing. Short examples are repeated as separate processes when possible; reports show `N/A` when they cannot provide enough post-startup samples.

## Credits and license

Asset attribution and third-party license information are in [`credits/CREDITS.md`](credits/CREDITS.md). Project code and project-owned assets are available under the [`MIT License`](LICENSE).
