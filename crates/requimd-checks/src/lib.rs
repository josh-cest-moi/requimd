//! Checks on the requimd item graph (FR-100 ff.).
//!
//! Every check has an ID and a default severity; the configuration may
//! override or disable it (FR-117).

use std::collections::{HashMap, HashSet};
use std::ops::Range;

mod language;

use requimd_core::config::{IMPL, REF, STEP, USECASE};
use requimd_core::lock::{link_key, target_hash};
use requimd_core::{Diagnostic, FileId, Item, Level, Project, Reference, Resolution, Severity};

/// Runs all checks and returns the diagnostics sorted by file and position.
pub fn check(project: &Project) -> Vec<Diagnostic> {
    let mut out = Out {
        project,
        diags: Vec::new(),
    };
    duplicate_id(&mut out);
    references(&mut out);
    metadata(&mut out);
    missing_key(&mut out);
    steps(&mut out);
    ref_cycle(&mut out);
    status_link(&mut out);
    placeholder(&mut out);
    suspect_links(&mut out);
    language::run(&mut out);
    let mut diags = out.diags;
    diags.sort_by(|a, b| {
        (a.file, a.span.start, a.check, &a.message).cmp(&(
            b.file,
            b.span.start,
            b.check,
            &b.message,
        ))
    });
    diags
}

pub(crate) struct Out<'a> {
    pub(crate) project: &'a Project,
    diags: Vec<Diagnostic>,
}

impl Out<'_> {
    /// FR-117: applies the configured severity or drops the diagnostic.
    pub(crate) fn push(
        &mut self,
        check: &'static str,
        default: Severity,
        file: FileId,
        span: Range<usize>,
        message: String,
    ) {
        let severity = match self.project.config.check_level(check) {
            Some(Level::Off) => return,
            Some(Level::On(s)) => s,
            None => default,
        };
        self.diags.push(Diagnostic {
            file,
            span,
            severity,
            check,
            message,
        });
    }

    fn location(&self, item: &Item) -> String {
        match self.project.files.get(item.file) {
            Some(f) => {
                let (line, _) = f.line_col(item.heading.start);
                format!("{}:{line}", f.rel)
            }
            None => String::new(),
        }
    }
}

fn items(p: &Project) -> impl Iterator<Item = (usize, &Item)> {
    p.items.iter().enumerate()
}

/// FR-100
fn duplicate_id(out: &mut Out) {
    let p = out.project;
    for item in &p.items {
        let Some(id) = &item.id else { continue };
        let decls = p.declarations(id);
        if decls.len() < 2 {
            continue;
        }
        let others: Vec<String> = decls
            .iter()
            .filter_map(|&i| p.items.get(i))
            .filter(|o| !std::ptr::eq(*o, item))
            .map(|o| out.location(o))
            .collect();
        out.push(
            "duplicate-id",
            Severity::Error,
            item.file,
            item.id_span.clone(),
            format!(
                "ID {id} is declared more than once, also at {}",
                others.join(", ")
            ),
        );
    }
}

/// FR-101 (dangling references) and FR-104 (link target types).
fn references(out: &mut Out) {
    let p = out.project;
    for (_, item) in items(p) {
        for link in &item.links {
            let r = &link.reference;
            match dangling(p, r) {
                Some(msg) => out.push(
                    "dangling-ref",
                    Severity::Error,
                    item.file,
                    r.span.clone(),
                    msg,
                ),
                None => link_target(out, item, &link.key, r),
            }
        }
        for r in &item.mentions {
            if let Some(msg) = dangling(p, r) {
                out.push(
                    "dangling-ref",
                    Severity::Warning,
                    item.file,
                    r.span.clone(),
                    format!("mention: {msg}"),
                );
            }
        }
    }
}

/// FR-101: why a reference does not resolve, if it does not. References
/// to duplicated IDs produce nothing (FR-100).
fn dangling(p: &Project, r: &Reference) -> Option<String> {
    let Some(target) = &r.target else {
        return Some(if r.text.ends_with("-?") {
            format!("placeholder {} cannot be a link target", r.text)
        } else {
            format!("\"{}\" is not a valid item ID or step reference", r.text)
        });
    };
    match p.resolve(target) {
        Resolution::Item(_) | Resolution::Step(..) | Resolution::Duplicate => None,
        Resolution::MissingItem => Some(format!("{} does not exist", target.id)),
        Resolution::MissingStep(_) => Some(format!(
            "{} has no step {}",
            target.id,
            target.step.as_deref().unwrap_or_default()
        )),
    }
}

