//! Code actions: FR-212 assign ID, FR-213 create stub and FR-219 language
//! check actions, plus the `requimd.toml` edits behind FR-219, which keep
//! comments and formatting.

use std::collections::HashMap;
use std::ops::Range;

use lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, Command, CreateFile, CreateFileOptions,
    DocumentChangeOperation, DocumentChanges, OneOf, OptionalVersionedTextDocumentIdentifier,
    ResourceOp, TextDocumentEdit, TextEdit, Uri, WorkspaceEdit,
};
use requimd_core::config::{ALLOW_CHECKS, FUNCTIONAL, IMPL, USECASE};
use requimd_core::parse::code_ranges;
use requimd_core::words::{is_word_char, markers, overlaps};
use requimd_core::{Diagnostic, FileId, Item, Project, Resolution};
use serde_json::json;

use crate::convert::{LineIndex, path_to_uri};
use crate::features::{self, Ctx};

pub const ALLOW_IN_PROJECT: &str = "requimd.allowInProject";
pub const ADD_WEAK_WORD: &str = "requimd.addWeakWord";

/// All code actions for `range` of `file`. `diags` are the project's
/// current diagnostics.
pub fn code_actions(
    ctx: &Ctx,
    file: FileId,
    range: Range<usize>,
    diags: &[Diagnostic],
) -> Vec<CodeActionOrCommand> {
    let mut out = features::code_actions(ctx, file, range.clone());
    out.extend(stub_actions(ctx, file, &range));
    out.extend(language_actions(ctx, file, &range, diags));
    out
}

fn action(
    title: String,
    edit: Option<WorkspaceEdit>,
    command: Option<Command>,
) -> CodeActionOrCommand {
    CodeActionOrCommand::CodeAction(CodeAction {
        title,
        kind: Some(CodeActionKind::QUICKFIX),
        edit,
        command,
        ..CodeAction::default()
    })
}

#[allow(clippy::mutable_key_type)] // lsp-types keys edits by Uri.
fn single_edit(uri: Uri, edit: TextEdit) -> WorkspaceEdit {
    WorkspaceEdit {
        changes: Some(HashMap::from([(uri, vec![edit])])),
        ..WorkspaceEdit::default()
    }
}

fn overlapping(span: &Range<usize>, range: &Range<usize>) -> bool {
    span.start <= range.end && range.start <= span.end
}

/// FR-213: stubs for dangling references and uncovered steps, and missing
/// steps for dangling step references.
fn stub_actions(ctx: &Ctx, file: FileId, range: &Range<usize>) -> Vec<CodeActionOrCommand> {
    let p = ctx.project;
    let mut out = Vec::new();
    let mut seen = Vec::new();
    for item in p.items.iter().filter(|it| it.file == file) {
        let level = "#".repeat(usize::from(item.level.max(1)));
        for r in item
            .links
            .iter()
            .map(|l| &l.reference)
            .chain(&item.mentions)
        {
            if !overlapping(&r.span, range) {
                continue;
            }
            let Some(t) = &r.target else { continue };
            if seen.contains(&r.text) {
                continue;
            }
            match p.resolve(t) {
                Resolution::MissingItem => {
                    seen.push(r.text.clone());
                    let stub = format!("{level} {}\n", t.id);
                    if let Some(edit) = insert_stub(ctx, file, &stub) {
                        out.push(action(format!("Create stub {}", t.id), Some(edit), None));
                    }
                }
                Resolution::MissingStep(uc) => {
                    seen.push(r.text.clone());
                    let label = t.step.clone().unwrap_or_default();
                    if let Some(edit) = add_step(ctx, uc, &label) {
                        out.push(action(
                            format!("Add step {label} to {}", t.id),
                            Some(edit),
                            None,
                        ));
                    }
                }
                _ => {}
            }
        }
    }
    // Uncovered steps.
    let Some(fr) = p.config.type_by_name(FUNCTIONAL) else {
        return out;
    };
    for (i, uc) in p.items.iter().enumerate() {
        if uc.file != file || uc.kind != USECASE {
            continue;
        }
        let Some(id) = &uc.id else { continue };
        for (s, step) in uc.steps.iter().enumerate() {
            let span = step.label_span.start..step.text_span.end.max(step.label_span.end);
            if !overlapping(&span, range) || covered(p, i, s) {
                continue;
            }
            let text = p
                .files
                .get(uc.file)
                .and_then(|f| f.text.get(step.text_span.clone()))
                .unwrap_or_default();
            let title: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
            let level = "#".repeat(usize::from(uc.level.max(1)));
            let reference = format!("{id}.{}", step.label);
            let stub = format!("{level} {}-? {title}\n{IMPL}: {reference}\n", fr.prefix);
            if let Some(edit) = insert_stub(ctx, file, &stub) {
                out.push(action(
                    format!("Create requirement for {reference}"),
                    Some(edit),
                    None,
                ));
            }
        }
    }
    out
}

