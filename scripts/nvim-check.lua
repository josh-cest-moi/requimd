-- NFR-007: checks every `must` language server feature with Neovim's
-- built-in client and no plugins, using the setup from docs/neovim.md.
--
-- Usage (from the repository root, after `cargo build --release`):
--   nvim --headless --clean -l scripts/nvim-check.lua target/release/requimd
-- Prints one PASS/FAIL line per feature; exits with 1 on any failure.

local bin = vim.fn.fnamemodify(arg[1] or 'target/release/requimd', ':p')
local examples = vim.fn.fnamemodify('examples', ':p'):gsub('[/\\]$', '')

-- Setup as in docs/neovim.md.
vim.lsp.config('requimd', {
  cmd = { bin, 'lsp' },
  filetypes = { 'markdown' },
  root_markers = { 'requimd.toml' },
})
vim.lsp.enable('requimd')

local failed = 0
local function check(name, ok, detail)
  if not ok then failed = failed + 1 end
  io.write(string.format('%s %s%s\n', ok and 'PASS' or 'FAIL', name, detail and (': ' .. detail) or ''))
end

local function open(path)
  vim.cmd.edit(vim.fn.fnameescape(path))
  local buf = vim.api.nvim_get_current_buf()
  local ok = vim.wait(5000, function()
    return #vim.lsp.get_clients({ bufnr = buf, name = 'requimd' }) > 0
  end, 20)
  return buf, ok
end

local buf, attached = open(examples .. '/shop/functional.md')
check('FR-200/FR-308 server starts and attaches', attached)
if not attached then os.exit(1) end
local client = vim.lsp.get_clients({ bufnr = buf, name = 'requimd' })[1]
check('position encoding negotiated', client.offset_encoding == 'utf-8', client.offset_encoding)

local function request(method, params, b)
  local res = vim.lsp.buf_request_sync(b or buf, method, params, 5000)
  local r = res and res[client.id]
  return r and r.result, r and r.err
end

local function at(line, col, extra)
  local p = { textDocument = { uri = vim.uri_from_bufnr(buf) }, position = { line = line, character = col } }
  for k, v in pairs(extra or {}) do p[k] = v end
  return p
end

-- Line 4 (1-based) of functional.md: "impl: UC-012.s1  prio: must".
local hover = request('textDocument/hover', at(3, 10))
check('FR-205 hover', hover and hover.contents.value:find('Customer opens the cart', 1, true) ~= nil)

local def = request('textDocument/definition', at(3, 10))
def = def and (def.uri and def or def[1])
check('FR-206 definition', def and def.uri:find('usecases.md', 1, true) ~= nil and def.range.start.line == 7)

