//! Integration tests; unwrap is fine outside user-input paths.
#![allow(clippy::unwrap_used)]

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use reqmd_checks::check;
use reqmd_core::{Config, Diagnostic, Project, Severity, SourceFile};

fn project_with(config: &str, files: &[(&str, &str)]) -> Project {
    let config = Config::from_toml(config, None).unwrap();
    let files = files
        .iter()
        .map(|(n, t)| SourceFile::new(PathBuf::from(n), n.to_string(), t.to_string()))
        .collect();
    Project::from_files(PathBuf::from("."), config, files)
}

fn run(text: &str) -> Vec<(usize, &'static str, Severity)> {
    run_with("", text)
}

/// (line, check, severity) of every diagnostic.
fn run_with(config: &str, text: &str) -> Vec<(usize, &'static str, Severity)> {
    let p = project_with(config, &[("a.md", text)]);
    check(&p)
        .into_iter()
        .map(|d| {
            (
                p.files[d.file].line_col(d.span.start).0,
                d.check,
                d.severity,
            )
        })
        .collect()
}

fn diags(text: &str) -> (Project, Vec<Diagnostic>) {
    let p = project_with("", &[("a.md", text)]);
    let d = check(&p);
    (p, d)
}

use Severity::{Error, Info, Warning};

#[test]
fn fr_100_duplicate_ids_reported_at_each_declaration() {
    let t = "## FR-001 A\n\n## FR-001 B\n\n## FR-002 C\nref: FR-001\n";
    assert_eq!(
        run(t),
        [(1, "duplicate-id", Error), (3, "duplicate-id", Error)]
    );
    let (_, d) = diags(t);
    assert!(d[0].message.contains("a.md:3"), "{}", d[0].message);
}

#[test]
fn fr_100_links_from_duplicated_items_are_evaluated() {
    let t = "## FR-001 A\nimpl: UC-404\n\n## FR-001 B\n";
    assert!(run(t).contains(&(2, "dangling-ref", Error)));
}

#[test]
fn fr_101_dangling_links_and_mentions() {
    let t = "## UC-001 U\n\n- s1: Go.\n\n## FR-001 A\nimpl: UC-001.s9, UC-404, foo, FR-?\n\nSee FR-999 and UC-001.s7.\n";
    let r = run(t);
    let dangling: Vec<_> = r.iter().filter(|d| d.1 == "dangling-ref").collect();
    assert_eq!(
        dangling,
        [
            &(6, "dangling-ref", Error),
            &(6, "dangling-ref", Error),
            &(6, "dangling-ref", Error),
            &(6, "dangling-ref", Error),
            &(8, "dangling-ref", Warning),
            &(8, "dangling-ref", Warning),
        ]
    );
}

#[test]
fn fr_101_override_applies_to_links_and_mentions() {
    let t = "## FR-001 A\nref: FR-404\n\nSee FR-999.\n";
    assert_eq!(
        run_with("[checks]\ndangling-ref = \"info\"\n", t),
        [(2, "dangling-ref", Info), (4, "dangling-ref", Info)]
    );
}

#[test]
fn fr_102_unknown_metadata_key_names_allowed_keys() {
    let (_, d) = diags("## FR-001 A\nprio: must  colour: red\n");
    assert_eq!(d.len(), 1);
    assert_eq!(d[0].check, "unknown-key");
    assert_eq!(d[0].severity, Warning);
    assert!(d[0].message.contains("status, prio"), "{}", d[0].message);
}

#[test]
fn fr_102_unknown_key_directly_after_heading() {
    assert_eq!(
        run("## FR-001 A\nNote: something.\n"),
        [(2, "unknown-key", Warning)]
    );
}

#[test]
fn fr_102_body_field_keys_are_not_unknown() {
    assert!(run("## UC-001 U\npre: Logged in.\n").is_empty());
}

#[test]
fn fr_103_invalid_value_lists_allowed_values() {
    let (_, d) = diags("## FR-001 A\nprio: urgent  status: done\n");
    let checks: Vec<_> = d.iter().map(|d| d.check).collect();
    assert_eq!(checks, ["invalid-value", "invalid-value"]);
    assert!(d[0].message.contains("must, should, could, wont"));
}

#[test]
fn fr_104_link_target_type() {
    let t = "## UC-001 U\n\n- s1: Go.\n\n## FR-001 A\nimpl: UC-001.s1, UC-001\n\n## FR-002 B\nimpl: FR-001\n\n## TC-001 T\nver: FR-001, UC-001.s1\n";
    assert_eq!(
        run(t),
        [(9, "link-target", Error), (12, "link-target", Error)]
    );
}

#[test]
fn fr_105_required_keys_with_alternatives() {
    let t = "## NFR-001 A\nscale: s  meter: m  plan: 1\n\n## NFR-002 B\nscale: s\n";
    let (_, d) = diags(t);
    assert_eq!(d.len(), 1);
    assert_eq!(d[0].check, "missing-key");
    assert!(
        d[0].message.contains("meter, must or plan"),
        "{}",
        d[0].message
    );
}

#[test]
fn fr_106_uncovered_steps() {
    let t = "## UC-001 U\n\n- s1: A.\n- s2: B.\n- s2a: C.\n\n## FR-001 F\nimpl: UC-001.s1\n\nMentions UC-001.s2 do not cover.\n\n## NFR-001 Q\nref: UC-001.s2a  scale: s  meter: m  must: 1\n";
    assert_eq!(
        run(t),
        [
            (4, "uncovered-step", Warning),
            (5, "uncovered-step", Warning)
        ]
    );
}

#[test]
fn fr_107_duplicate_step_labels() {
    let t = "## UC-001 U\n\n- s1: A.\n- s1: B.\n\n## FR-001 F\nimpl: UC-001.s1\n";
    assert_eq!(
        run(t),
        [(3, "duplicate-step", Error), (4, "duplicate-step", Error)]
    );
}

