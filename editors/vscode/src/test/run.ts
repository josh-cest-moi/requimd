// NFR-011: runs the extension tests in a real VS Code.
//
// Usage (from editors/vscode, after `cargo build` in the repository root):
//   npm run compile && npm test
// The server is target/debug/requimd unless REQUIMD_TEST_BIN names another
// executable. With REQUIMD_TEST_VSIX naming a .vsix package, the tests run
// against that unpacked package and its bundled executable instead (FR-501). The VS Code version is pinned to the oldest supported one
// (`engines.vscode`); it is downloaded once into .vscode-test/.

import * as fs from 'node:fs';
import * as os from 'node:os';
import * as path from 'node:path';
import { execFileSync } from 'node:child_process';
import { runTests } from '@vscode/test-electron';

const VSCODE_VERSION = '1.91.1';

async function main(): Promise<void> {
  const extension = path.resolve(__dirname, '..', '..');
  const repo = path.resolve(extension, '..', '..');
  const exe = process.platform === 'win32' ? 'requimd.exe' : 'requimd';
  const vsix = process.env.REQUIMD_TEST_VSIX;
  const bin = path.resolve(process.env.REQUIMD_TEST_BIN ?? path.join(repo, 'target', 'debug', exe));
  if (vsix === undefined && !fs.existsSync(bin)) {
    throw new Error(`no requimd executable at ${bin}; run cargo build first`);
  }
  const unpacked = vsix === undefined ? undefined : fs.mkdtempSync(path.join(os.tmpdir(), 'requimd-vsix-'));
  if (vsix !== undefined && unpacked !== undefined) {
    execFileSync('unzip', ['-q', path.resolve(vsix), '-d', unpacked]);
  }

  // The tests edit files, so they work on a copy of the examples.
  const workspace = fs.mkdtempSync(path.join(os.tmpdir(), 'requimd-vscode-'));
  for (const name of ['shop', 'broken']) {
    fs.cpSync(path.join(repo, 'examples', name), path.join(workspace, name), { recursive: true });
  }

  // Fresh user settings per run, so `requimd.path` does not carry over.
  const userData = fs.mkdtempSync(path.join(os.tmpdir(), 'requimd-vscode-user-'));

  try {
    await runTests({
      version: VSCODE_VERSION,
      extensionDevelopmentPath: unpacked === undefined ? extension : path.join(unpacked, 'extension'),
      extensionTestsPath: path.join(__dirname, 'suite', 'index'),
      extensionTestsEnv: unpacked === undefined ? { REQUIMD_TEST_BIN: bin } : {},
      launchArgs: [workspace, '--disable-extensions', '--disable-workspace-trust', `--user-data-dir=${userData}`],
    });
  } finally {
    fs.rmSync(workspace, { recursive: true, force: true });
    fs.rmSync(userData, { recursive: true, force: true });
    if (unpacked !== undefined) {
      fs.rmSync(unpacked, { recursive: true, force: true });
    }
  }
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