local refs = request('textDocument/references', at(3, 10, { context = { includeDeclaration = false } }))
check('FR-207 references', refs and #refs == 1, refs and tostring(#refs))

local impl = request('textDocument/implementation', at(3, 10))
check('FR-208 implementation', impl and #impl == 1, impl and tostring(#impl))

local syms = request('textDocument/documentSymbol', { textDocument = { uri = vim.uri_from_bufnr(buf) } })
check('FR-210 document symbols', syms and #syms == 5, syms and tostring(#syms))

local ws = request('workspace/symbol', { query = 'cart' })
check('FR-211 workspace symbols', ws and #ws >= 2, ws and tostring(#ws))

local comp = request('textDocument/completion', at(3, 6, { context = { triggerKind = 1 } }))
local labels = {}
for _, item in ipairs(comp and comp.items or {}) do labels[item.label] = true end
check('FR-203 reference completion', labels['UC-012.s2a'] == true and labels['FR-101'] == nil)

local vcomp = request('textDocument/completion', at(3, 23, { context = { triggerKind = 1 } }))
local values = {}
for _, item in ipairs(vcomp and vcomp.items or {}) do values[item.label] = true end
check('FR-204 value completion', values['should'] == true)

-- FR-212 on an unsaved placeholder, applied through the client.
local n = vim.api.nvim_buf_line_count(buf)
vim.api.nvim_buf_set_lines(buf, n, n, false, { '', '## FR-? New one' })
vim.wait(200)
local actions = request('textDocument/codeAction', {
  textDocument = { uri = vim.uri_from_bufnr(buf) },
  range = { start = { line = n + 1, character = 4 }, ['end'] = { line = n + 1, character = 4 } },
  context = { diagnostics = {} },
})
local action = actions and actions[1]
check('FR-212 assign ID offered', action and action.title == 'Assign ID FR-106', action and action.title)
if action then
  vim.lsp.util.apply_workspace_edit(action.edit, client.offset_encoding)
  check('FR-212 assign ID applied', vim.api.nvim_buf_get_lines(buf, n + 1, n + 2, false)[1] == '## FR-106 New one')
end
vim.cmd('silent! undo')
vim.cmd('silent! undo')

-- M3 (should items): inlay hints, glossary hover, code lens, rename.
local hints = request('textDocument/inlayHint', {
  textDocument = { uri = vim.uri_from_bufnr(buf) },
  range = { start = { line = 0, character = 0 }, ['end'] = { line = 10, character = 0 } },
})
check('FR-209 inlay hints', hints and hints[1] and hints[1].label == 'Customer opens the cart.')

-- Line 6 (1-based): "When the customer opens the cart, ..."
local ghover = request('textDocument/hover', at(5, 30))
check('FR-205 glossary hover', ghover and ghover.contents.value:find('not yet ordered', 1, true) ~= nil)

local ren = request('textDocument/rename', at(3, 8, { newName = 'UC-099' }))
local edits = 0
for _, list in pairs(ren and ren.changes or {}) do edits = edits + #list end
check('FR-214 rename', edits >= 6, tostring(edits))

-- The code lens command handler recommended in docs/neovim.md.
vim.lsp.commands['requimd.implementations'] = function(command, ctx)
  local c = vim.lsp.get_client_by_id(ctx.client_id)
  local items = vim.lsp.util.locations_to_items(command.arguments[1], c.offset_encoding)
  vim.fn.setqflist({}, ' ', { title = command.title, items = items })
end
local ucbuf = open(examples .. '/shop/usecases.md')
local lenses = request('textDocument/codeLens', { textDocument = { uri = vim.uri_from_bufnr(ucbuf) } }, ucbuf)
local lens = lenses and lenses[2]
check('FR-215 code lens', lens and lens.command.title == '2 impl: FR-104, FR-105', lens and lens.command.title)
if lens then
  vim.lsp.commands['requimd.implementations'](lens.command, { client_id = client.id })
  check('FR-215 code lens lists implementations', #vim.fn.getqflist() == 2)
end
vim.api.nvim_set_current_buf(buf)

-- FR-219 in a scratch project: "Allow in project" edits requimd.toml.
local scratch = vim.fn.tempname()
vim.fn.mkdir(scratch, 'p')
vim.fn.writefile({ '# my comment' }, scratch .. '/requimd.toml')
vim.fn.writefile({ '## FR-001 X', '', 'The system shall be fast.' }, scratch .. '/fr.md')
local frbuf = open(scratch .. '/fr.md')
vim.wait(5000, function() return #vim.diagnostic.get(frbuf) == 1 end, 20)
local sclient = vim.lsp.get_clients({ bufnr = frbuf, name = 'requimd' })[1]
local res = vim.lsp.buf_request_sync(frbuf, 'textDocument/codeAction', {
  textDocument = { uri = vim.uri_from_bufnr(frbuf) },
  range = { start = { line = 2, character = 21 }, ['end'] = { line = 2, character = 21 } },
  context = { diagnostics = {} },
}, 5000)
local allow
for _, a in ipairs(res and res[sclient.id] and res[sclient.id].result or {}) do
  if a.title:find('in project', 1, true) then allow = a end
end
if allow then sclient:exec_cmd(allow.command, { bufnr = frbuf }) end
local cleared = vim.wait(5000, function() return #vim.diagnostic.get(frbuf) == 0 end, 20)
local toml = table.concat(vim.fn.readfile(scratch .. '/requimd.toml'), '\n')
check('FR-219 allow in project', allow ~= nil and cleared
  and toml:find('# my comment', 1, true) ~= nil and toml:find('allow = ["fast"]', 1, true) ~= nil, toml)
vim.api.nvim_set_current_buf(buf)

-- FR-216: Neovim applies the semantic tokens without configuration.
local function token_at(row, col)
  local t
  vim.wait(5000, function()
    t = vim.lsp.semantic_tokens.get_at_pos(buf, row, col)
    return t and #t > 0
  end, 20)
  return t and t[1] or {}
end
local decl = token_at(2, 4) -- "## FR-101 Show cart"
check('FR-216 tokens: heading ID', decl.type == 'class' and decl.modifiers.declaration == true, decl.type)
check('FR-216 tokens: metadata key', token_at(3, 1).type == 'property', token_at(3, 1).type)
check('FR-216 tokens: reference', token_at(3, 8).type == 'class', token_at(3, 8).type)
-- Line 6 (1-based): "When the customer opens the cart, ..."
check('FR-216 tokens: glossary term', token_at(5, 30).type == 'namespace', token_at(5, 30).type)

-- M4: saving records links (FR-218), a changed target makes the link
-- suspect (FR-116), and the code action marks it as reviewed.
local m4 = vim.fn.tempname()
vim.fn.mkdir(m4, 'p')
vim.fn.writefile({}, m4 .. '/requimd.toml')
vim.fn.writefile({ '## UC-001 Pay', '', '- s1: Customer pays.' }, m4 .. '/uc.md')
vim.fn.writefile({ '## FR-001 Charge', 'impl: UC-001.s1', '', 'The system shall charge.' }, m4 .. '/fr.md')
local m4fr = open(m4 .. '/fr.md')
vim.wait(3000, function() return #vim.lsp.get_clients({ bufnr = m4fr, name = 'requimd' }) > 0 end, 20)
vim.cmd('silent write')
local saved = vim.wait(3000, function() return vim.fn.filereadable(m4 .. '/requimd.lock') == 1 end, 20)
check('FR-218 links recorded on save', saved)
local m4uc = open(m4 .. '/uc.md')
vim.api.nvim_buf_set_lines(m4uc, 2, 3, false, { '- s1: Customer pays by card.' })
local suspect = vim.wait(3000, function()
  for _, d in ipairs(vim.diagnostic.get(m4fr)) do
    if d.code == 'suspect-link' then return true end
  end
  return false
end, 20)
check('FR-116 suspect link', suspect)
local m4client = vim.lsp.get_clients({ bufnr = m4fr, name = 'requimd' })[1]
local acts = vim.lsp.buf_request_sync(m4fr, 'textDocument/codeAction', {
  textDocument = { uri = vim.uri_from_bufnr(m4fr) },
  range = { start = { line = 1, character = 8 }, ['end'] = { line = 1, character = 8 } },
  context = { diagnostics = {} },
}, 5000)
local mark = acts and acts[m4client.id] and acts[m4client.id].result and acts[m4client.id].result[1]
if mark and mark.command then m4client:exec_cmd(mark.command, { bufnr = m4fr }) end
local cleared = vim.wait(3000, function() return #vim.diagnostic.get(m4fr) == 0 end, 20)
check('FR-218 mark as reviewed', mark ~= nil and cleared, mark and mark.title)
vim.cmd('silent! %bwipeout!')
buf = open(examples .. '/shop/functional.md')

-- FR-202: diagnostics of the broken example, then live after an edit.
local broken = open(examples .. '/broken/broken.md')
local got = vim.wait(5000, function() return #vim.diagnostic.get(broken) > 0 end, 20)
local count = #vim.diagnostic.get(broken)
check('FR-202 diagnostics on open', got and count == 22, tostring(count))
vim.api.nvim_buf_set_lines(broken, 10, 11, false, { 'impl: UC-001.s1  prio: must' })
vim.wait(3000, function() return #vim.diagnostic.get(broken) == 20 end, 20)
count = #vim.diagnostic.get(broken)
check('FR-202 diagnostics after change', count == 20, tostring(count))

-- Default keys from docs/neovim.md are mapped (Neovim 0.11+ defaults).
local function mapped(lhs)
  return vim.fn.maparg(lhs, 'n') ~= ''
end
vim.api.nvim_set_current_buf(buf)
check('keys K, grr, gri, gra, grn, gO, ]d', mapped('K') and mapped('grr') and mapped('gri')
  and mapped('gra') and mapped('grn') and mapped('gO') and mapped(']d'))

-- FR-217: a file created outside the editor, in a scratch project.
local dir = vim.fn.tempname()
vim.fn.mkdir(dir, 'p')
vim.fn.writefile({}, dir .. '/requimd.toml')
vim.fn.writefile({ '## UC-001 U', '', '- s1: Go.' }, dir .. '/uc.md')
local uc = open(dir .. '/uc.md')
vim.wait(5000, function() return #vim.diagnostic.get(uc) == 1 end, 20)
local watches = vim.lsp.get_clients({ bufnr = uc, name = 'requimd' })[1]
  .capabilities.workspace.didChangeWatchedFiles.dynamicRegistration
vim.fn.writefile({ '## FR-001 F', 'impl: UC-001.s1' }, dir .. '/fr.md')
local covered = vim.wait(6000, function() return #vim.diagnostic.get(uc) == 0 end, 50)
check('FR-217 external file change', covered,
  watches and 'client file watching' or 'server polling')

-- Clean shutdown through shutdown/exit.
for _, c in ipairs(vim.lsp.get_clients()) do c:stop() end
local stopped = vim.wait(5000, function() return #vim.lsp.get_clients() == 0 end, 20)
check('shutdown', stopped)
os.exit(failed == 0 and 0 or 1)
