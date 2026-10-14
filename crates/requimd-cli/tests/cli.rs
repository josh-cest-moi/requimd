//! Integration tests; unwrap is fine outside user-input paths.
#![allow(clippy::unwrap_used)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn requimd(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_requimd"))
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
    let dir = std::env::temp_dir().join(format!("requimd-test-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write(dir: &Path, name: &str, text: &str) {
    fs::write(dir.join(name), text).unwrap();
}

#[test]
fn fr_300_text_output_and_exit_code_1_on_errors() {
    let o = requimd(&examples("broken"), &["check"]);
    assert_eq!(o.status.code(), Some(1));
    let out = stdout(&o);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 22, "{out}");
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
    let o = requimd(&examples("shop"), &["check"]);
    assert_eq!(o.status.code(), Some(0));
    assert_eq!(stdout(&o), "");
}

#[test]
fn fr_300_exit_code_2_without_configuration() {
    let dir = temp_dir("noconfig");
    let o = requimd(&dir, &["check"]);
    assert_eq!(o.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&o.stderr).contains("requimd.toml"));
}

#[test]
fn fr_300_exit_code_2_on_configuration_error() {
    let dir = temp_dir("badconfig");
    write(&dir, "requimd.toml", "[checks]\nno-such-check = \"info\"\n");
    assert_eq!(requimd(&dir, &["check"]).status.code(), Some(2));
}

#[test]
fn fr_300_exit_code_2_on_usage_error() {
    let o = requimd(&examples("shop"), &["check", "--format", "xml"]);
    assert_eq!(o.status.code(), Some(2));
    let o = requimd(&examples("shop"), &["check", "missing.md"]);
    assert_eq!(o.status.code(), Some(2));
}

#[test]
fn fr_300_paths_filter_output_but_whole_project_is_loaded() {
    let dir = temp_dir("paths");
    write(&dir, "requimd.toml", "");
    write(&dir, "a.md", "## UC-001 U\n\n- s1: Go.\n");
    write(
        &dir,
        "b.md",
        "## FR-001 F\nimpl: UC-001.s1\n\n## FR-? New\n",
    );
    let o = requimd(&dir, &["check", "a.md"]);
    // The link in b.md covers the step, and b.md is not reported.
    assert_eq!(o.status.code(), Some(0));
    assert_eq!(stdout(&o), "");
    let o = requimd(&dir, &["check", "b.md"]);
    let out = stdout(&o);
    assert!(out.contains("b.md:4:1: warning: placeholder"), "{out}");
    assert!(out.lines().all(|l| l.starts_with("b.md:")), "{out}");
    // Paths from different projects are a usage error.
    let other = examples("shop");
    let o = requimd(&dir, &["check", "a.md", other.to_str().unwrap()]);
    assert_eq!(o.status.code(), Some(2));
}

