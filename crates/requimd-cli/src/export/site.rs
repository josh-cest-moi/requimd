//! FR-400: static site. One page per source file with anchors per item and
//! step, clickable references and backlinks, an index of all items and the
//! traceability matrix (FR-401). Plain HTML and CSS, no JavaScript, relative
//! links only, so it works from the file system.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::ops::Range;
use std::path::Path;

use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd, html};
use requimd_core::{FileId, Item, Project, Reference, Resolution, Target};

use super::matrix;

/// Embedded in every page, so a single saved page keeps its look.
const STYLE: &str = include_str!("style.css");

/// Which items the site contains (`--status`).
pub struct Filter(pub Option<Vec<String>>);

impl Filter {
    fn includes(&self, p: &Project, item: &Item) -> bool {
        match &self.0 {
            None => true,
            Some(list) => status(p, item).is_some_and(|s| list.iter().any(|l| l == s)),
        }
    }
}

fn status<'a>(p: &'a Project, item: &'a Item) -> Option<&'a str> {
    item.meta_value("status")
        .or_else(|| p.config.defaults.get("status").map(String::as_str))
}

pub fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
    out
}

/// Site path of a source file's page, e.g. `requirements/cli.html`. The
/// generated pages `index.html` and `matrix.html` keep their names.
fn page_path(rel: &str) -> String {
    let base = rel.strip_suffix(".md").unwrap_or(rel);
    let page = format!("{base}.html");
    if page == "index.html" || page == "matrix.html" {
        format!("{base}-page.html")
    } else {
        page
    }
}

/// `../` once per directory level of `page`.
fn prefix(page: &str) -> String {
    "../".repeat(page.matches('/').count())
}

struct Site<'a> {
    p: &'a Project,
    filter: Filter,
    pages: Vec<String>,
}

