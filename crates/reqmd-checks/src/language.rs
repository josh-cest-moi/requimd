//! Language checks: FR-110 weak words, FR-111 missing "shall", FR-113 user
//! story pattern, FR-114 glossary aliases, FR-118 modal verbs and FR-119
//! allow lists and inline markers.

use std::collections::HashMap;
use std::ops::Range;

use reqmd_core::config::{ALLOW_CHECKS, FUNCTIONAL, QUALITY, STORY};
use reqmd_core::words::{Phrases, find_phrase, ignored_ranges, markers, overlaps};
use reqmd_core::{Item, Project, Severity};

use crate::Out;

/// A finding before allow lists and markers are applied.
struct Occurrence {
    check: &'static str,
    span: Range<usize>,
    message: String,
}

/// The word lists, compiled once per run.
struct Lists {
    weak: Phrases,
    reasons: Vec<String>,
    modal: Phrases,
    terms: Phrases,
    aliases: Phrases,
    /// Preferred term of each alias.
    alias_terms: Vec<String>,
    allow: HashMap<&'static str, Phrases>,
}

impl Lists {
    fn new(p: &Project) -> Lists {
        let weak = p.config.weak_words();
        let modal = p.config.modal_verbs();
        let mut aliases = Vec::new();
        let mut alias_terms = Vec::new();
        for t in &p.glossary {
            for a in &t.aliases {
                aliases.push(a.as_str());
                alias_terms.push(t.name.clone());
            }
        }
        Lists {
            weak: Phrases::new(weak.iter().map(|w| w.word.as_str())),
            reasons: weak
                .iter()
                .map(|w| {
                    if w.why.is_empty() {
                        "weak word (no reason given)".to_string()
                    } else {
                        w.why.clone()
                    }
                })
                .collect(),
            modal: Phrases::new(modal.iter().map(String::as_str)),
            terms: Phrases::new(p.glossary.iter().map(|t| t.name.as_str())),
            aliases: Phrases::new(aliases),
            alias_terms,
            allow: ALLOW_CHECKS
                .iter()
                .map(|c| {
                    (
                        *c,
                        Phrases::new(p.config.allow(c).iter().map(String::as_str)),
                    )
                })
                .collect(),
        }
    }
}

pub(crate) fn run(out: &mut Out) {
    let p = out.project;
    let lists = Lists::new(p);
    for item in &p.items {
        let Some(text) = p.files.get(item.file).map(|f| f.text.as_str()) else {
            continue;
        };
        // Code ranges of this item only.
        let code: Vec<Range<usize>> = p
            .code
            .get(item.file)
            .into_iter()
            .flatten()
            .filter(|r| r.end > item.heading.start && r.start < item.body.end)
            .cloned()
            .collect();
        let regions = [
            item.id_span.end..item.heading.end,
            item.meta_span.end..item.body.end,
        ];

        no_shall(out, item, text, &regions[1], &code);
        story(out, item, text, &regions[1]);

        let mut occurrences = Vec::new();
        for region in &regions {
            let mut ignored = ignored_ranges(text, region.clone(), &code);
            ignored.extend(code.iter().cloned());
            ignored.extend(item.mentions.iter().map(|m| m.span.clone()));
            let found = |phrases: &Phrases| -> Vec<(Range<usize>, usize)> {
                phrases
                    .find(text, region.clone())
                    .into_iter()
                    .filter(|(span, _)| !overlaps(&ignored, span))
                    .collect()
            };
            let word = |span: &Range<usize>| text.get(span.clone()).unwrap_or_default();
            // FR-110
            if item.kind == FUNCTIONAL || item.kind == QUALITY {
                for (span, k) in found(&lists.weak) {
                    let why = lists.reasons.get(k).map_or("", String::as_str);
                    occurrences.push(Occurrence {
                        check: "weak-word",
                        message: format!("\"{}\": {why}", word(&span)),
                        span,
                    });
                }
            }
            // FR-118
            if item.kind == FUNCTIONAL {
                for (span, _) in found(&lists.modal) {
                    occurrences.push(Occurrence {
                        check: "modal-verb",
                        message: format!("\"{}\": use \"shall\" for a requirement", word(&span)),
                        span,
                    });
                }
            }
            // FR-114: aliases, except inside an occurrence of a term or of
            // a longer alias.
            let aliases = found(&lists.aliases);
            if aliases.is_empty() {
                continue;
            }
            let terms: Vec<Range<usize>> = lists
                .terms
                .find(text, region.clone())
                .into_iter()
                .map(|(r, _)| r)
                .collect();
            let inside = |s: &Range<usize>, outer: &Range<usize>| {
                outer.start <= s.start && s.end <= outer.end && outer.len() > s.len()
            };
            for (span, k) in &aliases {
                let in_term = terms
                    .iter()
                    .any(|t| t.start <= span.start && span.end <= t.end);
                let in_alias = aliases.iter().any(|(a, _)| inside(span, a));
                if in_term || in_alias {
                    continue;
                }
                let term = lists.alias_terms.get(*k).map_or("", String::as_str);
                occurrences.push(Occurrence {
                    check: "glossary-alias",
                    message: format!(
                        "\"{}\" is an alias of \"{term}\"; use the preferred term",
                        word(span)
                    ),
                    span: span.clone(),
                });
            }
        }
        apply_allows(out, &lists, item, text, &code, occurrences);
    }
}

