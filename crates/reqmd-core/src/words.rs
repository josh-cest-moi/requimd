//! Word and phrase matching for the language checks (FR-110 rules): whole
//! words, case-insensitive, the words of a phrase separated by any
//! whitespace, no wildcards or stemming. Also finds the text to ignore
//! (quotes, HTML comments) and inline allow markers (FR-119).

use std::ops::Range;

/// Characters that belong to a word; `user-friendly` is one word.
pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-'
}

/// Non-overlapping occurrences of `phrase` in `text[range]`.
pub fn find_phrase(text: &str, range: Range<usize>, phrase: &str) -> Vec<Range<usize>> {
    Phrases::new([phrase])
        .find(text, range)
        .into_iter()
        .map(|(r, _)| r)
        .collect()
}

/// A set of words and phrases matched in one pass over a text.
pub struct Phrases {
    entries: Vec<Entry>,
}

struct Entry {
    words: Vec<String>,
    /// First character, lower-case.
    first: char,
    need_start: bool,
    need_end: bool,
}

impl Phrases {
    pub fn new<'a>(phrases: impl IntoIterator<Item = &'a str>) -> Phrases {
        let entries = phrases
            .into_iter()
            .filter_map(|p| {
                let words: Vec<String> = p.split_whitespace().map(str::to_string).collect();
                let first = words.first()?.chars().next()?;
                let last = words.last()?.chars().next_back()?;
                Some(Entry {
                    first: lower(first),
                    need_start: is_word_char(first),
                    need_end: is_word_char(last),
                    words,
                })
            })
            .collect();
        Phrases { entries }
    }

    /// Occurrences in `text[range]` with the index of the phrase; each
    /// phrase's occurrences do not overlap.
    pub fn find(&self, text: &str, range: Range<usize>) -> Vec<(Range<usize>, usize)> {
        let mut out = Vec::new();
        if self.entries.is_empty() {
            return out;
        }
        let end = range.end.min(text.len());
        let Some(region) = text.get(range.start..end) else {
            return out;
        };
        let mut skip_until = vec![range.start; self.entries.len()];
        let mut prev_is_word = text
            .get(..range.start)
            .and_then(|s| s.chars().next_back())
            .is_some_and(is_word_char);
        for (rel, c) in region.char_indices() {
            let i = range.start + rel;
            let lc = lower(c);
            for (k, e) in self.entries.iter().enumerate() {
                if e.first != lc || i < skip_until[k] || (e.need_start && prev_is_word) {
                    continue;
                }
                let Some(m_end) = match_words(text, i, end, &e.words) else {
                    continue;
                };
                if e.need_end
                    && text
                        .get(m_end..end)
                        .and_then(|s| s.chars().next())
                        .is_some_and(is_word_char)
                {
                    continue;
                }
                out.push((i..m_end, k));
                skip_until[k] = m_end;
            }
            prev_is_word = is_word_char(c);
        }
        out
    }
}

/// Lower-case of a character; ASCII without a table lookup.
fn lower(c: char) -> char {
    if c.is_ascii() {
        c.to_ascii_lowercase()
    } else {
        c.to_lowercase().next().unwrap_or(c)
    }
}

fn same_char(a: char, b: char) -> bool {
    a == b || lower(a) == lower(b)
}

/// Matches `words` at `at`, separated by whitespace; returns the end.
fn match_words(text: &str, at: usize, end: usize, words: &[String]) -> Option<usize> {
    let mut p = at;
    for (k, w) in words.iter().enumerate() {
        if k > 0 {
            let rest = text.get(p..end)?;
            let ws = rest.len() - rest.trim_start().len();
            if ws == 0 {
                return None;
            }
            p += ws;
        }
        let mut chars = text.get(p..end)?.char_indices();
        for wc in w.chars() {
            let (_, tc) = chars.next()?;
            if !same_char(tc, wc) {
                return None;
            }
            p += tc.len_utf8();
        }
    }
    Some(p)
}

/// Quoted text (straight or typographic quotes, within a paragraph) and
/// HTML comments in `text[range]`, which the language checks ignore.
pub fn ignored_ranges(text: &str, range: Range<usize>, code: &[Range<usize>]) -> Vec<Range<usize>> {
    let mut out = Vec::new();
    let end = range.end.min(text.len());
    let Some(region) = text.get(range.start..end) else {
        return out;
    };
    // HTML comments.
    let mut from = 0;
    while let Some(s) = region.get(from..).and_then(|r| r.find("<!--")) {
        let start = from + s;
        let close = region
            .get(start + 4..)
            .and_then(|r| r.find("-->"))
            .map_or(region.len(), |e| start + 4 + e + 3);
        out.push(range.start + start..range.start + close);
        from = close;
    }
    // Quotes; an unclosed quote at a paragraph end quotes nothing.
    let mut open: Option<(usize, char)> = None;
    let mut newline_seen = false;
    for (rel, c) in region.char_indices() {
        let i = range.start + rel;
        if c == '\n' {
            if newline_seen {
                open = None;
            }
            newline_seen = true;
            continue;
        }
        if !c.is_whitespace() {
            newline_seen = false;
        }
        if code.iter().any(|r| r.start <= i && i < r.end) {
            continue;
        }
        match (open, c) {
            (None, '"' | '\u{201C}' | '\u{201E}') => open = Some((i, c)),
            (Some((s, '"')), '"')
            | (Some((s, '\u{201C}')), '\u{201D}')
            | (Some((s, '\u{201E}')), '\u{201C}' | '\u{201D}') => {
                out.push(s..i + c.len_utf8());
                open = None;
            }
            _ => {}
        }
    }
    out.sort_by_key(|r| r.start);
    out
}

