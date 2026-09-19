# Separate source art and ship KayKit asset credits

- STATUS: CLOSED
- PRIORITY: 85
- TAGS: assets, licensing

## User facts

- Move files that belong in ./art rather than runtime assets.
- Credit the asset source at https://kaylousberg.itch.io/kaykit-medieval-hexagon.
- Generate license and credits output in the same general way as Nova Protocol.

## Agent findings

- The repository currently has no art directory.
- Runtime assets contain the full KayKit Medieval Hexagon glTF tree, repeated texture files, a shader, and undefined.png.
- credits/CREDITS.md currently credits only the Bevy icon.
- Nova keeps editable/source packs under art, shipped runtime files under assets, hand-maintained asset attribution under credits, and generated Rust dependency notices via cargo-about.

## Implementation notes

- Verified KayKit Medieval Hexagon Pack 1.0 FREE from the owner's downloaded archive. The distributed `License.txt` identifies Kay Lousberg as creator and distributor and licenses the pack under CC0 1.0.
- Recorded the archive name, acquisition date, creation date, source URL, and SHA-256 in `art/kaykit-medieval-hexagon/README.md`.
- Classified the five loaded glTF models, their five buffers, and three colocated textures as runtime KayKit inputs. The shader and question-mark image are project-owned runtime inputs.
- Moved all 442 unused KayKit glTF files to `art/kaykit-medieval-hexagon/gltf/`. The combined art and runtime trees are byte-identical to all 455 files in the archive's glTF tree. Duplicate FBX, Unity FBX, and OBJ formats were not added.
- Kept the Bevy-derived application icons under `build/` as packaging inputs and documented their MIT license.
- Added the exact distributed KayKit license to `credits/licenses/` and expanded `credits/CREDITS.md` as the attribution source of truth.
- Added cargo-about configuration, a template, a Nix-provided cargo-about tool, and `scripts/gen-licenses.sh`. The generated dependency notice is ignored and created with the locked release feature set.
- Added a CI license gate. Release and deployment workflows generate or download the notice before packaging credits. Native and web release builds now disable development defaults.

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

## Verification results

- The runtime asset audit found 15 files and resolved every glTF buffer and image URI.
- The downloaded and shipped KayKit license files compare byte-for-byte.
- cargo-about generated 7,840 lines of notices and rejected unapproved licenses during the initial default-feature probe.
- The native release smoke run reached Playing with all assets loaded.
- The staged native archive contained CREDITS, Bevy and KayKit asset licenses, and generated dependency notices, with no art tree.
- The Trunk release contained 15 runtime assets and the four expected credits files, with no art tree. wasm-opt accepted the release module with the workflow feature flags.
- Passed actionlint, cargo test with all features, clippy with warnings denied, rustfmt, the comment-policy gate, and diff validation.

## Done when

- art contains sources that do not ship, assets contains the minimal runtime set, and no loaded URI is broken.
- KayKit Medieval Hexagon is accurately attributed with its license.
- Dependency license generation is reproducible and gated in CI/release packaging.
