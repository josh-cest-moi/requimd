//! The server loop: documents, project state, diagnostics and requests.
//!
//! Synchronous by design (ADR-0003): one thread receives messages, keeps
//! the project up to date and answers requests. Diagnostics are published
//! after a short debounce (NFR-001).

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

use crossbeam_channel::RecvTimeoutError;
use lsp_server::{Connection, ErrorCode, Message, Notification, Request, RequestId, Response};
use lsp_types::{
    ClientCapabilities, CodeActionParams, CodeActionProviderCapability, CompletionOptions,
    CompletionParams, CompletionResponse, CompletionTriggerKind, DiagnosticSeverity,
    DidChangeTextDocumentParams, DidChangeWatchedFilesParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, DocumentSymbolParams, DocumentSymbolResponse, FileChangeType,
    GotoDefinitionParams, GotoDefinitionResponse, HoverProviderCapability,
    ImplementationProviderCapability, InitializeParams, MessageType, NumberOrString, OneOf,
    PositionEncodingKind, PublishDiagnosticsParams, ReferenceParams, Registration,
    RegistrationParams, ServerCapabilities, ShowMessageParams, TextDocumentSyncCapability,
    TextDocumentSyncKind, Uri, WorkspaceSymbolParams, WorkspaceSymbolResponse,
};
use lsp_types::{
    CodeLensOptions, CodeLensParams, ExecuteCommandOptions, ExecuteCommandParams, InlayHintParams,
    Location, RenameOptions, RenameParams, ShowDocumentParams, TextDocumentPositionParams,
    WorkDoneProgressOptions,
};
use requimd_core::config::CONFIG_FILE;
use requimd_core::parse::{Parsed, parse};
use requimd_core::{
    Config, FileId, Includes, Project, Severity, SourceFile, find_root, glossary_rel,
    project_files, read_text, relative_path,
};
use serde_json::Value;

use crate::actions;
use crate::convert::{Encoding, LineIndex, path_to_uri, uri_to_path};
use crate::features::{self, Ctx};

/// Wait after the last change before diagnostics are published.
const DEBOUNCE: Duration = Duration::from_millis(30);
/// Interval for checking files on disk if the client cannot watch them.
const POLL: Duration = Duration::from_secs(2);

pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// FR-200: runs the server on a connection until shutdown.
pub fn run(conn: &Connection) -> Result<(), Error> {
    let (id, params) = conn.initialize_start()?;
    let init: InitializeParams = serde_json::from_value(params)?;
    let encoding = negotiate_encoding(&init.capabilities);
    let result = serde_json::json!({
        "capabilities": capabilities(encoding),
        "serverInfo": { "name": "requimd", "version": env!("CARGO_PKG_VERSION") },
    });
    conn.initialize_finish(id, result)?;
    let mut server = Server::new(conn, encoding, &init);
    server.main_loop()
}

fn negotiate_encoding(caps: &ClientCapabilities) -> Encoding {
    let utf8 = caps
        .general
        .as_ref()
        .and_then(|g| g.position_encodings.as_ref())
        .is_some_and(|e| e.contains(&PositionEncodingKind::UTF8));
    if utf8 {
        Encoding::Utf8
    } else {
        Encoding::Utf16
    }
}

fn capabilities(encoding: Encoding) -> ServerCapabilities {
    ServerCapabilities {
        position_encoding: Some(match encoding {
            Encoding::Utf8 => PositionEncodingKind::UTF8,
            Encoding::Utf16 => PositionEncodingKind::UTF16,
        }),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(["-", ".", " ", ",", ":"].map(String::from).to_vec()),
            ..CompletionOptions::default()
        }),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        definition_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        implementation_provider: Some(ImplementationProviderCapability::Simple(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        workspace_symbol_provider: Some(OneOf::Left(true)),
        code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
        inlay_hint_provider: Some(OneOf::Left(true)),
        code_lens_provider: Some(CodeLensOptions {
            resolve_provider: Some(false),
        }),
        rename_provider: Some(OneOf::Right(RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: WorkDoneProgressOptions::default(),
        })),
        execute_command_provider: Some(ExecuteCommandOptions {
            commands: [
                features::IMPLEMENTATIONS_COMMAND,
                actions::ALLOW_IN_PROJECT,
                actions::ADD_WEAK_WORD,
            ]
            .map(String::from)
            .to_vec(),
            work_done_progress_options: WorkDoneProgressOptions::default(),
        }),
        ..ServerCapabilities::default()
    }
}