#[test]
fn fr_108_ref_cycles_reported_at_every_link() {
    let t = "## FR-001 A\nref: FR-002\n\n## FR-002 B\nref: FR-003\n\n## FR-003 C\nref: FR-001\n\n## FR-004 D\nref: FR-001, FR-004\n";
    assert_eq!(
        run(t),
        [
            (2, "ref-cycle", Error),
            (5, "ref-cycle", Error),
            (8, "ref-cycle", Error),
            (11, "ref-cycle", Error),
        ]
    );
}

#[test]
fn fr_108_no_cycle_without_back_link() {
    assert!(
        run("## FR-001 A\nref: FR-002\n\n## FR-002 B\n\n## FR-003 C\nref: FR-001, FR-002\n")
            .is_empty()
    );
}

#[test]
fn fr_109_approved_item_links_to_draft_or_deprecated() {
    let t = "## FR-001 A\nstatus: approved  ref: FR-002, FR-003, FR-004\n\n## FR-002 B\n\n## FR-003 C\nstatus: deprecated\n\n## FR-004 D\nstatus: review\n";
    assert_eq!(
        run(t),
        [(2, "status-link", Warning), (2, "status-link", Warning)]
    );
}

#[test]
fn fr_115_placeholder_reported_at_heading() {
    let (p, d) = diags("## FR-? New\n\nThe system shall do it.\n");
    assert_eq!(d.len(), 1);
    assert_eq!(d[0].check, "placeholder");
    assert_eq!(d[0].severity, Warning);
    assert_eq!(&p.files[0].text[d[0].span.clone()], "## FR-? New");
}

#[test]
fn fr_010_placeholders_take_part_in_checks_but_not_duplicates() {
    let t = "## FR-? A\nprio: high\n\n## FR-? B\n";
    assert_eq!(
        run(t),
        [
            (1, "placeholder", Warning),
            (2, "invalid-value", Error),
            (4, "placeholder", Warning),
        ]
    );
}

#[test]
fn fr_117_severity_override_and_off() {
    let t = "## UC-001 U\n\n- s1: A.\n\n## FR-? X\n";
    assert_eq!(
        run_with(
            "[checks]\nuncovered-step = \"info\"\n[checks.placeholder]\nsev = \"off\"\n",
            t
        ),
        [(3, "uncovered-step", Info)]
    );
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples")
        .join(name)
}

fn load(root: &Path) -> Project {
    Project::load(root, Config::load(root).unwrap()).unwrap()
}

#[test]
fn fr_100_to_fr_115_broken_example_matches_expected() {
    let root = fixture("broken");
    let p = load(&root);
    let mut actual: Vec<(String, usize, String, String)> = check(&p)
        .iter()
        .map(|d| {
            let f = &p.files[d.file];
            (
                f.rel.clone(),
                f.line_col(d.span.start).0,
                d.check.to_string(),
                d.severity.to_string(),
            )
        })
        .collect();
    actual.sort();

    // Rows of the table in EXPECTED.md, without checks of later milestones.
    let expected_md = std::fs::read_to_string(root.join("EXPECTED.md")).unwrap();
    let mut expected: Vec<(String, usize, String, String)> = expected_md
        .lines()
        .filter_map(|l| {
            let cols: Vec<&str> = l.split('|').map(str::trim).collect();
            let line = cols.get(1)?.parse().ok()?;
            let reason = cols.get(4)?;
            if reason.contains("(M3") {
                return None;
            }
            Some(("broken.md".into(), line, cols[2].into(), cols[3].into()))
        })
        .collect();
    expected.sort();
    assert_eq!(actual, expected);
}

#[test]
fn fr_300_shop_example_has_no_diagnostics() {
    let p = load(&fixture("shop"));
    assert!(p.items.len() >= 8);
    assert_eq!(check(&p), []);
}

#[test]
fn fr_300_specification_has_no_errors() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let p = load(&root);
    let errors: Vec<String> = check(&p)
        .into_iter()
        .filter(|d| d.severity == Error)
        .map(|d| {
            let f = &p.files[d.file];
            let (l, c) = f.line_col(d.span.start);
            format!("{}:{l}:{c}: {} [{}]", f.rel, d.message, d.check)
        })
        .collect();
    assert!(errors.is_empty(), "{errors:#?}");
}

#[test]
fn nfr_002_full_check_of_2000_items_in_200_files() {
    let mut files = Vec::new();
    for f in 0..200 {
        let mut text = format!("# File {f}\n\n## UC-{f:04} Use case {f}\nactor: Customer\n\n");
        for s in 1..=5 {
            text.push_str(&format!("- s{s}: Step {s} of the use case.\n"));
        }
        for i in 0..9 {
            let n = f * 9 + i;
            text.push_str(&format!(
                "\n## FR-{n:04} Requirement {n}\nimpl: UC-{f:04}.s{}  prio: must\n\nWhen the customer acts, the system shall respond (see FR-{:04}).\n",
                i % 5 + 1,
                (n + 1) % 1800
            ));
        }
        files.push((format!("f{f}.md"), text));
    }
    let refs: Vec<(&str, &str)> = files
        .iter()
        .map(|(n, t)| (n.as_str(), t.as_str()))
        .collect();
    let start = Instant::now();
    let p = project_with("", &refs);
    let d = check(&p);
    let elapsed = start.elapsed();
    assert_eq!(p.items.len(), 2000);
    assert_eq!(d, []);
    // must: 5 s on a standard laptop, measured on a release build of the
    // full command; this unoptimised test run is a coarse upper bound.
    assert!(elapsed < Duration::from_secs(5), "took {elapsed:?}");
}