/// Whether `r` overlaps any of `ranges`.
pub fn overlaps(ranges: &[Range<usize>], r: &Range<usize>) -> bool {
    ranges.iter().any(|o| o.start < r.end && r.start < o.end)
}

/// An inline marker `<!-- reqmd: allow a, b -->` at a line end (FR-119).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Marker {
    /// The line the marker applies to.
    pub line: Range<usize>,
    /// The whole comment.
    pub span: Range<usize>,
    /// Allowed words or phrases with their positions.
    pub entries: Vec<(String, Range<usize>)>,
}

/// Markers on the lines of `text[range]`.
pub fn markers(text: &str, range: Range<usize>) -> Vec<Marker> {
    let mut out = Vec::new();
    let end = range.end.min(text.len());
    let mut line_start = range.start;
    while line_start < end {
        let line_end = text
            .get(line_start..end)
            .and_then(|s| s.find('\n'))
            .map_or(end, |i| line_start + i);
        if let Some(m) = marker_on_line(text, line_start..line_end) {
            out.push(m);
        }
        line_start = line_end + 1;
    }
    out
}

fn marker_on_line(text: &str, line: Range<usize>) -> Option<Marker> {
    let s = text.get(line.clone())?;
    let trimmed = s.trim_end();
    let body = trimmed.strip_suffix("-->")?;
    let open = body.rfind("<!--")?;
    let inner = body.get(open + 4..)?;
    let after_reqmd = inner.trim_start().strip_prefix("reqmd:")?;
    let list = after_reqmd.trim_start().strip_prefix("allow")?;
    if !list.starts_with(char::is_whitespace) {
        return None;
    }
    let list_start = line.start + (trimmed.len() - 3 - list.len());
    let mut entries = Vec::new();
    let mut pos = list_start;
    for part in list.split(',') {
        let lead = part.len() - part.trim_start().len();
        let word = part.trim();
        if !word.is_empty() {
            let start = pos + lead;
            entries.push((word.to_string(), start..start + word.len()));
        }
        pos += part.len() + 1;
    }
    Some(Marker {
        line: line.start..line.start + trimmed.len(),
        span: line.start + open..line.start + trimmed.len(),
        entries,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn found<'a>(text: &'a str, phrase: &str) -> Vec<&'a str> {
        find_phrase(text, 0..text.len(), phrase)
            .into_iter()
            .map(|r| &text[r])
            .collect()
    }

    #[test]
    fn fr_110_whole_words_case_insensitive() {
        assert_eq!(
            found("Fast and fast, breakfast fastest", "fast"),
            ["Fast", "fast"]
        );
        assert_eq!(
            found("a user-friendly UI, friendly", "user-friendly"),
            ["user-friendly"]
        );
        assert_eq!(found("not user-friendly", "friendly"), Vec::<&str>::new());
        assert_eq!(found("Größe und GRÖSSE", "größe"), ["Größe"]);
    }

    #[test]
    fn fr_110_phrases_match_across_whitespace() {
        assert_eq!(
            found("as\n  needed and As Needed", "as needed"),
            ["as\n  needed", "As Needed"]
        );
        assert_eq!(found("asneeded", "as needed"), Vec::<&str>::new());
    }

    #[test]
    fn fr_110_punctuation_in_entries() {
        assert_eq!(found("cats, dogs etc. and more etc", "etc."), ["etc."]);
        assert_eq!(found("A and/or B", "and/or"), ["and/or"]);
    }

    #[test]
    fn fr_110_quotes_and_comments_are_ignored() {
        let t = "Click \"Quick order\" or “Quick buy” or „Quick“ <!-- quick -->\n\nAn \"open quote\n\nquick";
        let ign = ignored_ranges(t, 0..t.len(), &[]);
        let hits: Vec<&str> = find_phrase(t, 0..t.len(), "quick")
            .into_iter()
            .filter(|r| !overlaps(&ign, r))
            .map(|r| &t[r])
            .collect();
        assert_eq!(hits, ["quick"]);
        assert_eq!(&t[ign[0].clone()], "\"Quick order\"");
    }

    #[test]
    fn fr_119_inline_markers() {
        let t = "The Fast Track page. <!-- reqmd: allow fast, as needed -->\nNo marker <!-- other -->\n";
        let m = markers(t, 0..t.len());
        assert_eq!(m.len(), 1);
        let entries: Vec<&str> = m[0]
            .entries
            .iter()
            .map(|(w, r)| {
                assert_eq!(&t[r.clone()], w);
                w.as_str()
            })
            .collect();
        assert_eq!(entries, ["fast", "as needed"]);
        assert_eq!(
            &t[m[0].line.clone()],
            "The Fast Track page. <!-- reqmd: allow fast, as needed -->"
        );
        assert!(markers("x <!-- reqmd: allowfast -->", 0..27).is_empty());
    }
}
