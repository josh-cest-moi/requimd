//! FR-404: an item and its neighbours as a Mermaid flowchart.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use requimd_core::parse::parse_target;
use requimd_core::{Item, Project, Resolution};

const STATUS: &str = "status";
/// Characters of step text in a node.
const STEP_LENGTH: usize = 50;

/// Style per status value; other values are not styled.
const STATUS_STYLES: [(&str, &str); 4] = [
    ("draft", "stroke-dasharray:5 3"),
    ("review", "fill:#fff3bf,stroke:#f08c00,color:#000"),
    ("approved", "fill:#d3f9d8,stroke:#2f9e44,color:#000"),
    ("deprecated", "fill:#f1f3f5,stroke:#adb5bd,color:#868e96"),
];

/// FR-404 with FR-305 exit codes: the item to start from. A step
/// reference starts at its use case.
pub fn start(p: &Project, id: &str) -> Result<usize, (u8, String)> {
    let target = parse_target(&p.config, id)
        .ok_or_else(|| (2, format!("\"{id}\" is not an item ID or step reference")))?;
    match p.resolve(&target) {
        Resolution::Item(i) | Resolution::Step(i, _) => Ok(i),
        Resolution::Duplicate => Err((1, format!("{} is declared more than once", target.id))),
        Resolution::MissingItem | Resolution::MissingStep(_) => Err((1, format!("{id} not found"))),
    }
}

/// A reference between two items, or from an item to a step.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Edge {
    from: usize,
    to: usize,
    step: Option<usize>,
    /// Link key; `None` for a mention.
    key: Option<String>,
}

/// All resolvable references of the project, without self-references.
fn edges(p: &Project) -> BTreeSet<Edge> {
    let mut out = BTreeSet::new();
    for (from, item) in p.items.iter().enumerate() {
        let refs = item
            .links
            .iter()
            .map(|l| (&l.reference, Some(l.key.clone())))
            .chain(item.mentions.iter().map(|m| (m, None)));
        for (r, key) in refs {
            let Some(t) = &r.target else { continue };
            let (to, step) = match p.resolve(t) {
                Resolution::Item(i) => (i, None),
                Resolution::Step(i, s) => (i, Some(s)),
                _ => continue,
            };
            if to != from {
                out.insert(Edge {
                    from,
                    to,
                    step,
                    key,
                });
            }
        }
    }
    out
}

