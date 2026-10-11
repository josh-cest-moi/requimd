//! FR-402: the item graph as JSON.

use std::collections::BTreeMap;

use requimd_core::{Item, Project, Reference, Resolution};
use serde::Serialize;

/// Increased on incompatible changes of the output.
pub const FORMAT: u32 = 1;

#[derive(Serialize)]
struct Graph {
    format: u32,
    files: Vec<String>,
    items: Vec<JsonItem>,
}

#[derive(Serialize)]
struct JsonItem {
    /// `null` for a placeholder such as `FR-?`.
    id: Option<String>,
    display_id: String,
    #[serde(rename = "type")]
    kind: String,
    title: String,
    status: Option<String>,
    file: String,
    line: usize,
    meta: BTreeMap<String, String>,
    fields: BTreeMap<String, String>,
    text: String,
    steps: Vec<JsonStep>,
    links: Vec<JsonLink>,
    mentions: Vec<JsonRef>,
}

#[derive(Serialize)]
struct JsonStep {
    label: String,
    text: String,
    line: usize,
}

#[derive(Serialize)]
struct JsonLink {
    key: String,
    #[serde(flatten)]
    reference: JsonRef,
}

#[derive(Serialize)]
struct JsonRef {
    target: String,
    /// Whether the target exists and is unique.
    resolved: bool,
    line: usize,
    column: usize,
}

fn reference(p: &Project, item: &Item, r: &Reference) -> JsonRef {
    let (line, column) = p
        .files
        .get(item.file)
        .map_or((0, 0), |f| f.line_col(r.span.start));
    let resolved = r
        .target
        .as_ref()
        .is_some_and(|t| matches!(p.resolve(t), Resolution::Item(_) | Resolution::Step(..)));
    JsonRef {
        target: r.text.clone(),
        resolved,
        line,
        column,
    }
}

pub fn json(p: &Project) -> String {
    let items = p
        .items
        .iter()
        .map(|it| {
            let f = p.files.get(it.file);
            let text = f.map_or("", |f| f.text.as_str());
            let get =
                |r: std::ops::Range<usize>| text.get(r).unwrap_or_default().trim().to_string();
            JsonItem {
                id: it.id.clone(),
                display_id: it.display_id(),
                kind: it.kind.clone(),
                title: it.title.clone(),
                status: it
                    .meta_value("status")
                    .map(str::to_string)
                    .or_else(|| p.config.defaults.get("status").cloned()),
                file: f.map(|f| f.rel.clone()).unwrap_or_default(),
                line: f.map_or(0, |f| f.line_col(it.heading.start).0),
                meta: it
                    .meta
                    .iter()
                    .filter(|m| m.known && p.config.link(&m.key).is_none())
                    .map(|m| (m.key.clone(), m.value.clone()))
                    .collect(),
                fields: it
                    .fields
                    .iter()
                    .map(|b| (b.key.clone(), get(b.value_span.clone())))
                    .collect(),
                text: get(it.meta_span.end..it.body.end),
                steps: it
                    .steps
                    .iter()
                    .map(|s| JsonStep {
                        label: s.label.clone(),
                        text: get(s.text_span.clone()),
                        line: f.map_or(0, |f| f.line_col(s.label_span.start).0),
                    })
                    .collect(),
                links: it
                    .links
                    .iter()
                    .map(|l| JsonLink {
                        key: l.key.clone(),
                        reference: reference(p, it, &l.reference),
                    })
                    .collect(),
                mentions: it.mentions.iter().map(|m| reference(p, it, m)).collect(),
            }
        })
        .collect();
    let graph = Graph {
        format: FORMAT,
        files: p.files.iter().map(|f| f.rel.clone()).collect(),
        items,
    };
    serde_json::to_string_pretty(&graph).map_or_else(|_| "{}".into(), |s| s + "\n")
}
