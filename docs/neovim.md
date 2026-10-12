# Neovim setup

Target: Neovim 0.12 with the built-in LSP client, no plugins (NFR-007).
Everything below uses core APIs available since Neovim 0.11.

## Configuration

```lua
vim.lsp.config('requimd', {
  cmd = { 'requimd', 'lsp' },
  filetypes = { 'markdown' },
  root_markers = { 'requimd.toml' },
})
vim.lsp.enable('requimd')

vim.api.nvim_create_autocmd('LspAttach', {
  callback = function(ev)
    local client = vim.lsp.get_client_by_id(ev.data.client_id)
    if client and client.name == 'requimd' then
      vim.lsp.completion.enable(true, client.id, ev.buf, { autotrigger = true })
      vim.lsp.inlay_hint.enable(true, { bufnr = ev.buf })
    end
  end,
})

-- Optional, for the code lens on use case steps (FR-215): running a lens
-- lists the implementing items in the quickfix list. Without it, the
-- server opens the first implementing item.
vim.lsp.commands['requimd.implementations'] = function(command, ctx)
  local client = vim.lsp.get_client_by_id(ctx.client_id)
  local items = vim.lsp.util.locations_to_items(command.arguments[1], client.offset_encoding)
  vim.fn.setqflist({}, ' ', { title = command.title, items = items })
  vim.cmd.copen()
end
```

`root_markers` ensures the server only starts inside requimd projects.

## Features and default keys

Verified with Neovim 0.12.6, `--clean` (no plugins, no user config), by
`scripts/nvim-check.lua` (NFR-007). The `cargo test` suite runs it when
`nvim` is installed; CI installs it.

```bash
cargo build --release
nvim --headless --clean -l scripts/nvim-check.lua target/release/requimd
```

| Feature | Requirement | Neovim | Verified |
|---------|-------------|--------|----------|
| Diagnostics | FR-202 | `]d`, `[d`, `<C-w>d` | M2 |
| Completion | FR-203, FR-204 | automatic after `-`, `.`, and in link values after space or comma; anywhere with `<C-x><C-o>` | M2 |
| Hover | FR-205 | `K` | M2 |
| Go to definition | FR-206 | `<C-]>` (tagfunc) or map `gd` | M2 |
| Find references | FR-207 | `grr` | M2 |
| Go to implementation | FR-208 | `gri` | M2 |
| Document symbols | FR-210 | `gO` | M2 |
| Workspace symbols | FR-211 | `:lua vim.lsp.buf.workspace_symbol()` | M2 |
| Assign ID | FR-212 | `gra` on a `FR-?` heading | M2 |
| External file changes | FR-217 | automatic (Neovim's file watcher, else server polling every 2 s) | M2 |
| Glossary hover | FR-205 | `K` on a term or alias | M3 |
| Inlay hints | FR-209 | enabled above | M3 |
| Create stub | FR-213 | `gra` on a dangling reference or an uncovered step | M3 |
| Rename | FR-214 | `grn` on an item ID | M3 |
| Code lens | FR-215 | `:lua vim.lsp.codelens.run()` on a step | M3 |
| Semantic tokens | FR-216 | automatic (see "Highlighting" below) | after M5 |
| Language check actions | FR-219 | `gra` on a weak word, modal verb or alias, or on any word | M3 |
| Suspect links | FR-116 | diagnostics; links are recorded in `requimd.lock` on save (FR-218) | M4 |
| Mark as reviewed | FR-218 | `gra` on a suspect link | M4 |

Completion in body text finds items by ID or by a word of the title: type
`VAT` and press `<C-x><C-o>` to get `FR-104`.

Settings in `[lsp]` of `requimd.toml`: `hover-length` (characters of item
text on hover, default 500), `hint-length` (inlay hints, default 40) and
`stub-file` (where FR-213 puts new stubs; default the current file).

"Allow in project" and "Add to weak words" change `requimd.toml` on disk and
keep its comments. If you have `requimd.toml` open with unsaved changes, save
it first. "Add to weak words" then opens it at the new entry; type the
reason between the quotes of `why`.

Verify key defaults with `:help lsp-defaults` in your Neovim version.

## Highlighting

The server sends semantic tokens (FR-216); Neovim applies them without
configuration. The token types are standard LSP types, so every colour
scheme colours them:

| Text | Token type | Modifier | Default group |
|------|------------|----------|---------------|
| Item ID in a heading | `class` | `declaration` | `@type` |
| Reference (link or mention) | `class` | | `@type` |
| Step label | `enumMember` | `declaration` | `@constant` |
| Metadata key, body field key | `property` | | `@property` |
| Glossary term | `namespace` | | `@module` |
| Weak word | `keyword` | `weakWord` | `@keyword` |
| Modal verb | `keyword` | `modalVerb` | `@keyword` |
| Glossary alias | `keyword` | `glossaryAlias` | `@keyword` |

Weak words, modal verbs and aliases are highlighted exactly where a
diagnostic is reported: allowed words and disabled checks stay plain.
Other colours are set per type or per type and modifier, for example:

```lua
vim.api.nvim_set_hl(0, '@lsp.typemod.keyword.weakWord.markdown', { link = 'DiagnosticWarn' })
vim.api.nvim_set_hl(0, '@lsp.type.namespace.markdown', { italic = true })
```

`:Inspect` on a word shows its tokens. `vim.lsp.semantic_tokens.enable(false)`
switches highlighting off.