/// Mermaid node ID: letters, digits and underscores only.
fn node_id(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

/// Text for a quoted Mermaid label.
fn label(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace('"', "#quot;")
        .replace('<', "#lt;")
        .replace('>', "#gt;")
}

fn shorten(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

fn status<'a>(p: &'a Project, item: &'a Item) -> &'a str {
    item.meta_value(STATUS)
        .or_else(|| p.config.defaults.get(STATUS).map(String::as_str))
        .unwrap_or_default()
}

/// FR-404: the item `start` and the items up to `depth` references away,
/// as a Markdown code block with a Mermaid flowchart.
pub fn mermaid(p: &Project, start: usize, depth: usize) -> String {
    let all = edges(p);
    // Neighbours in both directions.
    let mut near: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for e in &all {
        near.entry(e.from).or_default().insert(e.to);
        near.entry(e.to).or_default().insert(e.from);
    }
    let mut shown = BTreeSet::from([start]);
    let mut queue = VecDeque::from([(start, 0)]);
    while let Some((i, d)) = queue.pop_front() {
        if d == depth {
            continue;
        }
        for &n in near.get(&i).into_iter().flatten() {
            if shown.insert(n) {
                queue.push_back((n, d + 1));
            }
        }
    }
    let edges: Vec<&Edge> = all
        .iter()
        .filter(|e| shown.contains(&e.from) && shown.contains(&e.to))
        .collect();
    // Steps that shown references point to, per use case.
    let mut steps: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for e in &edges {
        if let Some(s) = e.step {
            steps.entry(e.to).or_default().insert(s);
        }
    }

    let text = |i: usize| p.items.get(i);
    let id_of = |i: usize| text(i).map_or(String::new(), |it| node_id(&it.display_id()));
    let mut out = String::from("```mermaid\nflowchart LR\n");
    let mut used_status = BTreeSet::new();
    for &i in &shown {
        let Some(item) = text(i) else { continue };
        let st = status(p, item);
        let head = label(&format!("{} {}", item.display_id(), item.title));
        let node = if st.is_empty() {
            head
        } else {
            format!("{head}<br/>({})", label(st))
        };
        let id = id_of(i);
        match steps.get(&i) {
            // An untitled box with the use case node on top of its steps;
            // Mermaid clips subgraph titles that wrap.
            Some(labels) => {
                out.push_str(&format!(
                    "  subgraph {id}_box[\" \"]\n    direction TB\n    {id}[\"{node}\"]\n"
                ));
                for &s in labels {
                    let Some(step) = item.steps.get(s) else {
                        continue;
                    };
                    let file = p.files.get(item.file).map_or("", |f| f.text.as_str());
                    let step_text = file.get(step.text_span.clone()).unwrap_or_default();
                    out.push_str(&format!(
                        "    {id}_{}[\"{}: {}\"]\n",
                        node_id(&step.label),
                        step.label,
                        label(&shorten(&label(step_text), STEP_LENGTH))
                    ));
                }
                out.push_str("  end\n");
            }
            None => out.push_str(&format!("  {id}[\"{node}\"]\n")),
        }
        if STATUS_STYLES.iter().any(|(s, _)| *s == st) {
            used_status.insert(st);
            out.push_str(&format!("  class {id} {st}\n"));
        }
    }
    for e in &edges {
        let mut to = id_of(e.to);
        if let (Some(s), Some(item)) = (e.step, text(e.to))
            && let Some(step) = item.steps.get(s)
        {
            to = format!("{to}_{}", node_id(&step.label));
        }
        let from = id_of(e.from);
        match &e.key {
            Some(key) => {
                let name = p
                    .config
                    .links
                    .iter()
                    .find(|l| &l.key == key)
                    .and_then(|l| l.name.clone())
                    .unwrap_or_else(|| key.clone());
                out.push_str(&format!("  {from} -->|{}| {to}\n", label(&name)));
            }
            None => out.push_str(&format!("  {from} -.-> {to}\n")),
        }
    }
    for (st, style) in STATUS_STYLES {
        if used_status.contains(st) {
            out.push_str(&format!("  classDef {st} {style}\n"));
        }
    }
    out.push_str(&format!("  style {} stroke-width:3px\n```\n", id_of(start)));
    out
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use requimd_core::{Config, SourceFile};

    use super::*;

    fn project(files: &[(&str, &str)]) -> Project {
        let files = files
            .iter()
            .map(|(n, t)| SourceFile::new(PathBuf::from(n), n.to_string(), t.to_string()))
            .collect();
        Project::from_files(PathBuf::from("."), Config::defaults().unwrap(), files)
    }

    const FILES: [(&str, &str); 3] = [
        (
            "uc.md",
            "## UC-001 Borrow\n\n- s1: Member scans the card.\n- s2: System checks the \"member\" account.\n",
        ),
        (
            "fr.md",
            "## FR-001 Card login\nimpl: UC-001.s1  status: review\n\nThe system shall show loans, see FR-002.\n\n\
             ## FR-002 Account check\nimpl: UC-001.s2\n\nThe system shall check.\n\n\
             ## FR-003 Unrelated\n\nThe system shall wait.\n",
        ),
        (
            "tc.md",
            "## TC-001 Login\nver: FR-001  status: approved\n\nGiven a card.\n",
        ),
    ];

    #[test]
    fn fr_404_direct_neighbours() {
        let p = project(&FILES);
        let start = start(&p, "FR-001").unwrap();
        assert_eq!(
            mermaid(&p, start, 1),
            "```mermaid\nflowchart LR\n\
             \x20 subgraph UC_001_box[\" \"]\n\
             \x20   direction TB\n\
             \x20   UC_001[\"UC-001 Borrow<br/>(draft)\"]\n\
             \x20   UC_001_s1[\"s1: Member scans the card.\"]\n\
             \x20   UC_001_s2[\"s2: System checks the #quot;member#quot; account.\"]\n\
             \x20 end\n\
             \x20 class UC_001 draft\n\
             \x20 FR_001[\"FR-001 Card login<br/>(review)\"]\n\
             \x20 class FR_001 review\n\
             \x20 FR_002[\"FR-002 Account check<br/>(draft)\"]\n\
             \x20 class FR_002 draft\n\
             \x20 TC_001[\"TC-001 Login<br/>(approved)\"]\n\
             \x20 class TC_001 approved\n\
             \x20 FR_001 -->|implements| UC_001_s1\n\
             \x20 FR_001 -.-> FR_002\n\
             \x20 FR_002 -->|implements| UC_001_s2\n\
             \x20 TC_001 -->|verifies| FR_001\n\
             \x20 classDef draft stroke-dasharray:5 3\n\
             \x20 classDef review fill:#fff3bf,stroke:#f08c00,color:#000\n\
             \x20 classDef approved fill:#d3f9d8,stroke:#2f9e44,color:#000\n\
             \x20 style FR_001 stroke-width:3px\n```\n"
        );
    }

    #[test]
    fn fr_404_depth_and_step_reference() {
        let p = project(&FILES);
        // Depth 0: only the item. A step reference starts at its use case.
        let uc = start(&p, "UC-001.s2").unwrap();
        let out = mermaid(&p, uc, 0);
        assert!(
            out.contains("  UC_001[\"UC-001 Borrow<br/>(draft)\"]\n"),
            "{out}"
        );
        assert!(!out.contains("FR_"), "{out}");
        // Depth 2 from the use case reaches TC-001 via FR-001; quotes in
        // step text are escaped.
        let out = mermaid(&p, uc, 2);
        assert!(out.contains("TC_001 -->|verifies| FR_001"), "{out}");
        assert!(
            out.contains("System checks the #quot;member#quot; account."),
            "{out}"
        );
        assert!(!out.contains("FR_003"), "{out}");
    }

    #[test]
    fn fr_404_start_errors_as_show() {
        let p = project(&[("fr.md", "## FR-001 A\n\n## FR-001 B\n")]);
        assert_eq!(start(&p, "FR-001").unwrap_err().0, 1);
        assert_eq!(start(&p, "FR-002").unwrap_err().0, 1);
        assert_eq!(start(&p, "hello").unwrap_err().0, 2);
    }
}