fn covered(p: &Project, uc: usize, step: usize) -> bool {
    p.items.iter().any(|it| {
        it.links.iter().any(|l| {
            l.key == IMPL
                && l.reference
                    .target
                    .as_ref()
                    .is_some_and(|t| p.resolve(t) == Resolution::Step(uc, step))
        })
    })
}

/// Text to insert at the end of `text` so that `stub` starts after a
/// blank line.
fn separated(text: &str, stub: &str) -> String {
    let sep = if text.is_empty() || text.ends_with("\n\n") {
        ""
    } else if text.ends_with('\n') {
        "\n"
    } else {
        "\n\n"
    };
    format!("{sep}{stub}")
}

/// Appends a stub to the configured stub file, creating it if needed, or
/// to the end of the current file.
fn insert_stub(ctx: &Ctx, file: FileId, stub: &str) -> Option<WorkspaceEdit> {
    let p = ctx.project;
    let target = match &p.config.stub_file {
        Some(rel) => {
            let rel = rel.replace('\\', "/");
            match p.files.iter().position(|f| f.rel == rel) {
                Some(id) => Some(id),
                None => {
                    let path = p.root.join(&rel);
                    let uri = path_to_uri(&path)?;
                    let existing = std::fs::read_to_string(&path).unwrap_or_default();
                    let end = LineIndex::new(&existing, ctx.encoding).position(existing.len());
                    let edit = TextEdit {
                        range: lsp_types::Range { start: end, end },
                        new_text: separated(&existing, stub),
                    };
                    return Some(WorkspaceEdit {
                        document_changes: Some(DocumentChanges::Operations(vec![
                            DocumentChangeOperation::Op(ResourceOp::Create(CreateFile {
                                uri: uri.clone(),
                                options: Some(CreateFileOptions {
                                    overwrite: Some(false),
                                    ignore_if_exists: Some(true),
                                }),
                                annotation_id: None,
                            })),
                            DocumentChangeOperation::Edit(TextDocumentEdit {
                                text_document: OptionalVersionedTextDocumentIdentifier {
                                    uri,
                                    version: None,
                                },
                                edits: vec![OneOf::Left(edit)],
                            }),
                        ])),
                        ..WorkspaceEdit::default()
                    });
                }
            }
        }
        None => Some(file),
    }?;
    let text = p.files.get(target)?.text.as_str();
    let end = ctx.index(target)?.position(text.len());
    Some(single_edit(
        ctx.uri(target)?,
        TextEdit {
            range: lsp_types::Range { start: end, end },
            new_text: separated(text, stub),
        },
    ))
}

