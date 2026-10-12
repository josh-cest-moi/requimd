//! End-to-end tests over an in-memory connection.
#![allow(clippy::unwrap_used, clippy::panic)]

use std::fs;
use std::path::{Path, PathBuf};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use lsp_server::{Connection, Message, Notification, Request, RequestId};
use requimd_lsp::convert::path_to_uri;
use serde_json::{Value, json};

struct Client {
    conn: Connection,
    server: Option<JoinHandle<()>>,
    next_id: i32,
    /// Notifications received while waiting for responses.
    inbox: Vec<Notification>,
    /// Requests from the server, e.g. window/showDocument.
    server_requests: Vec<Request>,
}

impl Client {
    fn start(root: &Path, capabilities: Value) -> (Client, Value) {
        let (server_conn, conn) = Connection::memory();
        let server = std::thread::spawn(move || {
            requimd_lsp::run(&server_conn).unwrap();
        });
        let mut c = Client {
            conn,
            server: Some(server),
            next_id: 0,
            inbox: Vec::new(),
            server_requests: Vec::new(),
        };
        let init = c.request(
            "initialize",
            json!({
                "processId": null,
                "rootUri": uri(root),
                "capabilities": capabilities,
            }),
        );
        c.notify("initialized", json!({}));
        (c, init)
    }

    fn request(&mut self, method: &str, params: Value) -> Value {
        self.next_id += 1;
        let id = RequestId::from(self.next_id);
        let req = Request::new(id.clone(), method.into(), params);
        self.conn.sender.send(req.into()).unwrap();
        loop {
            match self
                .conn
                .receiver
                .recv_timeout(Duration::from_secs(10))
                .unwrap()
            {
                Message::Response(r) if r.id == id => match r.response_result {
                    Ok(v) => return v,
                    Err(e) => panic!("{method}: {e:?}"),
                },
                Message::Notification(n) => self.inbox.push(n),
                Message::Request(r) => self.server_requests.push(r),
                Message::Response(_) => {}
            }
        }
    }

    fn notify(&self, method: &str, params: Value) {
        let n = Notification::new(method.into(), params);
        self.conn.sender.send(n.into()).unwrap();
    }

    /// Waits for the next diagnostics of `uri`, also from the inbox.
    fn diagnostics(&mut self, uri: &str, timeout: Duration) -> Option<Vec<Value>> {
        let is_for = |n: &Notification| {
            n.method == "textDocument/publishDiagnostics" && n.params["uri"] == uri
        };
        if let Some(i) = self.inbox.iter().position(is_for) {
            let n = self.inbox.remove(i);
            return Some(n.params["diagnostics"].as_array().unwrap().clone());
        }
        let deadline = Instant::now() + timeout;
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            match self.conn.receiver.recv_timeout(left) {
                Ok(Message::Notification(n)) if is_for(&n) => {
                    return Some(n.params["diagnostics"].as_array().unwrap().clone());
                }
                Ok(Message::Notification(n)) => self.inbox.push(n),
                Ok(_) => {}
                Err(_) => return None,
            }
        }
    }

    fn open(&self, path: &Path, text: &str) {
        self.notify(
            "textDocument/didOpen",
            json!({ "textDocument": {
                "uri": uri(path), "languageId": "markdown", "version": 1, "text": text,
            }}),
        );
    }

    fn change(&self, path: &Path, version: i32, text: &str) {
        self.notify(
            "textDocument/didChange",
            json!({
                "textDocument": { "uri": uri(path), "version": version },
                "contentChanges": [{ "text": text }],
            }),
        );
    }

    fn shutdown(mut self) {
        self.request("shutdown", Value::Null);
        self.notify("exit", Value::Null);
        if let Some(s) = self.server.take() {
            s.join().unwrap();
        }
    }
}

fn uri(path: &Path) -> String {
    path_to_uri(path).unwrap().as_str().to_string()
}

fn temp_project(name: &str, config: &str, files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("requimd-lsp-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let dir = std::path::absolute(dir).unwrap();
    fs::write(dir.join("requimd.toml"), config).unwrap();
    for (n, t) in files {
        let p = dir.join(n);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, t).unwrap();
    }
    dir
}

