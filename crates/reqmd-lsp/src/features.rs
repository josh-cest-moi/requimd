//! Language features as pure functions on a project (FR-203 to FR-212).
//! Offsets are byte offsets into the file's text; conversion to LSP
//! positions happens through [`Ctx`].

use std::collections::HashMap;
use std::ops::Range;

use lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeLens, Command, CompletionItem,
    CompletionItemKind, CompletionList, CompletionTextEdit, DocumentSymbol, Hover, HoverContents,
    InlayHint, InlayHintLabel, Location, MarkupContent, MarkupKind, OneOf, SymbolKind, TextEdit,
    Uri, WorkspaceEdit, WorkspaceSymbol,
};
use reqmd_core::config::{IMPL, STEP, USECASE};
use reqmd_core::parse::{code_ranges, key_colon, parse_target};
use reqmd_core::words::find_phrase;
use reqmd_core::{FileId, Item, Project, Reference, Resolution, Target};

use crate::convert::{Encoding, LineIndex, path_to_uri};

/// A project plus what is needed to answer in LSP terms.
pub struct Ctx<'a> {
    pub project: &'a Project,
    pub encoding: Encoding,
    /// URIs as the client sent them, for open documents.
    pub uris: &'a HashMap<FileId, Uri>,
}

impl Ctx<'_> {
    pub fn uri(&self, file: FileId) -> Option<Uri> {
        if let Some(u) = self.uris.get(&file) {
            return Some(u.clone());
        }
        path_to_uri(&self.project.files.get(file)?.path)
    }

    pub fn index(&self, file: FileId) -> Option<LineIndex<'_>> {
        let f = self.project.files.get(file)?;
        Some(LineIndex::new(&f.text, self.encoding))
    }

    pub fn location(&self, file: FileId, span: &Range<usize>) -> Option<Location> {
        Some(Location {
            uri: self.uri(file)?,
            range: self.index(file)?.range(span),
        })
    }

    fn text(&self, file: FileId) -> &str {
        self.project.files.get(file).map_or("", |f| f.text.as_str())
    }
}

/// An item or a step of a use case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sym {
    Item(usize),
    Step(usize, usize),
}

impl Sym {
    fn item(self) -> usize {
        match self {
            Sym::Item(i) | Sym::Step(i, _) => i,
        }
    }
}

/// The item whose heading or body contains `off`.
fn item_at(p: &Project, file: FileId, off: usize) -> Option<usize> {
    p.items
        .iter()
        .position(|it| it.file == file && it.heading.start <= off && off <= it.body.end)
}

/// The link or mention at `off`; the end of the token counts as on it.
pub fn reference_at(p: &Project, file: FileId, off: usize) -> Option<&Reference> {
    let item = p.items.get(item_at(p, file, off)?)?;
    item.links
        .iter()
        .map(|l| &l.reference)
        .chain(&item.mentions)
        .find(|r| r.span.start <= off && off <= r.span.end)
}

fn resolve(p: &Project, t: &Target) -> Option<Sym> {
    match p.resolve(t) {
        Resolution::Item(i) => Some(Sym::Item(i)),
        Resolution::Step(i, s) => Some(Sym::Step(i, s)),
        _ => None,
    }
}

/// The item or step at `off`: a reference to it, its heading or its step.
pub fn symbol_at(p: &Project, file: FileId, off: usize) -> Option<Sym> {
    if let Some(r) = reference_at(p, file, off) {
        return resolve(p, r.target.as_ref()?);
    }
    let i = item_at(p, file, off)?;
    let item = p.items.get(i)?;
    if off <= item.heading.end {
        return Some(Sym::Item(i));
    }
    item.steps
        .iter()
        .position(|s| s.label_span.start <= off && off <= s.text_span.end.max(s.label_span.end))
        .map(|s| Sym::Step(i, s))
}

fn short(s: &str, max: usize) -> String {
    let s = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if s.chars().count() <= max {
        s
    } else {
        let cut: String = s.chars().take(max).collect();
        format!("{}…", cut.trim_end())
    }
}

fn step_text<'a>(p: &'a Project, item: &Item, s: usize) -> &'a str {
    item.steps
        .get(s)
        .and_then(|st| p.files.get(item.file)?.text.get(st.text_span.clone()))
        .unwrap_or_default()
}

/// FR-205 for glossary terms and aliases in text.
fn glossary_hover(ctx: &Ctx, file: FileId, off: usize) -> Option<Hover> {
    let p = ctx.project;
    let text = ctx.text(file);
    let line_start = text.get(..off)?.rfind('\n').map_or(0, |i| i + 1);
    let line_end = text.get(off..)?.find('\n').map_or(text.len(), |i| off + i);
    let gtext = p
        .glossary_file
        .and_then(|g| p.files.get(g))
        .map_or("", |f| f.text.as_str());
    for term in &p.glossary {
        for phrase in std::iter::once(&term.name).chain(&term.aliases) {
            let Some(span) = find_phrase(text, line_start..line_end, phrase)
                .into_iter()
                .find(|r| r.start <= off && off <= r.end)
            else {
                continue;
            };
            let mut value = format!("**{}**", term.name);
            if phrase != &term.name {
                value.push_str(&format!(" (\"{phrase}\" is an alias)"));
            }
            if !term.aliases.is_empty() {
                value.push_str(&format!("\n\nalias: {}", term.aliases.join(", ")));
            }
            let def = gtext.get(term.definition.clone()).unwrap_or_default();
            if !def.is_empty() {
                value.push_str("\n\n---\n\n");
                value.push_str(&truncate(def, p.config.hover_length));
            }
            return Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value,
                }),
                range: Some(ctx.index(file)?.range(&span)),
            });
        }
    }
    None
}

