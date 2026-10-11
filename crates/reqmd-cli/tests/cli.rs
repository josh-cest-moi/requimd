//! Integration tests; unwrap is fine outside user-input paths.
#![allow(clippy::unwrap_used)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn reqmd(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_reqmd"))
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap()
}

fn stdout(o: &Output) -> String {
    String::from_utf8(o.stdout.clone()).unwrap()
}

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn examples(name: &str) -> PathBuf {
    repo().join("examples").join(name)
}

/// A fresh, empty directory for one test.
fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("reqmd-test-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write(dir: &Path, name: &str, text: &str) {
    fs::write(dir.join(name), text).unwrap();
}

#[test]
fn fr_300_text_output_and_exit_code_1_on_errors() {
    let o = reqmd(&examples("broken"), &["check"]);
    assert_eq!(o.status.code(), Some(1));
    let out = stdout(&o);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 13, "{out}");
    assert!(
        lines.contains(&"broken.md:11:32: warning: unknown key \"colour\" for functional items; allowed: status, prio, impl, ref, ver [unknown-key]"),
        "{out}"
    );
    assert!(
        lines
            .iter()
            .all(|l| l.starts_with("broken.md:") && l.ends_with(']'))
    );
}

#[test]
fn fr_300_exit_code_0_without_errors() {
    let o = reqmd(&examples("shop"), &["check"]);
    assert_eq!(o.status.code(), Some(0));
    assert_eq!(stdout(&o), "");
}

#[test]
fn fr_300_exit_code_2_without_configuration() {
    let dir = temp_dir("noconfig");
    let o = reqmd(&dir, &["check"]);
    assert_eq!(o.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&o.stderr).contains("reqmd.toml"));
}

#[test]
fn fr_300_exit_code_2_on_configuration_error() {
    let dir = temp_dir("badconfig");
    write(&dir, "reqmd.toml", "[checks]\nno-such-check = \"info\"\n");
    assert_eq!(reqmd(&dir, &["check"]).status.code(), Some(2));
}

#[test]
fn fr_300_exit_code_2_on_usage_error() {
    let o = reqmd(&examples("shop"), &["check", "--format", "xml"]);
    assert_eq!(o.status.code(), Some(2));
    let o = reqmd(&examples("shop"), &["check", "missing.md"]);
    assert_eq!(o.status.code(), Some(2));
}

#[test]
fn fr_300_paths_filter_output_but_whole_project_is_loaded() {
    let dir = temp_dir("paths");
    write(&dir, "reqmd.toml", "");
    write(&dir, "a.md", "## UC-001 U\n\n- s1: Go.\n");
    write(
        &dir,
        "b.md",
        "## FR-001 F\nimpl: UC-001.s1\n\n## FR-? New\n",
    );
    let o = reqmd(&dir, &["check", "a.md"]);
    // The link in b.md covers the step, and b.md is not reported.
    assert_eq!(o.status.code(), Some(0));
    assert_eq!(stdout(&o), "");
    let o = reqmd(&dir, &["check", "b.md"]);
    assert!(stdout(&o).starts_with("b.md:4:1: warning: placeholder"));
    // Paths from different projects are a usage error.
    let other = examples("shop");
    let o = reqmd(&dir, &["check", "a.md", other.to_str().unwrap()]);
    assert_eq!(o.status.code(), Some(2));
}

