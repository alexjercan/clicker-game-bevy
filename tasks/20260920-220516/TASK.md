# Polish presentation feedback and effect controls

- STATUS: OPEN
- PRIORITY: 55
- TAGS: game-feel, accessibility, polish

## User facts

- Create a follow-up polish task directly on `master`.
- Include the remaining polish identified after reviewing the camera, audio, glue, render, and particle PR stack.

## Dependencies

- Follow `#1` camera, `#2` audio, `#3` glue modules, `#5` render subsystem, and `#7` particles.
- Treat those PRs as one stack and build this work from the integrated result.
- Coordinate HUD-owned level-up presentation with `20260919-195719` instead of duplicating that task's UI foundation work.

## Agent findings

- Resolved tile clicks already receive animation, tile-specific sound, camera shake, and tile-specific particles in the open stack.
- Clicking a ghost without a skill point is silently ignored.
- Level-up has an audio cue but no distinct visual response.
- Newly revealed ghosts appear without an entrance transition.
- Audio, camera, and particle subsystems expose settings, but reduced-motion behavior and user-facing effect controls are not complete.
- Rapid input still needs fatigue tuning, bounded transient-effect policies, and native and wasm profiling.
- Placement particles currently begin at logical placement while the tile spawn animation is still in progress, so placement and settlement timing needs rendered review.

## Decisions

- Do not add more ordinary tile-click effects unless rendered evidence shows a specific readability defect.
- Keep every effect presentation-only. Effects must not change progression, placement, random generation, or settlement outcomes.
- Use a distinct placement-denied message instead of inferring denial in presentation systems.
- Keep effect categories independently disableable. Reduced motion must preserve interaction and gameplay state.
- Do not delay or require `TileSettled` for optional presentation effects.
- Do not add a generic feedback-router abstraction unless concrete duplicate delivery cannot be solved within the subsystem glue owners.

## Delivery

1. Add explicit denied-placement feedback when a ghost is activated with no skill points.
   - Emit one semantic denial message and no `PlaceTile` request.
   - Provide a restrained visual response and audio cue without mutating gameplay state.
2. Add one bounded level-up visual response.
   - Coordinate any XP-bar, banner, or skill-point indicator work with `20260919-195719`.
   - Keep the existing level-up rule and audio cue unchanged.
3. Animate newly revealed ghosts with a short fade or scale transition.
   - Do not make ghost availability or world mutation depend on animation completion.
4. Complete effect accessibility behavior and controls.
   - Cover mute and SFX volume, camera shake, particles, animation, and reduced motion.
   - Define whether settings last only for the current session. Do not add save persistence without an approved save model.
5. Review placement feedback timing against `TilePlaced` and `TileSettled`.
   - Keep a clear start and finish without duplicate particle, sound, or animation cues.
6. Add bounded burst behavior for rapid interaction.
   - Limit or coalesce transient particles and audio where rendered and listening evidence shows overload.
   - Preserve camera impulse caps and exact return to the base transform.
7. Remove stale effect entities, animation markers, and obsolete glue paths exposed by the completed work.

## Risks

- Starting before the five-PR stack lands will cause conflicts in core glue, render ownership, audio, particles, and lockfiles.
- UI changes can overlap `20260919-195719`.
- Excess denial, hover, or selection feedback can become noisy under rapid pointer movement.
- Compile and headless tests cannot prove motion, timing, sound balance, contrast, or wasm performance.

## Verification

- Add a focused gameplay test proving a denied placement emits one denial message, emits no placement request, consumes no skill point, and creates no tile.
- Add focused Bevy tests proving each logical action requests each intended presentation cue once.
- Verify disabled effects and reduced motion produce the same deterministic gameplay result as fully enabled effects.
- Verify no-render mode does not block placement or settlement.
- Record rendered evidence for denial, level-up, ghost reveal, and placement start and finish.
- Listen to denial, placement, selection, click, and level-up flows during normal and rapid interaction.
- Measure active effect entities and frame time during representative bursts on native and wasm against an approved budget.
- Run formatting, workspace tests, clippy, native release, and Trunk release checks.

## Done when

- Denied placement has clear and restrained feedback.
- Level-up has one distinct visual response without duplicating HUD ownership.
- Newly revealed ghosts enter coherently without affecting world state.
- Players can independently disable sound, shake, particles, and nonessential motion, including through reduced motion.
- Placement feedback has intentional start and settlement timing with no duplicate cues.
- Rapid interaction remains bounded and readable on native and wasm.
- Effects remain presentation-only and all temporary presentation state is cleaned up.