/// FR-205
pub fn hover(ctx: &Ctx, file: FileId, off: usize) -> Option<Hover> {
    let p = ctx.project;
    let Some(r) = reference_at(p, file, off) else {
        return glossary_hover(ctx, file, off);
    };
    let target = r.target.as_ref()?;
    let value = match p.resolve(target) {
        Resolution::Item(i) => {
            let item = p.items.get(i)?;
            let text = p
                .files
                .get(item.file)?
                .text
                .get(item.meta_span.end..item.body.end)?;
            let status = item
                .meta_value("status")
                .or_else(|| p.config.defaults.get("status").map(String::as_str))
                .unwrap_or("-");
            let mut v = format!(
                "**{}** {}\n\n*{}* · status: {status}",
                item.display_id(),
                item.title,
                item.kind
            );
            let body = text.trim();
            if !body.is_empty() {
                v.push_str("\n\n---\n\n");
                v.push_str(&truncate(body, p.config.hover_length));
            }
            v
        }
        Resolution::Step(i, s) => {
            let item = p.items.get(i)?;
            format!(
                "**{}.{}** step of {} {}\n\n{}",
                item.display_id(),
                target.step.as_deref().unwrap_or_default(),
                item.display_id(),
                item.title,
                truncate(step_text(p, item, s), p.config.hover_length)
            )
        }
        Resolution::Duplicate => format!("**{}** is declared more than once", target.id),
        Resolution::MissingItem | Resolution::MissingStep(_) => return None,
    };
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value,
        }),
        range: Some(ctx.index(file)?.range(&r.span)),
    })
}

/// Keeps line breaks, unlike [`short`].
fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let cut: String = s.chars().take(max).collect();
        format!("{}…", cut.trim_end())
    }
}

/// FR-206
pub fn definition(ctx: &Ctx, file: FileId, off: usize) -> Option<Location> {
    let p = ctx.project;
    let r = reference_at(p, file, off)?;
    let (item, span) = match resolve(p, r.target.as_ref()?)? {
        Sym::Item(i) => {
            let it = p.items.get(i)?;
            (it, it.id_span.clone())
        }
        Sym::Step(i, s) => {
            let it = p.items.get(i)?;
            (it, it.steps.get(s)?.label_span.clone())
        }
    };
    ctx.location(item.file, &span)
}

/// Whether a reference points to `sym`. For an item this includes
/// references to its steps.
fn refers_to(p: &Project, r: &Reference, sym: Sym) -> bool {
    let Some(t) = &r.target else { return false };
    match (resolve(p, t), sym) {
        (Some(Sym::Item(a) | Sym::Step(a, _)), Sym::Item(b)) => a == b,
        (Some(found), Sym::Step(..)) => found == sym,
        _ => false,
    }
}

/// FR-207
pub fn references(ctx: &Ctx, file: FileId, off: usize, declaration: bool) -> Vec<Location> {
    let p = ctx.project;
    let Some(sym) = symbol_at(p, file, off) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    if declaration && let Some(it) = p.items.get(sym.item()) {
        let span = match sym {
            Sym::Item(_) => Some(it.id_span.clone()),
            Sym::Step(_, s) => it.steps.get(s).map(|st| st.label_span.clone()),
        };
        out.extend(span.and_then(|s| ctx.location(it.file, &s)));
    }
    for item in &p.items {
        let refs = item
            .links
            .iter()
            .map(|l| &l.reference)
            .chain(&item.mentions);
        for r in refs.filter(|r| refers_to(p, r, sym)) {
            out.extend(ctx.location(item.file, &r.span));
        }
    }
    out
}

/// FR-208: items that link with `impl` to the use case (or any of its
/// steps) or to the step under the cursor.
pub fn implementations(ctx: &Ctx, file: FileId, off: usize) -> Vec<Location> {
    let p = ctx.project;
    let Some(sym) = symbol_at(p, file, off) else {
        return Vec::new();
    };
    if p.items.get(sym.item()).is_none_or(|it| it.kind != USECASE) {
        return Vec::new();
    }
    p.items
        .iter()
        .filter(|it| {
            it.links
                .iter()
                .any(|l| l.key == IMPL && refers_to(p, &l.reference, sym))
        })
        .filter_map(|it| ctx.location(it.file, &it.id_span))
        .collect()
}

fn item_kind(item: &Item) -> SymbolKind {
    if item.kind == USECASE {
        SymbolKind::MODULE
    } else {
        SymbolKind::OBJECT
    }
}

/// FR-210: items with their steps as children.
pub fn document_symbols(ctx: &Ctx, file: FileId) -> Vec<DocumentSymbol> {
    let p = ctx.project;
    let Some(index) = ctx.index(file) else {
        return Vec::new();
    };
    let text = ctx.text(file);
    p.items
        .iter()
        .filter(|it| it.file == file)
        .map(|it| {
            let steps: Vec<DocumentSymbol> = it
                .steps
                .iter()
                .map(|s| {
                    let t = text.get(s.text_span.clone()).unwrap_or_default();
                    symbol(
                        format!("{}: {}", s.label, short(t, 60)),
                        SymbolKind::FIELD,
                        index.range(&(s.label_span.start..s.text_span.end.max(s.label_span.end))),
                        index.range(&s.label_span),
                        None,
                    )
                })
                .collect();
            symbol(
                format!("{} {}", it.display_id(), it.title),
                item_kind(it),
                index.range(&(it.heading.start..it.body.end)),
                index.range(&it.id_span),
                (!steps.is_empty()).then_some(steps),
            )
        })
        .collect()
}

