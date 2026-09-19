# Define Bevy project agent conventions and skills

- STATUS: OPEN
- PRIORITY: 100
- TAGS: foundation,agents

## User facts

- Add project-local AGENTS.md, CLAUDE.md, and skills for work on this Bevy game.
- Use Nova Protocol as the reference, but keep this project small.
- Do not add a website.
- Code comments are forbidden unless they carry an approved marker. TODO comments must name a Tatr task.

## Agent findings

- The repository has no project-local AGENTS.md, CLAUDE.md, or .agents/skills.
- Nova uses a short CLAUDE.md redirect and project skills for implementation, tasks, probes, review, content, and docs.
- Nova's public-API documentation rule conflicts with the requested no-comment experiment and must not be copied.

## Delivery

- Add root AGENTS.md with repository map, Nix-first commands, module/plugin conventions, evidence rules, asset ownership, no-website scope, and the exact comment policy.
- Add CLAUDE.md as a redirect to AGENTS.md.
- Add small project-local skills for implementation, Tatr task delivery, Bevy runtime verification, asset/license work, and review. Add agent metadata only where the harness supports it.
- Adapt useful Nova rules. Remove Nova names, large-project ceremony, mandatory public API docs, and unsupported commands.
- Make the default workflow preserve player behavior and require focused verification.

## Verification

- Validate every skill frontmatter and relative link.
- Confirm CLAUDE.md resolves to AGENTS.md and all documented commands exist in this repository.

## Done when

- A new agent can find the entry points, build and test commands, comment rules, and asset rules from AGENTS.md.
- Skills are concise, project-specific, and contain no Nova-only path or command.
- The docs explicitly reject adding a landing site or project web pages.