struct DiskFile {
    path: PathBuf,
    text: String,
    modified: Option<SystemTime>,
}

struct OpenDoc {
    uri: Uri,
    text: String,
    version: i32,
}

struct Server<'a> {
    conn: &'a Connection,
    encoding: Encoding,
    root: Option<PathBuf>,
    config: Config,
    includes: Option<Includes>,
    config_modified: Option<SystemTime>,
    /// Included files on disk by relative path.
    disk: BTreeMap<String, DiskFile>,
    /// Open, included documents by relative path.
    open: BTreeMap<String, OpenDoc>,
    project: Project,
    file_ids: HashMap<String, FileId>,
    uris: HashMap<FileId, Uri>,
    published: HashMap<String, Vec<lsp_types::Diagnostic>>,
    /// Text and parse result per file of the last rebuild.
    parse_cache: HashMap<String, (String, Parsed)>,
    dirty_since: Option<Instant>,
    client_watches: bool,
    next_poll: Instant,
}

fn modified(path: &Path) -> Option<SystemTime> {
    std::fs::metadata(path).and_then(|m| m.modified()).ok()
}

impl<'a> Server<'a> {
    fn new(conn: &'a Connection, encoding: Encoding, init: &InitializeParams) -> Server<'a> {
        let config = Config::defaults().unwrap_or_else(|_| empty_config());
        let client_watches = init
            .capabilities
            .workspace
            .as_ref()
            .and_then(|w| w.did_change_watched_files.as_ref())
            .and_then(|d| d.dynamic_registration)
            .unwrap_or(false);
        let mut server = Server {
            conn,
            encoding,
            root: None,
            project: Project::from_files(PathBuf::new(), config.clone(), Vec::new()),
            config,
            includes: None,
            config_modified: None,
            disk: BTreeMap::new(),
            open: BTreeMap::new(),
            file_ids: HashMap::new(),
            uris: HashMap::new(),
            published: HashMap::new(),
            parse_cache: HashMap::new(),
            dirty_since: None,
            client_watches,
            next_poll: Instant::now() + POLL,
        };
        if let Some(root) = workspace_root(init).and_then(|p| find_root(&p)) {
            server.set_root(root);
        }
        server
    }

    fn set_root(&mut self, root: PathBuf) {
        self.root = Some(root);
        self.load_config();
        if self.client_watches {
            self.register_watchers();
        }
    }

    /// Reads `requimd.toml` and all included files (FR-013, FR-217).
    fn load_config(&mut self) {
        let Some(root) = self.root.clone() else {
            return;
        };
        self.config_modified = modified(&root.join(CONFIG_FILE));
        match Config::load(&root) {
            Ok(config) => match Includes::new(&config.include) {
                Ok(inc) => {
                    self.config = config;
                    // Parsing depends on the configuration.
                    self.parse_cache.clear();
                    self.includes = Some(inc);
                }
                Err(e) => self.show_error(&e),
            },
            Err(e) => {
                // Keep the last valid configuration.
                self.show_error(&e.to_string());
                if self.includes.is_none() {
                    return;
                }
            }
        }
        self.rescan();
        // Open documents may no longer be included, or now be.
        let open: Vec<String> = self.open.keys().cloned().collect();
        for rel in open {
            if !self.is_included(&rel) {
                self.close(&rel);
            }
        }
        self.mark_dirty();
    }

    /// FR-201: included files and the glossary (FR-011) are processed.
    fn is_included(&self, rel: &str) -> bool {
        self.includes.as_ref().is_some_and(|i| i.matches(rel))
            || glossary_rel(&self.config).as_deref() == Some(rel)
    }

    /// Re-reads changed files on disk; returns whether anything changed.
    fn rescan(&mut self) -> bool {
        let (Some(root), Some(inc)) = (&self.root, &self.includes) else {
            return false;
        };
        let found = match project_files(root, &self.config, inc) {
            Ok(f) => f,
            Err(e) => {
                self.log(&e.to_string());
                return false;
            }
        };
        let mut changed = false;
        let mut seen = BTreeMap::new();
        for (rel, path) in found {
            let m = modified(&path);
            match self.disk.remove(&rel) {
                Some(f) if f.modified == m && m.is_some() => {
                    seen.insert(rel, f);
                }
                _ => {
                    changed = true;
                    if let Ok(text) = read_text(&path) {
                        seen.insert(
                            rel,
                            DiskFile {
                                path,
                                text,
                                modified: m,
                            },
                        );
                    }
                }
            }
        }
        changed |= !self.disk.is_empty();
        self.disk = seen;
        changed
    }

    fn register_watchers(&self) {
        let params = RegistrationParams {
            registrations: vec![Registration {
                id: "requimd-watch".into(),
                method: "workspace/didChangeWatchedFiles".into(),
                register_options: Some(serde_json::json!({
                    "watchers": [
                        { "globPattern": "**/*.md" },
                        { "globPattern": format!("**/{CONFIG_FILE}") },
                    ]
                })),
            }],
        };
        let req = Request::new(
            RequestId::from("requimd-watch".to_string()),
            "client/registerCapability".into(),
            params,
        );
        let _ = self.conn.sender.send(req.into());
    }

    fn main_loop(&mut self) -> Result<(), Error> {
        loop {
            let now = Instant::now();
            let mut deadline = now + Duration::from_secs(3600);
            if let Some(since) = self.dirty_since {
                deadline = deadline.min(since + DEBOUNCE);
            }
            if !self.client_watches && self.root.is_some() {
                deadline = deadline.min(self.next_poll);
            }
            match self
                .conn
                .receiver
                .recv_timeout(deadline.saturating_duration_since(now))
            {
                Ok(Message::Request(req)) => {
                    if self.conn.handle_shutdown(&req)? {
                        return Ok(());
                    }
                    self.handle_request(req);
                }
                Ok(Message::Notification(n)) => {
                    if n.method == "exit" {
                        return Ok(());
                    }
                    self.handle_notification(n);
                }
                Ok(Message::Response(_)) => {}
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => return Ok(()),
            }
            let now = Instant::now();
            if self.dirty_since.is_some_and(|s| now >= s + DEBOUNCE) {
                self.refresh();
            }
            if !self.client_watches && self.root.is_some() && now >= self.next_poll {
                self.next_poll = now + POLL;
                self.poll();
            }
        }
    }

    /// FR-217 without client support: compare modification times.
    fn poll(&mut self) {
        let Some(root) = self.root.clone() else {
            return;
        };
        if modified(&root.join(CONFIG_FILE)) != self.config_modified {
            self.load_config();
        } else if self.rescan() {
            self.mark_dirty();
        }
    }

    fn mark_dirty(&mut self) {
        if self.dirty_since.is_none() {
            self.dirty_since = Some(Instant::now());
        }
    }

    /// Rebuilds the project from disk and open documents.
    fn rebuild(&mut self) {
        let root = self.root.clone().unwrap_or_default();
        let mut rels: Vec<&String> = self.disk.keys().chain(self.open.keys()).collect();
        rels.sort();
        rels.dedup();
        let mut files = Vec::new();
        self.file_ids.clear();
        self.uris.clear();
        for rel in rels {
            let (path, text) = match (self.open.get(rel), self.disk.get(rel)) {
                (Some(doc), disk) => (
                    disk.map(|d| d.path.clone())
                        .or_else(|| uri_to_path(&doc.uri))
                        .unwrap_or_default(),
                    doc.text.clone(),
                ),
                (None, Some(d)) => (d.path.clone(), d.text.clone()),
                (None, None) => continue,
            };
            let id = files.len();
            if let Some(doc) = self.open.get(rel) {
                self.uris.insert(id, doc.uri.clone());
            }
            self.file_ids.insert(rel.clone(), id);
            files.push(SourceFile::new(path, rel.clone(), text));
        }
        // NFR-001: only changed files are parsed again.
        let mut parsed = Vec::with_capacity(files.len());
        let mut cache = HashMap::with_capacity(files.len());
        for (id, f) in files.iter().enumerate() {
            let result = match self.parse_cache.remove(&f.rel) {
                Some((text, p)) if text == f.text => p,
                _ => parse(&self.config, id, &f.text),
            };
            parsed.push(result.clone());
            cache.insert(f.rel.clone(), (f.text.clone(), result));
        }
        self.parse_cache = cache;
        self.project = Project::from_parsed(root, self.config.clone(), files, parsed);
    }

    /// FR-202: rebuilds and publishes changed diagnostics of open documents.
    fn refresh(&mut self) {
        self.dirty_since = None;
        self.rebuild();
        let mut by_file: HashMap<FileId, Vec<lsp_types::Diagnostic>> = HashMap::new();
        for d in requimd_checks::check(&self.project) {
            let Some(f) = self.project.files.get(d.file) else {
                continue;
            };
            if !self.open.contains_key(&f.rel) {
                continue;
            }
            let index = LineIndex::new(&f.text, self.encoding);
            by_file
                .entry(d.file)
                .or_default()
                .push(lsp_types::Diagnostic {
                    range: index.range(&d.span),
                    severity: Some(match d.severity {
                        Severity::Error => DiagnosticSeverity::ERROR,
                        Severity::Warning => DiagnosticSeverity::WARNING,
                        Severity::Info => DiagnosticSeverity::INFORMATION,
                    }),
                    code: Some(NumberOrString::String(d.check.to_string())),
                    source: Some("requimd".into()),
                    message: d.message,
                    ..lsp_types::Diagnostic::default()
                });
        }
        let open: Vec<String> = self.open.keys().cloned().collect();
        for rel in open {
            let diags = self
                .file_ids
                .get(&rel)
                .and_then(|id| by_file.remove(id))
                .unwrap_or_default();
            if self.published.get(&rel) == Some(&diags) {
                continue;
            }
            if let Some(doc) = self.open.get(&rel) {
                self.notify(
                    "textDocument/publishDiagnostics",
                    PublishDiagnosticsParams {
                        uri: doc.uri.clone(),
                        diagnostics: diags.clone(),
                        version: Some(doc.version),
                    },
                );
            }
            self.published.insert(rel, diags);
        }
    }

    fn ensure_fresh(&mut self) {
        if self.dirty_since.is_some() {
            self.refresh();
        }
    }

    /// Relative path of an included document (FR-201).
    fn rel_of(&mut self, uri: &Uri) -> Option<String> {
        let path = std::path::absolute(uri_to_path(uri)?).ok()?;
        if self.root.is_none()
            && let Some(root) = find_root(&path)
        {
            self.set_root(root);
        }
        let rel = relative_path(self.root.as_ref()?, &path)?;
        self.is_included(&rel).then_some(rel)
    }

    fn close(&mut self, rel: &str) {
        if let Some(doc) = self.open.remove(rel)
            && self.published.remove(rel).is_some_and(|d| !d.is_empty())
        {
            self.notify(
                "textDocument/publishDiagnostics",
                PublishDiagnosticsParams {
                    uri: doc.uri,
                    diagnostics: Vec::new(),
                    version: None,
                },
            );
        }
    }

    fn handle_notification(&mut self, n: Notification) {
        match n.method.as_str() {
            "textDocument/didOpen" => {
                let Ok(p) = serde_json::from_value::<DidOpenTextDocumentParams>(n.params) else {
                    return;
                };
                let doc = p.text_document;
                if let Some(rel) = self.rel_of(&doc.uri) {
                    self.open.insert(
                        rel,
                        OpenDoc {
                            uri: doc.uri,
                            text: doc.text,
                            version: doc.version,
                        },
                    );
                    // Publish at once on open.
                    self.refresh();
                }
            }
            "textDocument/didChange" => {
                let Ok(p) = serde_json::from_value::<DidChangeTextDocumentParams>(n.params) else {
                    return;
                };
                let Some(rel) = self.rel_of(&p.text_document.uri) else {
                    return;
                };
                // Full sync: the last change holds the whole text.
                if let (Some(doc), Some(change)) = (
                    self.open.get_mut(&rel),
                    p.content_changes.into_iter().last(),
                ) {
                    doc.text = change.text;
                    doc.version = p.text_document.version;
                    self.mark_dirty();
                }
            }
            "textDocument/didClose" => {
                let Ok(p) = serde_json::from_value::<DidCloseTextDocumentParams>(n.params) else {
                    return;
                };
                if let Some(rel) = self.rel_of(&p.text_document.uri) {
                    self.close(&rel);
                    self.mark_dirty();
                }
            }
            "workspace/didChangeWatchedFiles" => {
                let Ok(p) = serde_json::from_value::<DidChangeWatchedFilesParams>(n.params) else {
                    return;
                };
                self.watched_files_changed(p);
            }
            _ => {}
        }
    }

    /// FR-217 with client support.
    fn watched_files_changed(&mut self, p: DidChangeWatchedFilesParams) {
        let Some(root) = self.root.clone() else {
            return;
        };
        let mut config_changed = false;
        for change in p.changes {
            let Some(path) = uri_to_path(&change.uri) else {
                continue;
            };
            if path == root.join(CONFIG_FILE) {
                config_changed = true;
                continue;
            }
            let Some(rel) = relative_path(&root, &path) else {
                continue;
            };
            if !self.is_included(&rel) {
                continue;
            }
            if change.typ == FileChangeType::DELETED {
                self.disk.remove(&rel);
            } else if let Ok(text) = read_text(&path) {
                let modified = modified(&path);
                self.disk.insert(
                    rel,
                    DiskFile {
                        path,
                        text,
                        modified,
                    },
                );
            }
            self.mark_dirty();
        }
        if config_changed {
            self.load_config();
        }
    }

    fn handle_request(&mut self, req: Request) {
        self.ensure_fresh();
        let id = req.id.clone();
        let result = match req.method.as_str() {
            "textDocument/completion" => self.completion(req.params),
            "textDocument/hover" => self.with_position(req.params, |ctx, file, off| {
                serde_json::to_value(features::hover(ctx, file, off))
            }),
            "textDocument/definition" => self.with_position(req.params, |ctx, file, off| {
                serde_json::to_value(
                    features::definition(ctx, file, off).map(GotoDefinitionResponse::Scalar),
                )
            }),
            "textDocument/implementation" => self.with_position(req.params, |ctx, file, off| {
                serde_json::to_value(features::implementations(ctx, file, off))
            }),
            "textDocument/references" => self.references(req.params),
            "textDocument/documentSymbol" => self.document_symbols(req.params),
            "workspace/symbol" => self.workspace_symbols(req.params),
            "textDocument/codeAction" => self.code_actions(req.params),
            "textDocument/inlayHint" => self.inlay_hints(req.params),
            "textDocument/codeLens" => self.code_lenses(req.params),
            "textDocument/prepareRename" => self.prepare_rename(req.params),
            "textDocument/rename" => self.rename(req.params),
            "workspace/executeCommand" => self.execute_command(req.params),
            _ => {
                let resp = Response::new_err(
                    id,
                    ErrorCode::MethodNotFound as i32,
                    format!("unsupported method {}", req.method),
                );
                let _ = self.conn.sender.send(resp.into());
                return;
            }
        };
        let resp = match result {
            Ok(v) => Response::new_ok(id, v),
            // A refused rename or failed command is shown to the user.
            Err(e) => Response::new_err(id, ErrorCode::RequestFailed as i32, e),
        };
        let _ = self.conn.sender.send(resp.into());
    }

    /// FR-209
    fn inlay_hints(&mut self, params: Value) -> Result<Value, String> {
        let p: InlayHintParams = serde_json::from_value(params).map_err(|e| e.to_string())?;
        let uri = p.text_document.uri;
        let (Some((file, start)), Some((_, end))) = (
            self.locate(&uri, p.range.start),
            self.locate(&uri, p.range.end),
        ) else {
            return Ok(Value::Null);
        };
        let hints = features::inlay_hints(&self.ctx(), file, start..end);
        serde_json::to_value(hints).map_err(|e| e.to_string())
    }

    /// FR-215
    fn code_lenses(&mut self, params: Value) -> Result<Value, String> {
        let p: CodeLensParams = serde_json::from_value(params).map_err(|e| e.to_string())?;
        let Some(rel) = self.rel_of(&p.text_document.uri) else {
            return Ok(Value::Null);
        };
        let Some(&file) = self.file_ids.get(&rel) else {
            return Ok(Value::Null);
        };
        serde_json::to_value(features::code_lenses(&self.ctx(), file)).map_err(|e| e.to_string())
    }

    /// FR-214
    fn prepare_rename(&mut self, params: Value) -> Result<Value, String> {
        let p: TextDocumentPositionParams =
            serde_json::from_value(params).map_err(|e| e.to_string())?;
        let Some((file, off)) = self.locate(&p.text_document.uri, p.position) else {
            return Ok(Value::Null);
        };
        match features::prepare_rename(&self.ctx(), file, off) {
            Some((range, placeholder)) => Ok(serde_json::json!({
                "range": range, "placeholder": placeholder,
            })),
            None => Err("no item ID to rename here".into()),
        }
    }

    /// FR-214
    fn rename(&mut self, params: Value) -> Result<Value, String> {
        let p: RenameParams = serde_json::from_value(params).map_err(|e| e.to_string())?;
        let tdp = p.text_document_position;
        let Some((file, off)) = self.locate(&tdp.text_document.uri, tdp.position) else {
            return Err("not a file of the requimd project".into());
        };
        let edit = features::rename(&self.ctx(), file, off, &p.new_name)?;
        serde_json::to_value(edit).map_err(|e| e.to_string())
    }

    /// FR-215 and FR-219 commands.
    fn execute_command(&mut self, params: Value) -> Result<Value, String> {
        let p: ExecuteCommandParams = serde_json::from_value(params).map_err(|e| e.to_string())?;
        let arg = |i: usize| {
            p.arguments
                .get(i)
                .and_then(Value::as_str)
                .map(str::to_string)
        };
        match p.command.as_str() {
            features::IMPLEMENTATIONS_COMMAND => {
                // Without a client-side handler: open the first one.
                let locations: Vec<Location> = p
                    .arguments
                    .first()
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .unwrap_or_default();
                if let Some(loc) = locations.first() {
                    self.show_document(loc.uri.clone(), Some(loc.range));
                }
            }
            actions::ALLOW_IN_PROJECT => {
                let (Some(check), Some(word)) = (arg(0), arg(1)) else {
                    return Err("expected check and word".into());
                };
                self.edit_config(|text| {
                    actions::add_allow(text, &check, &word).map(|t| (t, None))
                })?;
            }
            actions::ADD_WEAK_WORD => {
                let Some(word) = arg(0) else {
                    return Err("expected a word".into());
                };
                self.edit_config(|text| {
                    actions::add_weak_word(text, &word).map(|(t, at)| (t, Some(at)))
                })?;
            }
            other => return Err(format!("unknown command {other}")),
        }
        Ok(Value::Null)
    }

    /// FR-219: changes `requimd.toml` on disk, reloads it and, if `edit`
    /// returns an offset, opens the file there.
    fn edit_config(
        &mut self,
        edit: impl FnOnce(&str) -> Result<(String, Option<usize>), String>,
    ) -> Result<(), String> {
        let root = self.root.clone().ok_or("no requimd project")?;
        let path = root.join(CONFIG_FILE);
        let text = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let (new, at) = edit(&text)?;
        Config::from_toml(&new, Some(&path)).map_err(|e| e.to_string())?;
        std::fs::write(&path, &new).map_err(|e| e.to_string())?;
        self.load_config();
        self.refresh();
        if let (Some(at), Some(uri)) = (at, path_to_uri(&path)) {
            let pos = LineIndex::new(&new, self.encoding).position(at);
            self.show_document(
                uri,
                Some(lsp_types::Range {
                    start: pos,
                    end: pos,
                }),
            );
        }
        Ok(())
    }

    fn show_document(&mut self, uri: Uri, selection: Option<lsp_types::Range>) {
        let params = ShowDocumentParams {
            uri,
            external: Some(false),
            take_focus: Some(true),
            selection,
        };
        let req = Request::new(
            RequestId::from("requimd-show".to_string()),
            "window/showDocument".into(),
            params,
        );
        let _ = self.conn.sender.send(req.into());
    }

    fn ctx(&self) -> Ctx<'_> {
        Ctx {
            project: &self.project,
            encoding: self.encoding,
            uris: &self.uris,
        }
    }

