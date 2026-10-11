//! FR-304: formatting edits. Placeholder IDs get the next free IDs, and
//! metadata pairs are written as `key: value`, separated by two spaces.

use std::collections::HashMap;
use std::ops::Range;

use crate::project::Project;
use crate::source::FileId;

/// A replacement of a byte range of a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    pub file: FileId,
    pub range: Range<usize>,
    pub text: String,
}

/// All formatting edits of the project, in file and position order.
pub fn format_edits(p: &Project) -> Vec<Edit> {
    let mut edits = Vec::new();
    let mut next: HashMap<&str, u64> = HashMap::new();
    for item in &p.items {
        let Some(text) = p.files.get(item.file).map(|f| f.text.as_str()) else {
            continue;
        };
        // Placeholders, numbered in file order (FR-212 for each one).
        if item.id.is_none() {
            let n = next
                .entry(item.prefix.as_str())
                .or_insert_with(|| p.highest_number(&item.prefix));
            *n = n.saturating_add(1);
            edits.push(Edit {
                file: item.file,
                range: item.id_span.clone(),
                text: p.format_id(&item.prefix, *n),
            });
        }
        // Metadata lines.
        let mut line_start = None;
        let mut pairs: Vec<&crate::model::MetaPair> = Vec::new();
        let mut flush = |start: Option<usize>, pairs: &mut Vec<&crate::model::MetaPair>| {
            let (Some(start), Some(first), Some(last)) = (start, pairs.first(), pairs.last())
            else {
                return;
            };
            let range = first.key_span.start..last.value_span.end.max(last.key_span.end + 1);
            let formatted = pairs
                .iter()
                .map(|m| {
                    if m.value.is_empty() {
                        format!("{}:", m.key)
                    } else {
                        format!("{}: {}", m.key, m.value)
                    }
                })
                .collect::<Vec<_>>()
                .join("  ");
            if text.get(range.clone()) != Some(formatted.as_str()) && start <= range.start {
                edits.push(Edit {
                    file: item.file,
                    range,
                    text: formatted,
                });
            }
            pairs.clear();
        };
        if item.meta_span.start < item.meta_span.end {
            for m in &item.meta {
                let ls = text
                    .get(..m.key_span.start)
                    .and_then(|s| s.rfind('\n'))
                    .map_or(0, |i| i + 1);
                if line_start != Some(ls) {
                    flush(line_start, &mut pairs);
                    line_start = Some(ls);
                }
                pairs.push(m);
            }
            flush(line_start, &mut pairs);
        }
    }
    edits.sort_by_key(|e| (e.file, e.range.start));
    edits
}

/// Applies edits of one file to its text.
pub fn apply(text: &str, edits: &[&Edit]) -> String {
    let mut sorted: Vec<&&Edit> = edits.iter().collect();
    sorted.sort_by_key(|e| std::cmp::Reverse(e.range.start));
    let mut out = text.to_string();
    for e in sorted {
        if e.range.start <= e.range.end
            && e.range.end <= out.len()
            && out.is_char_boundary(e.range.start)
            && out.is_char_boundary(e.range.end)
        {
            out.replace_range(e.range.clone(), &e.text);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::{Config, SourceFile};

    fn formatted(files: &[(&str, &str)]) -> Vec<String> {
        let files = files
            .iter()
            .map(|(n, t)| SourceFile::new(PathBuf::from(n), n.to_string(), t.to_string()))
            .collect();
        let p = Project::from_files(PathBuf::from("."), Config::defaults().unwrap(), files);
        let edits = format_edits(&p);
        p.files
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let mine: Vec<&Edit> = edits.iter().filter(|e| e.file == i).collect();
                apply(&f.text, &mine)
            })
            .collect()
    }

    #[test]
    fn fr_304_placeholders_get_consecutive_ids() {
        let out = formatted(&[
            ("a.md", "## FR-? A\n\n## FR-007 B\n\n## FR-? C\n"),
            ("b.md", "## FR-? D\n## UC-? U\n"),
        ]);
        assert_eq!(out[0], "## FR-008 A\n\n## FR-007 B\n\n## FR-009 C\n");
        assert_eq!(out[1], "## FR-010 D\n## UC-001 U\n");
    }

    #[test]
    fn fr_304_metadata_separators_normalised() {
        let out = formatted(&[(
            "a.md",
            "## FR-001 A\nimpl:UC-001 \tprio:    must\nstatus:\treview\n\nText: stays.\n",
        )]);
        // `impl:UC-001` is no key (no space after the colon) and stays text.
        assert_eq!(
            formatted(&[(
                "a.md",
                "## FR-001 A\nimpl: UC-001 \tprio:    must\nstatus:\treview\n"
            )])[0],
            "## FR-001 A\nimpl: UC-001  prio: must\nstatus: review\n"
        );
        assert!(out[0].contains("Text: stays."));
    }

    #[test]
    fn fr_304_formatted_text_is_stable() {
        let t = "## FR-001 A\nimpl: UC-001  prio: must\n\nText.\n";
        assert_eq!(formatted(&[("a.md", t)])[0], t);
    }
}