/// Appends `- label: ` after the last step of a use case.
fn add_step(ctx: &Ctx, uc: usize, label: &str) -> Option<WorkspaceEdit> {
    let p = ctx.project;
    let item: &Item = p.items.get(uc)?;
    let text = p.files.get(item.file)?.text.as_str();
    let (at, new_text) = match item.steps.last() {
        Some(last) => {
            let end = last.text_span.end.max(last.label_span.end);
            let line_end = text.get(end..)?.find('\n').map_or(text.len(), |i| end + i);
            let line_end = if text.get(..line_end)?.ends_with('\r') {
                line_end - 1
            } else {
                line_end
            };
            (line_end, format!("\n- {label}: "))
        }
        None => {
            let body = text.get(item.body.clone())?;
            (
                item.body.start + body.trim_end().len(),
                format!("\n\n- {label}: "),
            )
        }
    };
    let pos = ctx.index(item.file)?.position(at);
    Some(single_edit(
        ctx.uri(item.file)?,
        TextEdit {
            range: lsp_types::Range {
                start: pos,
                end: pos,
            },
            new_text,
        },
    ))
}

/// FR-219
fn language_actions(
    ctx: &Ctx,
    file: FileId,
    range: &Range<usize>,
    diags: &[Diagnostic],
) -> Vec<CodeActionOrCommand> {
    let p = ctx.project;
    let Some(text) = p.files.get(file).map(|f| f.text.as_str()) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut seen: Vec<(&str, String)> = Vec::new();
    let mut on_weak_word = false;
    for d in diags {
        if d.file != file || !ALLOW_CHECKS.contains(&d.check) || !overlapping(&d.span, range) {
            continue;
        }
        on_weak_word |= d.check == "weak-word";
        let word = text.get(d.span.clone()).unwrap_or_default().to_lowercase();
        let word = word.split_whitespace().collect::<Vec<_>>().join(" ");
        if word.is_empty() || seen.contains(&(d.check, word.clone())) {
            continue;
        }
        seen.push((d.check, word.clone()));
        if let Some(edit) = allow_here(ctx, file, text, d.span.start, &word) {
            out.push(action(
                format!("Allow \"{word}\" on this line"),
                Some(edit),
                None,
            ));
        }
        out.push(action(
            format!("Allow \"{word}\" in project ({})", d.check),
            None,
            Some(Command {
                title: format!("Allow \"{word}\" in project"),
                command: ALLOW_IN_PROJECT.into(),
                arguments: Some(vec![json!(d.check), json!(word)]),
            }),
        ));
    }
    if !on_weak_word
        && p.items.iter().any(|it| {
            it.file == file && it.heading.start <= range.start && range.start <= it.body.end
        })
        && let Some(word) = word_or_selection(text, range)
    {
        let known = p
            .config
            .weak_words()
            .iter()
            .any(|w| w.word.eq_ignore_ascii_case(&word));
        if !known {
            out.push(action(
                format!("Add \"{word}\" to weak words"),
                None,
                Some(Command {
                    title: format!("Add \"{word}\" to weak words"),
                    command: ADD_WEAK_WORD.into(),
                    arguments: Some(vec![json!(word)]),
                }),
            ));
        }
    }
    out
}

/// The selected text on one line, or the word at the cursor.
fn word_or_selection(text: &str, range: &Range<usize>) -> Option<String> {
    if range.start < range.end {
        let s = text.get(range.clone())?.trim();
        if s.contains('\n') || !s.chars().any(char::is_alphabetic) {
            return None;
        }
        return Some(s.split_whitespace().collect::<Vec<_>>().join(" "));
    }
    let at = range.start;
    let start = text
        .get(..at)?
        .char_indices()
        .rev()
        .take_while(|(_, c)| is_word_char(*c))
        .last()
        .map_or(at, |(i, _)| i);
    let end = text
        .get(at..)?
        .char_indices()
        .find(|(_, c)| !is_word_char(*c))
        .map_or(text.len(), |(i, _)| at + i);
    let word = text.get(start..end)?.trim_matches('-');
    (word.chars().any(char::is_alphabetic) && !word.chars().any(|c| c.is_ascii_digit()))
        .then(|| word.to_lowercase())
}

