// NFR-011: every `must` language server feature works in VS Code with the
// extension (FR-500), checked like scripts/nvim-check.lua does for Neovim
// (NFR-007). Positions are 0-based; the examples are ASCII.

import * as assert from 'node:assert';
import * as fs from 'node:fs';
import * as path from 'node:path';
import * as vscode from 'vscode';

const workspace = (): string => vscode.workspace.workspaceFolders![0].uri.fsPath;
const shop = (file: string): vscode.Uri => vscode.Uri.file(path.join(workspace(), 'shop', file));

async function waitFor<T>(probe: () => Promise<T> | T, done: (v: T) => boolean, ms = 10000): Promise<T> {
  const end = Date.now() + ms;
  for (;;) {
    // A request can be cancelled while the server handles an edit.
    const v = await Promise.resolve()
      .then(probe)
      .catch(() => undefined as T);
    if ((v !== undefined && done(v)) || Date.now() > end) {
      return v;
    }
    await new Promise((r) => setTimeout(r, 50));
  }
}

async function open(uri: vscode.Uri): Promise<vscode.TextDocument> {
  const doc = await vscode.workspace.openTextDocument(uri);
  await vscode.window.showTextDocument(doc);
  return doc;
}

function exec<T>(command: string, ...args: unknown[]): Promise<T> {
  return Promise.resolve(vscode.commands.executeCommand<T>(command, ...args));
}

function hover(uri: vscode.Uri, pos: vscode.Position): Promise<string> {
  return exec<vscode.Hover[]>('vscode.executeHoverProvider', uri, pos).then((hs) =>
    (hs ?? [])
      .flatMap((h) => h.contents)
      .map((c) => (typeof c === 'string' ? c : c.value))
      .join('\n'),
  );
}

const diagnostics = (uri: vscode.Uri): vscode.Diagnostic[] =>
  vscode.languages.getDiagnostics(uri).filter((d) => d.source === undefined || d.source === 'requimd');

/** A scratch project inside the workspace, so VS Code watches its files. */
function scratch(name: string, files: Record<string, string>): string {
  const dir = path.join(workspace(), name);
  fs.mkdirSync(dir, { recursive: true });
  for (const [file, text] of Object.entries(files)) {
    fs.writeFileSync(path.join(dir, file), text);
  }
  return dir;
}

// Line 4 (1-based) of functional.md: "impl: UC-012.s1  prio: must".
const ref = new vscode.Position(3, 10);

/** Without REQUIMD_TEST_BIN the extension uses its bundled executable. */
export async function setup(): Promise<void> {
  if (process.env.REQUIMD_TEST_BIN !== undefined) {
    await vscode.workspace
      .getConfiguration('requimd')
      .update('path', process.env.REQUIMD_TEST_BIN, vscode.ConfigurationTarget.Global);
  }
}

