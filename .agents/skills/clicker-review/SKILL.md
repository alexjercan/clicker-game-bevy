---
name: clicker-review
description: Review a Clicker game change for correctness, regressions, scope, and evidence.
disable-model-invocation: true
---

# Clicker Review

Use only when the user requests a review. The request authorizes findings, not
edits. Follow [AGENTS.md](../../../AGENTS.md).

Resolve the review range first. Default to the current session change when it is
clear. Include tracked and untracked files when reviewing a worktree. Read the
related Tatr task when present.

Review these lanes:

- Contract: requested behavior, explicit rejections, public interfaces, and
  task completion conditions.
- Correctness: state, ordering, events, resources, asset handles, bounds,
  arithmetic, random behavior, native/wasm differences, and failure paths.
- Craft: module ownership, duplicate paths, stale code, dependency direction,
  comment-policy compliance, docs, credits, and maintainability.
- Runtime: loading, interaction, visual regressions, and whether the supplied
  evidence can prove each claim.

Search replaced behavior for aliases, adapters, old IDs, implicit defaults,
stale tests, stale comments, and obsolete documentation. Do not demand broad
checks when a focused one observes the behavior.

Report BLOCKER, MAJOR, or MINOR findings with exact path and line evidence.
Explain the consequence and why the rating is appropriate. List skipped checks
and residual risks. If there are no findings, say so without inventing work.
