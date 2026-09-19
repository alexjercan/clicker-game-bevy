# Enforce the marked-comment-only code policy

- STATUS: OPEN
- PRIORITY: 88
- TAGS: foundation,quality

## User facts

- Ordinary code comments and public API documentation comments are forbidden.
- Allowed comments must carry an explicit marker such as NOTE, XXX, WTF, or TODO.
- Every TODO must refer to a Tatr task.
- Add a test that enforces the policy.

## Decisions to confirm during implementation

- Use the explicit marker allowlist NOTE, XXX, WTF, FIXME, and TODO(<task-id>), unless the owner narrows it.
- Apply the gate to tracked first-party source and executable build/config code. Exclude vendored assets, generated files, license text, Markdown prose, lockfiles, and required shebangs.

## Delivery

- Remove existing narrative and API documentation comments that only restate code.
- Rewrite comments that encode a real constraint with an approved marker.
- Convert each retained TODO to TODO(<existing-open-tatr-id>) and fail on missing or closed task IDs.
- Add a lexical test or checker that recognizes line, block, inner-doc, and outer-doc comments without treating strings, URLs, shader syntax, or shebangs as comments.
- Wire the checker into cargo test or the repository's standard check command and CI.
- Add fixtures that prove plain comments and untracked TODOs fail while each allowed form passes.

## Verification

- Run the policy test against the complete tracked scope.
- Mutate a fixture with // explanation, a doc comment, a block comment, and TODO(no-task), and prove each is rejected.
- Prove marker-like text inside a string does not satisfy or trigger the test.

## Done when

- The checked source tree contains no unmarked comment.
- Public items do not need doc comments.
- Every TODO comment names an existing open Tatr task.
- CI fails with a path and line number for any new violation.
