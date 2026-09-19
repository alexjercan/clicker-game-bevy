use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Copy)]
enum Syntax {
    Slash,
    Hash,
    Nix,
    Html,
}

#[derive(Debug, PartialEq, Eq)]
struct Violation {
    line: usize,
    reason: String,
}

fn syntax(path: &Path) -> Option<Syntax> {
    match path.extension().and_then(|value| value.to_str())? {
        "rs" | "js" | "css" | "rc" | "wgsl" => Some(Syntax::Slash),
        "toml" | "yaml" | "yml" | "sh" => Some(Syntax::Hash),
        "nix" => Some(Syntax::Nix),
        "html" => Some(Syntax::Html),
        _ => None,
    }
}

fn line_at(source: &str, offset: usize) -> usize {
    source.as_bytes()[..offset]
        .iter()
        .filter(|byte| **byte == b'\n')
        .count()
        + 1
}

fn validate_comment(
    body: &str,
    line: usize,
    documentation: bool,
    open_tasks: &HashSet<String>,
    violations: &mut Vec<Violation>,
) {
    if documentation {
        violations.push(Violation {
            line,
            reason: "documentation comments are forbidden".into(),
        });
        return;
    }

    let text = body.trim_start();
    if text.starts_with("NOTE:") || text.starts_with("XXX:") || text.starts_with("WTF:") {
        return;
    }

    if let Some(todo) = text.strip_prefix("TODO(") {
        if let Some((id, _)) = todo.split_once("):") {
            if open_tasks.contains(id) {
                return;
            }
            violations.push(Violation {
                line,
                reason: format!("TODO names a missing or closed task: {id}"),
            });
            return;
        }
        violations.push(Violation {
            line,
            reason: "TODO must use TODO(<open-task-id>):".into(),
        });
        return;
    }

    violations.push(Violation {
        line,
        reason: "comment must start with NOTE:, XXX:, WTF:, or TODO(<open-task-id>):".into(),
    });
}

fn skip_quoted(bytes: &[u8], mut index: usize, quote: u8) -> usize {
    index += 1;
    while index < bytes.len() {
        if bytes[index] == b'\\' {
            index = (index + 2).min(bytes.len());
        } else if bytes[index] == quote {
            return index + 1;
        } else {
            index += 1;
        }
    }
    index
}

fn skip_rust_raw_string(bytes: &[u8], index: usize) -> Option<usize> {
    if bytes[index] != b'r' {
        return None;
    }

    let mut cursor = index + 1;
    while cursor < bytes.len() && bytes[cursor] == b'#' {
        cursor += 1;
    }
    if cursor >= bytes.len() || bytes[cursor] != b'"' {
        return None;
    }

    let hashes = cursor - index - 1;
    cursor += 1;
    while cursor < bytes.len() {
        if bytes[cursor] == b'"'
            && cursor + hashes < bytes.len()
            && (hashes == 0
                || bytes[cursor + 1..=cursor + hashes]
                    .iter()
                    .all(|byte| *byte == b'#'))
        {
            return Some(cursor + hashes + 1);
        }
        cursor += 1;
    }
    Some(bytes.len())
}

fn scan_slash(source: &str, open_tasks: &HashSet<String>) -> Vec<Violation> {
    let bytes = source.as_bytes();
    let mut violations = Vec::new();
    let mut index = 0;

    while index < bytes.len() {
        if let Some(end) = skip_rust_raw_string(bytes, index) {
            index = end;
            continue;
        }
        if bytes[index] == b'"' || bytes[index] == b'`' {
            index = skip_quoted(bytes, index, bytes[index]);
            continue;
        }
        if bytes[index] == b'/' && bytes.get(index + 1) == Some(&b'/') {
            let start = index;
            let body_start = index + 2;
            let documentation = matches!(bytes.get(body_start), Some(b'/') | Some(b'!'));
            index = body_start;
            while index < bytes.len() && bytes[index] != b'\n' {
                index += 1;
            }
            validate_comment(
                &source[body_start..index],
                line_at(source, start),
                documentation,
                open_tasks,
                &mut violations,
            );
            continue;
        }
        if bytes[index] == b'/' && bytes.get(index + 1) == Some(&b'*') {
            let start = index;
            let body_start = index + 2;
            let documentation = matches!(bytes.get(body_start), Some(b'*') | Some(b'!'));
            let mut depth = 1;
            index = body_start;
            while index < bytes.len() && depth > 0 {
                if bytes[index] == b'/' && bytes.get(index + 1) == Some(&b'*') {
                    depth += 1;
                    index += 2;
                } else if bytes[index] == b'*' && bytes.get(index + 1) == Some(&b'/') {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    index += 2;
                } else {
                    index += 1;
                }
            }
            let body_end = index.min(bytes.len());
            validate_comment(
                &source[body_start..body_end],
                line_at(source, start),
                documentation,
                open_tasks,
                &mut violations,
            );
            index = (index + 2).min(bytes.len());
            continue;
        }
        index += 1;
    }

    violations
}