/** Run in this order: later tests rely on the server started by the first. */
export const tests: [string, () => Promise<void>][] = [

  ['NFR-011 FR-500 FR-200 server starts for a requimd project', async () => {
    const doc = await open(shop('functional.md'));
    const text = await waitFor(() => hover(doc.uri, ref), (t) => t.includes('Customer opens the cart'));
    assert.ok(text.includes('Customer opens the cart'), text);
  }],

  ['FR-501 FR-502 server from requimd.path or the bundled executable', async () => {
    const ext = vscode.extensions.getExtension('josh-cest-moi.requimd')!;
    const configured = vscode.workspace.getConfiguration('requimd').get<string>('path', '');
    const bundled = path.join(ext.extensionPath, 'bin', process.platform === 'win32' ? 'requimd.exe' : 'requimd');
    if (process.env.REQUIMD_TEST_BIN === undefined) {
      assert.strictEqual(configured, '');
      assert.ok(fs.existsSync(bundled), bundled);
    } else {
      assert.strictEqual(configured, process.env.REQUIMD_TEST_BIN);
    }
  }],

  ['FR-205 hover', async () => {
    assert.ok((await hover(shop('functional.md'), ref)).includes('Customer opens the cart'));
  }],

  ['FR-206 definition', async () => {
    const defs = await exec<(vscode.Location | vscode.LocationLink)[]>(
      'vscode.executeDefinitionProvider',
      shop('functional.md'),
      ref,
    );
    const d = defs[0];
    const [uri, range] = 'uri' in d ? [d.uri, d.range] : [d.targetUri, d.targetRange];
    assert.ok(uri.fsPath.endsWith('usecases.md'), uri.fsPath);
    assert.strictEqual(range.start.line, 7);
  }],

  ['FR-207 references', async () => {
    const refs = await exec<vscode.Location[]>('vscode.executeReferenceProvider', shop('functional.md'), ref);
    assert.ok(refs.some((r) => r.uri.fsPath.endsWith('functional.md') && r.range.start.line === 3));
  }],

  ['FR-208 implementation', async () => {
    const impl = await exec<vscode.Location[]>('vscode.executeImplementationProvider', shop('functional.md'), ref);
    assert.strictEqual(impl.length, 1);
  }],

  ['FR-210 document symbols', async () => {
    const syms = await exec<vscode.DocumentSymbol[]>('vscode.executeDocumentSymbolProvider', shop('functional.md'));
    const names = syms.map((s) => s.name);
    for (const id of ['FR-101', 'FR-102', 'FR-103', 'FR-104', 'FR-105']) {
      assert.ok(names.some((n) => n.startsWith(id)), `${id} in ${names.join(', ')}`);
    }
  }],

  ['FR-211 workspace symbols', async () => {
    const syms = await exec<vscode.SymbolInformation[]>('vscode.executeWorkspaceSymbolProvider', 'cart');
    assert.ok(syms.length >= 2, String(syms.length));
  }],

  ['FR-203 reference completion', async () => {
    const list = await exec<vscode.CompletionList>(
      'vscode.executeCompletionItemProvider',
      shop('functional.md'),
      new vscode.Position(3, 6),
    );
    const labels = list.items.map((i) => (typeof i.label === 'string' ? i.label : i.label.label));
    assert.ok(labels.includes('UC-012.s2a'), labels.join(', '));
    assert.ok(!labels.includes('FR-101'));
  }],

  ['FR-204 value completion', async () => {
    const list = await exec<vscode.CompletionList>(
      'vscode.executeCompletionItemProvider',
      shop('functional.md'),
      new vscode.Position(3, 23),
    );
    const labels = list.items.map((i) => (typeof i.label === 'string' ? i.label : i.label.label));
    assert.ok(labels.includes('should'), labels.join(', '));
  }],

  ['FR-212 assign ID', async () => {
    const doc = await open(shop('functional.md'));
    const edit = new vscode.WorkspaceEdit();
    edit.insert(doc.uri, doc.lineAt(doc.lineCount - 1).range.end, '\n## FR-? New one\n');
    await vscode.workspace.applyEdit(edit);
    const line = doc.getText().split('\n').findIndex((l) => l.startsWith('## FR-? New one'));
    const at = new vscode.Range(line, 4, line, 4);
    const actions = await waitFor(
      () => exec<vscode.CodeAction[]>('vscode.executeCodeActionProvider', doc.uri, at),
      (a) => a.some((x) => x.title.startsWith('Assign ID')),
    );
    const assign = actions?.find((a) => a.title.startsWith('Assign ID'));
    assert.strictEqual(assign?.title, 'Assign ID FR-106');
    await vscode.workspace.applyEdit(assign!.edit!);
    assert.strictEqual(doc.lineAt(line).text.trimEnd(), '## FR-106 New one');
    await vscode.commands.executeCommand('workbench.action.files.revert');
  }],

  ['FR-202 live diagnostics', async () => {
    const uri = vscode.Uri.file(path.join(workspace(), 'broken', 'broken.md'));
    const doc = await open(uri);
    const before = await waitFor(() => diagnostics(uri).length, (c) => c === 22);
    assert.strictEqual(before, 22);
    const edit = new vscode.WorkspaceEdit();
    edit.replace(uri, doc.lineAt(10).range, 'impl: UC-001.s1  prio: must');
    await vscode.workspace.applyEdit(edit);
    const after = await waitFor(() => diagnostics(uri).length, (c) => c === 20);
    assert.strictEqual(after, 20);
    await vscode.commands.executeCommand('workbench.action.files.revert');
  }],

  ['FR-217 external file changes', async () => {
    const dir = scratch('fr-217', { 'requimd.toml': '', 'uc.md': '## UC-001 U\n\n- s1: Go.\n' });
    const uc = (await open(vscode.Uri.file(path.join(dir, 'uc.md')))).uri;
    assert.strictEqual(await waitFor(() => diagnostics(uc).length, (c) => c === 1), 1);
    fs.writeFileSync(path.join(dir, 'fr.md'), '## FR-001 F\nimpl: UC-001.s1\n');
    assert.strictEqual(await waitFor(() => diagnostics(uc).length, (c) => c === 0), 0);
  }],

  ['FR-500 no language server outside requimd projects', async () => {
    const file = path.join(workspace(), 'notes.md');
    fs.writeFileSync(file, '## FR-001 Not an item here\nimpl: UC-001\n');
    const uri = (await open(vscode.Uri.file(file))).uri;
    await new Promise((r) => setTimeout(r, 1000));
    assert.strictEqual(diagnostics(uri).length, 0);
  }],

  // `should` items, as in scripts/nvim-check.lua.

  ['FR-209 inlay hints', async () => {
    const hints = await exec<vscode.InlayHint[]>(
      'vscode.executeInlayHintProvider',
      shop('functional.md'),
      new vscode.Range(0, 0, 10, 0),
    );
    const label = hints[0]?.label;
    const text = typeof label === 'string' ? label : label?.map((p) => p.value).join('');
    assert.strictEqual(text, 'Customer opens the cart.');
  }],

  ['FR-214 rename', async () => {
    const edit = await exec<vscode.WorkspaceEdit>(
      'vscode.executeDocumentRenameProvider',
      shop('functional.md'),
      new vscode.Position(3, 8),
      'UC-099',
    );
    const edits = edit.entries().reduce((n, [, list]) => n + list.length, 0);
    assert.ok(edits >= 6, String(edits));
  }],

  ['FR-500 FR-215 code lens lists implementations in the reference view', async () => {
    await open(shop('usecases.md'));
    const lenses = await waitFor(
      () => exec<vscode.CodeLens[]>('vscode.executeCodeLensProvider', shop('usecases.md')),
      (l) => l.some((x) => x.command?.title === '2 impl: FR-104, FR-105'),
    );
    const lens = lenses.find((l) => l.command?.title === '2 impl: FR-104, FR-105');
    assert.strictEqual(lens?.command?.command, 'editor.action.showReferences');
    const locations = lens!.command!.arguments![2] as vscode.Location[];
    assert.strictEqual(locations.length, 2);
    assert.ok(locations.every((l) => l.uri.fsPath.endsWith('functional.md')));
  }],

  ['FR-503 highlighting of language check findings', async () => {
    const legend = await exec<vscode.SemanticTokensLegend>(
      'vscode.provideDocumentSemanticTokensLegend',
      shop('functional.md'),
    );
    const pkg = vscode.extensions.getExtension('josh-cest-moi.requimd')!.packageJSON;
    const declared: string[] = pkg.contributes.semanticTokenModifiers.map((m: { id: string }) => m.id);
    const scoped = Object.keys(pkg.contributes.semanticTokenScopes[0].scopes);
    for (const modifier of legend.tokenModifiers.filter((m) => m !== 'declaration')) {
      assert.ok(declared.includes(modifier), `${modifier} declared`);
      assert.ok(scoped.includes(`*.${modifier}`), `${modifier} mapped to a theme scope`);
    }
    const tokens = await exec<vscode.SemanticTokens>('vscode.provideDocumentSemanticTokens', shop('functional.md'));
    assert.ok(tokens.data.length > 0);
  }],

  ['FR-219 allow in project runs a server command', async () => {
    const dir = scratch('fr-219', {
      'requimd.toml': '# my comment\n',
      'fr.md': '## FR-001 X\n\nThe system shall be fast.\n',
    });
    const uri = (await open(vscode.Uri.file(path.join(dir, 'fr.md')))).uri;
    assert.strictEqual(await waitFor(() => diagnostics(uri).length, (c) => c === 1), 1);
    const at = new vscode.Range(2, 21, 2, 21);
    const actions = await exec<vscode.CodeAction[]>('vscode.executeCodeActionProvider', uri, at);
    const allow = actions.find((a) => a.title.includes('in project'));
    assert.ok(allow?.command, actions.map((a) => a.title).join(', '));
    await vscode.commands.executeCommand(allow.command.command, ...(allow.command.arguments ?? []));
    assert.strictEqual(await waitFor(() => diagnostics(uri).length, (c) => c === 0), 0);
    const toml = fs.readFileSync(path.join(dir, 'requimd.toml'), 'utf8');
    assert.ok(toml.includes('# my comment') && toml.includes('allow = ["fast"]'), toml);
  }],
];