#[test]
fn fr_300_project_root_found_from_subdirectory() {
    let o = reqmd(&repo().join("requirements").join("functional"), &["check"]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
}

#[test]
fn fr_301_json_output() {
    let o = reqmd(&examples("broken"), &["check", "--format", "json"]);
    assert_eq!(o.status.code(), Some(1));
    let v: serde_json::Value = serde_json::from_str(&stdout(&o)).unwrap();
    let arr = v.as_array().unwrap();
    assert_eq!(arr.len(), 13);
    let d = &arr[0];
    for key in [
        "file",
        "line",
        "column",
        "end_line",
        "end_column",
        "severity",
        "check",
        "message",
    ] {
        assert!(d.get(key).is_some(), "missing {key}");
    }
    assert_eq!(d["file"], "broken.md");
    let o = reqmd(&examples("shop"), &["check", "--format", "json"]);
    assert_eq!(stdout(&o).trim(), "[]");
}

#[test]
fn fr_302_strict_treats_warnings_as_errors() {
    let dir = temp_dir("strict");
    write(&dir, "reqmd.toml", "");
    write(&dir, "a.md", "## FR-? New\n");
    assert_eq!(reqmd(&dir, &["check"]).status.code(), Some(0));
    assert_eq!(reqmd(&dir, &["check", "--strict"]).status.code(), Some(1));
}

#[test]
fn fr_303_init_creates_files_and_never_overwrites() {
    let dir = temp_dir("init");
    let o = reqmd(&dir, &["init"]);
    assert_eq!(o.status.code(), Some(0));
    assert!(
        fs::read_to_string(dir.join("reqmd.toml"))
            .unwrap()
            .contains("[types.usecase]")
    );
    assert_eq!(fs::read_to_string(dir.join("glossary.md")).unwrap(), "");
    assert_eq!(reqmd(&dir, &["check"]).status.code(), Some(0));

    write(&dir, "reqmd.toml", "# mine\n");
    write(&dir, "glossary.md", "## Cart\n");
    let o = reqmd(&dir, &["init"]);
    assert_eq!(o.status.code(), Some(0));
    assert!(stdout(&o).contains("left unchanged"));
    assert_eq!(
        fs::read_to_string(dir.join("reqmd.toml")).unwrap(),
        "# mine\n"
    );
    assert_eq!(
        fs::read_to_string(dir.join("glossary.md")).unwrap(),
        "## Cart\n"
    );
}

#[test]
fn nfr_006_no_network_crates_in_dependency_tree() {
    let lock = fs::read_to_string(repo().join("Cargo.lock")).unwrap();
    let names: Vec<&str> = lock
        .lines()
        .filter_map(|l| l.strip_prefix("name = \""))
        .map(|l| l.trim_end_matches('"'))
        .collect();
    assert!(names.contains(&"pulldown-cmark"));
    for net in [
        "reqwest",
        "hyper",
        "ureq",
        "curl",
        "tokio",
        "async-std",
        "h2",
        "rustls",
        "native-tls",
        "openssl",
        "socket2",
        "mio",
    ] {
        assert!(!names.contains(&net), "network crate {net} in Cargo.lock");
    }
}

/// Test function names in all crates, lowercased.
fn test_names() -> Vec<String> {
    let mut names = Vec::new();
    let mut dirs = vec![repo().join("crates")];
    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                dirs.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let text = fs::read_to_string(&path).unwrap();
                let mut lines = text.lines().map(str::trim);
                while let Some(l) = lines.next() {
                    if l == "#[test]"
                        && let Some(f) = lines.next().and_then(|n| n.strip_prefix("fn "))
                    {
                        names.push(f.split('(').next().unwrap_or_default().to_lowercase());
                    }
                }
            }
        }
    }
    names
}

#[test]
fn nfr_010_every_m1_must_requirement_has_a_test() {
    // `must` items of M1 in docs/roadmap.md.
    let mut ids: Vec<String> = Vec::new();
    ids.extend((1..=10).chain(12..=14).map(|n| format!("fr_{n:03}")));
    ids.extend((100..=109).chain([115, 117]).map(|n| format!("fr_{n:03}")));
    ids.extend((300..=303).map(|n| format!("fr_{n:03}")));
    ids.extend(["nfr_002", "nfr_004", "nfr_006", "nfr_008", "nfr_010"].map(String::from));
    ids.extend(["br_001", "br_003", "br_005"].map(String::from));
    let names = test_names();
    let missing: Vec<&String> = ids
        .iter()
        .filter(|id| {
            !names.iter().any(|n| {
                n.starts_with(&format!("{id}_"))
                    || n.contains(&format!("_{id}_"))
                    || n.ends_with(&format!("_{id}"))
            })
        })
        .collect();
    assert!(
        missing.is_empty(),
        "requirements without a test: {missing:?}"
    );
}

#[test]
fn nfr_008_shop_metadata_blocks_end_with_blank_line() {
    // A metadata block directly followed by text would render as one
    // paragraph with the body on GitHub; a blank line keeps them apart.
    for entry in fs::read_dir(examples("shop")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|e| e != "md") {
            continue;
        }
        let text = fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        for (i, l) in lines.iter().enumerate() {
            if !l.starts_with("## ") {
                continue;
            }
            let mut j = i + 1;
            while lines
                .get(j)
                .is_some_and(|n| n.contains(": ") && !n.starts_with('-'))
            {
                j += 1;
            }
            assert!(
                lines.get(j).is_none_or(|n| n.trim().is_empty()),
                "{}:{} metadata not followed by a blank line",
                path.display(),
                j + 1
            );
        }
    }
}
