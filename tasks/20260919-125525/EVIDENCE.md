# Evidence

## Workflow gates

- `nix run nixpkgs#actionlint -- .github/workflows/ci.yml .github/workflows/release.yaml .github/workflows/deploy-page.yaml` passed.
- `nix develop --command cargo test --locked --test comment_policy` passed all six policy tests with the new workflows and release checker present.
- `nix flake check --print-build-logs` completed with `all checks passed` on x86_64-linux.
- Every Cargo build, check, Clippy, and test command in the workflows uses `--locked`. Trunk release builds use `--locked`.
- Workflow files no longer name a Rust channel or use `dtolnay/rust-toolchain`; `rustup show active-toolchain` resolves `rust-toolchain.toml`.

## Release contents

`scripts/gen-licenses.sh` completed and produced `credits/THIRD-PARTY-LICENSES.md`. Cargo About emitted upstream harvest warnings but returned success under the accepted-license policy.

A local native staging tree was built from `target/release/clicker`, `assets/`, `credits/`, and `LICENSE`. `scripts/check-release-contents.sh native` passed before and after creating and extracting `target/clicker_linux_test.tar.gz`.

`nix develop --command trunk build --release --locked` completed successfully. `scripts/check-release-contents.sh web dist` passed. The resulting files were zipped to `target/clicker_web_test.zip`, extracted, and checked again successfully. The web payload contains the launcher, wasm module, assets, credits, generated dependency licenses, and top-level MIT license.

`nix develop --command cargo tree --locked -p clicker --no-default-features -e normal` contains no `clicker_debug`, `clicker_probe`, or `clicker_autopilot` crate. Release and Trunk commands both disable default features.

macOS DMG and Windows zip creation were not run locally because those platform runners are unavailable. Their workflows validate staging contents before packaging; the Windows workflow also extracts and checks the produced zip. The Linux and web archives provide the requested local archive inspection.

## Scope

The existing manually dispatched GitHub Pages workflow remains. It deploys only `dist/`, the playable wasm launcher produced by Trunk. No landing page, marketing page, documentation site, or other website pipeline was added.
