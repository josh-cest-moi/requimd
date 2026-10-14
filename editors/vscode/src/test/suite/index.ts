// Entry point that VS Code calls inside the extension host (NFR-011).
// Prints one PASS/FAIL line per test, like scripts/nvim-check.lua.

import { setup, tests } from './features';

export async function run(): Promise<void> {
  await setup();
  let failed = 0;
  for (const [name, test] of tests) {
    try {
      await test();
      console.log(`PASS ${name}`);
    } catch (e) {
      failed += 1;
      const message = e instanceof Error ? e.message : String(e);
      console.log(`FAIL ${name}: ${message.replace(/\s*\n\s*/g, ' ')}`);
    }
  }
  if (failed > 0) {
    throw new Error(`${failed} of ${tests.length} tests failed`);
  }
}
