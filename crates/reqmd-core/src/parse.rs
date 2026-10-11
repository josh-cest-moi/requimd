//! Parser for reqmd files (FR-001 to FR-010, FR-012).
//!
//! pulldown-cmark finds headings, lists and code; items, metadata, body
//! fields, steps and references are recognised on the source text. The
//! parser never fails: anything it does not recognise stays plain text.

use std::ops::Range;

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::config::{Config, ItemType, USECASE};
use crate::model::{BodyField, Item, Link, MetaPair, Reference, Step, Target, Term};
use crate::source::FileId;

/// Parses one file into its items.
pub fn parse_file(config: &Config, file: FileId, text: &str) -> Vec<Item> {
    parse(config, file, text).items
}

/// The result of parsing one file.
#[derive(Debug, Clone, Default)]
pub struct Parsed {
    pub items: Vec<Item>,
    /// Code blocks and inline code spans, in document order (FR-009).
    pub code: Vec<Range<usize>>,
}

/// Parses one file into its items and code ranges.
pub fn parse(config: &Config, file: FileId, text: &str) -> Parsed {
    let mut doc = scan(text);
    let mut heads: Vec<(usize, Head)> = Vec::new();
    for (i, h) in doc.headings.iter_mut().enumerate() {
        if let Some(head) = item_head(config, text, h) {
            h.is_item = true;
            heads.push((i, head));
        }
    }

    let mut items = Vec::new();
    for (hi, head) in heads {
        let Some(h) = doc.headings.get(hi) else {
            continue;
        };
        let Some(ty) = config.type_by_prefix(&head.prefix) else {
            continue;
        };
        // FR-002: the body ends at the next heading of the same or a higher
        // level, or at the next item heading of any level.
        let end = doc
            .headings
            .iter()
            .enumerate()
            .skip(hi + 1)
            .find(|(_, o)| o.level <= h.level || o.is_item)
            .map_or(text.len(), |(_, o)| o.range.start);
        let body_start = next_line(text, h.range.end.saturating_sub(1)).min(end);
        items.push(parse_item(
            config,
            ty,
            text,
            &doc,
            file,
            h,
            head,
            body_start..end,
        ));
    }
    Parsed {
        items,
        code: doc.code,
    }
}

struct Heading {
    level: u8,
    range: Range<usize>,
    /// Text and code pieces of the heading content.
    pieces: Vec<(Piece, Range<usize>)>,
    /// Set after item detection, for FR-002.
    is_item: bool,
}

enum Piece {
    Text(String),
    Code(String),
}

struct Doc {
    headings: Vec<Heading>,
    /// Code blocks and inline code spans, in document order (FR-009).
    code: Vec<Range<usize>>,
    list_items: Vec<Range<usize>>,
    lists: Vec<Range<usize>>,
}

impl Doc {
    fn in_code(&self, pos: usize) -> bool {
        in_ranges(&self.code, pos)
    }
}

fn scan(text: &str) -> Doc {
    let mut doc = Doc {
        headings: Vec::new(),
        code: Vec::new(),
        list_items: Vec::new(),
        lists: Vec::new(),
    };
    let mut current: Option<Heading> = None;
    for (event, range) in Parser::new_ext(text, Options::empty()).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                current = Some(Heading {
                    level: level_number(level),
                    range,
                    pieces: Vec::new(),
                    is_item: false,
                });
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(h) = current.take() {
                    doc.headings.push(h);
                }
            }
            Event::Start(Tag::CodeBlock(_)) => doc.code.push(range),
            Event::Code(code) => {
                if let Some(h) = current.as_mut() {
                    h.pieces
                        .push((Piece::Code(code.to_string()), range.clone()));
                }
                doc.code.push(range);
            }
            Event::Text(t) => {
                if let Some(h) = current.as_mut() {
                    h.pieces.push((Piece::Text(t.to_string()), range));
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if let Some(h) = current.as_mut() {
                    h.pieces.push((Piece::Text(" ".into()), range));
                }
            }
            Event::Start(Tag::Item) => doc.list_items.push(range),
            Event::Start(Tag::List(_)) => doc.lists.push(range),
            _ => {}
        }
    }
    doc.code.sort_by_key(|r| r.start);
    doc
}