#[test]
fn fr_300_project_root_found_from_subdirectory() {
    let o = requimd(&repo().join("requirements").join("functional"), &["check"]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
}

#[test]
fn fr_301_json_output() {
    let o = requimd(&examples("broken"), &["check", "--format", "json"]);
    assert_eq!(o.status.code(), Some(1));
    let v: serde_json::Value = serde_json::from_str(&stdout(&o)).unwrap();
    let arr = v.as_array().unwrap();
    assert_eq!(arr.len(), 22);
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
    let o = requimd(&examples("shop"), &["check", "--format", "json"]);
    assert_eq!(stdout(&o).trim(), "[]");
}

#[test]
fn fr_302_strict_treats_warnings_as_errors() {
    let dir = temp_dir("strict");
    write(&dir, "requimd.toml", "");
    write(&dir, "a.md", "## FR-? New\n");
    assert_eq!(requimd(&dir, &["check"]).status.code(), Some(0));
    assert_eq!(requimd(&dir, &["check", "--strict"]).status.code(), Some(1));
}

#[test]
fn fr_303_init_creates_files_and_never_overwrites() {
    let dir = temp_dir("init");
    let o = requimd(&dir, &["init"]);
    assert_eq!(o.status.code(), Some(0));
    assert!(
        fs::read_to_string(dir.join("requimd.toml"))
            .unwrap()
            .contains("[types.usecase]")
    );
    assert_eq!(fs::read_to_string(dir.join("glossary.md")).unwrap(), "");
    assert_eq!(requimd(&dir, &["check"]).status.code(), Some(0));

    write(&dir, "requimd.toml", "# mine\n");
    write(&dir, "glossary.md", "## Cart\n");
    let o = requimd(&dir, &["init"]);
    assert_eq!(o.status.code(), Some(0));
    assert!(stdout(&o).contains("left unchanged"));
    assert_eq!(
        fs::read_to_string(dir.join("requimd.toml")).unwrap(),
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
    let mut dirs = vec![
        repo().join("crates"),
        repo().join("editors/vscode/src/test"),
    ];
    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                dirs.push(path);
            } else if path.extension().is_some_and(|e| e == "ts") {
                // NFR-011: VS Code tests are `['FR-500 title', async () => …`.
                let text = fs::read_to_string(&path).unwrap();
                for l in text.lines().map(str::trim) {
                    if let Some(title) = l.strip_prefix("['").and_then(|t| t.split('\'').next()) {
                        names.push(title.to_lowercase().replace(['-', ' '], "_"));
                    }
                }
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
fn fr_304_fmt_assigns_ids_and_normalises_metadata() {
    let dir = temp_dir("fmt");
    write(&dir, "requimd.toml", "");
    let text = "## FR-001 A\nprio:   must\tstatus: draft\n\n## FR-? B\n";
    write(&dir, "a.md", text);
    let o = requimd(&dir, &["fmt", "--check"]);
    assert_eq!(o.status.code(), Some(1));
    assert_eq!(stdout(&o), "would format a.md\n");
    assert_eq!(fs::read_to_string(dir.join("a.md")).unwrap(), text);

    let o = requimd(&dir, &["fmt"]);
    assert_eq!(o.status.code(), Some(0));
    assert_eq!(
        fs::read_to_string(dir.join("a.md")).unwrap(),
        "## FR-001 A\nprio: must  status: draft\n\n## FR-002 B\n"
    );
    assert_eq!(requimd(&dir, &["fmt", "--check"]).status.code(), Some(0));
}

#[test]
fn fr_305_show_item_and_step_with_references() {
    let o = requimd(&examples("shop"), &["show", "FR-104"]);
    assert_eq!(o.status.code(), Some(0));
    let out = stdout(&o);
    assert!(out.starts_with("FR-104 VAT per line item\n"), "{out}");
    assert!(
        out.contains("impl     UC-012.s2    System shows the total incl. VAT."),
        "{out}"
    );
    assert!(
        out.contains("ver      TC-220       VAT for delivery to Austria  (tests.md:3)"),
        "{out}"
    );

    let out = stdout(&requimd(&examples("shop"), &["show", "UC-012.s2"]));
    assert!(
        out.contains("FR-104") && out.contains("FR-105") && out.contains("NFR-007"),
        "{out}"
    );
    assert!(!out.contains("FR-101"), "{out}");

    assert_eq!(
        requimd(&examples("shop"), &["show", "FR-999"])
            .status
            .code(),
        Some(1)
    );
    assert_eq!(
        requimd(&examples("shop"), &["show", "nonsense"])
            .status
            .code(),
        Some(2)
    );
    assert_eq!(
        requimd(&examples("broken"), &["show", "FR-001"])
            .status
            .code(),
        Some(1)
    );
}

const UC: &str = "## UC-001 Pay\n\n- s1: Customer pays.\n";
const FR: &str = "## FR-001 Charge\nimpl: UC-001.s1\n\nThe system shall charge the card.\n";

#[test]
fn fr_116_check_records_links_and_reports_changed_targets() {
    let dir = temp_dir("suspect");
    write(&dir, "requimd.toml", "");
    write(&dir, "uc.md", UC);
    write(&dir, "fr.md", FR);
    assert_eq!(requimd(&dir, &["check"]).status.code(), Some(0));
    let lock = fs::read_to_string(dir.join("requimd.lock")).unwrap();
    assert!(
        lock.contains("\"FR-001 impl UC-001.s1\" = \"7b2f59d518e62b12\""),
        "{lock}"
    );
    assert!(lock.contains("[ids]\nFR = 1\nUC = 1\n"), "{lock}");

    // Re-wrapping is no change; rewording is.
    write(&dir, "uc.md", "## UC-001 Pay\n\n- s1: Customer\n  pays.\n");
    assert_eq!(stdout(&requimd(&dir, &["check"])), "");
    write(
        &dir,
        "uc.md",
        "## UC-001 Pay\n\n- s1: Customer pays by card.\n",
    );
    let out = stdout(&requimd(&dir, &["check"]));
    assert!(
        out.starts_with("fr.md:2:7: warning: UC-001.s1 changed"),
        "{out}"
    );
    assert!(out.trim_end().ends_with("[suspect-link]"), "{out}");
    // A check never changes an existing hash.
    assert_eq!(fs::read_to_string(dir.join("requimd.lock")).unwrap(), lock);
}

#[test]
fn fr_300_locked_writes_nothing_and_fails_if_out_of_date() {
    let dir = temp_dir("locked");
    write(&dir, "requimd.toml", "");
    write(&dir, "uc.md", UC);
    write(&dir, "fr.md", FR);
    let o = requimd(&dir, &["check", "--locked"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&o.stderr).contains("requimd.lock is out of date"));
    assert!(!dir.join("requimd.lock").exists());
    assert_eq!(requimd(&dir, &["check"]).status.code(), Some(0));
    assert_eq!(requimd(&dir, &["check", "--locked"]).status.code(), Some(0));
}

#[test]
fn fr_300_conflicted_lock_stops_check_unchanged() {
    let dir = temp_dir("conflict");
    write(&dir, "requimd.toml", "");
    write(&dir, "fr.md", FR);
    let conflicted =
        "[links]\n<<<<<<< HEAD\n\"FR-001 impl UC-001.s1\" = \"1\"\n=======\n>>>>>>> b\n";
    write(&dir, "requimd.lock", conflicted);
    let o = requimd(&dir, &["check"]);
    assert_eq!(o.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&o.stderr).contains("merge conflicts"));
    assert_eq!(
        fs::read_to_string(dir.join("requimd.lock")).unwrap(),
        conflicted
    );
}

#[test]
fn fr_307_review_item_step_and_all() {
    let dir = temp_dir("review");
    write(&dir, "requimd.toml", "");
    write(
        &dir,
        "uc.md",
        "## UC-001 Pay\n\n- s1: Customer pays.\n- s2: Receipt.\n",
    );
    write(
        &dir,
        "fr.md",
        "## FR-001 Charge\nimpl: UC-001.s1\n\nThe system shall charge.\n\n## FR-002 Print\nimpl: UC-001.s2\n\nThe system shall print.\n",
    );
    requimd(&dir, &["check"]);
    write(
        &dir,
        "uc.md",
        "## UC-001 Pay\n\n- s1: Customer pays now.\n- s2: Receipt now.\n",
    );
    assert_eq!(stdout(&requimd(&dir, &["check"])).lines().count(), 2);

    let o = requimd(&dir, &["review", "UC-001.s1"]);
    assert_eq!(stdout(&o), "reviewed FR-001 impl UC-001.s1\n");
    assert_eq!(stdout(&requimd(&dir, &["check"])).lines().count(), 1);
    assert_eq!(
        stdout(&requimd(&dir, &["review", "FR-001"])),
        "no suspect links\n"
    );
    assert_eq!(
        stdout(&requimd(&dir, &["review", "--all"])),
        "reviewed FR-002 impl UC-001.s2\n"
    );
    assert_eq!(stdout(&requimd(&dir, &["check"])), "");

    assert_eq!(requimd(&dir, &["review", "FR-999"]).status.code(), Some(1));
    assert_eq!(
        requimd(&dir, &["review", "nonsense"]).status.code(),
        Some(2)
    );
    assert_eq!(requimd(&dir, &["review"]).status.code(), Some(2));
}

#[test]
fn fr_212_fmt_does_not_reuse_ids_of_deleted_items() {
    let dir = temp_dir("noreuse");
    write(&dir, "requimd.toml", "");
    write(
        &dir,
        "fr.md",
        "## FR-001 A\n\nIt shall.\n\n## FR-002 B\n\nIt shall.\n",
    );
    requimd(&dir, &["check"]);
    // FR-002 is deleted; a new placeholder must not get FR-002 again.
    write(
        &dir,
        "fr.md",
        "## FR-001 A\n\nIt shall.\n\n## FR-? C\n\nIt shall.\n",
    );
    requimd(&dir, &["fmt"]);
    assert!(
        fs::read_to_string(dir.join("fr.md"))
            .unwrap()
            .contains("## FR-003 C")
    );
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}

#[test]
fn fr_400_static_site_pages_anchors_links_backlinks() {
    let out = temp_dir("site");
    let o = requimd(
        &examples("shop"),
        &["export", "site", out.to_str().unwrap()],
    );
    assert_eq!(
        o.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&o.stderr)
    );
    for page in [
        "index.html",
        "matrix.html",
        "functional.html",
        "usecases.html",
        "glossary.html",
    ] {
        assert!(out.join(page).is_file(), "{page}");
    }
    let fr = read(&out.join("functional.html"));
    // Anchors per item, status badge, metadata with links, mentions.
    assert!(fr.contains("<h2 id=\"FR-104\" class=\"item\">"), "{fr}");
    assert!(fr.contains("<span class=\"status status-review\">review</span>"));
    assert!(fr.contains("<a href=\"usecases.html#UC-012.s2\">UC-012.s2</a>"));
    assert!(fr.contains("<a href=\"rules.html#BR-003\">BR-003</a>"));
    // Backlinks: TC-220 verifies FR-104.
    assert!(
        fr.contains("Referenced by") && fr.contains("<a href=\"tests.html#TC-220\">TC-220</a>")
    );
    // Anchors per step.
    let uc = read(&out.join("usecases.html"));
    assert!(uc.contains("<li id=\"UC-012.s2a\">"), "{uc}");
    // Index lists every item; no JavaScript anywhere; works from files.
    let index = read(&out.join("index.html"));
    assert!(index.contains("<a href=\"functional.html#FR-101\">FR-101</a>"));
    for e in fs::read_dir(&out).unwrap() {
        let html = read(&e.unwrap().path());
        assert!(!html.contains("<script"));
        assert!(!html.contains("href=\"/"), "absolute link");
    }
}

#[test]
fn fr_400_status_filter_and_nested_paths() {
    let dir = temp_dir("sitefilter");
    write(&dir, "requimd.toml", "");
    fs::create_dir_all(dir.join("req/sub")).unwrap();
    write(
        &dir,
        "req/sub/a.md",
        "## FR-001 Kept\nstatus: review\n\nIt shall, see FR-002.\n",
    );
    write(
        &dir,
        "req/b.md",
        "## FR-002 Draft\n\nIt shall. <!-- requimd: allow x -->\n\n<b>raw</b>\n",
    );
    let out = dir.join("site");
    let o = requimd(
        &dir,
        &["export", "site", "site", "--status", "review,approved"],
    );
    assert_eq!(o.status.code(), Some(0));
    let a = read(&out.join("req/sub/a.html"));
    assert!(a.contains("id=\"FR-001\""));
    // The filtered item is gone, and the mention of it is not a link.
    assert!(
        a.contains("<span class=\"unresolved\">FR-002</span>"),
        "{a}"
    );
    assert!(!read(&out.join("req/b.html")).contains("FR-002"));
    // Without the filter: the comment is left out, raw HTML shown as text.
    requimd(&dir, &["export", "site", "site"]);
    let b = read(&out.join("req/b.html"));
    assert!(
        !b.contains("requimd: allow") && b.contains("&lt;b&gt;raw"),
        "{b}"
    );
    let a = read(&out.join("req/sub/a.html"));
    // Links go up to the site root and down again: req/sub/a → req/b.
    assert!(
        a.contains("<a href=\"../../req/b.html#FR-002\">FR-002</a>"),
        "{a}"
    );
    assert!(read(&out.join("index.html")).contains("href=\"req/sub/a.html#FR-001\""));
    // Unknown status: usage error.
    assert_eq!(
        requimd(&dir, &["export", "site", "site", "--status", "done"])
            .status
            .code(),
        Some(2)
    );
}

#[test]
fn fr_401_traceability_matrix_csv() {
    let o = requimd(&examples("shop"), &["export", "matrix"]);
    assert_eq!(
        stdout(&o),
        "use_case,step,text,implemented_by,verified_by\n\
         UC-012,s1,Customer opens the cart.,FR-101,\n\
         UC-012,s2,System shows the total incl. VAT.,FR-104 FR-105,TC-220\n\
         UC-012,s3,Customer confirms the order.,FR-102,\n\
         UC-012,s2a,\"Cart is empty → system shows a hint, use case ends.\",FR-103,\n"
    );
    let out = temp_dir("matrix");
    let site = out.join("site");
    requimd(
        &examples("shop"),
        &["export", "site", site.to_str().unwrap()],
    );
    let m = read(&site.join("matrix.html"));
    assert!(m.contains("4 steps, 4 implemented, 1 verified."), "{m}");
    let file = out.join("m.csv");
    requimd(
        &examples("shop"),
        &["export", "matrix", "-o", file.to_str().unwrap()],
    );
    assert!(read(&file).starts_with("use_case,step"));
}

#[test]
fn fr_402_json_export_of_item_graph() {
    let o = requimd(&examples("shop"), &["export", "json"]);
    let v: serde_json::Value = serde_json::from_str(&stdout(&o)).unwrap();
    assert_eq!(v["format"], 1);
    let items = v["items"].as_array().unwrap();
    let fr = items.iter().find(|i| i["id"] == "FR-104").unwrap();
    assert_eq!(fr["type"], "functional");
    assert_eq!(fr["status"], "review");
    assert_eq!(fr["file"], "functional.md");
    assert_eq!(fr["line"], 21);
    assert_eq!(fr["meta"]["prio"], "must");
    assert_eq!(fr["links"][0]["key"], "impl");
    assert_eq!(fr["links"][0]["target"], "UC-012.s2");
    assert_eq!(fr["links"][0]["resolved"], true);
    assert_eq!(fr["mentions"][0]["target"], "BR-003");
    let uc = items.iter().find(|i| i["id"] == "UC-012").unwrap();
    assert_eq!(uc["steps"].as_array().unwrap().len(), 4);
    assert_eq!(uc["fields"]["pre"], "Customer is logged in.");
}

#[test]
fn nfr_005_single_binary_runs_alone() {
    // A copy of the binary in an otherwise empty directory works: no
    // runtime, libraries or data files next to it are needed.
    let dir = temp_dir("single");
    let exe = dir.join(if cfg!(windows) {
        "requimd.exe"
    } else {
        "requimd"
    });
    fs::copy(env!("CARGO_BIN_EXE_requimd"), &exe).unwrap();
    let project = dir.join("project");
    fs::create_dir_all(&project).unwrap();
    let run = |args: &[&str]| {
        Command::new(&exe)
            .args(args)
            .current_dir(&project)
            .output()
            .unwrap()
    };
    assert!(run(&["init"]).status.success());
    write(&project, "a.md", "## FR-001 A\n\nThe system shall work.\n");
    assert_eq!(run(&["check"]).status.code(), Some(0));
    assert!(run(&["export", "site", "site"]).status.success());
    assert!(project.join("site/index.html").is_file());
}

#[test]
fn fr_306_coverage_per_use_case() {
    let out = stdout(&requimd(&examples("broken"), &["coverage"]));
    assert_eq!(
        out,
        "UC-001 Checkout: 3 steps, 1 covered\n  uncovered: s2, s2\ntotal: 3 steps, 1 covered (33 %)\n"
    );
    let out = stdout(&requimd(&examples("shop"), &["coverage"]));
    assert!(
        out.ends_with("total: 4 steps, 4 covered (100 %)\n"),
        "{out}"
    );
}

/// One JSON-RPC message with its header.
fn frame(body: &str) -> String {
    format!("Content-Length: {}\r\n\r\n{body}", body.len())
}

#[test]
fn fr_308_lsp_command_speaks_lsp_over_stdio() {
    use std::io::{Read, Write};
    use std::process::Stdio;
    let mut child = Command::new(env!("CARGO_BIN_EXE_requimd"))
        .arg("lsp")
        .current_dir(examples("shop"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    for body in [
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{}}}"#,
        r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"shutdown"}"#,
        r#"{"jsonrpc":"2.0","method":"exit"}"#,
    ] {
        stdin.write_all(frame(body).as_bytes()).unwrap();
    }
    drop(stdin);
    let mut out = String::new();
    child
        .stdout
        .take()
        .unwrap()
        .read_to_string(&mut out)
        .unwrap();
    let status = child.wait().unwrap();
    assert!(status.success(), "{status:?}");
    assert!(out.contains(r#""hoverProvider":true"#), "{out}");
    assert!(out.contains(r#""id":2"#), "{out}");
}

#[test]
fn nfr_007_neovim_builtin_client_without_plugins() {
    // Runs scripts/nvim-check.lua if Neovim is installed; CI installs it.
    let nvim = if cfg!(windows) { "nvim.exe" } else { "nvim" };
    if Command::new(nvim).arg("--version").output().is_err() {
        eprintln!("nvim not found; NFR-007 check skipped");
        return;
    }
    let o = Command::new(nvim)
        .args(["--headless", "--clean", "-l", "scripts/nvim-check.lua"])
        .arg(env!("CARGO_BIN_EXE_requimd"))
        .current_dir(repo())
        .output()
        .unwrap();
    let out = stdout(&o);
    assert!(
        o.status.success(),
        "{out}{}",
        String::from_utf8_lossy(&o.stderr)
    );
    assert!(
        !out.contains("FAIL") && out.contains("PASS shutdown"),
        "{out}"
    );
}

#[test]
fn nfr_010_every_must_requirement_of_a_milestone_has_a_test() {
    // `must` items of M1 to M6 in docs/roadmap.md; M3 and M4 have none.
    let mut ids: Vec<String> = Vec::new();
    ids.extend((1..=10).chain(12..=14).map(|n| format!("fr_{n:03}")));
    ids.extend((100..=109).chain([115, 117]).map(|n| format!("fr_{n:03}")));
    ids.extend((300..=303).map(|n| format!("fr_{n:03}")));
    ids.extend(["nfr_002", "nfr_004", "nfr_006", "nfr_008", "nfr_010"].map(String::from));
    ids.extend(["br_001", "br_003", "br_005"].map(String::from));
    // M2
    ids.extend(
        (200..=208)
            .chain(210..=212)
            .chain([217, 308])
            .map(|n| format!("fr_{n:03}")),
    );
    ids.extend(["nfr_001", "nfr_003", "nfr_007", "nfr_009"].map(String::from));
    // M5
    ids.push("nfr_005".into());
    // M6 (VS Code tests in editors/vscode/src/test)
    ids.extend((500..=502).map(|n| format!("fr_{n:03}")));
    ids.push("nfr_011".into());
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

/// Runs git in `dir` with a fixed identity, so tests work without a
/// configured user.
fn git(dir: &Path, args: &[&str]) {
    let o = Command::new("git")
        .args([
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "-c",
            "core.autocrlf=false",
        ])
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        o.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&o.stderr)
    );
}

#[test]
fn fr_403_diff_between_references_and_working_tree() {
    let repo = temp_dir("diff");
    git(&repo, &["init", "-q"]);
    // Before the project existed.
    write(&repo, "README.md", "# Repo\n");
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-q", "-m", "start"]);
    git(&repo, &["tag", "v0"]);

    // The project lives in a subdirectory of the repository.
    let dir = repo.join("req");
    fs::create_dir_all(&dir).unwrap();
    write(&dir, "requimd.toml", "");
    write(
        &dir,
        "fr.md",
        "## FR-001 Show cart\n\nThe system shall list all items.\n\n## FR-002 Total\n\nThe system shall show the total.\n",
    );
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-q", "-m", "v1"]);
    git(&repo, &["tag", "v1"]);

    write(
        &dir,
        "fr.md",
        "## FR-001 Show cart\nstatus: review\n\nThe system shall list all items.\n\n## FR-003 Hint\n\nThe system shall show a hint.\n",
    );
    git(&repo, &["commit", "-q", "-am", "v2"]);
    git(&repo, &["tag", "v2"]);

    let o = requimd(&dir, &["diff", "v1", "v2"]);
    assert_eq!(o.status.code(), Some(0));
    assert_eq!(
        stdout(&o),
        "changed  FR-001  Show cart: status draft → review\n\
         removed  FR-002  Total\n\
         added    FR-003  Hint\n\
         1 added, 1 removed, 1 changed\n"
    );

    // Without ref2: the working tree, including uncommitted edits.
    write(
        &dir,
        "fr.md",
        "## FR-001 Show the cart\nstatus: review\n\nThe system shall list all items.\n\n## FR-003 Hint\n\nThe system shall show a hint.\n",
    );
    let o = requimd(&dir, &["diff", "v2"]);
    assert_eq!(
        stdout(&o),
        "changed  FR-001  Show the cart: title\n0 added, 0 removed, 1 changed\n"
    );
    assert_eq!(stdout(&requimd(&dir, &["diff", "HEAD"])), stdout(&o));

    // A reference before the project: everything is added.
    let o = requimd(&dir, &["diff", "v0", "v1", "--format", "json"]);
    let json: serde_json::Value = serde_json::from_slice(&o.stdout).unwrap();
    let ids: Vec<&str> = json
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids, ["FR-001", "FR-002"]);
    assert_eq!(json[0]["change"], "added");

    // Unknown reference: exit code 2 and a message.
    let o = requimd(&dir, &["diff", "no-such-tag"]);
    assert_eq!(o.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&o.stderr).contains("unknown git reference \"no-such-tag\""));
}

#[test]
fn fr_404_mermaid_export_command() {
    let shop = examples("shop");
    let o = requimd(&shop, &["export", "mermaid", "FR-104"]);
    assert_eq!(o.status.code(), Some(0));
    let out = stdout(&o);
    assert!(out.starts_with("```mermaid\nflowchart LR\n"), "{out}");
    assert!(out.contains("FR_104 -->|implements| UC_012_s2"), "{out}");
    assert!(out.contains("TC_220 -->|verifies| FR_104"), "{out}");
    assert!(out.contains("class FR_104 review"), "{out}");
    assert!(
        out.ends_with("style FR_104 stroke-width:3px\n```\n"),
        "{out}"
    );

    // -o writes the same to a file; --depth 0 shows the item alone.
    let dir = temp_dir("mermaid");
    let file = dir.join("d.md");
    let o = requimd(
        &shop,
        &[
            "export",
            "mermaid",
            "FR-104",
            "--depth",
            "0",
            "-o",
            file.to_str().unwrap(),
        ],
    );
    assert_eq!(o.status.code(), Some(0));
    let written = fs::read_to_string(&file).unwrap();
    assert!(!written.contains("-->"), "{written}");

    // Exit codes as in FR-305.
    assert_eq!(
        requimd(&shop, &["export", "mermaid", "FR-999"])
            .status
            .code(),
        Some(1)
    );
    assert_eq!(
        requimd(&shop, &["export", "mermaid", "nope"]).status.code(),
        Some(2)
    );
}
