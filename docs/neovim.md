# Neovim setup

Target: Neovim 0.12 with the built-in LSP client, no plugins (NFR-007).
Everything below uses core APIs available since Neovim 0.11.

## Configuration

```lua
vim.lsp.config('reqmd', {
  cmd = { 'reqmd', 'lsp' },
  filetypes = { 'markdown' },
  root_markers = { 'reqmd.toml' },
})
vim.lsp.enable('reqmd')

vim.api.nvim_create_autocmd('LspAttach', {
  callback = function(ev)
    local client = vim.lsp.get_client_by_id(ev.data.client_id)
    if client and client.name == 'reqmd' then
      vim.lsp.completion.enable(true, client.id, ev.buf, { autotrigger = true })
      vim.lsp.inlay_hint.enable(true, { bufnr = ev.buf })
    end
  end,
})
```

`root_markers` ensures the server only starts inside reqmd projects.

## Features and default keys

Verified with Neovim 0.12.6, `--clean` (no plugins, no user config), by
`scripts/nvim-check.lua` (NFR-007). The `cargo test` suite runs it when
`nvim` is installed; CI installs it.

```bash
cargo build --release
nvim --headless --clean -l scripts/nvim-check.lua target/release/reqmd
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
| Create stub, mark reviewed | FR-213, FR-218 | `gra` | M3/M4 |
| Rename | FR-214 | `grn` | M3 |
| Inlay hints | FR-209 | enabled above | M3 |
| Code lens | FR-215 | `vim.lsp.codelens` | M3 |

Completion in body text finds items by ID or by a word of the title: type
`VAT` and press `<C-x><C-o>` to get `FR-104`. Hover text length is set by
`hover-length` in `[lsp]` of `reqmd.toml`.

Verify key defaults with `:help lsp-defaults` in your Neovim version.
