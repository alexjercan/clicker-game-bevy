# Refactor evidence

## Baseline

Before editing, `nix develop --command cargo test --all-features` passed with no game unit tests.

The original game was launched at 1920x1080. `baseline.png` records the loaded initial ghost, camera, top bar, lighting, fading material, and debug presentation. `baseline.log` records successful asset loading with no panic or error.

## Runtime comparison

The refactored game was launched at the same resolution and feature set.

- `post-initial.png` shows the same initial ghost, camera framing, top bar, lighting, fading material, and debug presentation.
- `post-first-placement.png` shows the first center tile and its six neighboring ghosts after one click.
- The full flow moved the pointer to the center ghost, clicked once, clicked the resulting tile ten times, moved to the right neighboring ghost, and clicked once.
- `post-flow.png` shows two placed tiles. The second placement demonstrates that ten tile clicks granted the next skill point and that the point could be spent.
- `post-flow.log` records successful asset loading and no error or panic.
- `post-split-flow.png` and `post-split-flow.log` repeat that complete flow after assets, animation, gameplay, and UI moved into separate crates.

The runtime still draws from global entropy, so this interactive flow has no reproducible seed. The tile-kind unit test uses `WyRand` seed `[7; 8]` and pins the resulting sequence.

## Checks

- `nix develop --command cargo test --workspace --all-features`
- `nix develop --command cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `nix develop --command cargo fmt --all -- --check`
- `nix develop --command trunk build --release`
- `git diff --check`

All checks passed. The permanent deterministic player-flow harness remains scoped to task `20260919-125523`.