    /// File ID and byte offset of a document position.
    fn locate(&mut self, uri: &Uri, pos: lsp_types::Position) -> Option<(FileId, usize)> {
        let rel = self.rel_of(uri)?;
        let id = *self.file_ids.get(&rel)?;
        let f = self.project.files.get(id)?;
        Some((id, LineIndex::new(&f.text, self.encoding).offset(pos)))
    }

    fn with_position(
        &mut self,
        params: Value,
        f: impl Fn(&Ctx, FileId, usize) -> serde_json::Result<Value>,
    ) -> Result<Value, String> {
        let p: GotoDefinitionParams = serde_json::from_value(params).map_err(|e| e.to_string())?;
        let tdp = p.text_document_position_params;
        let Some((file, off)) = self.locate(&tdp.text_document.uri, tdp.position) else {
            return Ok(Value::Null);
        };
        f(&self.ctx(), file, off).map_err(|e| e.to_string())
    }

    fn completion(&mut self, params: Value) -> Result<Value, String> {
        let p: CompletionParams = serde_json::from_value(params).map_err(|e| e.to_string())?;
        let tdp = p.text_document_position;
        let Some((file, off)) = self.locate(&tdp.text_document.uri, tdp.position) else {
            return Ok(Value::Null);
        };
        let trigger = p.context.as_ref().and_then(|c| c.trigger_character.clone());
        let invoked = p
            .context
            .as_ref()
            .is_none_or(|c| c.trigger_kind == CompletionTriggerKind::INVOKED);
        let list = features::completion(&self.ctx(), file, off, trigger.as_deref(), invoked);
        serde_json::to_value(CompletionResponse::List(list)).map_err(|e| e.to_string())
    }

