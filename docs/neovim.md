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

To be verified per feature during M2 (see NFR-007).

| Feature | Requirement | Neovim |
|---------|-------------|--------|
| Completion | FR-203, FR-204 | automatic, or `<C-x><C-o>` |
| Hover | FR-205 | `K` |
| Go to definition | FR-206 | `<C-]>` (tagfunc) or map `gd` |
| Find references | FR-207 | `grr` |
| Go to implementation | FR-208 | `gri` |
| Document symbols | FR-210 | `gO` |
| Workspace symbols | FR-211 | `:lua vim.lsp.buf.workspace_symbol()` |
| Code actions | FR-212, FR-213, FR-218 | `gra` |
| Rename | FR-214 | `grn` |
| Diagnostics | FR-202 | `]d`, `[d`, `<C-w>d` |
| Inlay hints | FR-209 | enabled above |
| Code lens | FR-215 | `vim.lsp.codelens` |

Verify key defaults with `:help lsp-defaults` in your Neovim version.
