# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). No released versions are recorded yet.

## [Unreleased]

### Added

- A generic, bounded autopilot for deterministic gameplay examples.
- Rendered and no-render production app construction.
- Seeded tile simulation through `CLICKER_SEED`.
- Probe timelines, snapshots, continuous invariants, timing statistics, Chrome traces, and self-contained HTML reports.
- A native probe host with commands for one example or all examples.
- Asset attribution and shipped third-party license records.

### Changed

- Split shared lifecycle state, tile simulation, animation, gameplay, presentation glue, probes, and debug tooling into focused crates.
- Kept tile simulation independent of windows, rendering, animation, assets, and lifecycle state.
- Made root default features empty. The `debug` feature now controls in-game debug presentation, while `dev` enables development and probe tooling.
- Replaced custom debug performance text with Bevy's FPS overlay and a unified `F11` toggle.
- Updated native and WebAssembly packaging to include runtime assets and credits.