#[allow(deprecated)] // `deprecated` is a required field of the struct.
fn symbol(
    name: String,
    kind: SymbolKind,
    range: lsp_types::Range,
    selection_range: lsp_types::Range,
    children: Option<Vec<DocumentSymbol>>,
) -> DocumentSymbol {
    DocumentSymbol {
        name,
        detail: None,
        kind,
        tags: None,
        deprecated: None,
        range,
        selection_range,
        children,
    }
}

/// FR-211: all items whose ID or title contains the query, ignoring case.
pub fn workspace_symbols(ctx: &Ctx, query: &str) -> Vec<WorkspaceSymbol> {
    let q = query.to_lowercase();
    ctx.project
        .items
        .iter()
        .filter(|it| {
            format!("{} {}", it.display_id(), it.title)
                .to_lowercase()
                .contains(&q)
        })
        .filter_map(|it| {
            Some(WorkspaceSymbol {
                name: format!("{} {}", it.display_id(), it.title),
                kind: item_kind(it),
                tags: None,
                container_name: ctx.project.files.get(it.file).map(|f| f.rel.clone()),
                location: OneOf::Left(ctx.location(it.file, &it.id_span)?),
                data: None,
            })
        })
        .collect()
}

/// FR-212: assign the next free ID to a placeholder in `range`.
pub fn code_actions(ctx: &Ctx, file: FileId, range: Range<usize>) -> Vec<CodeActionOrCommand> {
    let p = ctx.project;
    let (Some(uri), Some(index)) = (ctx.uri(file), ctx.index(file)) else {
        return Vec::new();
    };
    p.items
        .iter()
        .filter(|it| {
            it.file == file
                && it.id.is_none()
                && it.heading.start <= range.end
                && range.start <= it.heading.end
        })
        .map(|it| {
            let id = p.next_id(&it.prefix);
            let edit = TextEdit {
                range: index.range(&it.id_span),
                new_text: id.clone(),
            };
            CodeActionOrCommand::CodeAction(CodeAction {
                title: format!("Assign ID {id}"),
                kind: Some(CodeActionKind::QUICKFIX),
                edit: Some(WorkspaceEdit {
                    changes: Some(HashMap::from([(uri.clone(), vec![edit])])),
                    ..WorkspaceEdit::default()
                }),
                is_preferred: Some(true),
                ..CodeAction::default()
            })
        })
        .collect()
}

/// FR-209: the target's title or step text after each link in `range`.
pub fn inlay_hints(ctx: &Ctx, file: FileId, range: Range<usize>) -> Vec<InlayHint> {
    let p = ctx.project;
    let Some(index) = ctx.index(file) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for item in p.items.iter().filter(|it| it.file == file) {
        for link in &item.links {
            let r = &link.reference;
            if r.span.end < range.start || range.end < r.span.start {
                continue;
            }
            let Some(sym) = r.target.as_ref().and_then(|t| resolve(p, t)) else {
                continue;
            };
            let label = match sym {
                Sym::Item(i) => p.items.get(i).map(|t| t.title.clone()).unwrap_or_default(),
                Sym::Step(i, s) => p
                    .items
                    .get(i)
                    .map(|t| step_text(p, t, s).to_string())
                    .unwrap_or_default(),
            };
            let label = short(&label, p.config.hint_length);
            if label.is_empty() {
                continue;
            }
            out.push(InlayHint {
                position: index.position(r.span.end),
                label: InlayHintLabel::String(label),
                kind: None,
                text_edits: None,
                tooltip: None,
                padding_left: Some(true),
                padding_right: None,
                data: None,
            });
        }
    }
    out
}

/// Command run by a step's code lens; its argument is the list of
/// locations of the implementing items (FR-215).
pub const IMPLEMENTATIONS_COMMAND: &str = "reqmd.implementations";

/// FR-215: number and IDs of the implementing items on each step.
pub fn code_lenses(ctx: &Ctx, file: FileId) -> Vec<CodeLens> {
    let p = ctx.project;
    let Some(index) = ctx.index(file) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (i, uc) in p.items.iter().enumerate() {
        if uc.file != file || uc.kind != USECASE {
            continue;
        }
        for (s, step) in uc.steps.iter().enumerate() {
            let implementers: Vec<&Item> = p
                .items
                .iter()
                .filter(|it| {
                    it.links
                        .iter()
                        .any(|l| l.key == IMPL && refers_to(p, &l.reference, Sym::Step(i, s)))
                })
                .collect();
            let ids: Vec<String> = implementers.iter().map(|it| it.display_id()).collect();
            let title = match ids.len() {
                0 => "no implementation".to_string(),
                n => format!("{n} impl: {}", ids.join(", ")),
            };
            let locations: Vec<Location> = implementers
                .iter()
                .filter_map(|it| ctx.location(it.file, &it.id_span))
                .collect();
            out.push(CodeLens {
                range: index.range(&step.label_span),
                command: Some(Command {
                    title,
                    command: IMPLEMENTATIONS_COMMAND.into(),
                    arguments: serde_json::to_value(locations).ok().map(|v| vec![v]),
                }),
                data: None,
            });
        }
    }
    out
}