    fn references(&mut self, params: Value) -> Result<Value, String> {
        let p: ReferenceParams = serde_json::from_value(params).map_err(|e| e.to_string())?;
        let tdp = p.text_document_position;
        let Some((file, off)) = self.locate(&tdp.text_document.uri, tdp.position) else {
            return Ok(Value::Null);
        };
        let locs = features::references(&self.ctx(), file, off, p.context.include_declaration);
        serde_json::to_value(locs).map_err(|e| e.to_string())
    }

    fn document_symbols(&mut self, params: Value) -> Result<Value, String> {
        let p: DocumentSymbolParams = serde_json::from_value(params).map_err(|e| e.to_string())?;
        let Some(rel) = self.rel_of(&p.text_document.uri) else {
            return Ok(Value::Null);
        };
        let Some(&file) = self.file_ids.get(&rel) else {
            return Ok(Value::Null);
        };
        let symbols = features::document_symbols(&self.ctx(), file);
        serde_json::to_value(DocumentSymbolResponse::Nested(symbols)).map_err(|e| e.to_string())
    }

    fn workspace_symbols(&mut self, params: Value) -> Result<Value, String> {
        let p: WorkspaceSymbolParams = serde_json::from_value(params).map_err(|e| e.to_string())?;
        let symbols = features::workspace_symbols(&self.ctx(), &p.query);
        serde_json::to_value(WorkspaceSymbolResponse::Nested(symbols)).map_err(|e| e.to_string())
    }