/// FR-104
fn link_target(out: &mut Out, item: &Item, key: &str, r: &Reference) {
    let p = out.project;
    let Some(allowed) = p.config.link(key).and_then(|l| l.to.as_ref()) else {
        return;
    };
    let Some(target) = &r.target else { return };
    let kind = match p.resolve(target) {
        Resolution::Item(i) => p.items.get(i).map(|t| t.kind.as_str()),
        Resolution::Step(..) => Some(STEP),
        _ => None,
    };
    let Some(kind) = kind else { return };
    if !allowed.iter().any(|a| a == kind) {
        out.push(
            "link-target",
            Severity::Error,
            item.file,
            r.span.clone(),
            format!(
                "\"{key}\" cannot point to {} ({kind}); allowed targets: {}",
                r.text,
                allowed.join(", ")
            ),
        );
    }
}

/// FR-102 (unknown keys) and FR-103 (invalid values).
fn metadata(out: &mut Out) {
    let p = out.project;
    for (_, item) in items(p) {
        let Some(ty) = p.config.type_by_name(&item.kind) else {
            continue;
        };
        for pair in &item.meta {
            if !pair.known {
                out.push(
                    "unknown-key",
                    Severity::Warning,
                    item.file,
                    pair.key_span.clone(),
                    format!(
                        "unknown key \"{}\" for {} items; allowed: {}",
                        pair.key,
                        item.kind,
                        p.config.meta_keys(ty).join(", ")
                    ),
                );
                continue;
            }
            if p.config.link(&pair.key).is_some() {
                continue;
            }
            let Some(values) = p.config.values.get(&pair.key) else {
                continue;
            };
            if !values.contains(&pair.value) {
                out.push(
                    "invalid-value",
                    Severity::Error,
                    item.file,
                    pair.value_span.clone(),
                    format!(
                        "\"{}\" is not a valid value for {}; allowed: {}",
                        pair.value,
                        pair.key,
                        values.join(", ")
                    ),
                );
            }
        }
    }
}

/// FR-105. A key with a configured default counts as present.
fn missing_key(out: &mut Out) {
    let p = out.project;
    for (_, item) in items(p) {
        let Some(ty) = p.config.type_by_name(&item.kind) else {
            continue;
        };
        let present = |key: &str| {
            item.meta.iter().any(|m| m.known && m.key == key)
                || item.fields.iter().any(|f| f.key == key)
                || p.config.defaults.contains_key(key)
        };
        let missing: Vec<String> = ty
            .required
            .iter()
            .filter(|alts| !alts.iter().any(|k| present(k)))
            .map(|alts| alts.join(" or "))
            .collect();
        if !missing.is_empty() {
            out.push(
                "missing-key",
                Severity::Error,
                item.file,
                item.heading.clone(),
                format!(
                    "{} lacks required {}: {}",
                    item.display_id(),
                    if missing.len() == 1 { "key" } else { "keys" },
                    missing.join(", ")
                ),
            );
        }
    }
}

/// FR-106 (uncovered steps) and FR-107 (duplicate step labels).
fn steps(out: &mut Out) {
    let p = out.project;
    // Steps covered by an impl link, by use case and label.
    let mut covered: HashSet<(usize, &str)> = HashSet::new();
    for (_, item) in items(p) {
        for link in item.links.iter().filter(|l| l.key == IMPL) {
            let Some(target) = &link.reference.target else {
                continue;
            };
            if let (Resolution::Step(uc, _), Some(label)) = (p.resolve(target), &target.step) {
                covered.insert((uc, label.as_str()));
            }
        }
    }
    for (i, item) in items(p).filter(|(_, it)| it.kind == USECASE) {
        let mut count: HashMap<&str, usize> = HashMap::new();
        for s in &item.steps {
            *count.entry(&s.label).or_default() += 1;
        }
        // References to a duplicated use case resolve to nothing, so its
        // steps cannot be covered; the duplicate is the only finding.
        let duplicated = item
            .id
            .as_ref()
            .is_some_and(|id| p.declarations(id).len() > 1);
        for s in &item.steps {
            if count.get(s.label.as_str()).copied().unwrap_or(0) > 1 {
                out.push(
                    "duplicate-step",
                    Severity::Error,
                    item.file,
                    s.label_span.clone(),
                    format!(
                        "step label {} is used more than once in {}",
                        s.label,
                        item.display_id()
                    ),
                );
            }
            if !duplicated && !covered.contains(&(i, s.label.as_str())) {
                out.push(
                    "uncovered-step",
                    Severity::Warning,
                    item.file,
                    s.label_span.clone(),
                    format!(
                        "step {} of {} has no implementing item",
                        s.label,
                        item.display_id()
                    ),
                );
            }
        }
    }
}

