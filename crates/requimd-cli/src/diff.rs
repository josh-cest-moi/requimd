//! FR-403: `requimd diff`, items added, removed or changed between two git
//! references or a reference and the working tree.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use requimd_core::config::CONFIG_FILE;
use requimd_core::{Config, Includes, Item, Project, SourceFile, glossary_rel};
use serde::Serialize;

/// The metadata key FR-403 reports with old and new value.
const STATUS: &str = "status";

/// Runs git in `dir` and returns its standard output.
fn git(dir: &Path, args: &[&str]) -> Result<Vec<u8>, String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .map_err(|e| format!("cannot run git: {e}"))?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        return Err(format!("git {}: {}", args.join(" "), err.trim()));
    }
    Ok(out.stdout)
}

/// Contents of `<rev>:<path>` for each path, read by one `git cat-file`.
/// `None` for a path that does not exist at `rev`.
fn read_blobs(root: &Path, rev: &str, paths: &[String]) -> Result<Vec<Option<String>>, String> {
    let mut child = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["cat-file", "--batch"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("cannot run git: {e}"))?;
    let input: String = paths.iter().map(|p| format!("{rev}:{p}\n")).collect();
    let mut stdin = child.stdin.take().ok_or("git cat-file: no input")?;
    // Written by a thread, so a full output pipe cannot block the input.
    let writer = std::thread::spawn(move || stdin.write_all(input.as_bytes()));
    let mut out = Vec::new();
    child
        .stdout
        .take()
        .ok_or("git cat-file: no output")?
        .read_to_end(&mut out)
        .map_err(|e| format!("git cat-file: {e}"))?;
    let _ = writer.join();
    let _ = child.wait();

    // Each answer is "<oid> blob <size>\n<content>\n" or "<name> missing\n".
    let mut blobs = Vec::new();
    let mut pos = 0;
    for _ in paths {
        let end = out
            .get(pos..)
            .and_then(|rest| rest.iter().position(|&b| b == b'\n'))
            .ok_or("git cat-file: incomplete output")?;
        let header = String::from_utf8_lossy(&out[pos..pos + end]).into_owned();
        pos += end + 1;
        let fields: Vec<&str> = header.split(' ').collect();
        match fields.as_slice() {
            [_, "blob", size] => {
                let size: usize = size.parse().map_err(|_| "git cat-file: bad size")?;
                let content = out
                    .get(pos..pos + size)
                    .ok_or("git cat-file: incomplete output")?;
                blobs.push(Some(String::from_utf8_lossy(content).into_owned()));
                pos += size + 1;
            }
            _ => blobs.push(None),
        }
    }
    Ok(blobs)
}

/// The project as of git reference `rev`, read with that reference's
/// `requimd.toml`. Without one, the project did not exist: no items.
fn project_at(root: &Path, rev: &str) -> Result<Project, String> {
    git(
        root,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("{rev}^{{commit}}"),
        ],
    )
    .map_err(|_| format!("unknown git reference \"{rev}\""))?;
    // Paths in git are relative to the repository; the project may be a
    // subdirectory of it.
    let prefix = String::from_utf8_lossy(&git(root, &["rev-parse", "--show-prefix"])?)
        .trim()
        .to_string();
    let listing = git(
        root,
        &[
            "ls-tree",
            "-r",
            "--name-only",
            "--full-name",
            rev,
            "--",
            ".",
        ],
    )?;
    let rels: Vec<String> = String::from_utf8_lossy(&listing)
        .lines()
        .filter_map(|l| l.strip_prefix(prefix.as_str()).map(str::to_string))
        .collect();
    if !rels.iter().any(|r| r == CONFIG_FILE) {
        let config = Config::defaults().map_err(|e| e.to_string())?;
        return Ok(Project::from_files(root.to_path_buf(), config, Vec::new()));
    }
    let config_text = read_blobs(root, rev, &[format!("{prefix}{CONFIG_FILE}")])?
        .pop()
        .flatten()
        .unwrap_or_default();
    let config = Config::from_toml(&config_text, None)
        .map_err(|e| format!("{CONFIG_FILE} at {rev}: {e}"))?;
    let includes =
        Includes::new(&config.include).map_err(|e| format!("{CONFIG_FILE} at {rev}: {e}"))?;
    let glossary = glossary_rel(&config);
    let rels: Vec<String> = rels
        .into_iter()
        .filter(|r| r.ends_with(".md") && (includes.matches(r) || Some(r) == glossary.as_ref()))
        .collect();
    let full: Vec<String> = rels.iter().map(|r| format!("{prefix}{r}")).collect();
    let files = read_blobs(root, rev, &full)?
        .into_iter()
        .zip(rels)
        .filter_map(|(text, rel)| Some(SourceFile::new(root.join(&rel), rel, text?)))
        .collect();
    Ok(Project::from_files(root.to_path_buf(), config, files))
}