fn scan_hash(source: &str, open_tasks: &HashSet<String>, nix: bool) -> Vec<Violation> {
    let bytes = source.as_bytes();
    let mut violations = Vec::new();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'"' || bytes[index] == b'\'' {
            if nix && bytes[index] == b'\'' && bytes.get(index + 1) == Some(&b'\'') {
                index += 2;
                while index + 1 < bytes.len()
                    && !(bytes[index] == b'\'' && bytes[index + 1] == b'\'')
                {
                    index += 1;
                }
                index = (index + 2).min(bytes.len());
            } else {
                index = skip_quoted(bytes, index, bytes[index]);
            }
            continue;
        }
        if nix && bytes[index] == b'/' && bytes.get(index + 1) == Some(&b'*') {
            let start = index;
            let body_start = index + 2;
            index = body_start;
            while index + 1 < bytes.len() && !(bytes[index] == b'*' && bytes[index + 1] == b'/') {
                index += 1;
            }
            validate_comment(
                &source[body_start..index],
                line_at(source, start),
                false,
                open_tasks,
                &mut violations,
            );
            index = (index + 2).min(bytes.len());
            continue;
        }
        if bytes[index] == b'#' {
            let start = index;
            let body_start = index + 1;
            index = body_start;
            while index < bytes.len() && bytes[index] != b'\n' {
                index += 1;
            }
            if start == 0 && source[start..index].starts_with("#!") {
                continue;
            }
            validate_comment(
                &source[body_start..index],
                line_at(source, start),
                false,
                open_tasks,
                &mut violations,
            );
            continue;
        }
        index += 1;
    }

    violations
}

fn scan_html(source: &str, open_tasks: &HashSet<String>) -> Vec<Violation> {
    let mut violations = Vec::new();
    let mut offset = 0;
    while let Some(relative_start) = source[offset..].find("<!--") {
        let start = offset + relative_start;
        let body_start = start + 4;
        let end = source[body_start..]
            .find("-->")
            .map_or(source.len(), |relative_end| body_start + relative_end);
        validate_comment(
            &source[body_start..end],
            line_at(source, start),
            false,
            open_tasks,
            &mut violations,
        );
        offset = (end + 3).min(source.len());
    }
    violations
}

fn scan(source: &str, syntax: Syntax, open_tasks: &HashSet<String>) -> Vec<Violation> {
    match syntax {
        Syntax::Slash => scan_slash(source, open_tasks),
        Syntax::Hash => scan_hash(source, open_tasks, false),
        Syntax::Nix => scan_hash(source, open_tasks, true),
        Syntax::Html => scan_html(source, open_tasks),
    }
}

fn open_tasks(root: &Path) -> HashSet<String> {
    let mut tasks = HashSet::new();
    let Ok(entries) = fs::read_dir(root.join("tasks")) else {
        return tasks;
    };

    for entry in entries.flatten() {
        let path = entry.path().join("TASK.md");
        let Ok(body) = fs::read_to_string(path) else {
            continue;
        };
        if body.lines().any(|line| line == "- STATUS: OPEN") {
            if let Some(id) = entry.file_name().to_str() {
                tasks.insert(id.to_owned());
            }
        }
    }
    tasks
}

fn source_files(root: &Path) -> Vec<PathBuf> {
    let output = Command::new("git")
        .args([
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
        ])
        .current_dir(root)
        .output()
        .expect("git must be available for the comment policy test");
    assert!(output.status.success(), "git ls-files failed");

    output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .filter_map(|path| std::str::from_utf8(path).ok())
        .map(PathBuf::from)
        .filter(|path| syntax(path).is_some())
        .collect()
}

#[test]
fn repository_has_only_marked_comments() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tasks = open_tasks(&root);
    let mut failures = Vec::new();

    for relative in source_files(&root) {
        let path = root.join(&relative);
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", relative.display()));
        for violation in scan(&source, syntax(&relative).unwrap(), &tasks) {
            failures.push(format!(
                "{}:{}: {}",
                relative.display(),
                violation.line,
                violation.reason
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "comment policy violations:\n{}",
        failures.join("\n")
    );
}

#[test]
fn marked_comments_pass() {
    let tasks = HashSet::from(["20260919-125520".to_owned()]);
    let source = "// NOTE: constraint\n/* XXX: hazard */\n// WTF: verified behavior\n// TODO(20260919-125520): follow-up\n";
    assert!(scan(source, Syntax::Slash, &tasks).is_empty());
}

#[test]
fn plain_and_documentation_comments_fail() {
    let tasks = HashSet::new();
    for source in [
        "// explanation\n",
        "/* explanation */\n",
        "/// public docs\n",
        "//! crate docs\n",
        "/** public docs */\n",
        "/*! crate docs */\n",
    ] {
        assert_eq!(scan(source, Syntax::Slash, &tasks).len(), 1);
    }
}

#[test]
fn untracked_todo_fails() {
    let violations = scan(
        "// TODO(20000101-000000): missing\n",
        Syntax::Slash,
        &HashSet::new(),
    );
    assert_eq!(violations.len(), 1);
    assert!(violations[0].reason.contains("missing or closed"));
}

#[test]
fn comment_text_inside_strings_is_ignored() {
    let tasks = HashSet::new();
    let source = r##"
let url = "https://example.com/path";
let plain = "// explanation";
let block = r#"/* explanation */"#;
"##;
    assert!(scan(source, Syntax::Slash, &tasks).is_empty());
    assert!(scan("value = \"# explanation\"", Syntax::Hash, &tasks).is_empty());
    assert_eq!(
        scan(
            "let marker = \"// NOTE: not a comment\"; // explanation\n",
            Syntax::Slash,
            &tasks,
        )
        .len(),
        1
    );
}

#[test]
fn config_and_html_comments_follow_the_policy() {
    let tasks = HashSet::new();
    assert!(scan("# NOTE: constraint\n", Syntax::Hash, &tasks).is_empty());
    assert_eq!(scan("# explanation\n", Syntax::Hash, &tasks).len(), 1);
    assert!(scan("<!-- NOTE: constraint -->", Syntax::Html, &tasks).is_empty());
    assert_eq!(scan("<!-- explanation -->", Syntax::Html, &tasks).len(), 1);
}
