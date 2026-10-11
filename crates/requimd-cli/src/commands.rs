//! `requimd fmt`, `show` and `coverage` (FR-304 to FR-306).

use std::collections::HashSet;
use std::fmt::Write as _;
use std::path::Path;

use requimd_core::config::{IMPL, USECASE};
use requimd_core::fmt::{Edit, apply, format_edits};
use requimd_core::lock::LOCK_FILE;
use requimd_core::parse::parse_target;
use requimd_core::{Item, Project, Reference, Resolution};

/// FR-304. Returns the output and whether files (would) change.
pub fn fmt(p: &Project, check: bool, cwd: &Path) -> Result<(String, bool), String> {
    let edits = format_edits(p);
    let mut out = String::new();
    let mut changed = false;
    for (id, f) in p.files.iter().enumerate() {
        let mine: Vec<&Edit> = edits.iter().filter(|e| e.file == id).collect();
        if mine.is_empty() {
            continue;
        }
        let new = apply(&f.text, &mine);
        if new == f.text {
            continue;
        }
        changed = true;
        let shown = crate::display_path(&f.path, cwd);
        if check {
            let _ = writeln!(out, "would format {shown}");
        } else {
            std::fs::write(&f.path, new).map_err(|e| format!("{shown}: {e}"))?;
            let _ = writeln!(out, "formatted {shown}");
        }
    }
    Ok((out, changed))
}

fn location(p: &Project, item: &Item, cwd: &Path) -> String {
    p.files.get(item.file).map_or_else(String::new, |f| {
        format!(
            "{}:{}",
            crate::display_path(&f.path, cwd),
            f.line_col(item.heading.start).0
        )
    })
}

fn one_line(s: &str, max: usize) -> String {
    let s = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if s.chars().count() <= max {
        s
    } else {
        format!("{}…", s.chars().take(max).collect::<String>().trim_end())
    }
}

/// What a reference points to, for the listing.
fn describe(p: &Project, r: &Reference) -> String {
    let Some(t) = &r.target else {
        return "(invalid)".into();
    };
    match p.resolve(t) {
        Resolution::Item(i) => p
            .items
            .get(i)
            .map_or_else(String::new, |it| it.title.clone()),
        Resolution::Step(i, s) => p
            .items
            .get(i)
            .and_then(|it| {
                let st = it.steps.get(s)?;
                let text = p.files.get(it.file)?.text.get(st.text_span.clone())?;
                Some(one_line(text, 60))
            })
            .unwrap_or_default(),
        Resolution::Duplicate => "(declared more than once)".into(),
        Resolution::MissingItem | Resolution::MissingStep(_) => "(not found)".into(),
    }
}

/// FR-305: the item or step and its incoming and outgoing references.
/// Errors carry the exit code: 1 if not found, 2 if not an ID.
pub fn show(p: &Project, id: &str, cwd: &Path) -> Result<String, (u8, String)> {
    let target = parse_target(&p.config, id)
        .ok_or_else(|| (2, format!("\"{id}\" is not an item ID or step reference")))?;
    let (item_idx, step) = match p.resolve(&target) {
        Resolution::Item(i) => (i, None),
        Resolution::Step(i, s) => (i, Some(s)),
        Resolution::Duplicate => {
            let places: Vec<String> = p
                .declarations(&target.id)
                .iter()
                .filter_map(|&i| p.items.get(i))
                .map(|it| location(p, it, cwd))
                .collect();
            return Err((
                1,
                format!(
                    "{} is declared more than once: {}",
                    target.id,
                    places.join(", ")
                ),
            ));
        }
        Resolution::MissingItem | Resolution::MissingStep(_) => {
            return Err((1, format!("{id} not found")));
        }
    };
    let Some(item) = p.items.get(item_idx) else {
        return Err((1, format!("{id} not found")));
    };
    let text = p.files.get(item.file).map_or("", |f| f.text.as_str());
    let mut out = String::new();
    let status = item
        .meta_value("status")
        .or_else(|| p.config.defaults.get("status").map(String::as_str))
        .unwrap_or("-");
    match step.and_then(|s| item.steps.get(s)) {
        None => {
            let _ = writeln!(out, "{} {}", item.display_id(), item.title);
            let _ = writeln!(
                out,
                "{}, status {status}, {}",
                item.kind,
                location(p, item, cwd)
            );
            let body = text
                .get(item.meta_span.end..item.body.end)
                .unwrap_or_default()
                .trim();
            if !body.is_empty() {
                let _ = writeln!(out, "\n{body}");
            }
            let _ = writeln!(out, "\noutgoing:");
            let mut any = false;
            for l in &item.links {
                any = true;
                let _ = writeln!(
                    out,
                    "  {:<8} {:<12} {}",
                    l.key,
                    l.reference.text,
                    describe(p, &l.reference)
                );
            }
            for m in &item.mentions {
                any = true;
                let _ = writeln!(out, "  {:<8} {:<12} {}", "mention", m.text, describe(p, m));
            }
            if !any {
                let _ = writeln!(out, "  (none)");
            }
        }
        Some(s) => {
            let st_text = text.get(s.text_span.clone()).unwrap_or_default();
            let _ = writeln!(
                out,
                "{}.{} {}",
                item.display_id(),
                s.label,
                one_line(st_text, 200)
            );
            let _ = writeln!(
                out,
                "step of {} {}, {}",
                item.display_id(),
                item.title,
                location(p, item, cwd)
            );
        }
    }
    let _ = writeln!(out, "\nincoming:");
    let mut any = false;
    for other in &p.items {
        let refs = other
            .links
            .iter()
            .map(|l| (l.key.as_str(), &l.reference))
            .chain(other.mentions.iter().map(|m| ("mention", m)));
        for (kind, r) in refs {
            let Some(t) = &r.target else { continue };
            let hit = match (p.resolve(t), step) {
                (Resolution::Item(i) | Resolution::Step(i, _), None) => i == item_idx,
                (Resolution::Step(i, s), Some(want)) => i == item_idx && s == want,
                _ => false,
            };
            if hit {
                any = true;
                let _ = writeln!(
                    out,
                    "  {:<8} {:<12} {}  ({})",
                    kind,
                    other.display_id(),
                    other.title,
                    location(p, other, cwd)
                );
            }
        }
    }
    if !any {
        let _ = writeln!(out, "  (none)");
    }
    Ok(out)
}