fn checks(diags: &[Value]) -> Vec<String> {
    diags
        .iter()
        .map(|d| d["code"].as_str().unwrap().to_string())
        .collect()
}

const SECOND: Duration = Duration::from_secs(1);

#[test]
fn fr_200_initialize_capabilities_and_shutdown() {
    let dir = temp_project("init", "", &[]);
    let caps = json!({ "general": { "positionEncodings": ["utf-8", "utf-16"] } });
    let (c, init) = Client::start(&dir, caps);
    let caps = &init["capabilities"];
    assert_eq!(caps["positionEncoding"], "utf-8");
    for key in [
        "hoverProvider",
        "definitionProvider",
        "referencesProvider",
        "implementationProvider",
        "documentSymbolProvider",
        "workspaceSymbolProvider",
        "codeActionProvider",
        "completionProvider",
    ] {
        assert!(!caps[key].is_null(), "missing {key}");
    }
    assert_eq!(init["serverInfo"]["name"], "requimd");
    c.shutdown();

    let (c, init) = Client::start(&dir, json!({}));
    assert_eq!(init["capabilities"]["positionEncoding"], "utf-16");
    c.shutdown();
}

#[test]
fn fr_201_files_outside_include_patterns_get_nothing() {
    let dir = temp_project(
        "scope",
        "[project]\ninclude = [\"req/*.md\"]\n",
        &[
            ("req/a.md", "## FR-? A\n\nIt shall.\n"),
            ("notes.md", "## FR-? B\n"),
        ],
    );
    let (mut c, _) = Client::start(&dir, json!({}));
    c.open(&dir.join("notes.md"), "## FR-? B\n\nFR-1\n");
    c.open(&dir.join("req/a.md"), "## FR-? A\n\nIt shall.\n");
    let d = c.diagnostics(&uri(&dir.join("req/a.md")), SECOND).unwrap();
    assert_eq!(checks(&d), ["placeholder"]);
    assert!(
        c.diagnostics(&uri(&dir.join("notes.md")), SECOND / 2)
            .is_none()
    );
    let r = c.request(
        "textDocument/completion",
        json!({ "textDocument": { "uri": uri(&dir.join("notes.md")) }, "position": { "line": 2, "character": 4 } }),
    );
    assert!(r.is_null());
    c.shutdown();
}

#[test]
fn fr_202_diagnostics_on_open_change_and_in_other_documents() {
    let uc = "## UC-001 U\n\n- s1: Go.\n";
    let fr = "## FR-001 F\n\nIt shall.\n";
    let dir = temp_project("live", "", &[("uc.md", uc), ("fr.md", fr)]);
    let (mut c, _) = Client::start(&dir, json!({}));
    let (uc_path, fr_path) = (dir.join("uc.md"), dir.join("fr.md"));
    c.open(&uc_path, uc);
    let d = c.diagnostics(&uri(&uc_path), SECOND).unwrap();
    assert_eq!(checks(&d), ["uncovered-step"]);
    assert_eq!(d[0]["range"]["start"], json!({ "line": 2, "character": 2 }));
    c.open(&fr_path, fr);
    assert_eq!(
        c.diagnostics(&uri(&fr_path), SECOND).unwrap(),
        Vec::<Value>::new()
    );
    // Editing fr.md (unsaved) covers the step in uc.md.
    c.change(
        &fr_path,
        2,
        "## FR-001 F\nimpl: UC-001.s1  prio: urgent\n\nIt shall.\n",
    );
    let d = c.diagnostics(&uri(&fr_path), SECOND).unwrap();
    assert_eq!(checks(&d), ["invalid-value"]);
    assert_eq!(
        c.diagnostics(&uri(&uc_path), SECOND).unwrap(),
        Vec::<Value>::new()
    );
    c.shutdown();
}