/// FR-108: a ref link is part of a cycle if source and target are in the
/// same strongly connected component of the ref graph.
fn ref_cycle(out: &mut Out) {
    let p = out.project;
    let n = p.items.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut edges = Vec::new();
    for (u, item) in items(p) {
        for link in item.links.iter().filter(|l| l.key == REF) {
            let Some(target) = &link.reference.target else {
                continue;
            };
            let v = match p.resolve(target) {
                Resolution::Item(v) | Resolution::Step(v, _) => v,
                _ => continue,
            };
            if let Some(a) = adj.get_mut(u) {
                a.push(v);
            }
            edges.push((u, v, &link.reference));
        }
    }
    let comp = components(&adj);
    for (u, v, r) in edges {
        if comp.get(u) == comp.get(v)
            && let Some(item) = p.items.get(u)
        {
            out.push(
                "ref-cycle",
                Severity::Error,
                item.file,
                r.span.clone(),
                format!(
                    "ref link from {} to {} is part of a refinement cycle",
                    item.display_id(),
                    r.text
                ),
            );
        }
    }
}

/// Strongly connected components (Tarjan, iterative). Returns the
/// component number of each node.
fn components(adj: &[Vec<usize>]) -> Vec<usize> {
    const NONE: usize = usize::MAX;
    let n = adj.len();
    let mut index = vec![NONE; n];
    let mut low = vec![0; n];
    let mut on_stack = vec![false; n];
    let mut comp = vec![NONE; n];
    let mut stack = Vec::new();
    let mut next = 0;
    let mut count = 0;
    for start in 0..n {
        if index[start] != NONE {
            continue;
        }
        index[start] = next;
        low[start] = next;
        next += 1;
        stack.push(start);
        on_stack[start] = true;
        let mut calls = vec![(start, 0usize)];
        while let Some(frame) = calls.last_mut() {
            let v = frame.0;
            if let Some(&w) = adj[v].get(frame.1) {
                frame.1 += 1;
                if index[w] == NONE {
                    index[w] = next;
                    low[w] = next;
                    next += 1;
                    stack.push(w);
                    on_stack[w] = true;
                    calls.push((w, 0));
                } else if on_stack[w] {
                    low[v] = low[v].min(index[w]);
                }
            } else {
                calls.pop();
                if let Some(&(parent, _)) = calls.last() {
                    low[parent] = low[parent].min(low[v]);
                }
                if low[v] == index[v] {
                    while let Some(w) = stack.pop() {
                        on_stack[w] = false;
                        comp[w] = count;
                        if w == v {
                            break;
                        }
                    }
                    count += 1;
                }
            }
        }
    }
    comp
}

/// FR-109
fn status_link(out: &mut Out) {
    let p = out.project;
    let status = |item: &Item| -> Option<String> {
        item.meta_value("status")
            .map(str::to_string)
            .or_else(|| p.config.defaults.get("status").cloned())
    };
    for (_, item) in items(p) {
        if status(item).as_deref() != Some("approved") {
            continue;
        }
        for link in &item.links {
            let Some(target) = &link.reference.target else {
                continue;
            };
            let t = match p.resolve(target) {
                Resolution::Item(t) | Resolution::Step(t, _) => t,
                _ => continue,
            };
            let Some(target_item) = p.items.get(t) else {
                continue;
            };
            if let Some(s) = status(target_item)
                && (s == "draft" || s == "deprecated")
            {
                out.push(
                    "status-link",
                    Severity::Warning,
                    item.file,
                    link.reference.span.clone(),
                    format!(
                        "approved item links to {}, which has status {s}",
                        link.reference.text
                    ),
                );
            }
        }
    }
}

/// FR-116: a link whose target changed since its last review. Links
/// without a lock entry are not suspect; a check records them (ADR-0007).
fn suspect_links(out: &mut Out) {
    let p = out.project;
    for item in &p.items {
        let Some(id) = &item.id else { continue };
        for link in &item.links {
            let key = link_key(id, &link.key, &link.reference.text);
            let Some(reviewed) = p.lock.links.get(&key) else {
                continue;
            };
            let current = link
                .reference
                .target
                .as_ref()
                .and_then(|t| target_hash(p, t));
            if current.is_some_and(|c| &c != reviewed) {
                out.push(
                    "suspect-link",
                    Severity::Warning,
                    item.file,
                    link.reference.span.clone(),
                    format!(
                        "{} changed since this link was last reviewed; check {id} and mark the link as reviewed",
                        link.reference.text
                    ),
                );
            }
        }
    }
}

/// FR-115
fn placeholder(out: &mut Out) {
    let p = out.project;
    for (_, item) in items(p).filter(|(_, it)| it.id.is_none()) {
        out.push(
            "placeholder",
            Severity::Warning,
            item.file,
            item.heading.clone(),
            format!(
                "placeholder ID {} has not been replaced by an ID",
                item.display_id()
            ),
        );
    }
}