/// Adds `word` to the inline marker of the line at `at`, or adds a marker.
fn allow_here(ctx: &Ctx, file: FileId, text: &str, at: usize, word: &str) -> Option<WorkspaceEdit> {
    let line_start = text.get(..at)?.rfind('\n').map_or(0, |i| i + 1);
    let line_end = text.get(at..)?.find('\n').map_or(text.len(), |i| at + i);
    let code = code_ranges(text);
    let marker = markers(text, line_start..line_end)
        .into_iter()
        .find(|m| !overlaps(&code, &m.span));
    let (pos, new_text) = match marker.as_ref().and_then(|m| m.entries.last()) {
        Some((_, last)) => (last.end, format!(", {word}")),
        None => {
            let content = text.get(line_start..line_end)?.trim_end();
            (
                line_start + content.len(),
                format!(" <!-- requimd: allow {word} -->"),
            )
        }
    };
    let pos = ctx.index(file)?.position(pos);
    Some(single_edit(
        ctx.uri(file)?,
        TextEdit {
            range: lsp_types::Range {
                start: pos,
                end: pos,
            },
            new_text,
        },
    ))
}

/// The settings of a check: `[checks.<check>]`, created if missing. A
/// severity string `check = "info"` becomes the inline table
/// `{ sev = "info" }` in place, so comments around it stay.
fn check_table<'a>(
    doc: &'a mut toml_edit::DocumentMut,
    check: &str,
) -> Result<&'a mut dyn toml_edit::TableLike, String> {
    let root = doc.as_table_mut();
    if !root.contains_key("checks") {
        let mut t = toml_edit::Table::new();
        t.set_implicit(true);
        root.insert("checks", toml_edit::Item::Table(t));
    }
    let checks = root
        .get_mut("checks")
        .and_then(toml_edit::Item::as_table_mut)
        .ok_or("[checks] is not a table")?;
    if !checks.contains_key(check) {
        checks.insert(check, toml_edit::Item::Table(toml_edit::Table::new()));
    }
    let item = checks
        .get_mut(check)
        .ok_or_else(|| format!("[checks.{check}] is missing"))?;
    if let Some(sev) = item.as_str().map(str::to_string) {
        let mut t = toml_edit::InlineTable::new();
        t.insert("sev", sev.into());
        *item = toml_edit::value(toml_edit::Value::InlineTable(t).decorated(" ", ""));
    }
    item.as_table_like_mut()
        .ok_or_else(|| format!("[checks.{check}] is not a table"))
}

fn array<'a>(
    t: &'a mut dyn toml_edit::TableLike,
    key: &str,
) -> Result<&'a mut toml_edit::Array, String> {
    if !t.contains_key(key) {
        t.insert(key, toml_edit::value(toml_edit::Array::new()));
    }
    t.get_mut(key)
        .and_then(toml_edit::Item::as_array_mut)
        .ok_or_else(|| format!("\"{key}\" is not a list"))
}

/// FR-219 "Allow in project": adds `word` to `allow` of the check.
pub fn add_allow(config: &str, check: &str, word: &str) -> Result<String, String> {
    let mut doc: toml_edit::DocumentMut = config
        .parse()
        .map_err(|e: toml_edit::TomlError| e.to_string())?;
    let allow = array(check_table(&mut doc, check)?, "allow")?;
    if !allow
        .iter()
        .any(|v| v.as_str().is_some_and(|s| s.eq_ignore_ascii_case(word)))
    {
        allow.push(word);
    }
    Ok(doc.to_string())
}

/// FR-219 "Add to weak words": adds `{ w = word, why = "" }` and returns
/// the new text and the byte offset inside the empty reason.
pub fn add_weak_word(config: &str, word: &str) -> Result<(String, usize), String> {
    let mut doc: toml_edit::DocumentMut = config
        .parse()
        .map_err(|e: toml_edit::TomlError| e.to_string())?;
    let words = array(check_table(&mut doc, "weak-word")?, "words")?;
    let mut entry = toml_edit::InlineTable::new();
    entry.insert("w", word.into());
    entry.insert("why", "".into());
    words.push(entry);
    let text = doc.to_string();
    let w = format!("w = {}", toml_edit::Value::from(word).to_string().trim());
    let at = text
        .rfind(&w)
        .and_then(|i| {
            text.get(i..)?
                .find("why = \"\"")
                .map(|j| i + j + "why = \"".len())
        })
        .unwrap_or(0);
    Ok((text, at))
}

