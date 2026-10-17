# ADR-0011 Offline by default

Status: **proposed** on 2026-10-17.

## Context

NFR-006 asks for zero network connections from `check`, `fmt` and `lsp`.
The other commands are not covered (a gap found by TC-605), although none
of them opens a connection; `requimd diff` calls the local `git` command.
ADR-0008 rejected downloading the executable on first start partly for
this reason.

requimd runs in CI and on analysts' machines behind corporate proxies and
firewalls, and the requirements it reads are often confidential. Two ideas
on the roadmap would need the network: an AI review (B4 in
`docs/feature-report.md`), unless it uses a local model, and fetching the
titles of issue tracker links (B1).

## Decision

Decision F5 of the feature report (2026-10-16):

1. **requimd opens no network connection by default.** NFR-006 covers all
   commands.
2. **A command may use the network only on explicit request:** the user
   invokes it for that purpose and configures it, for example with an
   endpoint in `requimd.toml`. No command uses the network as a side
   effect, and `check`, `fmt` and `lsp` never do.
3. **Each such command needs an ADR of its own** that says what is sent to
   which service.

## Considered

- **Always offline:** the strictest promise, but an AI review inside
  requimd would be impossible for good.
- **Decide with the first network feature:** no work now, but no principle
  to measure that feature against.

## Consequences

- NFR-006 changes its scale to all commands, with the exception of
  explicit network commands; its meter stays.
- B4 and a lookup of issue titles for B1 need an ADR before they are
  built.
- Requirements never leave the machine through `check`, the language
  server or CI runs of requimd.