/// Runs of whitespace as one space (FR-116).
fn norm(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The parts of an item that FR-403 compares, normalised.
#[derive(PartialEq)]
struct Parts {
    file: String,
    title: String,
    text: String,
    links: BTreeSet<String>,
    metadata: BTreeSet<String>,
    status: String,
    steps: BTreeMap<String, String>,
}

impl Parts {
    fn of(p: &Project, item: &Item) -> Parts {
        let text = p.files.get(item.file).map_or("", |f| f.text.as_str());
        let get = |r: std::ops::Range<usize>| text.get(r).unwrap_or_default();
        // The text without metadata and without the steps, which are
        // compared one by one.
        let mut body = String::new();
        let mut at = item.meta_span.end.max(item.heading.end);
        for step in &item.steps {
            body.push_str(get(at..step.label_span.start));
            body.push(' ');
            at = step.text_span.end.max(at);
        }
        body.push_str(get(at..item.body.end));
        let link_keys: BTreeSet<&str> = item.links.iter().map(|l| l.key.as_str()).collect();
        Parts {
            file: p
                .files
                .get(item.file)
                .map_or(String::new(), |f| f.rel.clone()),
            title: norm(&item.title),
            text: norm(&body),
            links: item
                .links
                .iter()
                .map(|l| format!("{} {}", l.key, l.reference.text))
                .collect(),
            metadata: item
                .meta
                .iter()
                .filter(|m| m.key != STATUS && !link_keys.contains(m.key.as_str()))
                .map(|m| format!("{}: {}", m.key, norm(&m.value)))
                .collect(),
            status: item
                .meta_value(STATUS)
                .or_else(|| p.config.defaults.get(STATUS).map(String::as_str))
                .unwrap_or_default()
                .to_string(),
            steps: item
                .steps
                .iter()
                .map(|s| (s.label.clone(), norm(get(s.text_span.clone()))))
                .collect(),
        }
    }
}

/// Items with a unique ID, by ID.
fn items(p: &Project) -> BTreeMap<String, &Item> {
    p.items
        .iter()
        .filter_map(|it| {
            let id = it.id.as_ref()?;
            (p.declarations(id).len() == 1).then(|| (id.clone(), it))
        })
        .collect()
}

#[derive(Serialize, Default, PartialEq, Debug)]
pub struct StepChanges {
    added: Vec<String>,
    removed: Vec<String>,
    changed: Vec<String>,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct StatusChange {
    from: String,
    to: String,
}

/// One line of the changelog.
#[derive(Serialize, PartialEq, Debug)]
pub struct Change {
    id: String,
    /// "added", "removed" or "changed".
    change: &'static str,
    title: String,
    file: String,
    /// Changed parts: title, text, links, metadata, status, steps, file.
    parts: Vec<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<StatusChange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    steps: Option<StepChanges>,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_file: Option<String>,
}

/// Sort key as in the site index (FR-400): prefix, then number, so FR-1000
/// sorts after FR-999.
fn id_key(id: &str) -> (String, u64) {
    let (prefix, n) = id.rsplit_once('-').unwrap_or((id, ""));
    (prefix.to_string(), n.parse().unwrap_or(u64::MAX))
}

/// FR-403: the changes from `old` to `new`, sorted by ID.
pub fn changes(old: &Project, new: &Project) -> Vec<Change> {
    let before = items(old);
    let after = items(new);
    let mut ids: Vec<&String> = before.keys().chain(after.keys()).collect();
    ids.sort_by_key(|id| id_key(id));
    ids.dedup();
    let mut out = Vec::new();
    for id in ids {
        let change = |change, p: &Project, item: &Item| Change {
            id: id.clone(),
            change,
            title: item.title.clone(),
            file: p
                .files
                .get(item.file)
                .map_or(String::new(), |f| f.rel.clone()),
            parts: Vec::new(),
            status: None,
            steps: None,
            previous_file: None,
        };
        match (before.get(id), after.get(id)) {
            (None, Some(b)) => out.push(change("added", new, b)),
            (Some(a), None) => out.push(change("removed", old, a)),
            (Some(a), Some(b)) => {
                let (pa, pb) = (Parts::of(old, a), Parts::of(new, b));
                if pa == pb {
                    continue;
                }
                let mut c = change("changed", new, b);
                for (name, differs) in [
                    ("title", pa.title != pb.title),
                    ("text", pa.text != pb.text),
                    ("links", pa.links != pb.links),
                    ("metadata", pa.metadata != pb.metadata),
                ] {
                    if differs {
                        c.parts.push(name);
                    }
                }
                if pa.status != pb.status {
                    c.parts.push("status");
                    c.status = Some(StatusChange {
                        from: pa.status.clone(),
                        to: pb.status.clone(),
                    });
                }
                if pa.steps != pb.steps {
                    let mut s = StepChanges::default();
                    for (label, text) in &pb.steps {
                        match pa.steps.get(label) {
                            None => s.added.push(label.clone()),
                            Some(t) if t != text => s.changed.push(label.clone()),
                            Some(_) => {}
                        }
                    }
                    s.removed = pa
                        .steps
                        .keys()
                        .filter(|l| !pb.steps.contains_key(*l))
                        .cloned()
                        .collect();
                    c.parts.push("steps");
                    c.steps = Some(s);
                }
                if pa.file != pb.file {
                    c.parts.push("file");
                    c.previous_file = Some(pa.file.clone());
                }
                out.push(c);
            }
            (None, None) => {}
        }
    }
    out
}

/// One line per change and a summary, e.g.
/// `changed  FR-003  Loan period: text, status draft → review`.
pub fn text(changes: &[Change]) -> String {
    let mut out = String::new();
    for c in changes {
        let mut details: Vec<String> = Vec::new();
        for part in &c.parts {
            match *part {
                "status" => {
                    if let Some(s) = &c.status {
                        details.push(format!("status {} → {}", s.from, s.to));
                    }
                }
                "steps" => {
                    if let Some(s) = &c.steps {
                        let mut d = Vec::new();
                        for (what, labels) in [
                            ("added", &s.added),
                            ("removed", &s.removed),
                            ("changed", &s.changed),
                        ] {
                            if !labels.is_empty() {
                                d.push(format!("{what} {}", labels.join(" ")));
                            }
                        }
                        details.push(format!("steps {}", d.join(", ")));
                    }
                }
                "file" => {
                    if let Some(f) = &c.previous_file {
                        details.push(format!("moved from {f}"));
                    }
                }
                other => details.push(other.to_string()),
            }
        }
        let details = if details.is_empty() {
            String::new()
        } else {
            format!(": {}", details.join(", "))
        };
        out.push_str(&format!("{:<8} {}  {}{details}\n", c.change, c.id, c.title));
    }
    let count = |what: &str| changes.iter().filter(|c| c.change == what).count();
    if changes.is_empty() {
        out.push_str("no changes\n");
    } else {
        out.push_str(&format!(
            "{} added, {} removed, {} changed\n",
            count("added"),
            count("removed"),
            count("changed")
        ));
    }
    out
}

/// FR-403: the project at `rev1` against `rev2` or, without it, against
/// `working`, the project in the working tree.
pub fn diff(working: &Project, rev1: &str, rev2: Option<&str>) -> Result<Vec<Change>, String> {
    let root = working.root.as_path();
    let old = project_at(root, rev1)?;
    match rev2 {
        Some(rev2) => Ok(changes(&old, &project_at(root, rev2)?)),
        None => Ok(changes(&old, working)),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn project(files: &[(&str, &str)]) -> Project {
        let files = files
            .iter()
            .map(|(n, t)| SourceFile::new(PathBuf::from(n), n.to_string(), t.to_string()))
            .collect();
        Project::from_files(PathBuf::from("."), Config::defaults().unwrap(), files)
    }

    const UC: &str = "## UC-001 Order\n\n- s1: Customer opens the cart.\n- s2: System shows the total.\n- s2a: Cart is empty.\n";
    const FR: &str = "## FR-001 Show cart\nimpl: UC-001.s1  prio: must\n\nThe system shall list all items.\n\n## FR-002 Total\nimpl: UC-001.s2\n\nThe system shall show the total.\n\n## FR-003 Hint\n\nThe system shall show a hint.\n";

    #[test]
    fn fr_403_no_changes_ignores_whitespace() {
        let old = project(&[("uc.md", UC), ("fr.md", FR)]);
        let rewrapped = FR
            .replace("list all items", "list  all\n  items")
            .replace('\n', "\r\n");
        let new = project(&[("uc.md", UC), ("fr.md", &rewrapped)]);
        assert_eq!(changes(&old, &new), []);
        assert_eq!(text(&[]), "no changes\n");
    }

    #[test]
    fn fr_403_added_removed_changed_with_parts() {
        let old = project(&[("uc.md", UC), ("fr.md", FR)]);
        let fr = FR
            .replace(
                "## FR-001 Show cart\nimpl: UC-001.s1  prio: must",
                "## FR-001 Show the cart\nimpl: UC-001.s1  prio: should  status: review",
            )
            .replace("impl: UC-001.s2\n", "impl: UC-001.s2a\n")
            .replace(
                "## FR-003 Hint\n\nThe system shall show a hint.\n",
                "## FR-004 New\n\nThe system shall do more.\n",
            );
        let uc = UC
            .replace("- s2a: Cart is empty.\n", "- s3: Customer pays.\n")
            .replace("opens the cart", "opens the basket");
        let new = project(&[("cases.md", &uc), ("fr.md", &fr)]);
        let out = text(&changes(&old, &new));
        assert_eq!(
            out,
            "changed  FR-001  Show the cart: title, metadata, status draft → review\n\
             changed  FR-002  Total: links\n\
             removed  FR-003  Hint\n\
             added    FR-004  New\n\
             changed  UC-001  Order: steps added s3, removed s2a, changed s1, moved from uc.md\n\
             1 added, 1 removed, 3 changed\n"
        );
    }

    #[test]
    fn fr_403_only_unique_ids() {
        let old = project(&[("fr.md", "## FR-001 A\n\n## FR-? B\n")]);
        let new = project(&[("fr.md", "## FR-001 A\n\n## FR-001 A again\n\n## FR-? C\n")]);
        // FR-001 is duplicated now: it is no longer a unique item.
        let c = changes(&old, &new);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].change, "removed");
    }

    #[test]
    fn fr_403_json_names_parts() {
        let old = project(&[("fr.md", "## FR-001 A\n\nThe system shall work.\n")]);
        let new = project(&[(
            "fr.md",
            "## FR-001 A\nstatus: review\n\nThe system shall work.\n",
        )]);
        let json = serde_json::to_value(changes(&old, &new)).unwrap();
        assert_eq!(
            json,
            serde_json::json!([{
                "id": "FR-001", "change": "changed", "title": "A", "file": "fr.md",
                "parts": ["status"], "status": { "from": "draft", "to": "review" },
            }])
        );
    }
}