/// FR-307: marks suspect links from or to `id` (or all with `all`) as
/// reviewed and writes the lock file. Errors carry the exit code.
pub fn review(p: &Project, id: Option<&str>, all: bool) -> Result<String, (u8, String)> {
    // New links are recorded first, as by a check (FR-116).
    let mut lock = p.lock.updated(p);
    let marked = if all {
        lock.review(p, |_, _| true)
    } else {
        let id = id.unwrap_or_default();
        let target = parse_target(&p.config, id)
            .ok_or_else(|| (2, format!("\"{id}\" is not an item ID or step reference")))?;
        if p.declarations(&target.id).is_empty() {
            return Err((1, format!("{} not found", target.id)));
        }
        match &target.step {
            None => lock.review(p, |source, t| {
                source.id.as_deref() == Some(target.id.as_str()) || t.id == target.id
            }),
            Some(step) => lock.review(p, |_, t| {
                t.id == target.id && t.step.as_deref() == Some(step.as_str())
            }),
        }
    };
    let path = p.root.join(LOCK_FILE);
    if lock != p.lock || !path.exists() {
        std::fs::write(&path, lock.to_toml())
            .map_err(|e| (2, format!("{}: {e}", path.display())))?;
    }
    let mut out = String::new();
    if marked.is_empty() {
        out.push_str("no suspect links\n");
    }
    for key in marked {
        let _ = writeln!(out, "reviewed {key}");
    }
    Ok(out)
}

/// FR-306: steps, covered steps and uncovered steps per use case.
pub fn coverage(p: &Project) -> String {
    let mut covered: HashSet<(usize, &str)> = HashSet::new();
    for item in &p.items {
        for l in item.links.iter().filter(|l| l.key == IMPL) {
            if let Some(t) = &l.reference.target
                && let (Resolution::Step(uc, _), Some(label)) = (p.resolve(t), &t.step)
            {
                covered.insert((uc, label.as_str()));
            }
        }
    }
    let mut out = String::new();
    let (mut total, mut total_covered) = (0, 0);
    for (i, item) in p
        .items
        .iter()
        .enumerate()
        .filter(|(_, it)| it.kind == USECASE)
    {
        let uncovered: Vec<&str> = item
            .steps
            .iter()
            .map(|s| s.label.as_str())
            .filter(|l| !covered.contains(&(i, *l)))
            .collect();
        let n = item.steps.len();
        total += n;
        total_covered += n - uncovered.len();
        let _ = writeln!(
            out,
            "{} {}: {n} steps, {} covered",
            item.display_id(),
            item.title,
            n - uncovered.len()
        );
        if !uncovered.is_empty() {
            let _ = writeln!(out, "  uncovered: {}", uncovered.join(", "));
        }
    }
    let percent = (total_covered * 100).checked_div(total).unwrap_or(100);
    let _ = writeln!(
        out,
        "total: {total} steps, {total_covered} covered ({percent} %)"
    );
    out
}