#[test]
fn fr_217_external_changes_by_polling() {
    let uc = "## UC-001 U\n\n- s1: Go.\n";
    let dir = temp_project("poll", "", &[("uc.md", uc)]);
    let (mut c, _) = Client::start(&dir, json!({}));
    let uc_path = dir.join("uc.md");
    c.open(&uc_path, uc);
    assert_eq!(
        checks(&c.diagnostics(&uri(&uc_path), SECOND).unwrap()),
        ["uncovered-step"]
    );
    // As by a git checkout: a new file covers the step.
    fs::write(dir.join("fr.md"), "## FR-001 F\nimpl: UC-001.s1\n").unwrap();
    let d = c.diagnostics(&uri(&uc_path), 5 * SECOND).unwrap();
    assert!(d.is_empty(), "{d:?}");
    // A configuration change is picked up as well.
    fs::write(
        dir.join("requimd.toml"),
        "[checks]\nuncovered-step = \"off\"\n",
    )
    .unwrap();
    fs::remove_file(dir.join("fr.md")).unwrap();
    std::thread::sleep(3 * SECOND);
    let r = c.request("workspace/symbol", json!({ "query": "FR-001" }));
    assert_eq!(r.as_array().unwrap().len(), 0);
    assert!(
        c.diagnostics(&uri(&uc_path), SECOND)
            .is_none_or(|d| d.is_empty())
    );
    c.shutdown();
}

#[test]
fn fr_217_external_changes_reported_by_client() {
    let uc = "## UC-001 U\n\n- s1: Go.\n";
    let dir = temp_project("watch", "", &[("uc.md", uc)]);
    let caps = json!({ "workspace": { "didChangeWatchedFiles": { "dynamicRegistration": true } } });
    let (mut c, _) = Client::start(&dir, caps);
    let uc_path = dir.join("uc.md");
    c.open(&uc_path, uc);
    assert_eq!(
        checks(&c.diagnostics(&uri(&uc_path), SECOND).unwrap()),
        ["uncovered-step"]
    );
    let fr = dir.join("fr.md");
    fs::write(&fr, "## FR-001 F\nimpl: UC-001.s1\n").unwrap();
    c.notify(
        "workspace/didChangeWatchedFiles",
        json!({ "changes": [{ "uri": uri(&fr), "type": 1 }] }),
    );
    assert!(c.diagnostics(&uri(&uc_path), SECOND).unwrap().is_empty());
    fs::remove_file(&fr).unwrap();
    c.notify(
        "workspace/didChangeWatchedFiles",
        json!({ "changes": [{ "uri": uri(&fr), "type": 3 }] }),
    );
    assert_eq!(
        checks(&c.diagnostics(&uri(&uc_path), SECOND).unwrap()),
        ["uncovered-step"]
    );
    // A garbled event, as Neovim sends on Windows below 8.3 short paths:
    // wrong path and "deleted" for a new file. The server rescans the disk.
    fs::write(&fr, "## FR-001 F\nimpl: UC-001.s1\n").unwrap();
    let garbled = dir.join("LONGDI~1").join("garbled-tail").join("fr.md");
    c.notify(
        "workspace/didChangeWatchedFiles",
        json!({ "changes": [{ "uri": uri(&garbled), "type": 3 }] }),
    );
    assert!(c.diagnostics(&uri(&uc_path), SECOND).unwrap().is_empty());
    c.shutdown();
}