/// FR-119: drops allowed occurrences, reports the rest and unused marker
/// entries. Markers in code are examples, not markers (FR-009).
fn apply_allows(
    out: &mut Out,
    lists: &Lists,
    item: &Item,
    text: &str,
    code: &[Range<usize>],
    occurrences: Vec<Occurrence>,
) {
    let item_range = item.heading.start..item.body.end;
    let marks: Vec<_> = markers(text, item_range.clone())
        .into_iter()
        .filter(|m| !overlaps(code, &m.span))
        .collect();
    let mut used: Vec<Vec<bool>> = marks.iter().map(|m| vec![false; m.entries.len()]).collect();
    // Occurrences of the project allow lists, computed when needed.
    let mut allowed: HashMap<&str, Vec<Range<usize>>> = HashMap::new();
    for occ in occurrences {
        let spans = allowed.entry(occ.check).or_insert_with(|| {
            lists
                .allow
                .get(occ.check)
                .map(|a| {
                    a.find(text, item_range.clone())
                        .into_iter()
                        .map(|(r, _)| r)
                        .collect()
                })
                .unwrap_or_default()
        });
        let project_allowed = spans
            .iter()
            .any(|r| r.start <= occ.span.start && occ.span.end <= r.end);
        let mut marker_allowed = false;
        for (m, marker) in marks.iter().enumerate() {
            if !(marker.line.start <= occ.span.start && occ.span.start < marker.line.end) {
                continue;
            }
            for (e, (entry, _)) in marker.entries.iter().enumerate() {
                let hit = find_phrase(text, marker.line.start..marker.span.start, entry)
                    .iter()
                    .any(|r| r.start <= occ.span.start && occ.span.end <= r.end);
                if hit {
                    marker_allowed = true;
                    if let Some(u) = used.get_mut(m).and_then(|u| u.get_mut(e)) {
                        *u = true;
                    }
                }
            }
        }
        if !project_allowed && !marker_allowed {
            out.push(
                occ.check,
                Severity::Warning,
                item.file,
                occ.span,
                occ.message,
            );
        }
    }
    for (m, marker) in marks.iter().enumerate() {
        for (e, (entry, span)) in marker.entries.iter().enumerate() {
            if !used.get(m).and_then(|u| u.get(e)).copied().unwrap_or(true) {
                out.push(
                    "unused-allow",
                    Severity::Warning,
                    item.file,
                    span.clone(),
                    format!("\"{entry}\" is allowed here, but nothing on this line needs it"),
                );
            }
        }
    }
}

/// FR-111
fn no_shall(out: &mut Out, item: &Item, text: &str, region: &Range<usize>, code: &[Range<usize>]) {
    if item.kind != FUNCTIONAL {
        return;
    }
    let has_shall = find_phrase(text, region.clone(), "shall")
        .iter()
        .any(|r| !overlaps(code, r));
    if !has_shall {
        out.push(
            "no-shall",
            Severity::Info,
            item.file,
            item.heading.clone(),
            format!("{} has no \"shall\" in its text", item.display_id()),
        );
    }
}

/// FR-113: "As a …, I want … so that …" and an `ac` body field.
fn story(out: &mut Out, item: &Item, text: &str, region: &Range<usize>) {
    if item.kind != STORY {
        return;
    }
    let body = text.get(region.clone()).unwrap_or_default().trim_start();
    let first = body.split("\n\n").next().unwrap_or_default();
    let first = first
        .split("\r\n\r\n")
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let follows = (first.starts_with("as a ") || first.starts_with("as an "))
        && first.find("i want ").is_some_and(|w| {
            first
                .get(w..)
                .is_some_and(|rest| rest.contains(" so that "))
        });
    if !follows {
        out.push(
            "story-pattern",
            Severity::Warning,
            item.file,
            item.heading.clone(),
            "user story does not follow \"As a …, I want … so that …\"".into(),
        );
    }
    if !item.fields.iter().any(|f| f.key == "ac") {
        out.push(
            "story-pattern",
            Severity::Warning,
            item.file,
            item.heading.clone(),
            "user story has no acceptance criteria (\"ac:\")".into(),
        );
    }
}