/// The ID under the cursor that a rename would change, and its span here.
fn rename_target(p: &Project, file: FileId, off: usize) -> Option<(String, Range<usize>)> {
    if let Some(r) = reference_at(p, file, off) {
        let id = r.target.as_ref()?.id.clone();
        let span = r.span.start..r.span.start + id.len();
        return Some((id, span));
    }
    let item = p.items.get(item_at(p, file, off)?)?;
    if off < item.id_span.start || item.id_span.end < off {
        return None;
    }
    Some((item.id.clone()?, item.id_span.clone()))
}

/// FR-214: range and text of the ID a rename would change.
pub fn prepare_rename(ctx: &Ctx, file: FileId, off: usize) -> Option<(lsp_types::Range, String)> {
    let (id, span) = rename_target(ctx.project, file, off)?;
    Some((ctx.index(file)?.range(&span), id))
}

/// FR-214: renames an item ID in its declaration and all references.
#[allow(clippy::mutable_key_type)] // lsp-types keys edits by Uri.
pub fn rename(ctx: &Ctx, file: FileId, off: usize, new_id: &str) -> Result<WorkspaceEdit, String> {
    let p = ctx.project;
    let (old, _) = rename_target(p, file, off).ok_or("no item ID at the cursor")?;
    let decl = match p.declarations(&old) {
        [] => return Err(format!("{old} does not exist")),
        [d] => *d,
        _ => return Err(format!("{old} is declared more than once")),
    };
    let new_id = new_id.trim();
    let valid = parse_target(&p.config, new_id).is_some_and(|t| t.step.is_none());
    if !valid {
        return Err(format!("\"{new_id}\" is not a valid item ID"));
    }
    let prefix = |id: &str| id.rsplit_once('-').map(|(p, _)| p.to_string());
    if prefix(new_id) != prefix(&old) {
        return Err(format!(
            "{new_id} has a different prefix than {old}; a rename keeps the item type"
        ));
    }
    if new_id == old {
        return Err("the new ID is the old ID".into());
    }
    if !p.declarations(new_id).is_empty() {
        return Err(format!("{new_id} already exists"));
    }
    let mut changes: HashMap<Uri, Vec<TextEdit>> = HashMap::new();
    let mut edit = |file: FileId, span: Range<usize>| {
        if let (Some(uri), Some(index)) = (ctx.uri(file), ctx.index(file)) {
            changes.entry(uri).or_default().push(TextEdit {
                range: index.range(&span),
                new_text: new_id.to_string(),
            });
        }
    };
    if let Some(item) = p.items.get(decl) {
        edit(item.file, item.id_span.clone());
    }
    for item in &p.items {
        for r in item
            .links
            .iter()
            .map(|l| &l.reference)
            .chain(&item.mentions)
        {
            if r.target.as_ref().is_some_and(|t| t.id == old) {
                edit(item.file, r.span.start..r.span.start + old.len());
            }
        }
    }
    Ok(WorkspaceEdit {
        changes: Some(changes),
        ..WorkspaceEdit::default()
    })
}

/// Characters that make up a reference or a word of a title.
fn is_token_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '-' | '.' | '_' | '?')
}

/// What the cursor is completing.
#[derive(Debug, PartialEq, Eq)]
enum Context {
    /// A metadata key; the partial key starts at the offset.
    Key(usize),
    /// A value of a key with a value set.
    Value(String, usize),
    /// A link target; `None` allows any type.
    Link(Option<Vec<String>>, usize),
    /// A reference in body text.
    Body(usize),
}

/// FR-203, FR-204. `trigger` is the trigger character, if any; `invoked`
/// means the user asked explicitly.
pub fn completion(
    ctx: &Ctx,
    file: FileId,
    off: usize,
    trigger: Option<&str>,
    invoked: bool,
) -> CompletionList {
    let empty = CompletionList {
        is_incomplete: true,
        items: Vec::new(),
    };
    let p = ctx.project;
    let text = ctx.text(file);
    let Some(off) = (off <= text.len() && text.is_char_boundary(off)).then_some(off) else {
        return empty;
    };
    let code = code_ranges(text);
    if code.iter().any(|r| r.start < off && off < r.end) {
        return empty;
    }
    let Some(i) = item_at(p, file, off) else {
        return empty;
    };
    let Some(item) = p.items.get(i) else {
        return empty;
    };
    if off < item.body.start {
        return empty;
    }
    let Some(context) = context(p, item, text, off) else {
        return empty;
    };
    let Some(index) = ctx.index(file) else {
        return empty;
    };
    let (start, items) = match context {
        Context::Key(start) => {
            let ty = p.config.type_by_name(&item.kind);
            let keys = ty.map(|t| p.config.meta_keys(t)).unwrap_or_default();
            let used: Vec<&str> = item
                .meta
                .iter()
                .filter(|m| m.known && !m.key_span.contains(&off) && m.key_span.end != off)
                .map(|m| m.key.as_str())
                .collect();
            let partial = text.get(start..off).unwrap_or_default();
            let items = keys
                .into_iter()
                .filter(|k| !used.contains(k) && k.starts_with(partial))
                .map(|k| {
                    let detail = p
                        .config
                        .link(k)
                        .map(|l| format!("link: {}", l.name.as_deref().unwrap_or(k)));
                    (
                        k.to_string(),
                        format!("{k}: "),
                        detail,
                        CompletionItemKind::PROPERTY,
                    )
                })
                .collect();
            (start, items)
        }
        Context::Value(key, start) => {
            let partial = text.get(start..off).unwrap_or_default();
            let items = p
                .config
                .values
                .get(&key)
                .into_iter()
                .flatten()
                .filter(|v| v.starts_with(partial))
                .map(|v| (v.clone(), v.clone(), None, CompletionItemKind::ENUM_MEMBER))
                .collect();
            (start, items)
        }
        Context::Link(allowed, start) => {
            if matches!(trigger, Some("-" | ".")) && start == off {
                return empty;
            }
            (
                start,
                ref_candidates(
                    p,
                    allowed.as_deref(),
                    text.get(start..off).unwrap_or_default(),
                ),
            )
        }
        Context::Body(start) => {
            // Space, comma and colon trigger only in link values.
            if matches!(trigger, Some(" " | "," | ":")) || (start == off && !invoked) {
                return empty;
            }
            (
                start,
                ref_candidates(p, None, text.get(start..off).unwrap_or_default()),
            )
        }
    };
    let partial = text.get(start..off).unwrap_or_default();
    let range = index.range(&(start..off));
    CompletionList {
        is_incomplete: true,
        items: items
            .into_iter()
            .map(|(label, insert, detail, kind)| CompletionItem {
                // The server filters by ID and title; the partial text as
                // filter text keeps clients from filtering them out again.
                filter_text: Some(partial.to_string()),
                sort_text: Some(label.clone()),
                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                    range,
                    new_text: insert,
                })),
                label,
                detail,
                kind: Some(kind),
                ..CompletionItem::default()
            })
            .collect(),
    }
}