#[test]
fn fr_203_to_fr_212_requests_over_protocol() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/shop");
    let root = std::path::absolute(root).unwrap();
    let (mut c, _) = Client::start(&root, json!({}));
    let fr = root.join("functional.md");
    let text = fs::read_to_string(&fr).unwrap();
    c.open(&fr, &text);
    let doc = json!({ "uri": uri(&fr) });
    // Line 3 (0-based): "impl: UC-012.s1  prio: must", cursor on the step reference.
    let pos = json!({ "textDocument": doc, "position": { "line": 3, "character": 10 } });

    let hover = c.request("textDocument/hover", pos.clone());
    assert!(
        hover["contents"]["value"]
            .as_str()
            .unwrap()
            .contains("Customer opens the cart.")
    );
    let def = c.request("textDocument/definition", pos.clone());
    assert!(def["uri"].as_str().unwrap().ends_with("usecases.md"));
    assert_eq!(def["range"]["start"]["line"], 7);
    let mut refs_params = pos.clone();
    refs_params["context"] = json!({ "includeDeclaration": false });
    let refs = c.request("textDocument/references", refs_params);
    assert_eq!(refs.as_array().unwrap().len(), 1);
    let imp = c.request("textDocument/implementation", pos);
    assert_eq!(imp.as_array().unwrap().len(), 1);

    let symbols = c.request(
        "textDocument/documentSymbol",
        json!({ "textDocument": doc }),
    );
    assert_eq!(symbols.as_array().unwrap().len(), 5);
    let ws = c.request("workspace/symbol", json!({ "query": "vat" }));
    // FR-104, BR-003 and TC-220 have "VAT" in the title.
    assert_eq!(ws.as_array().unwrap().len(), 3);

    let completion = c.request(
        "textDocument/completion",
        json!({ "textDocument": doc, "position": { "line": 3, "character": 6 },
                "context": { "triggerKind": 1 } }),
    );
    let labels: Vec<&str> = completion["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i["label"].as_str().unwrap())
        .collect();
    assert!(labels.contains(&"UC-012.s2a"), "{labels:?}");

    // FR-212 on an unsaved placeholder.
    c.change(&fr, 2, &format!("{text}\n## FR-? New\n"));
    let line = text.lines().count() + 1;
    let actions = c.request(
        "textDocument/codeAction",
        json!({ "textDocument": doc,
                "range": { "start": { "line": line, "character": 4 }, "end": { "line": line, "character": 4 } },
                "context": { "diagnostics": [] } }),
    );
    assert_eq!(actions[0]["title"], "Assign ID FR-106");
    c.shutdown();
}

#[test]
fn fr_209_to_fr_219_authoring_features_over_protocol() {
    let uc = "## UC-001 Checkout\n\n- s1: Customer pays.\n- s2: Customer gets a receipt.\n";
    let fr = "## FR-001 Payment\nimpl: UC-001.s1\n\nThe system shall be fast for the basket.\n";
    let dir = temp_project(
        "m3",
        "# keep this comment\n[project]\ninclude = [\"req/*.md\"]\nglossary = \"glossary.md\"\n",
        &[
            ("req/uc.md", uc),
            ("req/fr.md", fr),
            (
                "glossary.md",
                "## Cart\nalias: basket\n\nItems not yet ordered.\n",
            ),
        ],
    );
    let (mut c, _) = Client::start(&dir, json!({}));
    let (uc_path, fr_path) = (dir.join("req/uc.md"), dir.join("req/fr.md"));
    c.open(&uc_path, uc);
    c.open(&fr_path, fr);
    let d = c.diagnostics(&uri(&fr_path), SECOND).unwrap();
    assert_eq!(checks(&d), ["weak-word", "glossary-alias"]);
    let fr_doc = json!({ "uri": uri(&fr_path) });

    // FR-209
    let hints = c.request(
        "textDocument/inlayHint",
        json!({ "textDocument": fr_doc, "range": { "start": { "line": 0, "character": 0 }, "end": { "line": 4, "character": 0 } } }),
    );
    assert_eq!(hints[0]["label"], "Customer pays.");
    // FR-205: glossary hover from a file outside the include patterns.
    let hover = c.request(
        "textDocument/hover",
        json!({ "textDocument": fr_doc, "position": { "line": 3, "character": 36 } }),
    );
    assert!(
        hover["contents"]["value"]
            .as_str()
            .unwrap()
            .contains("Items not yet ordered.")
    );
    // FR-215
    let lenses = c.request(
        "textDocument/codeLens",
        json!({ "textDocument": { "uri": uri(&uc_path) } }),
    );
    assert_eq!(lenses[0]["command"]["title"], "1 impl: FR-001");
    assert_eq!(lenses[1]["command"]["title"], "no implementation");
    // FR-214
    let renamed = c.request(
        "textDocument/rename",
        json!({ "textDocument": { "uri": uri(&uc_path) }, "position": { "line": 0, "character": 5 }, "newName": "UC-002" }),
    );
    let edits: usize = renamed["changes"]
        .as_object()
        .unwrap()
        .values()
        .map(|v| v.as_array().unwrap().len())
        .sum();
    assert_eq!(edits, 2);
    // FR-213 on the uncovered step s2.
    let actions = c.request(
        "textDocument/codeAction",
        json!({ "textDocument": { "uri": uri(&uc_path) },
                "range": { "start": { "line": 3, "character": 3 }, "end": { "line": 3, "character": 3 } },
                "context": { "diagnostics": [] } }),
    );
    assert_eq!(actions[0]["title"], "Create requirement for UC-001.s2");

    // FR-219: "Allow in project" changes requimd.toml and the diagnostics.
    let actions = c.request(
        "textDocument/codeAction",
        json!({ "textDocument": fr_doc,
                "range": { "start": { "line": 3, "character": 21 }, "end": { "line": 3, "character": 21 } },
                "context": { "diagnostics": [] } }),
    );
    let titles: Vec<&str> = actions
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["title"].as_str().unwrap())
        .collect();
    assert_eq!(
        titles,
        [
            "Allow \"fast\" on this line",
            "Allow \"fast\" in project (weak-word)"
        ]
    );
    c.request("workspace/executeCommand", actions[1]["command"].clone());
    let config = fs::read_to_string(dir.join("requimd.toml")).unwrap();
    assert!(config.starts_with("# keep this comment\n"), "{config}");
    assert!(config.contains("allow = [\"fast\"]"), "{config}");
    let d = c.diagnostics(&uri(&fr_path), SECOND).unwrap();
    assert_eq!(checks(&d), ["glossary-alias"]);

    // "Add to weak words" opens requimd.toml at the empty reason.
    c.request(
        "workspace/executeCommand",
        json!({ "command": "requimd.addWeakWord", "arguments": ["payment"] }),
    );
    let config = fs::read_to_string(dir.join("requimd.toml")).unwrap();
    assert!(
        config.contains("{ w = \"payment\", why = \"\" }"),
        "{config}"
    );
    let shown = &c
        .server_requests
        .iter()
        .rfind(|r| r.method == "window/showDocument")
        .unwrap()
        .params;
    assert!(shown["uri"].as_str().unwrap().ends_with("requimd.toml"));
    c.shutdown();
}

