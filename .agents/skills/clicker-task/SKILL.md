---
name: clicker-task
description: Create and maintain Clicker game work as Tatr delivery specifications.
---

# Clicker Task

Use this skill when the user requests tracked work. Follow
[AGENTS.md](../../../AGENTS.md).

Keep one request and its follow-ups in one task. Record only facts the user gave
as user facts. Keep code findings and recommendations separate.

A useful task contains the sections needed from this list:

- `User facts`: requested behavior, constraints, and explicit rejections.
- `Decisions`: approved interfaces, names, defaults, and failure rules.
- `Agent findings`: code evidence, dependencies, and risks.
- `Delivery`: end state, deletion list, blast radius, and work sequence.
- `Verification`: the behavior or invariant and the proof that observes it.
- `Done when`: observable completion conditions.

Use Tatr from the repository root:

```bash
tatr new "Title" -p 100 -t backlog -b details.md
tatr ls --sort priority
tatr edit <id> --status CLOSED
```

Keep investigation notes, decisions, evidence, and reviews in the task
directory. Before closing, compare the implementation and evidence with `Done
when`. Record skipped checks. Do not cite a closed task from production code or
durable documentation.
