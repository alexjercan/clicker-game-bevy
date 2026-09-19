# Enforce the marked-comment-only code policy

- STATUS: CLOSED
- PRIORITY: 88
- TAGS: foundation, quality

## User facts

- Ordinary code comments and public API documentation comments are forbidden.
- Allowed comments must carry an explicit marker such as NOTE, XXX, WTF, or TODO.
- Every TODO must refer to a Tatr task.
- Add a test that enforces the policy.

## Decisions

- Use the project allowlist: NOTE, XXX, WTF, and TODO(<open-task-id>). FIXME and bare TODO are forbidden.
- Check tracked and new first-party Rust, JavaScript, CSS, Windows resource, WGSL, TOML, YAML, shell, Nix, and HTML files.
- Exclude vendored assets, generated files, license text, Markdown prose, lockfiles, and required shebangs.
- Reject Rust inner and outer documentation comments even when their text begins with an allowed marker.

## Implementation notes

- Added a cargo integration test with lexical scanners for slash, hash, Nix block, and HTML comments.
- The scanner skips quoted and Rust raw strings, URLs, shader directives, and required shebangs.
- TODO validation reads current Tatr status and accepts only existing OPEN task IDs.
- Removed narrative comments from the release workflow and web audio bootstrap.
- The existing CI test command runs the policy test through cargo test.

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

## Verification results

- The initial repository scan rejected the release workflow comment and all narrative web audio comments with paths and line numbers.
- Fixture tests reject plain line comments, block comments, Rust documentation comments, and TODOs without an OPEN task.
- Fixture tests accept every allowed marker and ignore marker-like text in strings.
- Passed cargo test --all-features, cargo clippy --all-targets --all-features with warnings denied, rustfmt, and git diff validation.

## Done when

- The checked source tree contains no unmarked comment.
- Public items do not need doc comments.
- Every TODO comment names an existing open Tatr task.
- CI fails with a path and line number for any new violation.
