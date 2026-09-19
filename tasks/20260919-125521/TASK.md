# Separate source art and ship KayKit asset credits

- STATUS: OPEN
- PRIORITY: 85
- TAGS: assets,licensing

## User facts

- Move files that belong in ./art rather than runtime assets.
- Credit the asset source at https://kaylousberg.itch.io/kaykit-medieval-hexagon.
- Generate license and credits output in the same general way as Nova Protocol.

## Agent findings

- The repository currently has no art directory.
- Runtime assets contain the full KayKit Medieval Hexagon glTF tree, repeated texture files, a shader, and undefined.png.
- credits/CREDITS.md currently credits only the Bevy icon.
- Nova keeps editable/source packs under art, shipped runtime files under assets, hand-maintained asset attribution under credits, and generated Rust dependency notices via cargo-about.

## Delivery

- Inventory each tracked asset and classify it as source pack, editable source, runtime input, generated runtime output, or project-owned file.
- Keep only files loaded by the game or required by packaging under assets. Move source-pack material and non-runtime originals to art/kaykit-medieval-hexagon without breaking glTF buffer and texture references.
- Record the exact KayKit author, pack URL, license text, acquisition/source details available in the pack, shipped subset, and any transformations. Do not guess a license; verify it from the distributed pack or authoritative page before committing.
- Expand credits/CREDITS.md as the single source of truth for the Bevy icon, KayKit pack, project-owned assets, and dependency licenses.
- Add cargo-about configuration, template, and a generation script for third-party Rust licenses. Generate at check/release time rather than committing unstable generated output.
- Make native and wasm distributions include credits and all required asset license files.

## Verification

- Run an asset-reference audit and launch the game through loading into Playing with no missing asset.
- Run the license generator and fail on unknown or unapproved dependency licenses.
- Inspect native and wasm release archives for credits and required license texts.

## Done when

- art contains sources that do not ship, assets contains the minimal runtime set, and no loaded URI is broken.
- KayKit Medieval Hexagon is accurately attributed with its license.
- Dependency license generation is reproducible and gated in CI/release packaging.
