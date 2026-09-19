# Align CI and release checks with the modernized project

- STATUS: CLOSED
- PRIORITY: 45
- TAGS: ci,release

## User facts

- Bring the project up to the same engineering standard as Nova where relevant.
- Do not add a website.

## Agent findings

- CI currently runs broad Cargo checks on three operating systems.
- A deploy-page workflow publishes the wasm game through GitHub Pages.
- Release workflows build native and web artifacts but do not yet gate generated dependency licenses or the requested comment policy.

## Decisions

- Retain GitHub Pages only as a manually dispatched deployment of the playable wasm launcher. Do not add a separate site or landing page.
- Keep one Ubuntu quality job and separate native, wasm, dependency-license, and Nix jobs. Cross-platform release jobs remain responsible for platform builds.
- Use `rust-toolchain.toml` as the only Rust toolchain version source.
- Generate dependency licenses before packaging and reject any native or web staging tree that lacks assets, credits, the dependency-license manifest, or the project license.

## Delivery

- Make CI use the pinned toolchain and locked dependencies.
- Add focused gates for formatting, Clippy, tests, the marked-comment policy, dependency license generation, native build, Nix flake check, and wasm compilation.
- Keep job scope and cache keys bounded. Do not copy Nova's probe matrix until this project has enough probes to justify it.
- Ensure release archives contain runtime assets, credits, and required licenses.
- Do not add a landing site. Record and confirm whether the existing GitHub Pages deployment is retained strictly as the playable wasm build or removed; preserving current game features favors retaining wasm release artifacts while rejecting a separate project website.
- Remove obsolete CI paths after replacements are proven.

## Verification

- Run workflow-equivalent commands locally where possible.
- Inspect one native archive and one wasm archive.
- Confirm release builds exclude debug, autopilot, and probe host code.

## Done when

- CI blocks stale formatting, warnings, tests, unmarked comments, license-policy violations, Nix regressions, and wasm regressions.
- Release bundles are runnable and include legal notices.
- No website or landing-page pipeline has been introduced.