#[test]
fn fr_116_fr_218_suspect_links_over_protocol() {
    let uc = "## UC-001 Pay\n\n- s1: Customer pays.\n";
    let fr = "## FR-001 Charge\nimpl: UC-001.s1\n\nThe system shall charge.\n";
    let dir = temp_project("suspect", "", &[("uc.md", uc), ("fr.md", fr)]);
    let (mut c, _) = Client::start(&dir, json!({}));
    let (uc_path, fr_path) = (dir.join("uc.md"), dir.join("fr.md"));
    c.open(&fr_path, fr);
    c.open(&uc_path, uc);
    c.diagnostics(&uri(&fr_path), SECOND).unwrap();

    // FR-218: saving records the link.
    c.notify(
        "textDocument/didSave",
        json!({ "textDocument": { "uri": uri(&fr_path) } }),
    );
    let lock_path = dir.join("requimd.lock");
    let recorded = (0..50).any(|_| {
        std::thread::sleep(Duration::from_millis(20));
        fs::read_to_string(&lock_path).is_ok_and(|l| l.contains("\"FR-001 impl UC-001.s1\""))
    });
    assert!(recorded, "lock not written on save");

    // FR-116: rewording the step (unsaved) makes the link suspect.
    c.change(
        &uc_path,
        2,
        "## UC-001 Pay\n\n- s1: Customer pays by card.\n",
    );
    let d = loop {
        let d = c.diagnostics(&uri(&fr_path), SECOND).unwrap();
        if !d.is_empty() {
            break d;
        }
    };
    assert_eq!(checks(&d), ["suspect-link"]);

    // FR-218: the code action marks it as reviewed.
    let actions = c.request(
        "textDocument/codeAction",
        json!({ "textDocument": { "uri": uri(&fr_path) },
                "range": { "start": { "line": 1, "character": 8 }, "end": { "line": 1, "character": 8 } },
                "context": { "diagnostics": [] } }),
    );
    assert_eq!(actions[0]["title"], "Mark link to UC-001.s1 as reviewed");
    c.request("workspace/executeCommand", actions[0]["command"].clone());
    assert!(c.diagnostics(&uri(&fr_path), SECOND).unwrap().is_empty());
    let reviewed = fs::read_to_string(&lock_path).unwrap();

    // FR-214: a rename moves the lock entry once the client applied it, so
    // nothing turns suspect. A rename that is only requested changes nothing.
    c.request(
        "textDocument/rename",
        json!({ "textDocument": { "uri": uri(&uc_path) }, "position": { "line": 0, "character": 5 }, "newName": "UC-002" }),
    );
    c.request("workspace/symbol", json!({ "query": "" }));
    assert_eq!(fs::read_to_string(&lock_path).unwrap(), reviewed);
    c.change(
        &uc_path,
        3,
        "## UC-002 Pay\n\n- s1: Customer pays by card.\n",
    );
    c.change(&fr_path, 2, &fr.replace("UC-001", "UC-002"));
    let moved = (0..50).any(|_| {
        std::thread::sleep(Duration::from_millis(20));
        fs::read_to_string(&lock_path).is_ok_and(|l| l.contains("UC-002"))
    });
    assert!(moved, "lock entry not renamed");
    let lock = fs::read_to_string(&lock_path).unwrap();
    assert!(lock.contains("\"FR-001 impl UC-002.s1\""), "{lock}");
    assert!(!lock.contains("UC-001"), "{lock}");
    assert_eq!(
        reviewed.split("= ").last(),
        lock.split("= ").last(),
        "the reviewed hash is kept"
    );
    c.shutdown();
}

