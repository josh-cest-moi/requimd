// FR-500: starts `requimd lsp` for the Markdown files of requimd projects.
//
// The server serves one project, so there is one client per project root
// (the nearest directory with a `requimd.toml`), as with Neovim's
// `root_markers` (docs/neovim.md). The commands of the servers are VS Code
// commands, which exist only once, so the extension registers them itself
// and sends each call to the server of the active editor's project.

import * as fs from 'node:fs';
import * as path from 'node:path';
import * as vscode from 'vscode';
import {
  DynamicFeature,
  ExecuteCommandRequest,
  LanguageClient,
  LanguageClientOptions,
  Location as LspLocation,
  ServerOptions,
  StaticFeature,
} from 'vscode-languageclient/node';

/** Command of the step code lens (FR-215), see `IMPLEMENTATIONS_COMMAND`. */
const IMPLEMENTATIONS_COMMAND = 'requimd.implementations';

/** A client that leaves the server's commands to `registerCommands`. */
class RequimdClient extends LanguageClient {
  override registerFeature(feature: StaticFeature | DynamicFeature<unknown>): void {
    if ('registrationType' in feature && feature.registrationType.method === ExecuteCommandRequest.method) {
      return;
    }
    super.registerFeature(feature);
  }
}

const clients = new Map<string, RequimdClient>();
const starting = new Map<string, Promise<void>>();
const commands = new Set<string>();
let context: vscode.ExtensionContext;
let output: vscode.LogOutputChannel;

export function activate(ctx: vscode.ExtensionContext): void {
  context = ctx;
  output = vscode.window.createOutputChannel('requimd', { log: true });
  ctx.subscriptions.push(
    output,
    vscode.workspace.onDidOpenTextDocument(didOpen),
    vscode.workspace.onDidChangeConfiguration((e) => {
      if (e.affectsConfiguration('requimd.path')) {
        void restart();
      }
    }),
  );
  vscode.workspace.textDocuments.forEach(didOpen);
}

export async function deactivate(): Promise<void> {
  await stopAll();
}

function didOpen(doc: vscode.TextDocument): void {
  if (doc.languageId !== 'markdown' || doc.uri.scheme !== 'file') {
    return;
  }
  const root = projectRoot(path.dirname(doc.uri.fsPath));
  if (root === undefined || clients.has(root) || starting.has(root)) {
    return;
  }
  const start = startClient(root).finally(() => starting.delete(root));
  starting.set(root, start);
}

/** Nearest directory at or above `dir` with a `requimd.toml` (FR-013). */
function projectRoot(dir: string): string | undefined {
  for (;;) {
    if (fs.existsSync(path.join(dir, 'requimd.toml'))) {
      return dir;
    }
    const parent = path.dirname(dir);
    if (parent === dir) {
      return undefined;
    }
    dir = parent;
  }
}

/**
 * FR-501: `requimd.path` if set, else the executable bundled in `bin/`,
 * else `requimd` from `PATH`.
 */
function serverCommand(): string {
  const configured = vscode.workspace.getConfiguration('requimd').get<string>('path', '').trim();
  if (configured !== '') {
    return configured;
  }
  const exe = process.platform === 'win32' ? 'requimd.exe' : 'requimd';
  const bundled = path.join(context.extensionPath, 'bin', exe);
  if (!fs.existsSync(bundled)) {
    return 'requimd';
  }
  if (process.platform !== 'win32') {
    // FR-502: the execute permission may not survive unpacking the .vsix.
    try {
      fs.chmodSync(bundled, 0o755);
    } catch {
      // Read-only installation: try to start it anyway.
    }
  }
  return bundled;
}

async function startClient(root: string): Promise<void> {
  const command = serverCommand();
  const serverOptions: ServerOptions = { command, args: ['lsp'], options: { cwd: root } };
  const rootUri = vscode.Uri.file(root);
  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      { scheme: 'file', language: 'markdown', pattern: { baseUri: rootUri.toString(), pattern: '**/*' } },
    ],
    workspaceFolder: { uri: rootUri, name: path.basename(root), index: 0 },
    outputChannel: output,
    middleware: { provideCodeLenses: codeLensesAsReferences },
  };
  const client = new RequimdClient('requimd', 'requimd', serverOptions, clientOptions);
  clients.set(root, client);
  try {
    await client.start();
    registerCommands(client.initializeResult?.capabilities.executeCommandProvider?.commands ?? []);
    output.info(`Started ${command} lsp for ${root}`);
  } catch (e) {
    clients.delete(root);
    output.error(`Cannot start ${command} lsp for ${root}: ${String(e)}`);
    void vscode.window.showErrorMessage(
      `requimd: cannot start the language server "${command}". Check the setting requimd.path.`,
    );
  }
}

function registerCommands(names: string[]): void {
  for (const command of names) {
    if (commands.has(command)) {
      continue;
    }
    commands.add(command);
    context.subscriptions.push(
      vscode.commands.registerCommand(command, async (...args: unknown[]) => {
        const client = activeClient();
        if (client === undefined) {
          return;
        }
        try {
          await client.sendRequest(ExecuteCommandRequest.type, { command, arguments: args });
        } catch (e) {
          void vscode.window.showErrorMessage(`requimd: ${e instanceof Error ? e.message : String(e)}`);
        }
      }),
    );
  }
}

/** The client of the active editor's project, else the only one. */
function activeClient(): RequimdClient | undefined {
  const doc = vscode.window.activeTextEditor?.document;
  const root = doc?.uri.scheme === 'file' ? projectRoot(path.dirname(doc.uri.fsPath)) : undefined;
  const client = root === undefined ? undefined : clients.get(root);
  return client ?? (clients.size === 1 ? [...clients.values()][0] : undefined);
}

/**
 * FR-500, FR-215: the step code lens lists the implementing items in VS
 * Code's reference view instead of opening the first one.
 */
async function codeLensesAsReferences(
  doc: vscode.TextDocument,
  token: vscode.CancellationToken,
  next: (doc: vscode.TextDocument, token: vscode.CancellationToken) => vscode.ProviderResult<vscode.CodeLens[]>,
): Promise<vscode.CodeLens[] | null | undefined> {
  const lenses = await next(doc, token);
  for (const lens of lenses ?? []) {
    const cmd = lens.command;
    if (cmd?.command !== IMPLEMENTATIONS_COMMAND) {
      continue;
    }
    const locations = ((cmd.arguments?.[0] ?? []) as LspLocation[]).map(
      (l) =>
        new vscode.Location(
          vscode.Uri.parse(l.uri),
          new vscode.Range(l.range.start.line, l.range.start.character, l.range.end.line, l.range.end.character),
        ),
    );
    lens.command = {
      title: cmd.title,
      command: 'editor.action.showReferences',
      arguments: [doc.uri, lens.range.start, locations],
    };
  }
  return lenses;
}

async function stopAll(): Promise<void> {
  const all = [...clients.values()];
  clients.clear();
  await Promise.all(all.map((c) => c.stop().catch(() => undefined)));
}

async function restart(): Promise<void> {
  await stopAll();
  vscode.workspace.textDocuments.forEach(didOpen);
}