#[cfg(test)]
mod tests {
    use requimd_core::{Config, SourceFile};

    use super::*;
    use crate::convert::Encoding;

    fn project(config: &str, files: &[(&str, &str)]) -> Project {
        let files = files
            .iter()
            .map(|(n, t)| {
                SourceFile::new(std::env::temp_dir().join(n), n.to_string(), t.to_string())
            })
            .collect();
        let config = Config::from_toml(config, None).unwrap();
        Project::from_files(std::env::temp_dir(), config, files)
    }

    fn titles(a: &[CodeActionOrCommand]) -> Vec<String> {
        a.iter()
            .map(|a| match a {
                CodeActionOrCommand::CodeAction(c) => c.title.clone(),
                CodeActionOrCommand::Command(c) => c.title.clone(),
            })
            .collect()
    }

    #[allow(clippy::mutable_key_type)]
    fn first_edit(a: &CodeActionOrCommand) -> TextEdit {
        let CodeActionOrCommand::CodeAction(c) = a else {
            panic!()
        };
        let edit = c.edit.as_ref().unwrap();
        if let Some(changes) = &edit.changes {
            return changes.values().next().unwrap()[0].clone();
        }
        let Some(DocumentChanges::Operations(ops)) = &edit.document_changes else {
            panic!()
        };
        let DocumentChangeOperation::Edit(e) = &ops[1] else {
            panic!()
        };
        let OneOf::Left(t) = &e.edits[0] else {
            panic!()
        };
        t.clone()
    }

    fn actions(p: &Project, file: usize, needle: &str) -> Vec<CodeActionOrCommand> {
        let uris = HashMap::new();
        let ctx = Ctx {
            project: p,
            encoding: Encoding::Utf8,
            uris: &uris,
        };
        let off = p.files[file].text.find(needle).unwrap() + 1;
        code_actions(&ctx, file, off..off, &requimd_checks::check(p))
    }

    #[test]
    fn fr_213_stub_for_dangling_reference() {
        let p = project(
            "",
            &[(
                "a.md",
                "## FR-001 A\nref: FR-999\n\nThe system shall do it.\n",
            )],
        );
        let a = actions(&p, 0, "FR-999");
        assert_eq!(titles(&a)[0], "Create stub FR-999");
        let e = first_edit(&a[0]);
        assert_eq!(e.new_text, "\n## FR-999\n");
        assert_eq!(e.range.start, lsp_types::Position::new(4, 0));
    }

    #[test]
    fn fr_213_missing_step_is_added_to_use_case() {
        let p = project(
            "",
            &[(
                "a.md",
                "## UC-001 U\n\n- s1: Go.\n- s2: Pay.\n\n## FR-001 A\nimpl: UC-001.s9\n\nThe system shall.\n",
            )],
        );
        let a = actions(&p, 0, "UC-001.s9");
        assert_eq!(titles(&a)[0], "Add step s9 to UC-001");
        let e = first_edit(&a[0]);
        assert_eq!(e.new_text, "\n- s9: ");
        assert_eq!(e.range.start, lsp_types::Position::new(3, 10));
    }

    #[test]
    fn fr_213_requirement_stub_for_uncovered_step_in_stub_file() {
        let p = project(
            "[lsp]\nstub-file = \"inbox.md\"\n",
            &[
                ("a.md", "## UC-001 U\n\n- s1: Customer pays.\n"),
                ("inbox.md", "# Inbox\n"),
            ],
        );
        let a = actions(&p, 0, "s1:");
        assert_eq!(titles(&a)[0], "Create requirement for UC-001.s1");
        let e = first_edit(&a[0]);
        assert_eq!(e.new_text, "\n## FR-? Customer pays.\nimpl: UC-001.s1\n");
        assert_eq!(e.range.start, lsp_types::Position::new(1, 0));
        let CodeActionOrCommand::CodeAction(c) = &a[0] else {
            panic!()
        };
        let uri = c
            .edit
            .as_ref()
            .unwrap()
            .changes
            .as_ref()
            .unwrap()
            .keys()
            .next()
            .unwrap()
            .clone();
        assert!(uri.as_str().ends_with("inbox.md"));
    }