/// 2,000 items in 200 files (NFR-001 meter).
#[test]
fn fr_216_semantic_tokens_over_protocol() {
    let fr_text = "## FR-001 Quick cart\nprio: must\n\nThe system shall work.\n";
    let g_text = "## Cart\n\nItems.\n";
    let dir = temp_project(
        "tokens",
        "[project]\nglossary = \"g.md\"\n",
        &[("fr.md", fr_text), ("g.md", g_text)],
    );
    let caps = json!({ "workspace": { "semanticTokens": { "refreshSupport": true } } });
    let (mut c, init) = Client::start(&dir, caps);
    let legend = &init["capabilities"]["semanticTokensProvider"]["legend"];
    assert_eq!(legend["tokenTypes"][0], "class");
    assert_eq!(legend["tokenModifiers"][0], "declaration");

    let fr = dir.join("fr.md");
    let g = dir.join("g.md");
    c.open(&fr, fr_text);
    c.open(&g, g_text);
    let fr_doc = json!({ "textDocument": { "uri": uri(&fr) } });
    let count = |v: &Value| v["data"].as_array().unwrap().len() / 5;
    // FR-001, "Quick" (weak word), "cart" (term), "prio".
    let tokens = c.request("textDocument/semanticTokens/full", fr_doc.clone());
    assert_eq!(count(&tokens), 4);

    // Moving text in the same file changes no other file's tokens.
    c.server_requests.clear();
    c.change(&fr, 2, &format!("\n{fr_text}"));
    c.request("textDocument/semanticTokens/full", fr_doc.clone());
    assert!(c.server_requests.is_empty());

    // A new glossary term changes the tokens of fr.md: refresh requested.
    c.change(&g, 2, "## Cart\n\nItems.\n\n## System\n\nThe software.\n");
    let tokens = c.request("textDocument/semanticTokens/full", fr_doc);
    assert_eq!(count(&tokens), 5);
    assert!(
        c.server_requests
            .iter()
            .any(|r| r.method == "workspace/semanticTokens/refresh")
    );
    c.shutdown();
}

fn generated_project(name: &str) -> PathBuf {
    let mut files = Vec::new();
    for f in 0..200 {
        let mut text = format!("# File {f}\n\n## UC-{f:04} Use case {f}\nactor: Customer\n\n");
        for s in 1..=5 {
            text.push_str(&format!("- s{s}: Step {s} of the use case.\n"));
        }
        for i in 0..9 {
            let n = f * 9 + i;
            text.push_str(&format!(
                "\n## FR-{n:04} Requirement {n}\nimpl: UC-{f:04}.s{}  prio: must\n\nWhen the customer acts, the system shall respond (see FR-{:04}).\n",
                i % 5 + 1,
                (n + 1) % 1800
            ));
        }
        files.push((format!("req/f{f:03}.md"), text));
    }
    let refs: Vec<(&str, &str)> = files
        .iter()
        .map(|(n, t)| (n.as_str(), t.as_str()))
        .collect();
    temp_project(name, "", &refs)
}

