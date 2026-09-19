# Credits

This directory is the source of truth for asset attribution and shipped license
texts. Native releases bundle it beside the executable. Web builds copy it to
`dist/credits/`.

The game's code and project-owned assets are covered by the top-level
[`LICENSE`](../LICENSE).

## Project-owned assets

- The skill-point question-mark image (`assets/undefined.png`) is original to
  the project.
- The fading shader (`assets/shaders/fading.wgsl`) is original to the project.

## Third-party assets

- **Bevy icon** (`build/icon_1024x1024.png` and its macOS and Windows
  derivatives) - from the Bevy project under the [MIT
  License](licenses/Bevy_MIT_License.md).
- **KayKit Medieval Hexagon Pack 1.0 FREE** - created and distributed by Kay
  Lousberg, downloaded from
  [itch.io](https://kaylousberg.itch.io/kaykit-medieval-hexagon) on 2026-09-19,
  and released under [CC0 1.0](licenses/KayKit_Medieval_Hexagon_CC0-1.0.txt).
  Attribution is not required and is provided as a courtesy. The game ships
  the unmodified glTF models `hex_grass`, `trees_A_large`, `hills_A`,
  `building_dirt`, and `building_grain`, with their buffers and textures. The
  remaining unmodified glTF exports are retained as non-shipping source art in
  `art/kaykit-medieval-hexagon/`.

## Third-party code

Rust dependencies and their license texts are listed in
`THIRD-PARTY-LICENSES.md`. That file is generated from `Cargo.lock` with
cargo-about at check and release time. It is not committed because generated
cargo-about output is not byte-stable. Run `scripts/gen-licenses.sh` to create
it locally.
