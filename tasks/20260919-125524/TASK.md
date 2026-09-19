# Write the project README and changelog

- STATUS: OPEN
- PRIORITY: 50
- TAGS: docs,release

## User facts

- Add a useful README quickstart and a CHANGELOG.
- Do not add project web pages yet.

## Delivery

- Rewrite README.md with the game's current premise, controls, native quickstart, Nix commands, non-Nix prerequisites, checks, debug/probe commands when available, asset attribution link, and license link.
- Add CHANGELOG.md in Keep a Changelog style with an Unreleased section and an entry for this modernization work. Do not invent historical releases.
- Document only commands and behavior that exist after the preceding tasks.
- State that no standalone website or landing page is part of the repository. Do not copy Nova's web, book, news, or marketing structure.
- Link to credits rather than duplicating license details.

## Verification

- Execute every quickstart and check command on a clean checkout or clean shell.
- Check all local links and file paths.
- Confirm the README does not advertise unimplemented gameplay or tooling.

## Done when

- A new contributor can build, run, test, and find credits from README.md.
- CHANGELOG.md accurately describes only shipped or Unreleased changes.
- No new website source, generated site, or site deployment is added.