impl Site<'_> {
    fn included(&self, i: usize) -> bool {
        self.p
            .items
            .get(i)
            .is_some_and(|it| self.filter.includes(self.p, it))
    }

    /// Link from `from_page` to the target of a reference, if the target is
    /// on the site.
    fn href(&self, from_page: &str, target: &Target) -> Option<String> {
        let (i, anchor) = match self.p.resolve(target) {
            Resolution::Item(i) => (i, target.id.clone()),
            Resolution::Step(i, s) => (
                i,
                format!("{}.{}", target.id, self.p.items.get(i)?.steps.get(s)?.label),
            ),
            _ => return None,
        };
        self.included(i).then_some(())?;
        let page = self.pages.get(self.p.items.get(i)?.file)?;
        let target_page = if page == from_page {
            String::new()
        } else {
            format!("{}{page}", prefix(from_page))
        };
        Some(format!("{target_page}#{}", escape(&anchor)))
    }

    fn item_href(&self, from_page: &str, i: usize) -> Option<String> {
        let id = self.p.items.get(i)?.id.clone()?;
        self.href(from_page, &Target { id, step: None })
    }

    fn reference_html(&self, from_page: &str, r: &Reference) -> String {
        match r.target.as_ref().and_then(|t| self.href(from_page, t)) {
            Some(href) => format!("<a href=\"{href}\">{}</a>", escape(&r.text)),
            None => format!("<span class=\"unresolved\">{}</span>", escape(&r.text)),
        }
    }

    fn badge(&self, item: &Item) -> String {
        match status(self.p, item) {
            Some(s) => format!(" <span class=\"status status-{0}\">{0}</span>", escape(s)),
            None => String::new(),
        }
    }

    /// The metadata block as a definition list; links are clickable and the
    /// status is shown as badge instead.
    fn meta_html(&self, from_page: &str, item: &Item) -> String {
        let mut out = String::new();
        let mut keys: Vec<&str> = Vec::new();
        for m in &item.meta {
            if m.key == "status" || keys.contains(&m.key.as_str()) {
                continue;
            }
            keys.push(&m.key);
            let value = if self.p.config.link(&m.key).is_some() {
                item.links
                    .iter()
                    .filter(|l| l.key == m.key)
                    .map(|l| self.reference_html(from_page, &l.reference))
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                escape(&m.value)
            };
            let _ = write!(out, "<dt>{}</dt><dd>{value}</dd>", escape(&m.key));
        }
        if out.is_empty() {
            out
        } else {
            format!("<dl class=\"meta\">{out}</dl>\n")
        }
    }

    /// References from other included items to `item` or its steps.
    fn backlinks_html(&self, from_page: &str, idx: usize) -> String {
        let mut rows = Vec::new();
        for (j, other) in self.p.items.iter().enumerate() {
            if j == idx || !self.included(j) {
                continue;
            }
            let refs = other
                .links
                .iter()
                .map(|l| (l.key.as_str(), &l.reference))
                .chain(other.mentions.iter().map(|m| ("mention", m)));
            for (kind, r) in refs {
                let Some(t) = &r.target else { continue };
                let hit = matches!(self.p.resolve(t), Resolution::Item(i) | Resolution::Step(i, _) if i == idx);
                if !hit {
                    continue;
                }
                let source = match self.item_href(from_page, j) {
                    Some(h) => format!("<a href=\"{h}\">{}</a>", escape(&other.display_id())),
                    None => escape(&other.display_id()),
                };
                rows.push(format!(
                    "<li>{source} {} <span class=\"kind\">{}</span> {}</li>",
                    escape(&other.title),
                    escape(kind),
                    escape(&r.text)
                ));
            }
        }
        if rows.is_empty() {
            String::new()
        } else {
            format!(
                "<section class=\"backlinks\"><h4>Referenced by</h4><ul>{}</ul></section>\n",
                rows.join("")
            )
        }
    }

    /// The HTML of one source file.
    fn file_html(&self, file: FileId) -> String {
        let p = self.p;
        let Some(f) = p.files.get(file) else {
            return String::new();
        };
        let text = f.text.as_str();
        let page = self.pages.get(file).cloned().unwrap_or_default();
        let items: Vec<usize> = (0..p.items.len())
            .filter(|&i| p.items.get(i).is_some_and(|it| it.file == file))
            .collect();
        let excluded: Vec<Range<usize>> = items
            .iter()
            .filter(|&&i| !self.included(i))
            .filter_map(|&i| p.items.get(i).map(|it| it.heading.start..it.body.end))
            .collect();
        let headings: HashMap<usize, usize> = items
            .iter()
            .filter_map(|&i| p.items.get(i).map(|it| (it.heading.start, i)))
            .collect();
        // Spans in text that become links: mentions and step labels.
        let mut spans: Vec<(Range<usize>, String)> = Vec::new();
        for &i in &items {
            let Some(it) = p.items.get(i) else { continue };
            for m in &it.mentions {
                spans.push((m.span.clone(), self.reference_html(&page, m)));
            }
            for s in &it.steps {
                let anchor = format!("{}.{}", it.display_id(), s.label);
                spans.push((
                    s.label_span.clone(),
                    format!(
                        "<a class=\"step\" href=\"#{0}\">{1}</a>",
                        escape(&anchor),
                        escape(&s.label)
                    ),
                ));
            }
        }

        let mut out: Vec<Event> = Vec::new();
        let mut skip_depth = 0usize;
        let mut open: Option<usize> = None;
        let mut in_item_heading = false;
        for (event, range) in Parser::new_ext(text, Options::empty()).into_offset_iter() {
            // Backlinks after the body of the open item.
            if let Some(i) = open
                && p.items.get(i).is_some_and(|it| range.start >= it.body.end)
            {
                out.push(Event::Html(CowStr::from(self.backlinks_html(&page, i))));
                open = None;
            }
            if excluded.iter().any(|r| r.contains(&range.start)) {
                continue;
            }
            if in_item_heading {
                if matches!(event, Event::End(TagEnd::Heading(_))) {
                    in_item_heading = false;
                }
                continue;
            }
            if skip_depth > 0 {
                match event {
                    Event::Start(_) => skip_depth += 1,
                    Event::End(_) => skip_depth -= 1,
                    _ => {}
                }
                continue;
            }
            let meta = open
                .and_then(|i| p.items.get(i))
                .map(|it| it.meta_span.clone());
            let in_meta = meta
                .as_ref()
                .is_some_and(|m| m.start <= range.start && range.start < m.end);
            match event {
                Event::Start(Tag::Heading { level, .. }) if headings.contains_key(&range.start) => {
                    let Some(&i) = headings.get(&range.start) else {
                        continue;
                    };
                    let Some(it) = p.items.get(i) else { continue };
                    let n = level as usize;
                    let id_attr = it
                        .id
                        .as_ref()
                        .map(|id| format!(" id=\"{}\"", escape(id)))
                        .unwrap_or_default();
                    let anchor = it.id.as_ref().map_or_else(
                        || escape(&it.display_id()),
                        |id| format!("<a class=\"id\" href=\"#{0}\">{0}</a>", escape(id)),
                    );
                    out.push(Event::Html(CowStr::from(format!(
                        "<h{n}{id_attr} class=\"item\">{anchor} {}{}</h{n}>\n{}",
                        escape(&it.title),
                        self.badge(it),
                        self.meta_html(&page, it)
                    ))));
                    in_item_heading = true;
                    open = Some(i);
                }
                // The metadata block is shown as a list after the heading.
                Event::Start(Tag::Paragraph)
                    if in_meta && meta.as_ref().is_some_and(|m| range.end <= m.end) =>
                {
                    skip_depth = 1;
                }
                _ if in_meta && !matches!(event, Event::Start(_) | Event::End(_)) => {}
                Event::Start(Tag::Item) => {
                    let step = items.iter().filter_map(|&i| p.items.get(i)).find_map(|it| {
                        it.steps
                            .iter()
                            .find(|s| {
                                s.label_span.start > range.start
                                    && s.label_span.start - range.start <= 8
                            })
                            .map(|s| format!("{}.{}", it.display_id(), s.label))
                    });
                    match step {
                        Some(anchor) => out.push(Event::Html(CowStr::from(format!(
                            "<li id=\"{}\">",
                            escape(&anchor)
                        )))),
                        None => out.push(Event::Start(Tag::Item)),
                    }
                }
                Event::Text(t) => {
                    let exact = text.get(range.clone()) == Some(t.as_ref());
                    let inside: Vec<&(Range<usize>, String)> = spans
                        .iter()
                        .filter(|(s, _)| range.start <= s.start && s.end <= range.end)
                        .collect();
                    if !exact || inside.is_empty() {
                        out.push(Event::Text(t));
                        continue;
                    }
                    let mut sorted = inside;
                    sorted.sort_by_key(|(s, _)| s.start);
                    let mut html = String::new();
                    let mut pos = range.start;
                    for (s, link) in sorted {
                        if s.start < pos {
                            continue;
                        }
                        html.push_str(&escape(text.get(pos..s.start).unwrap_or_default()));
                        html.push_str(link);
                        pos = s.end;
                    }
                    html.push_str(&escape(text.get(pos..range.end).unwrap_or_default()));
                    out.push(Event::Html(CowStr::from(html)));
                }
                // Raw HTML is shown as text; comments such as allow markers
                // (FR-119) are left out.
                Event::Html(h) | Event::InlineHtml(h) => {
                    if !h.trim_start().starts_with("<!--") {
                        out.push(Event::Text(h));
                    }
                }
                e => out.push(e),
            }
        }
        if let Some(i) = open {
            out.push(Event::Html(CowStr::from(self.backlinks_html(&page, i))));
        }
        let mut s = String::new();
        html::push_html(&mut s, out.into_iter());
        s
    }

    fn page(&self, page: &str, title: &str, body: &str) -> String {
        let pre = prefix(page);
        format!(
            "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
             <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
             <title>{}</title>\n<style>\n{STYLE}</style>\n</head>\n<body>\n\
             <header><a href=\"{pre}index.html\">Index</a> · <a href=\"{pre}matrix.html\">Traceability matrix</a></header>\n\
             <main>\n{body}</main>\n<footer>Generated by requimd</footer>\n</body>\n</html>\n",
            escape(title)
        )
    }

    fn index_html(&self) -> String {
        let p = self.p;
        let mut order: Vec<usize> = (0..p.items.len()).filter(|&i| self.included(i)).collect();
        let number = |i: usize| -> (String, u64, String) {
            let it = p.items.get(i);
            let prefix = it.map(|it| it.prefix.clone()).unwrap_or_default();
            let n = it
                .and_then(|it| it.id.as_ref())
                .and_then(|id| id.rsplit('-').next()?.parse().ok())
                .unwrap_or(u64::MAX);
            (prefix, n, it.map(|it| it.title.clone()).unwrap_or_default())
        };
        order.sort_by_key(|&i| number(i));
        let mut rows = String::new();
        for i in order {
            let Some(it) = p.items.get(i) else { continue };
            let id = match self.item_href("index.html", i) {
                Some(h) => format!("<a href=\"{h}\">{}</a>", escape(&it.display_id())),
                None => escape(&it.display_id()),
            };
            let page = self.pages.get(it.file).cloned().unwrap_or_default();
            let file = p
                .files
                .get(it.file)
                .map(|f| f.rel.clone())
                .unwrap_or_default();
            let _ = writeln!(
                rows,
                "<tr><td>{id}</td><td>{}</td><td>{}</td><td>{}</td><td><a href=\"{}\">{}</a></td></tr>",
                escape(&it.title),
                escape(&it.kind),
                self.badge(it),
                escape(&page),
                escape(&file)
            );
        }
        let mut files = String::new();
        for (i, f) in p.files.iter().enumerate() {
            let page = self.pages.get(i).cloned().unwrap_or_default();
            let _ = write!(
                files,
                "<li><a href=\"{}\">{}</a></li>",
                escape(&page),
                escape(&f.rel)
            );
        }
        format!(
            "<h1>Requirements</h1>\n<h2>Files</h2>\n<ul>{files}</ul>\n<h2>Items</h2>\n\
             <table>\n<thead><tr><th>ID</th><th>Title</th><th>Type</th><th>Status</th><th>File</th></tr></thead>\n\
             <tbody>\n{rows}</tbody>\n</table>\n"
        )
    }

    fn matrix_html(&self) -> String {
        let p = self.p;
        let cell = |items: &[usize]| -> String {
            items
                .iter()
                .filter(|&&i| self.included(i))
                .filter_map(|&i| {
                    let it = p.items.get(i)?;
                    Some(match self.item_href("matrix.html", i) {
                        Some(h) => format!("<a href=\"{h}\">{}</a>", escape(&it.display_id())),
                        None => escape(&it.display_id()),
                    })
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        let mut rows = String::new();
        let (mut total, mut implemented, mut verified) = (0, 0, 0);
        for r in matrix::rows(p) {
            if !self.included(r.use_case) {
                continue;
            }
            total += 1;
            let imp = cell(&r.implemented_by);
            let ver = cell(&r.verified_by);
            implemented += usize::from(!imp.is_empty());
            verified += usize::from(!ver.is_empty());
            let step = match p.items.get(r.use_case).and_then(|it| it.id.clone()) {
                Some(id) => {
                    let t = Target {
                        id,
                        step: Some(r.label.clone()),
                    };
                    match self.href("matrix.html", &t) {
                        Some(h) => format!("<a href=\"{h}\">{}</a>", escape(&r.label)),
                        None => escape(&r.label),
                    }
                }
                None => escape(&r.label),
            };
            let gap = if imp.is_empty() { " class=\"gap\"" } else { "" };
            let _ = writeln!(
                rows,
                "<tr{gap}><td>{}</td><td>{step}</td><td>{}</td><td>{imp}</td><td>{ver}</td></tr>",
                escape(&r.use_case_id),
                escape(&r.text)
            );
        }
        format!(
            "<h1>Traceability matrix</h1>\n<p>{total} steps, {implemented} implemented, {verified} verified.</p>\n\
             <table>\n<thead><tr><th>Use case</th><th>Step</th><th>Text</th><th>Implemented by</th><th>Verified by</th></tr></thead>\n\
             <tbody>\n{rows}</tbody>\n</table>\n"
        )
    }
}

/// Writes the site into `dir` and returns the number of pages written.
pub fn write_site(p: &Project, filter: Filter, dir: &Path) -> Result<usize, String> {
    let pages: Vec<String> = p.files.iter().map(|f| page_path(&f.rel)).collect();
    let site = Site { p, filter, pages };
    let write = |rel: &str, content: &str| -> Result<(), String> {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        std::fs::write(&path, content).map_err(|e| format!("{}: {e}", path.display()))
    };
    write(
        "index.html",
        &site.page("index.html", "Requirements", &site.index_html()),
    )?;
    write(
        "matrix.html",
        &site.page("matrix.html", "Traceability matrix", &site.matrix_html()),
    )?;
    for (i, f) in p.files.iter().enumerate() {
        let page = site.pages.get(i).cloned().unwrap_or_default();
        let body = site.file_html(i);
        write(&page, &site.page(&page, &f.rel, &body))?;
    }
    Ok(p.files.len() + 2)
}