/// Items and steps whose ID or title/text contains `partial`, ignoring
/// case. `allowed` limits the target types (FR-104).
fn ref_candidates(
    p: &Project,
    allowed: Option<&[String]>,
    partial: &str,
) -> Vec<(String, String, Option<String>, CompletionItemKind)> {
    let q = partial.to_lowercase();
    let ok = |kind: &str| allowed.is_none_or(|a| a.iter().any(|t| t == kind));
    let mut out = Vec::new();
    for item in &p.items {
        let Some(id) = &item.id else { continue };
        if ok(&item.kind) && format!("{id} {}", item.title).to_lowercase().contains(&q) {
            out.push((
                id.clone(),
                id.clone(),
                Some(item.title.clone()),
                CompletionItemKind::REFERENCE,
            ));
        }
        if !ok(STEP) {
            continue;
        }
        for (s, step) in item.steps.iter().enumerate() {
            let label = format!("{id}.{}", step.label);
            let t = short(step_text(p, item, s), 80);
            if format!("{label} {t}").to_lowercase().contains(&q) {
                out.push((label.clone(), label, Some(t), CompletionItemKind::REFERENCE));
            }
        }
    }
    out
}

/// Determines the completion context at `off` within `item`'s body.
fn context(p: &Project, item: &Item, text: &str, off: usize) -> Option<Context> {
    let line_start = text.get(..off)?.rfind('\n').map_or(0, |i| i + 1);
    let before = text.get(line_start..off)?;
    let at_meta_line = line_start == item.body.start
        || (item.meta_span.start < item.meta_span.end
            && item.meta_span.start <= line_start
            && line_start <= item.meta_span.end);

    let token_start = before
        .char_indices()
        .rev()
        .take_while(|(_, c)| is_token_char(*c))
        .last()
        .map_or(off, |(i, _)| line_start + i);

    if at_meta_line {
        let indent = before.len() - before.trim_start_matches(' ').len();
        // The last pair start: line start or after two spaces or a tab.
        let b = before.as_bytes();
        let mut last_key = None;
        for k in indent..before.len() {
            let sep = k == indent
                || (b.get(k - 1).is_some_and(|c| *c == b'\t')
                    || (k >= 2 && b.get(k - 2..k) == Some(b"  ")));
            if sep && let Some((key_end, after)) = key_colon(text, line_start + k) {
                last_key = Some((line_start + k, key_end, after));
            }
        }
        match last_key {
            None => {
                if before[indent..]
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
                {
                    return Some(Context::Key(line_start + indent));
                }
            }
            Some((k, key_end, after)) => {
                let key = text.get(k..key_end)?.to_string();
                let rest = text.get(after.min(off)..off)?;
                // Two spaces or a tab after the value start a new pair.
                let value_start = after + (rest.len() - rest.trim_start().len());
                let value = text.get(value_start.min(off)..off)?;
                if let Some(sep) = value
                    .rfind("  ")
                    .map(|i| i + 2)
                    .or(value.rfind('\t').map(|i| i + 1))
                {
                    let partial_start = value_start + sep;
                    let partial = text.get(partial_start..off)?;
                    if partial
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
                    {
                        return Some(Context::Key(partial_start));
                    }
                    return None;
                }
                if let Some(link) = p.config.link(&key) {
                    let start = value
                        .rfind(',')
                        .map_or(value_start, |i| value_start + i + 1);
                    let rest = text.get(start..off)?;
                    let start = start + (rest.len() - rest.trim_start().len());
                    return Some(Context::Link(link.to.clone(), start));
                }
                if p.config.values.contains_key(&key) {
                    return Some(Context::Value(key, value_start));
                }
                return None;
            }
        }
    }
    Some(Context::Body(token_start))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use reqmd_core::{Config, SourceFile};

    use super::*;

    const UC: &str = "## UC-012 Place order\nactor: Customer  prio: must\n\n- s1: Customer opens the cart.\n- s2: System shows the total.\n";
    const FR: &str = "## FR-104 VAT per line item\nimpl: UC-012.s2  prio: must  status: review\n\nWhen the cart is opened, the system shall calculate VAT (see UC-012).\n\n## FR-? New one\n";
    const TC: &str = "## TC-220 Austria\nver: FR-104\n\nTax at 10 %.\n";

    fn project(extra: &[(&str, &str)]) -> Project {
        let mut files = vec![("uc.md", UC), ("fr.md", FR), ("tc.md", TC)];
        files.extend_from_slice(extra);
        let files = files
            .iter()
            .map(|(n, t)| {
                let path = std::env::temp_dir().join(n);
                SourceFile::new(path, n.to_string(), t.to_string())
            })
            .collect();
        Project::from_files(PathBuf::from("."), Config::defaults().unwrap(), files)
    }

    fn ctx<'a>(p: &'a Project, uris: &'a HashMap<FileId, Uri>) -> Ctx<'a> {
        Ctx {
            project: p,
            encoding: Encoding::Utf16,
            uris,
        }
    }

    fn file(p: &Project, name: &str) -> FileId {
        p.files.iter().position(|f| f.rel == name).unwrap()
    }

    /// Offset of the first occurrence of `needle` plus `delta`.
    fn at(p: &Project, f: FileId, needle: &str, delta: usize) -> usize {
        p.files[f].text.find(needle).unwrap() + delta
    }

    fn labels(l: &CompletionList) -> Vec<String> {
        l.items.iter().map(|i| i.label.clone()).collect()
    }

    #[test]
    fn fr_203_link_completion_limited_to_target_types() {
        let p = project(&[("new.md", "## FR-200 X\nimpl: \n")]);
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let f = file(&p, "new.md");
        let off = at(&p, f, "impl: ", 6);
        let l = completion(&c, f, off, Some(" "), false);
        assert_eq!(labels(&l), ["UC-012", "UC-012.s1", "UC-012.s2"]);
        assert_eq!(l.items[0].detail.as_deref(), Some("Place order"));
        assert_eq!(
            l.items[2].detail.as_deref(),
            Some("System shows the total.")
        );
    }

    #[test]
    fn fr_203_link_completion_after_comma_and_by_title() {
        let p = project(&[("new.md", "## TC-300 X\nver: FR-104, vat\n")]);
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let f = file(&p, "new.md");
        let off = at(&p, f, "vat", 3);
        let l = completion(&c, f, off, None, true);
        assert_eq!(labels(&l), ["FR-104"]);
        let Some(CompletionTextEdit::Edit(e)) = &l.items[0].text_edit else {
            panic!()
        };
        assert_eq!(e.new_text, "FR-104");
        assert_eq!(e.range.start.character, 13);
        assert_eq!(e.range.end.character, 16);
    }

    #[test]
    fn fr_203_body_completion_by_id_prefix_and_title() {
        let p = project(&[("new.md", "## FR-200 X\n\nAs in FR-1 and customer\n")]);
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let f = file(&p, "new.md");
        let l = completion(&c, f, at(&p, f, "FR-1", 4), Some("-"), false);
        assert_eq!(labels(&l), ["FR-104"]);
        let l = completion(&c, f, at(&p, f, "customer", 8), None, true);
        assert_eq!(labels(&l), ["UC-012.s1"]);
        // A space in body text does not trigger.
        let l = completion(&c, f, at(&p, f, " and", 1), Some(" "), false);
        assert!(l.items.is_empty());
    }

    #[test]
    fn fr_203_no_completion_in_code() {
        let p = project(&[("new.md", "## FR-200 X\n\nSee `FR-1` here.\n")]);
        let uris = HashMap::new();
        let f = file(&p, "new.md");
        let l = completion(&ctx(&p, &uris), f, at(&p, f, "FR-1", 4), Some("-"), false);
        assert!(l.items.is_empty());
    }

    #[test]
    fn fr_204_key_completion_at_metadata_line_start() {
        let p = project(&[("new.md", "## FR-200 X\nprio: must\nst\n\nText.\n")]);
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let f = file(&p, "new.md");
        let l = completion(&c, f, at(&p, f, "\nst\n", 3), None, true);
        assert_eq!(labels(&l), ["status"]);
        let Some(CompletionTextEdit::Edit(e)) = &l.items[0].text_edit else {
            panic!()
        };
        assert_eq!(e.new_text, "status: ");
        // Directly after the heading, all keys except used ones.
        let p = project(&[("new.md", "## FR-200 X\n\n")]);
        let f = file(&p, "new.md");
        let l = completion(&ctx(&p, &uris), f, at(&p, f, "\n\n", 1), None, true);
        assert_eq!(labels(&l), ["status", "prio", "impl", "ref", "ver"]);
    }

    #[test]
    fn fr_204_key_completion_after_pair_separator() {
        let p = project(&[("new.md", "## FR-200 X\nprio: must  im\n")]);
        let uris = HashMap::new();
        let f = file(&p, "new.md");
        let l = completion(&ctx(&p, &uris), f, at(&p, f, "im\n", 2), None, true);
        assert_eq!(labels(&l), ["impl"]);
    }

    #[test]
    fn fr_204_value_completion() {
        let p = project(&[("new.md", "## FR-200 X\nprio: must  status: r\n")]);
        let uris = HashMap::new();
        let f = file(&p, "new.md");
        let l = completion(&ctx(&p, &uris), f, at(&p, f, ": r\n", 3), Some(" "), false);
        assert_eq!(labels(&l), ["review"]);
        let l = completion(&ctx(&p, &uris), f, at(&p, f, "prio: ", 6), None, true);
        assert_eq!(labels(&l), ["must", "should", "could", "wont"]);
    }

    #[test]
    fn fr_205_hover_on_item_and_step_reference() {
        let p = project(&[]);
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let f = file(&p, "tc.md");
        let h = hover(&c, f, at(&p, f, "FR-104", 2)).unwrap();
        let HoverContents::Markup(m) = h.contents else {
            panic!()
        };
        assert!(
            m.value.starts_with("**FR-104** VAT per line item"),
            "{}",
            m.value
        );
        assert!(m.value.contains("*functional* · status: review"));
        assert!(m.value.contains("calculate VAT"));
        assert!(!m.value.contains("impl:"));

        let f = file(&p, "fr.md");
        let h = hover(&c, f, at(&p, f, "UC-012.s2", 8)).unwrap();
        let HoverContents::Markup(m) = h.contents else {
            panic!()
        };
        assert!(m.value.contains("System shows the total."), "{}", m.value);
        assert!(hover(&c, f, at(&p, f, "When", 0)).is_none());
    }

    #[test]
    fn fr_205_hover_text_limited_to_hover_length() {
        let mut p = project(&[]);
        p.config.hover_length = 10;
        let uris = HashMap::new();
        let f = file(&p, "tc.md");
        let h = hover(&ctx(&p, &uris), f, at(&p, f, "FR-104", 0)).unwrap();
        let HoverContents::Markup(m) = h.contents else {
            panic!()
        };
        assert!(m.value.ends_with("When the c…"), "{}", m.value);
    }

    #[test]
    fn fr_206_definition_of_item_and_step() {
        let p = project(&[]);
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let f = file(&p, "fr.md");
        let loc = definition(&c, f, at(&p, f, "UC-012.s2", 0)).unwrap();
        assert!(loc.uri.as_str().ends_with("uc.md"));
        assert_eq!(loc.range.start, lsp_types::Position::new(4, 2));
        let loc = definition(&c, f, at(&p, f, "UC-012)", 3)).unwrap();
        assert_eq!(loc.range.start, lsp_types::Position::new(0, 3));
    }

    #[test]
    fn fr_207_references_to_item_and_step() {
        let p = project(&[]);
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let uc = file(&p, "uc.md");
        // On the use case heading: the step link and the mention.
        let refs = references(&c, uc, 4, false);
        assert_eq!(refs.len(), 2);
        let with_decl = references(&c, uc, 4, true);
        assert_eq!(with_decl.len(), 3);
        // On step s2: only the step link.
        let refs = references(&c, uc, at(&p, uc, "s2:", 0), false);
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].range.start, lsp_types::Position::new(1, 6));
        // On step s1: none.
        assert!(references(&c, uc, at(&p, uc, "s1:", 0), false).is_empty());
    }

    #[test]
    fn fr_208_implementations_of_use_case_and_step() {
        let p = project(&[("more.md", "## FR-300 Y\nimpl: UC-012\n")]);
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let uc = file(&p, "uc.md");
        let names = |locs: Vec<Location>| -> Vec<String> {
            let mut v: Vec<String> = locs
                .iter()
                .map(|l| l.uri.as_str().rsplit('/').next().unwrap().to_string())
                .collect();
            v.sort();
            v
        };
        assert_eq!(names(implementations(&c, uc, 4)), ["fr.md", "more.md"]);
        assert_eq!(
            names(implementations(&c, uc, at(&p, uc, "s2:", 0))),
            ["fr.md"]
        );
        assert!(implementations(&c, uc, at(&p, uc, "s1:", 0)).is_empty());
        // Not a use case: nothing.
        let fr = file(&p, "fr.md");
        assert!(implementations(&c, fr, 4).is_empty());
    }

    #[test]
    fn fr_210_document_symbols_with_steps() {
        let p = project(&[]);
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let s = document_symbols(&c, file(&p, "uc.md"));
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].name, "UC-012 Place order");
        let children: Vec<&str> = s[0]
            .children
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.name.as_str())
            .collect();
        assert_eq!(
            children,
            [
                "s1: Customer opens the cart.",
                "s2: System shows the total."
            ]
        );
        let s = document_symbols(&c, file(&p, "fr.md"));
        let names: Vec<&str> = s.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, ["FR-104 VAT per line item", "FR-? New one"]);
    }

    #[test]
    fn fr_211_workspace_symbols_by_id_and_title() {
        let p = project(&[]);
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let names = |q: &str| -> Vec<String> {
            workspace_symbols(&c, q)
                .into_iter()
                .map(|s| s.name)
                .collect()
        };
        assert_eq!(names("fr-104"), ["FR-104 VAT per line item"]);
        assert_eq!(names("order"), ["UC-012 Place order"]);
        assert_eq!(names("").len(), 4);
    }

    #[test]
    fn fr_209_inlay_hints_after_links() {
        let mut p = project(&[]);
        p.config.hint_length = 12;
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let f = file(&p, "fr.md");
        let hints = inlay_hints(&c, f, 0..p.files[f].text.len());
        assert_eq!(hints.len(), 1);
        let InlayHintLabel::String(label) = &hints[0].label else {
            panic!()
        };
        assert_eq!(label, "System shows…");
        assert_eq!(hints[0].position, lsp_types::Position::new(1, 15));
        // Outside the range: none.
        assert!(inlay_hints(&c, f, 0..5).is_empty());
    }

    #[test]
    fn fr_215_code_lens_per_step() {
        let p = project(&[("more.md", "## FR-300 Y\nimpl: UC-012.s2\n")]);
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let lenses = code_lenses(&c, file(&p, "uc.md"));
        let titles: Vec<&str> = lenses
            .iter()
            .map(|l| l.command.as_ref().unwrap().title.as_str())
            .collect();
        assert_eq!(titles, ["no implementation", "2 impl: FR-104, FR-300"]);
        let cmd = lenses[1].command.as_ref().unwrap();
        assert_eq!(cmd.command, IMPLEMENTATIONS_COMMAND);
        let locs: Vec<Location> =
            serde_json::from_value(cmd.arguments.as_ref().unwrap()[0].clone()).unwrap();
        assert_eq!(locs.len(), 2);
    }

    #[test]
    #[allow(clippy::mutable_key_type)]
    fn fr_214_rename_updates_declaration_and_references() {
        let p = project(&[]);
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let uc = file(&p, "uc.md");
        let (range, id) = prepare_rename(&c, uc, 4).unwrap();
        assert_eq!(id, "UC-012");
        assert_eq!(range.start, lsp_types::Position::new(0, 3));
        let edit = rename(&c, uc, 4, "UC-013").unwrap();
        let changes = edit.changes.unwrap();
        let count: usize = changes.values().map(Vec::len).sum();
        // Declaration, the step link and the mention in fr.md.
        assert_eq!(count, 3);
        let fr_edits = changes
            .iter()
            .find(|(u, _)| u.as_str().ends_with("fr.md"))
            .unwrap()
            .1;
        assert!(fr_edits.iter().all(|e| e.new_text == "UC-013"));
        // The step link keeps its step: only the ID part is replaced.
        assert!(
            fr_edits
                .iter()
                .any(|e| e.range.start == lsp_types::Position::new(1, 6)
                    && e.range.end == lsp_types::Position::new(1, 12))
        );
    }

    #[test]
    fn fr_214_rename_refuses_invalid_existing_or_other_prefix() {
        let p = project(&[]);
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let fr = file(&p, "fr.md");
        let off = 4;
        assert!(
            rename(&c, fr, off, "FR-12")
                .unwrap_err()
                .contains("not a valid")
        );
        assert!(
            rename(&c, fr, off, "FR-104")
                .unwrap_err()
                .contains("old ID")
        );
        assert!(
            rename(&c, fr, off, "NFR-104")
                .unwrap_err()
                .contains("prefix")
        );
        let p2 = project(&[("x.md", "## FR-105 Taken\n")]);
        let c2 = ctx(&p2, &uris);
        assert!(
            rename(&c2, file(&p2, "fr.md"), off, "FR-105")
                .unwrap_err()
                .contains("exists")
        );
        // A placeholder has no ID to rename.
        assert!(prepare_rename(&c, fr, at(&p, fr, "FR-?", 1)).is_none());
    }

    #[test]
    fn fr_205_hover_on_glossary_term_and_alias() {
        let files = [
            ("a.md", "## TC-001 X\n\nPut it in the basket or the cart.\n"),
            ("g.md", "## Cart\nalias: basket\n\nItems not yet ordered.\n"),
        ];
        let files = files
            .iter()
            .map(|(n, t)| {
                SourceFile::new(std::env::temp_dir().join(n), n.to_string(), t.to_string())
            })
            .collect();
        let config = Config::from_toml("[project]\nglossary = \"g.md\"\n", None).unwrap();
        let p = Project::from_files(PathBuf::from("."), config, files);
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let h = hover(&c, 0, at(&p, 0, "basket", 2)).unwrap();
        let HoverContents::Markup(m) = h.contents else {
            panic!()
        };
        assert!(
            m.value.starts_with("**Cart** (\"basket\" is an alias)"),
            "{}",
            m.value
        );
        assert!(m.value.contains("Items not yet ordered."));
        let h = hover(&c, 0, at(&p, 0, "cart.", 1)).unwrap();
        let HoverContents::Markup(m) = h.contents else {
            panic!()
        };
        assert!(
            m.value.starts_with("**Cart**\n\nalias: basket"),
            "{}",
            m.value
        );
    }

    #[test]
    fn fr_212_assign_id_code_action() {
        let p = project(&[]);
        let uris = HashMap::new();
        let c = ctx(&p, &uris);
        let f = file(&p, "fr.md");
        let off = at(&p, f, "FR-?", 1);
        let actions = code_actions(&c, f, off..off);
        assert_eq!(actions.len(), 1);
        let CodeActionOrCommand::CodeAction(a) = &actions[0] else {
            panic!()
        };
        assert_eq!(a.title, "Assign ID FR-105");
        // lsp-types keys `changes` by Uri, which clippy considers mutable.
        #[allow(clippy::mutable_key_type)]
        let edits = a.edit.as_ref().unwrap().changes.as_ref().unwrap();
        let e = &edits.values().next().unwrap()[0];
        assert_eq!(e.new_text, "FR-105");
        assert_eq!(e.range.start, lsp_types::Position::new(5, 3));
        assert_eq!(e.range.end, lsp_types::Position::new(5, 7));
        // Not on a placeholder: nothing.
        assert!(code_actions(&c, f, 0..0).is_empty());
    }
}