    fn code_actions(&mut self, params: Value) -> Result<Value, String> {
        let p: CodeActionParams = serde_json::from_value(params).map_err(|e| e.to_string())?;
        let Some((file, start)) = self.locate(&p.text_document.uri, p.range.start) else {
            return Ok(Value::Null);
        };
        let Some((_, end)) = self.locate(&p.text_document.uri, p.range.end) else {
            return Ok(Value::Null);
        };
        let diags = requimd_checks::check(&self.project);
        let list = actions::code_actions(&self.ctx(), file, start..end.max(start), &diags);
        serde_json::to_value(list).map_err(|e| e.to_string())
    }

    fn notify(&self, method: &str, params: impl serde::Serialize) {
        let n = Notification::new(method.to_string(), params);
        let _ = self.conn.sender.send(n.into());
    }

    fn show_error(&self, message: &str) {
        self.notify(
            "window/showMessage",
            ShowMessageParams {
                typ: MessageType::ERROR,
                message: format!("requimd: {message}"),
            },
        );
    }

    fn log(&self, message: &str) {
        self.notify(
            "window/logMessage",
            ShowMessageParams {
                typ: MessageType::WARNING,
                message: format!("requimd: {message}"),
            },
        );
    }
}

/// The first workspace folder, or the root URI.
fn workspace_root(init: &InitializeParams) -> Option<PathBuf> {
    if let Some(folder) = init.workspace_folders.as_ref().and_then(|f| f.first()) {
        return uri_to_path(&folder.uri);
    }
    #[allow(deprecated)] // Older clients only send `rootUri`.
    init.root_uri.as_ref().and_then(uri_to_path)
}

/// Fallback if the built-in defaults fail to parse, which tests prevent.
fn empty_config() -> Config {
    Config {
        include: Vec::new(),
        glossary: None,
        digits: 3,
        types: Vec::new(),
        links: Vec::new(),
        common_meta: Vec::new(),
        values: BTreeMap::new(),
        defaults: BTreeMap::new(),
        checks: BTreeMap::new(),
        hover_length: 500,
        hint_length: 40,
        stub_file: None,
    }
}