#[test]
fn nfr_001_diagnostics_latency_on_2000_items() {
    let dir = generated_project("latency");
    let (mut c, _) = Client::start(&dir, json!({}));
    let path = dir.join("req/f100.md");
    let text = fs::read_to_string(&path).unwrap();
    c.open(&path, &text);
    c.diagnostics(&uri(&path), 5 * SECOND).unwrap();
    let mut times = Vec::new();
    for v in 0..20 {
        // Alternate between a valid and an invalid value.
        let prio = if v % 2 == 0 { "urgent" } else { "must" };
        let edited = text.replacen("prio: must", &format!("prio: {prio}"), 1);
        let start = Instant::now();
        c.change(&path, v + 2, &edited);
        c.diagnostics(&uri(&path), 5 * SECOND).unwrap();
        times.push(start.elapsed());
    }
    times.sort();
    let p95 = times[times.len() * 95 / 100 - 1];
    eprintln!("NFR-001 p95 {p95:?}");
    // must: 300 ms for the shipped (release) binary; CI runs this test in
    // release mode. Unoptimised builds only get a coarse bound.
    let bound = if cfg!(debug_assertions) { 1000 } else { 300 };
    assert!(
        p95 < Duration::from_millis(bound),
        "p95 {p95:?}, all {times:?}"
    );
    c.shutdown();
}

#[test]
fn nfr_003_server_start_until_item_graph_complete() {
    let dir = generated_project("start");
    let start = Instant::now();
    let (mut c, _) = Client::start(&dir, json!({}));
    let r = c.request("workspace/symbol", json!({ "query": "FR-1799" }));
    let elapsed = start.elapsed();
    assert_eq!(r.as_array().unwrap().len(), 1);
    assert!(elapsed < 2 * SECOND, "took {elapsed:?}");
    c.shutdown();
}

#[test]
fn nfr_009_no_crash_on_10000_random_edits() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let root = std::path::absolute(root).unwrap();
    let mut docs = Vec::new();
    for dir in ["shop", "broken"] {
        for e in fs::read_dir(root.join(dir)).unwrap() {
            let p = e.unwrap().path();
            if p.extension().is_some_and(|x| x == "md") {
                let t = fs::read_to_string(&p).unwrap();
                docs.push((p, t));
            }
        }
    }
    let mut seed: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut rand = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let alphabet: Vec<char> = "#-: \n\t`*[]|.,?sSFRUC0123456789äé→\r".chars().collect();
    let methods = [
        "textDocument/hover",
        "textDocument/definition",
        "textDocument/implementation",
        "textDocument/completion",
    ];
    let mut clients = Vec::new();
    for project in ["shop", "broken"] {
        let (c, _) = Client::start(&root.join(project), json!({}));
        clients.push(c);
    }
    let mut texts: Vec<Vec<char>> = docs.iter().map(|(_, t)| t.chars().collect()).collect();
    for (i, (p, t)) in docs.iter().enumerate() {
        let c = &clients[usize::from(p.to_string_lossy().contains("broken"))];
        c.open(p, t);
        let _ = i;
    }
    for edit in 0..10_000 {
        let d = (rand() as usize) % docs.len();
        let chars = &mut texts[d];
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
        let text: String = chars.iter().collect();
        let path = &docs[d].0;
        let c = &mut clients[usize::from(path.to_string_lossy().contains("broken"))];
        c.change(path, edit + 2, &text);
        if edit % 5 == 0 {
            let line = rand() % 40;
            let character = rand() % 60;
            let method = methods[(rand() as usize) % methods.len()];
            c.request(
                method,
                json!({ "textDocument": { "uri": uri(path) },
                        "position": { "line": line, "character": character } }),
            );
        }
        c.inbox.clear();
    }
    for c in clients {
        c.shutdown();
    }
}
