# AGENTS.md

`~/AGENTS.md` applies. Keep this project small. Preserve the current game unless
a task explicitly changes player-visible behavior.

## Project paths

- `assets/` contains files loaded or shipped by the game.
- `art/` contains art candidates, external source material, original packs, and
  editable sources that are not part of the game build.
- `credits/` contains attribution and license texts shipped with the game.
- `tasks/` contains Tatr specifications, evidence, and reviews.

## Commands

Run project commands through the Nix development shell from the repository root:

```bash
nix develop --command cargo run
nix develop --command cargo test --all-features
nix develop --command cargo clippy --all-targets --all-features -- -D warnings
nix develop --command cargo fmt --all -- --check
nix develop --command trunk build --release
```

Use the smallest affected check during implementation. Run broader checks before
closing a task when its blast radius requires them.

## Work

- Start with `clicker-implement` unless the user requests another mode.
- Read the task, owning code, callers, assets, and closest checks before editing.
- State the intended result, preserved behavior, deletion list, and risks.
- Stop for unresolved interfaces, defaults, ownership, failure policy, or scope.
- Keep one requested change and its follow-ups in one Tatr task.
- Do not add speculative compatibility, abstractions, content, or gameplay.
- Delete replaced paths after all callers move.
- Update invalidated README, changelog, credits, and task evidence with the code.

## Architecture

- Keep `main.rs` thin. Put reusable app construction and game behavior in the
  library.
- Give each subsystem one module owner. Use `<Subsystem>Plugin` for plugin types
  and `<Subsystem>Systems` for system sets.
- Register plugins in dependency order: Bevy, assets/loading, gameplay, UI, then
  debug tooling.
- State cross-plugin ordering with system sets instead of relying on insertion
  order.
- Keep components, resources, events, and constants private unless another
  module needs them.
- Put shared types in the lowest module used by every consumer.
- Prefer bundles and focused system parameters over crate-wide lint allowances.
- Keep native-only CLI and debug dependencies out of wasm and release builds.

## Comment policy

Do not add ordinary comments or Rust documentation comments, including comments
on public items. A code comment is allowed only when its first text is one of:

- `NOTE:` for a concrete constraint or non-obvious reason.
- `XXX:` for a hazardous but intentional implementation.
- `WTF:` for verified behavior that cannot yet be explained cleanly.
- `TODO(<task-id>):` for tracked removal or follow-up work.

A TODO task ID must name an existing OPEN task under `tasks/`. Delete the comment
when that task closes. Do not use `FIXME`, bare `TODO`, narrative headings,
history, commented-out code, or comments that restate the code. This policy
applies to first-party source and executable build/config code. It does not
apply to Markdown prose, license text, generated files, vendored assets,
lockfiles, or required shebangs.

## Evidence

- Reproduce a defect before fixing it and preserve useful before/after evidence.
- Use unit tests for pure rules and focused Bevy app tests for ECS behavior.
- Use `clicker-runtime` for loading, input, state, or visual changes.
- A successful build proves compilation only.
- Headless output cannot prove layout, materials, animation, lighting, or other
  visual claims. Inspect rendered output for those claims.
- Report the exact commands run and any checks skipped.
- Do not add tests for prose, implementation details, or raw coverage.

## Assets and licenses

- Keep only runtime inputs under `assets/`.
- Keep original packs, Blender files, and other editable sources under `art/`.
- Preserve relative glTF buffer and texture references when moving files.
- Treat `credits/CREDITS.md` as the attribution source of truth.
- Verify an asset license from the distributed pack or an authoritative source.
  Never infer one from a download page summary.
- Ship required attribution and license files with native and wasm releases.
- Do not hand-edit generated dependency-license output.

## Scope

Do not add a landing site, marketing site, documentation site, news page, or
other project web page. The existing HTML is only a launcher for the wasm game.
Do not copy Nova Protocol's website or book infrastructure.
