---
name: clicker-assets
description: Change Clicker runtime assets, source art, credits, and license records safely.
---

# Clicker Assets

Follow [AGENTS.md](../../../AGENTS.md). Read the loading handles, glTF files,
credits, and release paths before moving an asset.

## Ownership

- `assets/` contains files loaded or shipped at runtime.
- `art/` contains original packs and editable sources that do not ship.
- `credits/` contains the attribution source of truth and shipped license text.
- `build/` contains application icons and platform packaging inputs.

Inventory each changed file as source, runtime input, generated output, or
license material. Search code and glTF URI references before moving it. Preserve
relative `.bin` and texture references. Verify author, source URL, license, and
required notices from the pack or an authoritative source. Do not guess.

After a move, run the game through asset loading and the affected scene. Inspect
the release paths when attribution or packaging changes. Use the repository's
license generator only after it exists; until then, do not claim dependency
license output is generated.

Do not add a gallery, asset webpage, or other site. Link durable attribution
from `credits/CREDITS.md` rather than duplicating it.
