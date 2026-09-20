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

## Decisions

- Start with subtle camera shake when an existing tile click resolves.
- Use a bounded additive impulse that decays and does not change gameplay state.
- Provide an enabled setting so camera shake can be disabled independently.
- Isolate shake on a dedicated camera child transform. The parent rig owns the
  camera pose, so other camera systems cannot be corrupted by shake.
- Use `bevy_hanabi` for the later particle work.
- Generate audio assets with the Nix-provided Python, NumPy, and SciPy environment.
- Use one click cue for each tile kind, one shared select/deselect cue, one ghost
  placement cue, and one level-up cue.
- Keep the frequently repeated select/deselect cue low-pitched and quieter than
  action cues.
- Drop a sound cue that Bevy cannot start, because Bevy skips its audio playback
  systems when the machine has no audio output stream.
- Keep gameplay-to-render adaptation in core glue and move mesh, scene, and tile
  decoration construction into a dedicated `clicker_render` subsystem.

## Decisions needed

- Approve an effect language, intensity range, and color palette beyond the initial camera shake and generated sound set.
- Decide reduced-motion, mute, volume, and how camera-shake controls appear in the UI.
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
