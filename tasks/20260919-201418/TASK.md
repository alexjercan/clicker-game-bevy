# Add particles, audio, camera feedback, and animation juice

- STATUS: OPEN
- PRIORITY: 60
- TAGS: game-feel, audio, animation

## User facts

- The game needs more game feel and visual/audio feedback.
- Candidate work includes a `clicker_particles` subsystem, subtle camera shake, sound effects, and more animation.
- Keep this work separate from the current automation and architecture task.

## Agent findings

- `clicker_animation` currently owns selection, deselection, click, and tile-spawn tweens.
- Tile placement now has explicit logical placement and settled messages that effects can consume.
- The project has no particle owner, audio feedback policy, camera impulse API, effect intensity settings, or reduced-motion behavior.
- Effects must not own gameplay outcomes or change deterministic simulation state.

## Decisions needed

- Choose the first actions that need feedback: tile click, resource gain, level-up, purchase, placement, or world unlock.
- Approve an effect language, intensity range, color palette, and audio direction.
- Decide reduced-motion, mute, volume, and camera-shake controls.
- Decide whether particles use Bevy entities, a maintained particle dependency, or a small purpose-built pool after profiling wasm.
- Identify and license any external sound or visual assets before adding them to runtime assets.

## Delivery

- Define effect messages derived from gameplay and tile messages without coupling gameplay rules to rendering or audio.
- Add `clicker_particles` only when its first concrete effects and ownership boundary are approved.
- Add subtle bounded camera impulses with stacking, decay, maximum amplitude, and a disabled path.
- Add SFX routing with named cues, volume controls, and no hard-coded asset access in gameplay systems.
- Extend `clicker_animation` with purpose-specific bundles and completion messages only where another subsystem consumes completion.
- Provide no-render behavior that consumes or omits presentation effects without blocking gameplay settlement.
- Keep particles, audio, shake, and animation independently disableable for testing and accessibility.

## Verification

- Unit-test camera impulse accumulation, decay, caps, and disabled behavior.
- Use focused Bevy tests to prove gameplay messages request each effect once and presentation does not mutate gameplay state.
- Profile entity counts and frame time during effect bursts on native and wasm.
- Record rendered before/after evidence for every visual effect and listen to every SFX flow.
- Verify reduced-motion, shake-off, mute, and no-render paths.
- Run formatting, tests, clippy, native release, and Trunk release checks.

## Done when

- Core player actions have clear, coherent, and restrained feedback.
- Effects remain presentation-only and can be disabled independently.
- Native and wasm performance stays within an approved budget during representative bursts.
- Assets have verified licenses and shipped attribution where required.