    #[test]
    fn fr_213_stub_file_is_created_if_missing() {
        let p = project(
            "[lsp]\nstub-file = \"no-such-inbox-requimd-test.md\"\n",
            &[("a.md", "## FR-001 A\nref: FR-999\n\nThe system shall.\n")],
        );
        let a = actions(&p, 0, "FR-999");
        let CodeActionOrCommand::CodeAction(c) = &a[0] else {
            panic!()
        };
        let Some(DocumentChanges::Operations(ops)) = &c.edit.as_ref().unwrap().document_changes
        else {
            panic!()
        };
        assert!(matches!(
            ops[0],
            DocumentChangeOperation::Op(ResourceOp::Create(_))
        ));
        assert_eq!(first_edit(&a[0]).new_text, "## FR-999\n");
    }

    #[test]
    fn fr_219_allow_here_adds_or_extends_marker() {
        let p = project(
            "",
            &[(
                "a.md",
                "## FR-001 A\n\nThe system shall be fast.\nIt shall be easy. <!-- requimd: allow quick -->\n",
            )],
        );
        let a = actions(&p, 0, "fast");
        let t = titles(&a);
        assert_eq!(t[0], "Allow \"fast\" on this line");
        assert_eq!(t[1], "Allow \"fast\" in project (weak-word)");
        let e = first_edit(&a[0]);
        assert_eq!(e.new_text, " <!-- requimd: allow fast -->");
        assert_eq!(e.range.start, lsp_types::Position::new(2, 25));
        let a = actions(&p, 0, "easy");
        let e = first_edit(&a[0]);
        assert_eq!(e.new_text, ", easy");
        assert_eq!(e.range.start, lsp_types::Position::new(3, 43));
    }

    #[test]
    fn fr_219_add_to_weak_words_offered_on_plain_words() {
        let p = project(
            "",
            &[("a.md", "## FR-001 A\n\nThe system shall be seamless.\n")],
        );
        let a = actions(&p, 0, "seamless");
        assert_eq!(titles(&a), ["Add \"seamless\" to weak words"]);
        // Not on a known weak word.
        let p = project(
            "",
            &[("a.md", "## FR-001 A\n\nThe system shall be fast.\n")],
        );
        assert!(
            !titles(&actions(&p, 0, "fast"))
                .iter()
                .any(|t| t.starts_with("Add"))
        );
    }

    #[test]
    fn fr_219_toml_edits_keep_comments() {
        let config = "# my config\n[checks]\n# keep me\nweak-word = \"info\"\n";
        let out = add_allow(config, "weak-word", "fast").unwrap();
        assert!(
            out.contains("# my config") && out.contains("# keep me"),
            "{out}"
        );
        let c = Config::from_toml(&out, None).unwrap();
        assert_eq!(c.allow("weak-word"), ["fast"]);
        assert_eq!(
            c.check_level("weak-word"),
            Some(requimd_core::Level::On(requimd_core::Severity::Info))
        );
        // Adding twice keeps one entry.
        let again = add_allow(&out, "weak-word", "FAST").unwrap();
        assert_eq!(
            Config::from_toml(&again, None).unwrap().allow("weak-word"),
            ["fast"]
        );

        let (out, at) = add_weak_word("[project]\ninclude = [\"*.md\"]\n", "seamless").unwrap();
        let c = Config::from_toml(&out, None).unwrap();
        assert!(
            c.weak_words()
                .iter()
                .any(|w| w.word == "seamless" && w.why.is_empty())
        );
        assert_eq!(&out[at - 1..at + 1], "\"\"", "{out}");
        assert!(out.contains("include = [\"*.md\"]"));
    }
}
