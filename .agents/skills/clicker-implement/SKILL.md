---
name: clicker-implement
description: Deliver a Clicker game change from code-backed scope through focused proof.
---

# Clicker Implement

Follow [AGENTS.md](../../../AGENTS.md). Use this skill by default.

## Ground the change

1. Read the Tatr task when one exists.
2. Find the owner, entry point, callers, state, assets, ordering, and closest test.
3. State the intended result, behavior that must stay unchanged, deletion list,
   likely breakage, and proof.
4. Resolve interface, naming, ownership, default, and failure-policy decisions
   before implementing them.

For a defect, reproduce the failure first. For uncertain Bevy behavior, use the
smallest disposable app or focused test that answers the question. Do not turn
uncertainty into an abstraction, fallback, or compatibility path.

## Deliver

1. Change the owning interface first.
2. Use compiler errors and searches to update every consumer.
3. Update affected systems, assets, UI, docs, credits, and task evidence.
4. Delete replaced code, aliases, stale tests, and obsolete paths.
5. Follow the marked-comment-only policy in AGENTS.md.
6. Run the smallest proof that observes the real behavior.

Use unit tests for pure rules. Use focused Bevy app tests for ECS behavior. Read
[Clicker Runtime](../clicker-runtime/SKILL.md) for loading, input, state, and
visual changes. A compile check is not gameplay evidence.
