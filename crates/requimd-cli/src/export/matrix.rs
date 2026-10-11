//! FR-401: traceability matrix, one row per use case step.

use requimd_core::config::{IMPL, USECASE};
use requimd_core::{Item, Project, Resolution};

/// The link key of verifying items, as `impl` for implementing ones.
pub const VER: &str = "ver";

pub struct Row {
    /// Index of the use case item.
    pub use_case: usize,
    pub use_case_id: String,
    pub label: String,
    pub text: String,
    /// Indices of items that implement the step.
    pub implemented_by: Vec<usize>,
    /// Indices of items that verify the step or one of its implementing
    /// items.
    pub verified_by: Vec<usize>,
}

fn one_line(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn links_to(p: &Project, item: &Item, key: &str, want: impl Fn(Resolution) -> bool) -> bool {
    item.links.iter().any(|l| {
        l.key == key
            && l.reference
                .target
                .as_ref()
                .is_some_and(|t| want(p.resolve(t)))
    })
}

/// One row per step of every use case, in document order.
pub fn rows(p: &Project) -> Vec<Row> {
    let mut rows = Vec::new();
    for (i, uc) in p
        .items
        .iter()
        .enumerate()
        .filter(|(_, it)| it.kind == USECASE)
    {
        let text = p.files.get(uc.file).map_or("", |f| f.text.as_str());
        for (s, step) in uc.steps.iter().enumerate() {
            let implemented_by: Vec<usize> = (0..p.items.len())
                .filter(|&j| {
                    p.items
                        .get(j)
                        .is_some_and(|it| links_to(p, it, IMPL, |r| r == Resolution::Step(i, s)))
                })
                .collect();
            let verified_by: Vec<usize> = (0..p.items.len())
                .filter(|&j| {
                    p.items.get(j).is_some_and(|it| {
                        links_to(p, it, VER, |r| match r {
                            Resolution::Item(k) => implemented_by.contains(&k),
                            Resolution::Step(u, t) => u == i && t == s,
                            _ => false,
                        })
                    })
                })
                .collect();
            rows.push(Row {
                use_case: i,
                use_case_id: uc.display_id(),
                label: step.label.clone(),
                text: one_line(text.get(step.text_span.clone()).unwrap_or_default()),
                implemented_by,
                verified_by,
            });
        }
    }
    rows
}

fn ids(p: &Project, items: &[usize]) -> String {
    items
        .iter()
        .filter_map(|&i| p.items.get(i))
        .map(Item::display_id)
        .collect::<Vec<_>>()
        .join(" ")
}

fn csv_field(s: &str) -> String {
    if s.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

/// The matrix as CSV with a header line; several IDs are separated by
/// spaces.
pub fn csv(p: &Project) -> String {
    let mut out = String::from("use_case,step,text,implemented_by,verified_by\n");
    for r in rows(p) {
        let fields = [
            r.use_case_id.clone(),
            r.label.clone(),
            r.text.clone(),
            ids(p, &r.implemented_by),
            ids(p, &r.verified_by),
        ];
        let line: Vec<String> = fields.iter().map(|f| csv_field(f)).collect();
        out.push_str(&line.join(","));
        out.push('\n');
    }
    out
}