fn level_number(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

struct Head {
    id: Option<String>,
    prefix: String,
    id_span: Range<usize>,
    title: String,
}

/// FR-001, FR-010: a heading whose first token is an item ID or a
/// placeholder ID. A heading that starts with code is not an item (FR-009).
fn item_head(config: &Config, text: &str, h: &Heading) -> Option<Head> {
    let (Piece::Text(first), range) = h.pieces.first()? else {
        return None;
    };
    let trimmed = first.trim_start();
    let lead = first.len() - trimmed.len();
    let token = trimmed.split_whitespace().next()?;
    // Map the token to the source; exact if the text piece is unescaped.
    let start = if text.get(range.clone()) == Some(first.as_str()) {
        range.start + lead
    } else {
        range.start
    };
    let id_span = start..start + token.len();

    let (id, prefix) = if let Some(prefix) = token.strip_suffix("-?") {
        config.type_by_prefix(prefix)?;
        (None, prefix.to_string())
    } else {
        let target = parse_target(config, token)?;
        if target.step.is_some() {
            return None;
        }
        let prefix = target.id.split('-').next().unwrap_or_default().to_string();
        (Some(target.id), prefix)
    };

    let mut full = String::new();
    for (piece, _) in &h.pieces {
        match piece {
            Piece::Text(t) | Piece::Code(t) => full.push_str(t),
        }
    }
    let title = full
        .trim_start()
        .get(token.len()..)
        .unwrap_or_default()
        .trim()
        .to_string();
    Some(Head {
        id,
        prefix,
        id_span,
        title,
    })
}

#[allow(clippy::too_many_arguments)]
fn parse_item(
    config: &Config,
    ty: &ItemType,
    text: &str,
    doc: &Doc,
    file: FileId,
    h: &Heading,
    head: Head,
    body: Range<usize>,
) -> Item {
    let mut item = Item {
        file,
        id: head.id,
        prefix: head.prefix,
        kind: ty.name.clone(),
        title: head.title,
        level: h.level,
        heading: h.range.start..trim_end(text, h.range.clone()).end,
        id_span: head.id_span,
        body: body.clone(),
        meta_span: body.start..body.start,
        meta: Vec::new(),
        links: Vec::new(),
        fields: Vec::new(),
        steps: Vec::new(),
        mentions: Vec::new(),
    };

    let meta_end = parse_meta(config, ty, text, doc, body.clone(), &mut item);
    item.meta_span = body.start..meta_end;
    let rest = meta_end..body.end;
    parse_fields(ty, text, doc, rest.clone(), &mut item);
    if ty.name == USECASE {
        parse_steps(text, doc, rest, &mut item);
    }
    // FR-008: mentions anywhere in the body except metadata and code.
    scan_refs(
        config,
        text,
        body.clone(),
        |p| (body.start..meta_end).contains(&p) || doc.in_code(p),
        &mut item.mentions,
    );
    item
}

/// FR-003, FR-004: parses the metadata block and returns where it ends.
fn parse_meta(
    config: &Config,
    ty: &ItemType,
    text: &str,
    doc: &Doc,
    body: Range<usize>,
    item: &mut Item,
) -> usize {
    let b = text.as_bytes();
    let mut pos = body.start;
    let mut first = true;
    while pos < body.end {
        let end = trim_end(text, pos..line_end(text, pos).min(body.end)).end;
        let at = skip_indent(b, pos, end);
        if at >= end || doc.in_code(at) {
            break;
        }
        let Some((key_end, _)) = ident_colon(b, at) else {
            break;
        };
        let key = text.get(at..key_end).unwrap_or_default();
        if ty.body.iter().any(|k| k == key) {
            // A body field, not metadata (FR-102).
            break;
        }
        if first && !config.is_meta_key(ty, key) {
            // FR-003: no metadata block; FR-102 still reports the key.
            item.meta.push(MetaPair {
                key: key.to_string(),
                key_span: at..key_end,
                value: String::new(),
                value_span: key_end..key_end,
                known: false,
            });
            return body.start;
        }
        for (k, v) in split_pairs(b, at, end) {
            let key = text.get(k.clone()).unwrap_or_default().to_string();
            let value = text.get(v.clone()).unwrap_or_default().to_string();
            let known = config.is_meta_key(ty, &key) || ty.body.contains(&key);
            if known && config.link(&key).is_some() {
                add_links(config, text, &key, v.clone(), &mut item.links);
            }
            item.meta.push(MetaPair {
                key,
                key_span: k,
                value,
                value_span: v,
                known,
            });
        }
        first = false;
        pos = next_line(text, pos);
    }
    pos.min(body.end)
}

/// FR-004: link values are split at commas.
fn add_links(config: &Config, text: &str, key: &str, value: Range<usize>, out: &mut Vec<Link>) {
    let mut start = value.start;
    while start <= value.end {
        let end = text
            .get(start..value.end)
            .and_then(|s| s.find(','))
            .map_or(value.end, |i| start + i);
        let part = trim(text, start..end);
        if !part.is_empty() {
            let s = text.get(part.clone()).unwrap_or_default();
            out.push(Link {
                key: key.to_string(),
                reference: Reference {
                    text: s.to_string(),
                    span: part,
                    target: parse_target(config, s),
                },
            });
        }
        start = end + 1;
    }
}

/// Splits a metadata line into key and value ranges. A new pair starts
/// after two or more spaces or a tab, followed by `key:` (FR-003).
fn split_pairs(b: &[u8], at: usize, end: usize) -> Vec<(Range<usize>, Range<usize>)> {
    let mut pairs = Vec::new();
    let Some((mut key_end, mut value_start)) = ident_colon(b, at) else {
        return pairs;
    };
    let mut key_start = at;
    let mut i = value_start;
    while i < end {
        if matches!(b.get(i), Some(b' ' | b'\t')) {
            let ws = i;
            let mut tab = false;
            while i < end && matches!(b.get(i), Some(b' ' | b'\t')) {
                tab |= b.get(i) == Some(&b'\t');
                i += 1;
            }
            if (tab || i - ws >= 2)
                && i < end
                && let Some((ke, after)) = ident_colon(b, i)
            {
                pairs.push((key_start..key_end, trim_bytes(b, value_start..ws)));
                key_start = i;
                key_end = ke;
                value_start = after.min(end);
                i = value_start;
            }
        } else {
            i += 1;
        }
    }
    pairs.push((key_start..key_end, trim_bytes(b, value_start.min(end)..end)));
    pairs
}

/// FR-005: body fields of the item type.
fn parse_fields(ty: &ItemType, text: &str, doc: &Doc, rest: Range<usize>, item: &mut Item) {
    if ty.body.is_empty() {
        return;
    }
    let b = text.as_bytes();
    let mut pos = rest.start;
    while pos < rest.end {
        let end = trim_end(text, pos..line_end(text, pos).min(rest.end)).end;
        let at = skip_indent(b, pos, end);
        if at < end
            && !doc.in_code(at)
            && let Some((key_end, after)) = ident_colon(b, at)
        {
            let key = text.get(at..key_end).unwrap_or_default();
            if ty.body.iter().any(|k| k == key) {
                let mut value = trim(text, after.min(end)..end);
                let next = next_line(text, pos);
                let i = doc.lists.partition_point(|l| l.start < next);
                if let Some(list) = doc.lists.get(i)
                    && list.start < rest.end
                    && text
                        .get(next..list.start)
                        .is_some_and(|s| s.len() <= 3 && s.bytes().all(|c| c == b' '))
                {
                    let list_end = trim_end(text, list.start..list.end.min(rest.end)).end;
                    value = if value.is_empty() {
                        list.start..list_end
                    } else {
                        value.start..list_end
                    };
                }
                item.fields.push(BodyField {
                    key: key.to_string(),
                    key_span: at..key_end,
                    value_span: value,
                });
            }
        }
        pos = next_line(text, pos);
    }
}

/// FR-006: list items that start with a step label. The label is
/// independent of the list position (BR-003).
fn parse_steps(text: &str, doc: &Doc, rest: Range<usize>, item: &mut Item) {
    let b = text.as_bytes();
    for li in doc.list_items.iter().filter(|r| rest.contains(&r.start)) {
        let end = li.end.min(rest.end);
        let mut p = skip_indent(b, li.start, end);
        // List marker: -, *, + or digits followed by . or ).
        match b.get(p) {
            Some(b'-' | b'*' | b'+') => p += 1,
            Some(c) if c.is_ascii_digit() => {
                while b.get(p).is_some_and(u8::is_ascii_digit) {
                    p += 1;
                }
                if !matches!(b.get(p), Some(b'.' | b')')) {
                    continue;
                }
                p += 1;
            }
            _ => continue,
        }
        while p < end && matches!(b.get(p), Some(b' ' | b'\t')) {
            p += 1;
        }
        let Some(label_end) = match_label(b, p, end) else {
            continue;
        };
        if b.get(label_end) != Some(&b':') {
            continue;
        }
        item.steps.push(Step {
            label: text.get(p..label_end).unwrap_or_default().to_string(),
            label_span: p..label_end,
            text_span: trim(text, label_end + 1..end),
        });
    }
}

/// FR-008: references as whole-word tokens in `range`, skipping positions
/// for which `skip` is true.
fn scan_refs(
    config: &Config,
    text: &str,
    range: Range<usize>,
    skip: impl Fn(usize) -> bool,
    out: &mut Vec<Reference>,
) {
    let b = text.as_bytes();
    let mut i = range.start;
    while i < range.end {
        let c = b.get(i).copied().unwrap_or(b' ');
        if c.is_ascii_alphanumeric() {
            if c.is_ascii_alphabetic()
                && !word_before(text, i)
                && !skip(i)
                && let Some((end, target)) = match_ref(config, text, i, range.end)
            {
                out.push(Reference {
                    text: text.get(i..end).unwrap_or_default().to_string(),
                    span: i..end,
                    target: Some(target),
                });
                i = end;
                continue;
            }
            // Skip the rest of the word.
            while b.get(i).is_some_and(|c| c.is_ascii_alphanumeric()) && i < range.end {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
}

/// FR-011: every heading of the glossary file is a term; a first line
/// `alias: a, b` lists alternative spellings.
pub fn parse_glossary(text: &str) -> Vec<Term> {
    let doc = scan(text);
    let mut terms = Vec::new();
    for (i, h) in doc.headings.iter().enumerate() {
        let mut name = String::new();
        for (piece, _) in &h.pieces {
            match piece {
                Piece::Text(t) | Piece::Code(t) => name.push_str(t),
            }
        }
        let name = name.split_whitespace().collect::<Vec<_>>().join(" ");
        if name.is_empty() {
            continue;
        }
        let end = doc
            .headings
            .get(i + 1)
            .map_or(text.len(), |n| n.range.start);
        let mut start = next_line(text, h.range.end.saturating_sub(1)).min(end);
        let mut aliases = Vec::new();
        if let Some((key_end, after)) = ident_colon(text.as_bytes(), start)
            && text.get(start..key_end) == Some("alias")
        {
            let line_end = line_end(text, start).min(end);
            aliases = text
                .get(after..line_end)
                .unwrap_or_default()
                .split(',')
                .map(str::trim)
                .filter(|a| !a.is_empty())
                .map(str::to_string)
                .collect();
            start = next_line(text, start).min(end);
        }
        terms.push(Term {
            name,
            heading: h.range.start..trim_end(text, h.range.clone()).end,
            aliases,
            definition: trim(text, start..end),
        });
    }
    terms
}

/// Code blocks and inline code spans of a text, in document order (FR-009).
pub fn code_ranges(text: &str) -> Vec<Range<usize>> {
    scan(text).code
}

/// `key:` at `at` followed by whitespace or the line end, as in metadata
/// lines and body fields. Returns the end of the key and the position after
/// the colon.
pub fn key_colon(text: &str, at: usize) -> Option<(usize, usize)> {
    ident_colon(text.as_bytes(), at)
}

/// Parses a whole string as a reference (FR-007), e.g. `UC-012.s2`.
pub fn parse_target(config: &Config, s: &str) -> Option<Target> {
    match match_ref(config, s, 0, s.len()) {
        Some((end, target)) if end == s.len() => Some(target),
        _ => None,
    }
}

/// Matches an item ID (BR-001), optionally with a step suffix, at `at`.
/// Returns the end and the target.
fn match_ref(config: &Config, text: &str, at: usize, end: usize) -> Option<(usize, Target)> {
    let b = text.as_bytes();
    let mut i = at;
    while i < end && b.get(i).is_some_and(u8::is_ascii_alphanumeric) {
        i += 1;
    }
    if i >= end || b.get(i) != Some(&b'-') {
        return None;
    }
    config.type_by_prefix(text.get(at..i)?)?;
    let digits = i + 1;
    let mut j = digits;
    while j < end && b.get(j).is_some_and(u8::is_ascii_digit) {
        j += 1;
    }
    if j - digits < config.digits.max(1) {
        return None;
    }
    let id = text.get(at..j)?.to_string();
    if j < end
        && b.get(j) == Some(&b'.')
        && let Some(label_end) = match_label(b, j + 1, end)
        && !word_after(text, label_end, end)
    {
        let step = text.get(j + 1..label_end)?.to_string();
        return Some((
            label_end,
            Target {
                id,
                step: Some(step),
            },
        ));
    }
    if word_after(text, j, end) {
        return None;
    }
    Some((j, Target { id, step: None }))
}

/// FR-006: a step label `s` + digits + lowercase letters and digits.
/// Returns the end of the label.
fn match_label(b: &[u8], at: usize, end: usize) -> Option<usize> {
    if at >= end || b.get(at) != Some(&b's') {
        return None;
    }
    let mut i = at + 1;
    let digits = i;
    while i < end && b.get(i).is_some_and(u8::is_ascii_digit) {
        i += 1;
    }
    if i == digits {
        return None;
    }
    while i < end
        && b.get(i)
            .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    {
        i += 1;
    }
    Some(i)
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-'
}

fn word_before(text: &str, i: usize) -> bool {
    text.get(..i)
        .and_then(|s| s.chars().next_back())
        .is_some_and(is_word_char)
}

fn word_after(text: &str, i: usize, end: usize) -> bool {
    i < end
        && text
            .get(i..end)
            .and_then(|s| s.chars().next())
            .is_some_and(is_word_char)
}

/// `key:` followed by whitespace or the line end. Returns the end of the
/// key and the position after the colon.
fn ident_colon(b: &[u8], at: usize) -> Option<(usize, usize)> {
    if !b.get(at)?.is_ascii_alphabetic() {
        return None;
    }
    let mut i = at + 1;
    while b
        .get(i)
        .is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_' || *c == b'-')
    {
        i += 1;
    }
    if b.get(i) != Some(&b':') {
        return None;
    }
    match b.get(i + 1) {
        None | Some(b' ' | b'\t' | b'\r' | b'\n') => Some((i, i + 1)),
        _ => None,
    }
}

/// Skips up to three spaces of indentation.
fn skip_indent(b: &[u8], pos: usize, end: usize) -> usize {
    let mut p = pos;
    while p < end && p - pos < 3 && b.get(p) == Some(&b' ') {
        p += 1;
    }
    p
}

fn in_ranges(ranges: &[Range<usize>], pos: usize) -> bool {
    let i = ranges.partition_point(|r| r.start <= pos);
    i > 0 && ranges.get(i - 1).is_some_and(|r| pos < r.end)
}

/// End of the line at `pos`, excluding the newline.
fn line_end(text: &str, pos: usize) -> usize {
    text.get(pos..)
        .and_then(|s| s.find('\n'))
        .map_or(text.len(), |i| pos + i)
}

/// Start of the line after the one containing `pos`.
fn next_line(text: &str, pos: usize) -> usize {
    let end = line_end(text, pos);
    if end < text.len() { end + 1 } else { end }
}

fn trim(text: &str, r: Range<usize>) -> Range<usize> {
    trim_bytes(text.as_bytes(), r)
}

fn trim_end(text: &str, r: Range<usize>) -> Range<usize> {
    let b = text.as_bytes();
    let mut end = r.end;
    while end > r.start && b.get(end - 1).is_some_and(u8::is_ascii_whitespace) {
        end -= 1;
    }
    r.start..end
}

fn trim_bytes(b: &[u8], r: Range<usize>) -> Range<usize> {
    let mut start = r.start;
    let mut end = r.end.max(r.start);
    while start < end && b.get(start).is_some_and(u8::is_ascii_whitespace) {
        start += 1;
    }
    while end > start && b.get(end - 1).is_some_and(u8::is_ascii_whitespace) {
        end -= 1;
    }
    start..end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> Config {
        Config::defaults().unwrap()
    }

    fn parse(text: &str) -> Vec<Item> {
        parse_file(&cfg(), 0, text)
    }

    fn at<'a>(text: &'a str, r: &Range<usize>) -> &'a str {
        &text[r.clone()]
    }

    #[test]
    fn fr_001_heading_with_id_declares_item() {
        let t =
            "# Title\n\n## FR-104 VAT per line item\n\nText.\n\n### FR-12 Not an ID\n## Other\n";
        let items = parse(t);
        assert_eq!(items.len(), 1);
        let i = &items[0];
        assert_eq!(i.id.as_deref(), Some("FR-104"));
        assert_eq!(i.kind, "functional");
        assert_eq!(i.title, "VAT per line item");
        assert_eq!(i.level, 2);
        assert_eq!(at(t, &i.id_span), "FR-104");
        assert_eq!(at(t, &i.heading), "## FR-104 VAT per line item");
    }

    #[test]
    fn fr_001_unconfigured_prefix_and_other_first_tokens_are_no_items() {
        let t = "## XY-001 Unknown prefix\n## See FR-001\n## FR-001: colon\n## UC-001.s1 step\n";
        assert!(parse(t).is_empty());
    }

    #[test]
    fn fr_001_any_heading_level_and_setext() {
        let t = "# UC-001 One\n\n###### FR-002 Six\n\nTC-003 Setext\n------\n";
        let ids: Vec<_> = parse(t).into_iter().map(|i| i.display_id()).collect();
        assert_eq!(ids, ["UC-001", "FR-002", "TC-003"]);
    }

    #[test]
    fn fr_002_body_ends_at_same_or_higher_level_or_item_heading() {
        let t = "## FR-001 A\nbody a\n### Details\nstill a FR-005\n### FR-002 B\nbody b\n## Next\nnot b FR-006\n";
        let items = parse(t);
        assert_eq!(items.len(), 2);
        assert_eq!(
            at(t, &items[0].body),
            "body a\n### Details\nstill a FR-005\n"
        );
        assert_eq!(items[0].mentions.len(), 1);
        assert_eq!(at(t, &items[1].body), "body b\n");
        assert!(items[1].mentions.is_empty());
    }

    #[test]
    fn fr_002_body_extends_to_end_of_file() {
        let t = "## FR-001 A\ntext";
        assert_eq!(at(t, &parse(t)[0].body), "text");
    }

    #[test]
    fn fr_003_metadata_block_with_pairs() {
        let t = "## FR-104 VAT\nimpl: UC-012.s2  prio: must\nstatus:\treview\n\nWhen ...\n";
        let i = &parse(t)[0];
        let pairs: Vec<_> = i
            .meta
            .iter()
            .map(|p| (p.key.as_str(), p.value.as_str(), p.known))
            .collect();
        assert_eq!(
            pairs,
            [
                ("impl", "UC-012.s2", true),
                ("prio", "must", true),
                ("status", "review", true)
            ]
        );
        assert_eq!(at(t, &i.meta[1].value_span), "must");
        assert_eq!(i.meta_value("status"), Some("review"));
    }

    #[test]
    fn fr_003_single_space_does_not_split_pairs() {
        let t = "## BR-001 X\nsrc: Finance memo  prio: must\n";
        let i = &parse(t)[0];
        assert_eq!(i.meta[0].value, "Finance memo");
        assert_eq!(i.meta[1].value, "must");
    }

    #[test]
    fn fr_003_no_metadata_if_first_line_is_not_a_key() {
        let t = "## FR-001 X\nThe system shall do: things.\nprio: must\n";
        let i = &parse(t)[0];
        assert!(i.meta.is_empty());
    }

    #[test]
    fn fr_003_metadata_ends_at_blank_line_or_text() {
        let t = "## FR-001 X\nprio: must\nThe text.\n\nstatus: draft\n";
        let i = &parse(t)[0];
        assert_eq!(i.meta.len(), 1);
    }

    #[test]
    fn fr_102_unknown_first_key_is_recorded_without_metadata_block() {
        let t = "## FR-001 X\ncolour: red  prio: must\n";
        let i = &parse(t)[0];
        assert_eq!(i.meta.len(), 1);
        assert!(!i.meta[0].known);
        assert_eq!(i.meta[0].key, "colour");
        assert_eq!(i.meta_value("prio"), None);
    }

    #[test]
    fn fr_102_unknown_key_in_later_pair() {
        let t = "## FR-001 X\nimpl: UC-001.s1  prio: urgent  colour: red\n";
        let i = &parse(t)[0];
        let unknown: Vec<_> = i
            .meta
            .iter()
            .filter(|p| !p.known)
            .map(|p| at(t, &p.key_span))
            .collect();
        assert_eq!(unknown, ["colour"]);
    }

    #[test]
    fn fr_004_link_values_split_at_commas() {
        let t = "## FR-001 X\nimpl: UC-001.s1, UC-002 ,UC-003.s2a\n";
        let i = &parse(t)[0];
        let refs: Vec<_> = i.links.iter().map(|l| at(t, &l.reference.span)).collect();
        assert_eq!(refs, ["UC-001.s1", "UC-002", "UC-003.s2a"]);
        assert!(
            i.links
                .iter()
                .all(|l| l.key == "impl" && l.reference.target.is_some())
        );
    }

    #[test]
    fn fr_004_invalid_link_value_has_no_target() {
        let t = "## FR-001 X\nimpl: UC-12, foo, FR-?\n";
        let i = &parse(t)[0];
        assert_eq!(i.links.len(), 3);
        assert!(i.links.iter().all(|l| l.reference.target.is_none()));
    }

    #[test]
    fn fr_005_body_fields_with_following_list() {
        let t = "## US-001 S\nprio: must\n\nAs a user ...\n\nac:\n- one\n- two\n\nAfter.\n";
        let i = &parse(t)[0];
        assert_eq!(i.fields.len(), 1);
        assert_eq!(i.fields[0].key, "ac");
        assert_eq!(at(t, &i.fields[0].value_span), "- one\n- two");
    }

    #[test]
    fn fr_005_body_field_rest_of_line() {
        let t = "## UC-001 U\n\npre: Customer is logged in.\n\n- s1: Go.\n\npost: Done.\n";
        let i = &parse(t)[0];
        let f: Vec<_> = i
            .fields
            .iter()
            .map(|f| (f.key.as_str(), at(t, &f.value_span)))
            .collect();
        assert_eq!(f, [("pre", "Customer is logged in."), ("post", "Done.")]);
    }

    #[test]
    fn fr_005_body_field_directly_after_heading_is_not_metadata() {
        let t = "## UC-001 U\npre: Logged in.\n";
        let i = &parse(t)[0];
        assert!(i.meta.is_empty());
        assert_eq!(i.fields.len(), 1);
    }

    #[test]
    fn fr_005_body_fields_only_for_their_type() {
        let t = "## FR-001 X\n\nac: not a field here\n";
        assert!(parse(t)[0].fields.is_empty());
    }

    #[test]
    fn fr_006_steps_in_use_cases() {
        let t = "## UC-012 Order\n\n- s1: Open.\n- s2: Show.\n- s3: Confirm.\n- s2a: Empty → hint.\n- s2a1: Deep.\n- no step\n1. s4: Numbered.\n";
        let i = &parse(t)[0];
        let labels: Vec<_> = i.steps.iter().map(|s| s.label.as_str()).collect();
        assert_eq!(labels, ["s1", "s2", "s3", "s2a", "s2a1", "s4"]);
        assert_eq!(at(t, &i.steps[3].text_span), "Empty → hint.");
    }

    #[test]
    fn fr_006_steps_only_in_use_cases() {
        let t = "## FR-001 X\n\n- s1: Not a step.\n";
        assert!(parse(t)[0].steps.is_empty());
    }

    #[test]
    fn br_003_step_label_independent_of_position() {
        let t = "## UC-001 U\n\n- s3: First in list.\n- s1: Second.\n";
        let i = &parse(t)[0];
        assert_eq!(i.steps[0].label, "s3");
        assert_eq!(i.steps[1].label, "s1");
    }

    #[test]
    fn fr_007_step_reference() {
        let c = cfg();
        assert_eq!(
            parse_target(&c, "UC-012.s2"),
            Some(Target {
                id: "UC-012".into(),
                step: Some("s2".into())
            })
        );
        assert_eq!(
            parse_target(&c, "UC-012"),
            Some(Target {
                id: "UC-012".into(),
                step: None
            })
        );
        assert_eq!(parse_target(&c, "UC-012.x"), None);
        assert_eq!(parse_target(&c, "UC-012.s"), None);
    }

    #[test]
    fn fr_008_mentions_in_body_fields_and_steps() {
        let t = "## UC-001 U\n\npre: see BR-003.\n\n- s1: As in FR-104, then UC-002.s2a.\n\nAlso NFR-007 and (TC-220).\n";
        let i = &parse(t)[0];
        let m: Vec<_> = i.mentions.iter().map(|r| r.text.as_str()).collect();
        assert_eq!(m, ["BR-003", "FR-104", "UC-002.s2a", "NFR-007", "TC-220"]);
    }

    #[test]
    fn fr_008_only_whole_words() {
        let t = "## FR-001 X\n\nXFR-002 FR-003x FR-004_ a-FR-005 FR-12 FR-0067 äFR-008 FR-009.\n";
        let m: Vec<_> = parse(t)[0]
            .mentions
            .iter()
            .map(|r| r.text.clone())
            .collect();
        assert_eq!(m, ["FR-0067", "FR-009"]);
    }

    #[test]
    fn fr_008_metadata_is_not_mentioned() {
        let t = "## FR-001 X\nimpl: UC-001\n\nText.\n";
        assert!(parse(t)[0].mentions.is_empty());
    }

    #[test]
    fn fr_009_code_is_ignored() {
        let t = "## FR-001 X\n\nInline `FR-888` code.\n\n```markdown\n## FR-002 In fence\nimpl: UC-404.s1\nFR-777\n```\n\n    ## FR-003 indented FR-666\n\n## `FR-004` code heading\n";
        let items = parse(t);
        assert_eq!(items.len(), 1);
        assert!(items[0].mentions.is_empty());
    }

    #[test]
    fn fr_009_indented_code_after_heading_is_no_metadata() {
        let t = "## FR-001 X\n    impl: UC-001\n";
        let i = &parse(t)[0];
        assert!(i.meta.is_empty() && i.links.is_empty());
    }

    #[test]
    fn fr_010_placeholder_id() {
        let t = "## FR-? New requirement\n\nText FR-?.\n## XY-? Not configured\n";
        let items = parse(t);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, None);
        assert_eq!(items[0].display_id(), "FR-?");
        assert_eq!(items[0].title, "New requirement");
        assert!(items[0].mentions.is_empty());
    }

    #[test]
    fn fr_011_glossary_terms_with_aliases() {
        let t = "# Glossary\n\n## Cart\nalias: basket, shopping cart\n\nItems a customer has selected.\n\n## Delivery country\n\nThe country.\n";
        let terms = parse_glossary(t);
        let names: Vec<&str> = terms.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(names, ["Glossary", "Cart", "Delivery country"]);
        assert_eq!(terms[1].aliases, ["basket", "shopping cart"]);
        assert_eq!(
            &t[terms[1].definition.clone()],
            "Items a customer has selected."
        );
        assert!(terms[2].aliases.is_empty());
        assert_eq!(&t[terms[2].definition.clone()], "The country.");
    }

    #[test]
    fn br_001_id_needs_configured_digits() {
        let c = cfg();
        assert!(parse_target(&c, "FR-007").is_some());
        assert!(parse_target(&c, "FR-1234").is_some());
        assert!(parse_target(&c, "FR-07").is_none());
        assert!(parse_target(&c, "FR-").is_none());
        assert!(parse_target(&c, "FR-00a").is_none());
        let c4 = Config::from_toml("[ids]\ndigits = 4\n", None).unwrap();
        assert!(parse_target(&c4, "FR-007").is_none());
    }

    #[test]
    fn fr_012_crlf_line_endings() {
        let t = "## FR-001 X\r\nimpl: UC-001.s1  prio: must\r\n\r\nText FR-002.\r\n";
        let i = &parse(t)[0];
        assert_eq!(i.meta[1].value, "must");
        assert_eq!(i.links[0].reference.text, "UC-001.s1");
        assert_eq!(i.title, "X");
        assert_eq!(i.mentions[0].text, "FR-002");
    }

    #[test]
    fn fr_012_survives_arbitrary_input() {
        // Every prefix of every fixture plus pseudo-random mutations: the
        // parser must not panic (NFR-009 covers the full fuzz run).
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../examples");
        let mut texts = Vec::new();
        for dir in ["shop", "broken"] {
            for e in std::fs::read_dir(format!("{root}/{dir}")).unwrap() {
                let p = e.unwrap().path();
                if p.extension().is_some_and(|x| x == "md") {
                    texts.push(std::fs::read_to_string(p).unwrap());
                }
            }
        }
        let c = cfg();
        let mut seed: u64 = 0x2545_f491_4f6c_dd1d;
        let mut rand = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let alphabet: Vec<char> = "#-: \n\t`*[]()|.,?sSFRUC0123456789äé→\r>".chars().collect();
        for t in &texts {
            for (i, _) in t.char_indices() {
                parse_file(&c, 0, &t[..i]);
            }
            for _ in 0..300 {
                let mut chars: Vec<char> = t.chars().collect();
                for _ in 0..1 + rand() % 8 {
                    let pos = (rand() as usize) % (chars.len() + 1);
                    let ch = alphabet[(rand() as usize) % alphabet.len()];
                    match rand() % 3 {
                        0 => chars.insert(pos, ch),
                        1 if pos < chars.len() => {
                            chars.remove(pos);
                        }
                        _ if pos < chars.len() => chars[pos] = ch,
                        _ => {}
                    }
                }
                let s: String = chars.into_iter().collect();
                parse_file(&c, 0, &s);
            }
        }
    }
}
