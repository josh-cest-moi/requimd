# DD-7 Spec-driven development and agent interfaces: requimd as the deterministic, governed layer beneath agent-written specs

Scan date: 18 November 2026. Unlike Phase 1, this deep dive opened the primary sources: the Spec Kit README, quickstart, releases and the `analyze` and spec templates; the kiro.dev pages for specs, feature specs, correctness, steering and hooks; the OpenSpec README, `docs/concepts.md`, `docs/cli.md` and the validator source (`src/core/validation/validator.ts`); agents.md; llmstxt.org; the MCP specification 2026-07-28 (overview, changelog, tools, transports, stdio, schema); the official Rust MCP SDK; the tracey and Reqvire READMEs; and the Jama MCP press release. Tessl's 2026 status, the Jama tool list and the community Jama servers come from search results only, and are marked as such. Nothing was installed or run.

## What do Spec Kit, Kiro, OpenSpec, Tessl and AGENTS.md define, and what do their validators check and not check?

### Takeaway
All four SDD tools store Markdown specs in the repository, and all changed visibly during 2026. Only OpenSpec has a deterministic validator in code, and it checks structure only: headers, SHALL/MUST, scenarios, delta conflicts and scenario loss. There are no IDs, no cross-references, no traceability, no glossary, no approval state and no change impact. Spec Kit's `analyze` and Kiro's "Analyze Requirements" are LLM-executed reviews, so they are useful but not deterministic. AGENTS.md has no schema at all. None of them keeps content hashes or suspect links, and none gives requirements stable IDs across features. Those are exactly requimd's strengths.

### Cited Findings

**GitHub Spec Kit (MIT, Python CLI `specify`, version 1.4.2 of 17 Nov 2026)**
- Licence MIT. It needs Python 3.11+ and uv. GitHub Copilot is the example agent, and other agents use "its equivalent invocation syntax" — [Spec Kit README](https://github.com/github/spec-kit)
- **Commands, current naming (hyphenated):**
  - `/speckit-constitution`: sets project principles, run once per project.
  - `/speckit-specify`: creates the feature spec ("what and why").
  - `/speckit-clarify`: asks targeted questions and writes the answers back into the spec.
  - `/speckit-plan`: creates design artifacts (stack, architecture).
  - `/speckit-checklist`: creates a custom quality checklist that "reviewers own".
  - `/speckit-tasks`: writes a dependency-ordered `tasks.md`.
  - `/speckit-analyze`: a read-only check across `spec.md`, `plan.md` and `tasks.md`.
  - `/speckit-implement`: checks the checklist state first, then implements.
  - `/speckit-converge`: checks the codebase against spec, plan and tasks and appends tasks for gaps. You repeat implement and converge until it reports "Converged".
  - A shorter path skips clarify, checklist, analyze and the constitution.
  
  — [Spec Kit quickstart](https://github.github.io/spec-kit/quickstart.html); [README](https://github.com/github/spec-kit)
- **Naming conflict:** the docs index still shows the dotted form `/speckit.specify`, while the README and quickstart use the hyphen form — [Spec Kit docs](https://github.github.io/spec-kit/)
- **Files:**
  - `.specify/feature.json` records the active feature directory.
  - `checklists/requirements.md` is the built-in checklist.
  - Branch names look like `001-feature-name` — [quickstart](https://github.github.io/spec-kit/quickstart.html)
  - The constitution lives at `/memory/constitution.md`, inside `.specify/` according to the template paths — [analyze template](https://raw.githubusercontent.com/github/spec-kit/main/templates/commands/analyze.md)
  - Sources conflict on where specs live: `specs/<branch>/spec.md` or `.specify/specs/001-…/`. One skill falls back to `SPECKIT_SPEC_DIR=".specify/specs"` — [search summary: Xebia, Context7, skill file](https://xebia.com/blog/building-software-with-spec-kit/)
- **Spec template:**
  - User stories are titled "User Story N - … (Priority: PN)". Each has "Why this priority", "Independent Test" and Given/When/Then acceptance scenarios.
  - Functional requirements are bullets numbered `FR-001`, `FR-002` (three digits, zero-padded, sequential) and start with "System MUST" or "Users MUST be able to".
  - Open points are marked inline as `[NEEDS CLARIFICATION: …]`.
  - Success criteria are numbered `SC-001`.
  - Further sections: Key Entities, Edge Cases, Assumptions.
  
  — [spec-template.md](https://raw.githubusercontent.com/github/spec-kit/main/templates/spec-template.md)
- **What `analyze` checks:** it is a command template, that is, instructions the agent executes. It has six detection passes:
  - A. Duplication
  - B. Ambiguity: vague terms such as "fast" or "secure", and placeholders TODO, TKTK and `???`
  - C. Underspecification
  - D. Constitution alignment
  - E. Coverage gaps: requirements with no task, tasks with no requirement, success criteria with no task, and constrained fields without an `Entity.field` citation in `data-model.md`
  - F. Inconsistency: terminology drift, entities missing from some files, task ordering, conflicting technology choices
  
  Severities are CRITICAL, HIGH, MEDIUM and LOW. The output has a findings table capped at 50 rows, a requirement coverage table (FR/SC key, has task, task IDs) and metrics (coverage %, ambiguity count). It modifies no files, never changes the constitution, and needs `tasks.md` to exist — [analyze.md](https://raw.githubusercontent.com/github/spec-kit/main/templates/commands/analyze.md)
- **Churn in November 2026:**
  - 1.3.0 (3 Nov) removed the core `taskstoissues` command in favour of a `github` extension.
  - 1.4.x added a community catalogue of extensions and presets (Jira, Azure DevOps lifecycle, Multi-Repo Workspace) and "task-generation checks against canonical field constraints and analyze drift".
  
  — [Spec Kit releases](https://github.com/github/spec-kit/releases)
- **Presets** "adapt templates, commands, terminology, or methodology" and can enforce organisational or regulatory standards. **Extensions** add commands and quality gates, for example `bug` and `assess` — [README](https://github.com/github/spec-kit)
- The docs index mentions "an experimental MCP server" among the CLI references. Its contents were not found. Search found only third-party Spec Kit MCP servers (`@anyicode/spec-kit-mcp`, the Rust crate `spec-kit-mcp`, `jthom233/speckitmcp`), which wrap the CLI — [Spec Kit docs](https://github.github.io/spec-kit/); [glama listing](https://glama.ai/mcp/servers/@anyiba/spec-kit-mcp)

**AWS Kiro (proprietary IDE, CLI, Web and Mobile; pages updated Aug–Nov 2026)**
- **Spec types:** Feature Specs (Requirements-First or Design-First), Bugfix Specs (a `bugfix.md` that records current, expected and unchanged behaviour) and Quick Spec. Files are `requirements.md` (or `bugfix.md`), `design.md` and `tasks.md` — [kiro.dev specs, updated 12 Nov 2026](https://kiro.dev/docs/specs/)
- **requirements.md** uses EARS: "WHEN [condition/event] THE SYSTEM SHALL [expected behavior]". The page does not state the numbering scheme — [feature specs, updated 4 Aug 2026](https://kiro.dev/docs/specs/feature-specs/)
- **New in 2026:**
  - "Run all tasks" builds a dependency graph and runs tasks in waves ("Waves execute sequentially; tasks within a wave execute concurrently").
  - "Analyze Requirements" (IDE and CLI) looks for inconsistencies, ambiguities and gaps before design.
  - "Correctness" (IDE only) does property-based testing.
  - A Markdown Review mode lets you comment on passages.
  
  — [kiro.dev specs](https://kiro.dev/docs/specs/)
- **Correctness:**
  - Kiro extracts testable properties from EARS requirements and generates property-based tests. Hovering over a property shows its link to the requirement and the task.
  - The page itself says PBT "provides evidence of correctness, not formal proof", and that weak properties can pass while the behaviour is wrong.
  
  — [kiro.dev correctness](https://kiro.dev/docs/specs/correctness/)
- **Steering:**
  - Location: `.kiro/steering/` (workspace) and `~/.kiro/steering/` (global; workspace wins). The default files are `product.md`, `tech.md` and `structure.md`.
  - Front matter sets the inclusion mode: `inclusion: always` (default), `fileMatch` with `fileMatchPattern`, `manual` (`#name` in chat), or `auto` with `name` and `description`.
  - Live file references use `#[[file:path]]`, `#[[file:path:start-end]]` and `#[[folder:…]]`.
  - **Kiro reads AGENTS.md** in the workspace root and in subdirectories, always included, with no inclusion modes.
  
  — [kiro.dev steering](https://kiro.dev/docs/steering/)
- **Hooks:**
  - Each hook is a JSON file in `.kiro/hooks/` (global in `~/.kiro/hooks/`) with `version: "v1"` and a `hooks` array.
  - Fields per hook: `name`, `trigger`, `action`, `matcher`, `timeout` (60 s default), `enabled` and `confirm`.
  - Triggers named on the page: `PostFileSave` (regex on path), `PreToolUse` and `PostToolUse` (matcher can be `@mcp` or `@<server>`) and `Stop`.
  - Actions: `"command"` runs a shell command and passes session context as JSON on stdin; `"agent"` injects a prompt.
  
  — [kiro.dev hooks, updated 12 Nov 2026](https://kiro.dev/docs/hooks/)

**OpenSpec (Fission-AI, MIT, 2.x in 2026)**
- **Layout:**
  - `openspec/specs/<domain>/spec.md` is the source of truth.
  - `openspec/changes/<name>/` holds `proposal.md`, `design.md`, `tasks.md`, an optional `.openspec.yaml` and `specs/` with delta specs.
  - Archived changes move to `openspec/changes/archive/YYYY-MM-DD-<name>/`.
  - A schema (`openspec/schemas/<name>/schema.yaml`) defines an artifact graph. The default `spec-driven` runs proposal → specs → design → tasks → implement. Dependencies are "enablers rather than gates".
  
  — [OpenSpec concepts](https://raw.githubusercontent.com/Fission-AI/OpenSpec/main/docs/concepts.md)
- **Format:**
  - A requirement is `### Requirement: <Name>` with RFC 2119 keywords (SHALL, MUST and so on). Under it come one or more `#### Scenario: <name>` blocks with GIVEN/WHEN/THEN bullets.
  - Delta headings are `## ADDED Requirements`, `## MODIFIED Requirements`, `## REMOVED Requirements` and `## Purpose` (new capability only).
  - On archive, ADDED items are appended, MODIFIED items replace the existing requirement, and REMOVED items are deleted. Retiring a whole capability needs `retire_capabilities: true`.
  
  — [concepts.md](https://raw.githubusercontent.com/Fission-AI/OpenSpec/main/docs/concepts.md)
  - `## RENAMED Requirements` uses `FROM:` / `TO:` lines. A behaviour change needs RENAMED plus a MODIFIED entry under the new name — [community guide, search result](https://oyi77.github.io/openspec/AGENTS); confirmed by the validator messages below.
- **Requirements are identified by their header name, not by an ID.** No ID syntax appears in the format docs — [concepts.md](https://raw.githubusercontent.com/Fission-AI/OpenSpec/main/docs/concepts.md)
- **CLI:**
  - `openspec validate [item] --all|--changes|--specs|--archived|--strict|--json`. The JSON output has `version`, `results.changes[]` with `name`, `valid` and `warnings`, and a `summary`.
  - Other commands: `show --json --deltas-only|--requirements`, `list --json`, `archive -y --skip-specs --no-validate`, `status --json`, `instructions`, `context`, `store`, `workset` and `schema`.
  - Exit code 1 means a validation failure. `archive` validates, merges and moves the change, and restores the specs if a step fails.
  
  — [OpenSpec CLI docs](https://raw.githubusercontent.com/Fission-AI/OpenSpec/main/docs/cli.md)
- **Slash commands in 2.x:**
  - Default profile: `/openspec-explore`, `/openspec-propose`, `/openspec-apply-change` and `/openspec-archive-change`.
  - Expanded profile adds `new`, `continue`, `ff`, `verify`, `bulk-archive` and `onboard`.
  - The v1 `/opsx:*` commands were replaced, with an "Upgrade to 2.0" guide.
  - The README claims 30+ supported tools and 75.2k stars.
  - **Telemetry is on by default** (command names and version), disabled in CI and with `DO_NOT_TRACK=1`.
  
  — [OpenSpec README](https://github.com/Fission-AI/OpenSpec)
- **What `openspec validate` checks (deterministic TypeScript):**
  - **Errors:**
    - spec missing a Purpose or Requirements section
    - requirement without SHALL or MUST in its body
    - change without deltas (unless `skip_specs: true`)
    - a `specs/spec.md` at the wrong place
    - a RENAMED `FROM:` without a matching `TO:`, and the reverse
    - duplicate requirements within a section, and duplicate FROM or TO
    - an ADDED or MODIFIED requirement without a scenario
    - **scenario loss**: a MODIFIED requirement that "omits scenario(s) the current spec still has"
    - cross-section conflicts: MODIFIED+REMOVED, MODIFIED+ADDED, ADDED+REMOVED, RENAMED+REMOVED, a "MODIFIED references old name from RENAMED", and a "RENAMED TO collides with ADDED"
  - **Warnings** (fail only with `--strict`):
    - placeholder or too-brief Purpose
    - requirement too long
    - a main-spec requirement without scenarios
    - missing SHALL or MUST
    - a scenario missing its WHEN/GIVEN or THEN step
    - missing task checkboxes and task numbering problems
    - unknown metadata keys
  - **Info:** a dry run of the archive merge reports "Archive would refuse this delta".
  
  — [validator.ts](https://raw.githubusercontent.com/Fission-AI/OpenSpec/main/src/core/validation/validator.ts)

**Tessl (status 2026, secondary sources only)**
- The Spec Registry launched in open beta on 16 Sep 2025 and the Framework in closed beta — [Tessl blog (search result)](https://tessl.io/blog/tessl-launches-spec-driven-framework-and-registry/)
- On 29 Jan 2026 Tessl repositioned the registry as a **skills registry**: "versioned, evaluated bundles of instructions, docs, and rules", installable with `tessl install` — [Tessl blog, 29 Jan 2026 (search result)](https://tessl.io/blog/skills-are-software-and-they-need-a-lifecycle-introducing-skills-on-tessl/); [codemyspec review (search result)](https://codemyspec.com/blog/tessl-review)
- Sources conflict on the Framework. One says development was paused and the Framework was removed from the rebuilt CLI. Another says it had still not reached GA by mid-2026 — [codemyspec](https://codemyspec.com/blog/tessl-review); [rywalker research](https://rywalker.com/research/tessl) (both search results, not opened)

**AGENTS.md**
- An open format, "just standard Markdown", with no required fields. The suggested sections are project overview, build and test commands, code style, testing instructions and security. "The closest AGENTS.md to the edited file wins; explicit user chat prompts override everything." — [agents.md](https://agents.md/)
- It claims use in over 60k open-source projects and is stewarded by the Agentic AI Foundation under the Linux Foundation.
- Agents listed: Codex, Jules, Factory, Aider, goose, opencode, Zed, Warp, VS Code, Devin, Junie, Amp, Cursor, RooCode, Gemini CLI, Kilo Code, the Copilot coding agent, Windsurf, Augment and others. **Claude Code is not on that list** — [agents.md](https://agents.md/)
- Kiro also reads AGENTS.md (see above) — [kiro.dev steering](https://kiro.dev/docs/steering/)

### Inferences
- **What none of them checks:** stable, project-wide IDs, which are what requimd's BR-002 and BR-003 protect. Also absent:
  - typed links between requirements, and from requirements to tests
  - glossary consistency, beyond `analyze`'s LLM "terminology drift"
  - approval status, and who may set it
  - content hashes and suspect links after a change
  - reproducible diagnostics that CI can gate on
  
  OpenSpec's validator is the only deterministic one, and it is structural.
- **Spec Kit's FR-001 numbering is per feature folder**, so FR-001 recurs in every spec. In a repository that also holds requimd items, an agent will meet two incompatible FR-001s. Interoperability must address this; see the proposal.
- **Spec Kit's `analyze`, Kiro's "Analyze Requirements" and OpenSpec's `/openspec-verify-change` are agent prompts.** Their coverage tables are inferred by an LLM each time. That is the "false sense of control" Böckeler described in Phase 1, and it is where a deterministic `requimd check` / `coverage` adds value beneath them.
- **Churn favours a neutral, file-based layer.** Spec Kit renamed its command syntax and removed a core command within one month. OpenSpec replaced `/opsx:*` with 2.0 names. Tessl pivoted. Integrations should therefore target stable mechanisms (AGENTS.md, MCP, CLI with JSON, Kiro hook JSON) rather than any tool's slash commands.

### Gaps
- Kiro's requirement and acceptance-criterion numbering, and the requirement references in `tasks.md`, are not documented on the pages opened. Phase 1's "tasks traced to requirement numbers" remains secondary.
- The Spec Kit "experimental MCP server" in the CLI reference was not found; its tool list is unknown.
- What OpenSpec `--strict` adds was read from the validator source, not from user docs. RENAMED syntax beyond `FROM:`/`TO:` comes from a community guide.
- Tessl: no primary 2026 page was opened. Its spec-as-source status is unverified.

## How could requimd map onto these tools: OpenSpec deltas from `requimd diff`, validating Kiro or Spec Kit files, or requimd as the source they read?

### Takeaway
OpenSpec's delta vocabulary maps almost one to one onto `requimd diff` (FR-403), with one difference: OpenSpec matches requirements by name, requimd by ID. A `--format delta` rendering of `requimd diff` is cheap and gives reviewers OpenSpec-style change proposals, plus suspect-link impact that OpenSpec lacks. The reverse direction is weak. Validating Kiro or Spec Kit files natively, or importing OpenSpec specs, would mean ID-less items or generated IDs, which conflicts with BR-002. **requimd items should be the source that these tools read**, through AGENTS.md, a context bundle and MCP, while their tasks and specs cite requimd IDs.

### Cited Findings
- `requimd diff <ref1> [ref2]` matches items by ID: a renamed ID shows as removed plus added. An item counts as changed if its title, text, links, other metadata, status (old and new value), use-case steps (labels added, removed or changed) or file differ. `--format json` gives a JSON array. Exit code 2 on git failure — [FR-403, requimd export.md](C:/Users/grafj/projects/reqmd/requirements/functional/export.md)
- OpenSpec's delta categories are ADDED, MODIFIED (full replacement text), REMOVED and RENAMED (FROM/TO, name only). The validator rejects a MODIFIED requirement that loses scenarios, and rejects cross-section conflicts — [concepts.md](https://raw.githubusercontent.com/Fission-AI/OpenSpec/main/docs/concepts.md); [validator.ts](https://raw.githubusercontent.com/Fission-AI/OpenSpec/main/src/core/validation/validator.ts)
- OpenSpec's archive merges deltas into `specs/`. If a step fails, the specs are restored — [OpenSpec CLI](https://raw.githubusercontent.com/Fission-AI/OpenSpec/main/docs/cli.md)
- Jama positions OpenSpec as the planning layer and its own platform as the governed record. It calls OpenSpec proposals "not a governed requirements record" — [Jama OpenSpec guide (Phase 1)](https://www.jamasoftware.com/blog/openspec-guide/)
- Spec Kit presets can change templates and terminology and "enforce organizational or regulatory standards" — [Spec Kit README](https://github.com/github/spec-kit)
- Kiro steering supports live file references (`#[[file:…]]`) and reads AGENTS.md. Kiro hooks can run shell commands on `PostFileSave` and get context as JSON on stdin — [steering](https://kiro.dev/docs/steering/); [hooks](https://kiro.dev/docs/hooks/)
- requimd's parser reads items only from headings with an ID (`## FR-104 VAT per line item`) followed by a metadata block — [FR-001, FR-003, format.md](C:/Users/grafj/projects/reqmd/requirements/functional/format.md). Spec Kit's FRs are bullets (`- **FR-001**: System MUST …`) — [spec-template](https://raw.githubusercontent.com/github/spec-kit/main/templates/spec-template.md). OpenSpec's are `### Requirement: Name` headings without IDs — [concepts.md](https://raw.githubusercontent.com/Fission-AI/OpenSpec/main/docs/concepts.md)

### Inferences
**Mapping `requimd diff` onto OpenSpec deltas:**

| requimd diff result | OpenSpec delta | Note |
|---|---|---|
| item added | ADDED | full item text |
| item removed | REMOVED | requimd also lists incoming links that now dangle (from `check`) |
| item changed: only the title | RENAMED (FROM title / TO title) | the requimd ID stays stable, which is better than OpenSpec's name matching |
| item changed: text, links or metadata | MODIFIED | full new text, plus the changed parts |
| changed steps of a use case | MODIFIED, step labels added, removed or changed | analogous to OpenSpec's scenario-loss rule; requimd could warn when an incoming `impl` targets a removed step |
| status change | MODIFIED (status only) | OpenSpec has no status; show it separately as a "governance" change |
| ID renamed | REMOVED + ADDED | as specified in FR-403; BR-002 forbids renumbering anyway |

**Recommended delta work:**
- Add `requimd diff <ref> --format delta` (Markdown, grouped per file, under ADDED/MODIFIED/REMOVED/RENAMED headings). For each changed item it should also give **suspect impact**: the links that will become suspect when the lock file is compared. This is the "change proposal view" from Phase 1. Merge-base as the default `ref1` for PR review would be a convenience.
- Do **not** generate an `openspec/changes/` folder by default. Teams that use OpenSpec for code changes can let the proposal reference requimd IDs, for example "Implements FR-012, changes FR-014; see `requimd diff`".

**Validating foreign files:**
- Kiro and Spec Kit requirement files are not requimd items. Making requimd parse them would mean second-class ID-less items, or IDs that change per feature.
- A lighter option is a language-only mode: run FR-110 weak words, FR-111 "shall", FR-112 EARS and the glossary aliases over arbitrary Markdown paragraphs. Kiro's EARS `requirements.md` would benefit most.
- This is an owner decision; see the open questions.

**Recommended direction: requimd is the source.**
- The analyst's approved items are the requirements level.
- Kiro's `requirements.md`, Spec Kit's `spec.md` and OpenSpec's change specs become *change or task-level* artefacts that cite requimd IDs.
- The AGENTS.md section, a Kiro steering file with `#[[file:requirements/…]]` references and MCP tools carry the context.
- A Spec Kit preset, from the community or later from requimd, could replace "FR-###" in its template with "Implements: <requimd ID>". That is an inference from the preset mechanism; no such preset was found.

### Gaps
- No public example was found of a team combining an RM tool's IDs with Spec Kit or Kiro files in one repository. The ID collision risk is reasoned, not observed.
- Whether OpenSpec's archive merge can be driven from outside (a requimd-generated delta applied by `openspec archive`) was not tested; nothing was run.

## MCP in 2026: the current specification, and what requirements MCP servers expose

### Takeaway
The current MCP revision is **2026-07-28**, a breaking redesign:
- stateless, with no `initialize` handshake
- metadata carried in `_meta` on every request
- a mandatory `server/discover`
- roots, sampling and logging deprecated
- tasks moved to an extension
- stdio still one of two standard transports

Tools carry hint annotations (`readOnlyHint`, `destructiveHint`, `idempotentHint`, `openWorldHint`) and can return `structuredContent` validated against an `outputSchema`. Requirements MCP servers cluster around get item, search, trace up and down, coverage and validate. Write tools are usually gated behind an explicit flag (Reqvire's `--enable-mutations`, read-only community Jama servers). Jama's official server (May 2026) enforces existing permissions and audit trails, but its tool list is not public.

### Cited Findings

**MCP specification 2026-07-28**
- Servers offer Resources, Prompts and Tools. Clients offer Elicitation. The protocol is JSON-RPC 2.0 with "stateless, self-contained requests" and per-request capability negotiation — [MCP spec overview](https://modelcontextprotocol.io/specification/latest)
- **Major changes since 2025-11-25:**
  1. Protocol sessions and `Mcp-Session-Id` are removed, and the `initialize` handshake with them. Every request carries `io.modelcontextprotocol/protocolVersion` and `clientCapabilities` in `_meta`.
  2. Servers MUST implement `server/discover`.
  3. `subscriptions/listen` replaces resource subscriptions.
  4. `ping` and `logging/setLevel` are removed.
  5. Tasks become the extension `io.modelcontextprotocol/tasks`.
  6. Multi round-trip requests (`InputRequiredResult`) replace server-initiated requests.
  7. Results carry `resultType`.
  8. List results carry `ttlMs` and `cacheScope`.
  9. Servers SHOULD return tools in a deterministic order.
  
  **Deprecated:** Roots, Sampling and Logging. "Pass directories or files via tool parameters, resource URIs, or server configuration instead of Roots"; log to stderr on stdio. There is now a 12-month deprecation policy — [MCP changelog 2026-07-28](https://modelcontextprotocol.io/specification/2026-07-28/changelog)
- **stdio transport:**
  - The client launches the server as a subprocess, and messages are newline-delimited JSON-RPC with no embedded newlines.
  - The server "MUST NOT write anything to its stdout that is not a valid MCP message" and MAY log to stderr.
  - Shutdown: the client closes stdin, and servers SHOULD exit on EOF.
  - Clients that also support older versions SHOULD probe with `server/discover` and fall back to `initialize`.
  - The standard transports are stdio and Streamable HTTP.
  
  — [MCP stdio](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/stdio); [transports](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports)
- **Tools:**
  - Fields: `name`, `title`, `description`, `inputSchema` (JSON Schema 2020-12, root `type: "object"`), an optional `outputSchema` and `annotations`.
  - `structuredContent` MUST conform to `outputSchema` if one is given, and SHOULD also be serialised in a text block.
  - Errors: protocol errors are JSON-RPC errors; tool execution errors set `isError: true` so the model can self-correct.
  - Safety: "there SHOULD always be a human in the loop with the ability to deny tool invocations". Servers MUST validate inputs and sanitise outputs.
  - A "Stateful Tools" note recommends explicit server-minted handles instead of connection state.
  
  — [MCP tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools)
- **ToolAnnotations** are "hints", and "clients should never make tool use decisions based on ToolAnnotations received from untrusted servers":
  - `readOnlyHint`: default false.
  - `destructiveHint`: default true; meaningful only if not read-only.
  - `idempotentHint`: default false.
  - `openWorldHint`: default true; for example, a web search is open world and a memory tool is not.
  
  — [MCP schema 2026-07-28](https://modelcontextprotocol.io/specification/2026-07-28/schema)
- Resource templates use RFC 6570 `uriTemplate`. Prompts have `name`, `title`, `description` and `arguments` — [MCP schema](https://modelcontextprotocol.io/specification/2026-07-28/schema)
- **Security principles:** hosts must obtain explicit user consent before exposing user data to servers, and before invoking any tool — [MCP spec overview](https://modelcontextprotocol.io/specification/latest)

**Rust SDK**
- `rmcp` is "The official Rust SDK for the Model Context Protocol". It implements 2026-07-28 and stays compatible with 2025-11-25 and earlier.
  - stdio comes with the feature `transport-io`.
  - Macros: `#[tool]`, `#[tool_router]`, `#[tool_handler]`, plus prompt macros.
  - It runs on the tokio async runtime and depends on serde and schemars.
  - The legacy HTTP+SSE transport is intentionally not provided.
  - The page shows no version or licence name; it links a 3.x migration guide.
  
  — [modelcontextprotocol/rust-sdk](https://github.com/modelcontextprotocol/rust-sdk)

**Requirements MCP servers**
- **Jama (official):**
  - Launched 4 May 2026 with Jama Connect 9.35 as "the first engineering management software to deliver a Model Context Protocol (MCP) Server".
  - Named clients: Claude, Codex, Cursor, GitHub Copilot and Visual Studio. It "enforces existing permissions", lifecycle workflows and audit requirements.
  - **No tool list is published** in the press release.
  
  — [Jama press release](https://www.jamasoftware.com/press/jama-software-launches-model-context-protocol-mcp-server/)
  - It is a separate-purchase add-on — [search summary of 9.35 release notes](https://support.jamasoftware.com/hc/en-us/articles/45272636845837-Version-9-35-Cloud); the page itself returned 403.
- **Community Jama servers (search results):**
  - `t-j-thomas/jama-mcp-server`: read-only, about 21 tools, including `get_jama_item`, `get_jama_item_upstream_related`, `get_jama_item_downstream_related`, `get_jama_item_children`, `get_jama_item_types` and `get_jama_tagged_items` — [policylayer listing](https://policylayer.com/tools/t-j-thomas-jama-mcp-server); [GitHub](https://github.com/t-j-thomas/jama-mcp-server)
  - `KeisukeShima/mcp-jama`: `get_projects`, `get_items` (50 per page), `get_item`, `search_items` (wildcards), `create_item` and `update_item` — [glama](https://glama.ai/mcp/servers/KeisukeShima/mcp-jama)
  - `greeves89/jama-mcp-service`: claims 53 tools in 8 "gated toolsets" with an admin dashboard and audit trail. The repository returned 404 when fetched — [search result](https://github.com/greeves89/jama-mcp-service)
- **tracey:**
  - MCP tools: `tracey_status`, `tracey_uncovered`, `tracey_untested`, `tracey_stale`, `tracey_unmapped`, `tracey_rule`, `tracey_config` and `tracey_validate` ("and more").
  - It runs over stdio and auto-starts a daemon. `tracey ai` registers the server and installs a skill for Claude or Codex.
  - Its concepts are uncovered (no impl), untested (impl but no verify) and stale (a reference to an older rule version `r[id+N]`).
  
  — [tracey](https://github.com/bearcove/tracey)
- **Reqvire 1.5.2:**
  - `reqvire mcp` defaults to **HTTP at 127.0.0.1:8081**. `--transport stdio` runs without a listening port.
  - Authoring and CRUD tools need `--enable-mutations`, and commits need `--enable-commits`. `reqvire.format` (preview) and `reqvire.lint` remain available read-only.
  - Named tools: `reqvire.branch.list/create/open/close/refresh`.
  - Four skill packages ship separately.
  
  — [Reqvire](https://github.com/reqvire-org/reqvire)
- **StrictDoc:** `dapperfu/strictdoc-mcp` offers tools only, over stdio. **Doorstop:** `jannled/doorstopmcp` offers create, retrieve and manage, and is not production-ready — [Phase 1 notes: strictdoc-mcp](https://github.com/dapperfu/strictdoc-mcp); [agentseal listing](https://agentseal.org/mcp/https-githubcom-jannled-doorstopmcp)

### Inferences
- **The 2026-07-28 redesign suits requimd well.** Statelessness matches "every request reads the files". Deprecating roots means the project root is given as a server argument (`requimd mcp --root .`). Deterministic tool ordering and `outputSchema` fit requimd's JSON outputs, such as FR-301 and FR-402's format version.
- **Not all agent hosts will speak 2026-07-28 yet.** Supporting the legacy `initialize` era for a while matters, and rmcp claims both.
- **The common tool set** across Jama community servers, tracey and Reqvire is: get item, search, trace upstream and downstream, uncovered or untested, stale or suspect, and validate. Write tools are opt-in. requimd's proposed list matches this convergence; see the proposal.

### Gaps
- Jama's official tool names, read/write split and transport (presumably remote HTTP with auth) are not public in the sources reached. The support pages returned 403.
- ubCode's MCP tool list was not found; Phase 1's claim that it ships one is unverified here.
- rmcp's licence, current version and whether tokio can be avoided were not confirmed.

## Which tools would an agent need from requimd; are there read-only vs. write concerns; does a stdio MCP server stay within an offline single binary?

### Takeaway
An agent needs about ten read-only tools and at most two or three opt-in write tools. A stdio MCP server opens no socket: the agent host launches requimd as a child process and talks over stdin and stdout, so it fits ADR-0011's "no network connection". The new concern is not the network but **data egress through the agent's LLM**, and **hidden writes**: `requimd check` silently records new links in `requimd.lock` (FR-116), so a "read-only" check tool would write a file unless a no-write mode exists. Suspect review (FR-307) and `status: approved` must stay human-only.

### Cited Findings
- stdio means a client-launched subprocess using the standard streams; the alternative standard transport is Streamable HTTP — [MCP transports](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports)
- Reqvire's default MCP transport listens on HTTP at 127.0.0.1:8081; stdio is "a single local process with no listening port" — [Reqvire](https://github.com/reqvire-org/reqvire)
- ADR-0011:
  - "requimd opens no network connection by default". A command may use the network only on explicit request, and each such command needs its own ADR.
  - Its consequences include that "Requirements never leave the machine through `check`, the language server or CI runs of requimd."
  
  — [ADR-0011](C:/Users/grafj/projects/reqmd/docs/decisions/0011-offline-by-default.md)
- FR-116: "Links without an entry in the lock file are recorded silently with the current hash on their first check … A check never changes the hash of an existing entry; only a review does (FR-218, FR-307)." — [checks.md](C:/Users/grafj/projects/reqmd/requirements/functional/checks.md)
- FR-307: `requimd review <ID>` / `--all` marks suspect links as reviewed — [cli.md](C:/Users/grafj/projects/reqmd/requirements/functional/cli.md)
- MCP: hosts must obtain consent before exposing user data to servers and before invoking any tool. Tool annotations are untrusted hints, and `readOnlyHint` defaults to false — [MCP overview](https://modelcontextprotocol.io/specification/latest); [schema](https://modelcontextprotocol.io/specification/2026-07-28/schema)
- rmcp brings tokio, serde and schemars; requimd currently uses lsp-server (ADR-0003) — [rust-sdk](https://github.com/modelcontextprotocol/rust-sdk); [requimd CLAUDE.md](C:/Users/grafj/projects/reqmd/CLAUDE.md)
- The stdio framing is plain newline-delimited JSON-RPC — [MCP stdio](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/stdio)

### Inferences
- **Single binary:** `requimd mcp` is a subcommand like `requimd lsp`, in the same binary (NFR-005).
  - Option A: rmcp with `transport-io`. This adds tokio and schemars, and costs binary size and dependency count.
  - Option B: a small hand-written newline-delimited JSON-RPC loop with serde_json, which requimd already needs for JSON output. It is feasible because a tools-only stateless server is simple, but era negotiation (`server/discover` versus legacy `initialize`) would then be requimd's own work.
  
  Measuring both is an owner and implementation decision.
- **No HTTP transport.** A localhost listener is still a socket and invites remote exposure. Keep stdio only; any HTTP transport would need its own ADR under ADR-0011 point 3.
- **Read-only must mean really read-only.** The MCP `check` tool should run with lock writes disabled, reporting new links as "unrecorded" rather than writing `requimd.lock`. This needs a spec change, for example a `--no-lock` flag for `check`, or making first recording explicit.
- **`diff` calls local git.** Arguments must be passed as refs after a `--` separator, and refs starting with `-` rejected, to avoid option injection from agent input.
- **Governance acts stay with humans:** `review` (resetting suspect hashes), setting `status: approved` (BR-004), renumbering (BR-002 and BR-003) and deleting items. Write tools that exist should only write working-tree files on the current branch, never commit, and always return `check` diagnostics.

### Gaps
- No measurement exists of rmcp's binary-size impact on a requimd-sized binary.
- Which MCP protocol versions the analysts' likely hosts (Claude Code, Copilot, Cursor, Kiro) speak in November 2026 was not checked.

## Context export: which formats do agents use, and what should an item-with-neighbours bundle contain?

### Takeaway
There is no standard "requirement context" format. The relevant conventions are:
- llms.txt: an H1 title, a blockquote summary and H2 link lists, with an "Optional" section an agent may skip
- Repomix bundles: a file summary with purpose and usage guidelines, a directory structure and files, in XML, Markdown, JSON or plain text
- Kiro steering: live `#[[file:]]` references
- AGENTS.md: free Markdown

A requimd bundle should borrow the summary-first, skippable-tail structure. It must also add what none of these have: provenance (commit, dirty flag, format version), status and suspect state, and typed neighbours.

### Cited Findings
- **llms.txt:**
  - It sits at `/llms.txt` or a subpath. Only the H1 is required. Then come a blockquote summary, optional body paragraphs without headings, and H2 sections with `[name](url): notes` lists.
  - An H2 "Optional" section marks secondary links that can be skipped. Pages offer a `.md` twin.
  - The proposal is by Jeremy Howard, 3 Sep 2024, v2 modified 10 Aug 2026.
  
  — [llmstxt.org](https://llmstxt.org/)
- The MCP docs themselves publish an llms.txt index ("Fetch the complete documentation index at: https://modelcontextprotocol.io/llms.txt") — [MCP spec page](https://modelcontextprotocol.io/specification/latest)
- **Repomix:**
  - Output styles: XML (default), Markdown, JSON and plain text.
  - Sections: File Summary (in JSON, `fileSummary` with `generationHeader`, `purpose`, `fileFormat`, `usageGuidelines` and `notes`), Directory Structure, Files and Git Logs.
  - Optional parts: a user header and custom instructions.
  
  — [Repomix output guide](https://repomix.com/guide/output)
- **Kiro steering:** inclusion modes `always`, `fileMatch`, `manual` and `auto`, and live references `#[[file:…]]` — [kiro.dev steering](https://kiro.dev/docs/steering/)
- **requimd today:**
  - FR-305 `show <ID>` prints an item with incoming and outgoing references — [cli.md](C:/Users/grafj/projects/reqmd/requirements/functional/cli.md)
  - FR-404 `export mermaid <ID> --depth n` already defines neighbours as items that link to, or mention, the item and the items it links to or mentions, up to n steps — [export.md](C:/Users/grafj/projects/reqmd/requirements/functional/export.md)
  - FR-402 JSON export carries the whole graph with a format version — [export.md](C:/Users/grafj/projects/reqmd/requirements/functional/export.md)
- BMAD's "context-embedded" story files are a prior example of per-task context bundles — [Phase 1 notes](https://github.com/bmad-code-org/BMAD-METHOD/discussions/113)

### Inferences
- `requimd context <ID>` should reuse FR-404's neighbour definition and depth, so there is one notion of neighbourhood across `show`, `mermaid` and `context`.
- **Markdown first**, because agents and analysts both read it. Offer `--format json` with the same format version as FR-402, so MCP's `requimd_context` can return `structuredContent` with the Markdown also as text.
- **Ordering and size:** the target item in full first. Then depth-1 neighbours in full, and deeper neighbours as title plus status only, under an "Optional" heading in llms.txt style. A `--max-bytes` budget should cut from the tail deterministically.
- **Determinism:** the same commit and the same arguments should give byte-identical output. That makes bundles diffable and cacheable, and matches MCP's deterministic ordering advice.

### Gaps
- No study was found on which context shape makes coding agents follow requirements best. The proposed format is design reasoning, not evidence.

## Do analysts, not only developers, benefit? Accounts and critiques

### Takeaway
Evidence for analysts is thin and mostly promotional. In a May 2026 podcast Böckeler still finds it unclear whether SDD is for developers, product managers or analysts paired with developers. She observes that most users are developers working alone, which she sees as an alignment risk. She expects a backlash against Markdown-heavy workflows. The concrete analyst benefits are indirect:
- the analyst's governed spec becomes the agents' context
- deterministic checks catch what LLM reviews miss
- an agent can answer impact and coverage questions over the spec

No independent study of BAs or PMs using SDD tools or MCP with an RM tool was found.

### Cited Findings
- **Böckeler and Tacho** (Thoughtworks Technology Podcast, 28 May 2026):
  - Tacho says business roles such as PMs and designers should use the same tools in the same workspace as developers, and that a spec should never end up in a backlog.
  - Böckeler says it is unclear whether SDD is for developers alone, PMs, or requirements analysts paired with developers. Most people she sees use it alone as developers.
  - Both criticise long Markdown files that people likely do not review, the slide back to waterfall, the blurring of functional and technical specs, and AI making its own choices, such as inventing a customer-number format that changes between generations.
  - Böckeler is sceptical of spec-as-source and expects a backlash against Markdown-heavy workflows.
  
  — [Thoughtworks podcast page](https://www.thoughtworks.com/en-gb/insights/podcasts/technology-podcasts/what-is-spec-driven-development)
- **Institute of AI Product Management** (24 Jun 2026):
  - Its role model: the PM is the primary author of feature specs (stories, acceptance criteria, out-of-scope), reviews the agent's plan and does spec-anchored acceptance; engineers add implementation notes.
  - It warns of spec drift when behaviour is changed by prompting instead of editing the spec, and calls SDD "engineering theater" for prototypes and one-file fixes.
  - Its figures are unsourced, for example "50 to 70%" less back-and-forth and "95% of the business logic", and the article promotes a course.
  
  — [institutepm.com](https://www.institutepm.com/knowledge-hub/spec-driven-development-for-product-managers)
- **Search overview (Nov 2026):**
  - Kiro is described as the most PM-friendly (EARS is learnable by non-engineers) but is the only paid SDD IDE among seven compared. BMAD names explicit Business Analyst and Product Manager personas.
  - No independent evidence of BA or PM day-to-day performance was found.
  - Kiro's GA date is reported inconsistently (Nov 2025 versus 2026).
  
  — [ssojet comparison](https://ssojet.com/blog/best-spec-driven-development-tools); [augmentcode](https://www.augmentcode.com/tools/best-spec-driven-development-tools) (search results, vendor-authored)
- **Phase 1 critiques still stand:** Kiro turned a small bug into 4 user stories with 16 acceptance criteria, and agents ignored or over-applied the constitution — [Böckeler, Oct 2025](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)
- **Jama's governance pitch:** the MCP server "enforces existing permissions", lifecycle workflows and audit requirements. This is a vendor claim — [Jama press release](https://www.jamasoftware.com/press/jama-software-launches-model-context-protocol-mcp-server/)

### Inferences
- **Analysts benefit most where requimd turns agent output into reviewable, checkable artefacts.** Agents draft items in the right format with the next free ID, `check` validates them, and the analyst reviews a PR with a `diff --format delta` view. The analyst never has to read raw Markdown dumps.
- **The verbosity critique suggests checks on size as well as wording.** Examples: acceptance criteria per item, item length, and items per change. This is an idea for the owner; no source proposes it for RM tools.
- **An analyst using Claude Code or Copilot with `requimd mcp` can ask impact questions** ("what depends on FR-012, what is suspect, which UC steps are uncovered") and get deterministic answers. That fits Böckeler's hope for tools that help roles talk to each other. Whether analysts will run an agent at all is an open question.

### Gaps
- No first-hand account of a BA or analyst using an MCP server of an RM tool (Jama or otherwise) was found.
- No 2026 written article by Böckeler on SDD was found; only the podcast.

## Proposal for requimd

### Takeaway
Position requimd as the **governed requirements layer that agents read and propose against, never approve**. Six pieces, in this order:
1. A generated AGENTS.md section (S effort).
2. `requimd context <ID>` (S).
3. `requimd diff --format delta` (S).
4. A stdio-only `requimd mcp` with read-only tools (M).
5. Opt-in write tools that only write working-tree files (M).
6. Recipes for Kiro hooks and steering, and for Spec Kit and OpenSpec, that cite requimd IDs. Recipes only, no importers.

All of it stays within ADR-0011. A new ADR should record the agent interface and the data-egress caveat.

### Cited Findings
- The convergent tool set (item, search, trace, uncovered, stale or suspect, validate) and write gating — [tracey](https://github.com/bearcove/tracey); [Reqvire](https://github.com/reqvire-org/reqvire); [community Jama servers](https://policylayer.com/tools/t-j-thomas-jama-mcp-server)
- MCP 2026-07-28 tool annotations, `outputSchema`, deterministic order, the stdio rules and deprecated roots — [MCP tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools); [schema](https://modelcontextprotocol.io/specification/2026-07-28/schema); [changelog](https://modelcontextprotocol.io/specification/2026-07-28/changelog)
- AGENTS.md is read by most agents and by Kiro; Claude Code is not on the agents.md list — [agents.md](https://agents.md/); [kiro steering](https://kiro.dev/docs/steering/)
- Kiro command hooks on `PostFileSave` get JSON on stdin — [kiro hooks](https://kiro.dev/docs/hooks/)
- OpenSpec's delta vocabulary and validation rules — [validator.ts](https://raw.githubusercontent.com/Fission-AI/OpenSpec/main/src/core/validation/validator.ts)
- requimd constraints: BR-002, BR-003 and BR-004, FR-116's silent lock recording, FR-307 review, FR-402 format version, FR-403 diff, FR-404 neighbours, and ADR-0011 — [requimd requirements](C:/Users/grafj/projects/reqmd/requirements/); [ADR-0011](C:/Users/grafj/projects/reqmd/docs/decisions/0011-offline-by-default.md)

### Inferences

#### 1. MCP tool list (`requimd mcp --root <dir> [--allow-writes]`, stdio only)

All tools use `openWorldHint: false`. Outputs are `structuredContent` with an `outputSchema`, plus the same content as text (Markdown for humans, JSON mirror). IDs are validated against the configured prefixes. Errors on user input return `isError: true` with a diagnostic, never a panic (NFR-009).

| Tool | Input | Output | Class | Annotations |
|---|---|---|---|---|
| `requimd_get_item` | `id` (item or step, e.g. `UC-003.s2`); `include?`: `text`, `links`, `steps`, `source` | the item as in FR-402 (ID, type, title, status, prio, metadata, text, steps, outgoing links with key and target, incoming links, mentions, `file`, `line`) | read | readOnly |
| `requimd_search` | `query` (words, glossary aliases expanded); `type?`, `status?`, `prio?`, `file?`; `limit` (default 20) | `[{id, title, type, status, file, line, snippet}]`, in a deterministic order (score, then ID) | read | readOnly |
| `requimd_trace` | `id`; `direction`: `up`, `down` or `both`; `depth` (default 1, max 5); `link_types?` (`impl`, `ref`, `ver`, project keys) | `{nodes:[{id,title,status}], edges:[{from,to,key,suspect}]}`, plus optional Mermaid (FR-404) | read | readOnly |
| `requimd_suspects` | `id?` (otherwise the whole project) | `[{from, to, key, reason: "target changed since review", changed_parts?}]`; the diff of target text since review is a later option | read | readOnly |
| `requimd_check` | `paths?`; `strict?` | FR-301 diagnostics (`file`, `line`, `col`, `severity`, `check`, `message`, `item?`) plus a summary; **runs without writing `requimd.lock`** and reports unrecorded links as info | read | readOnly (after a spec change) |
| `requimd_coverage` | `uc?` | per use case: steps, covered, uncovered labels (FR-306); later also unverified requirements (no `ver`) | read | readOnly |
| `requimd_diff` | `ref1` (default: merge-base with the main branch); `ref2?`; `format`: `json` or `delta` | FR-403 JSON, or the delta Markdown (section 3), with suspect impact | read; runs local `git` | readOnly; refs validated |
| `requimd_context` | `id`; `depth` (default 1); `max_bytes?` | the bundle of section 2, as Markdown and JSON | read | readOnly |
| `requimd_glossary` | `term?` | `[{term, definition, aliases, file, line}]` (FR-011, FR-114) | read | readOnly |
| `requimd_project_info` | none | format summary, configured types, prefixes, ID ranges with the **next free ID per range**, link keys, statuses, the rules BR-002, BR-003 and BR-004, and the requimd version | read | readOnly |
| `requimd_propose_item` *(only with `--allow-writes`)* | `file`; `type` or `range`; `title`; `text`; `metadata` (`status` forced to `draft`); `links?` | writes the item at the end of `file` with the next free ID, then returns `{id, file, line, diff, diagnostics}` from `check` | write, additive | readOnly false, destructive false, idempotent false |
| `requimd_propose_change` *(only with `--allow-writes`)* | `id`; `title?`; `text?`; `metadata?`; `links?` | edits the item in place and returns the diff plus diagnostics plus newly suspect links; **refuses** ID changes, step relabelling, `status: approved`, and deletion | write | readOnly false, destructive true (it replaces text), idempotent true |
| `requimd_fmt` *(only with `--allow-writes`)* | `paths?`; `dry_run` (default true) | the diff of formatting changes (FR-304) | write when `dry_run` is false | idempotent true |

**Deliberately not exposed:** `review` (FR-307: resetting suspect hashes is the analyst's judgement), any `approve`, delete or renumber, `init`, site and matrix exports that write files, and git commit or push.

**Resources:**
- `requimd://item/{id}` (template, `text/markdown`)
- `requimd://glossary`
- `requimd://agents` (the AGENTS.md section text)

**Prompts:** optional, later, such as `review-item {id}` and `draft-tests {id}`. They give the user's own agent a template and send nothing anywhere.

#### 2. `requimd context <ID>` output format (Markdown default, `--format json`)

```markdown
# Context: FR-012 Tolerant parsing
> requimd 0.1.0-alpha.N · format 1 · commit 712e3ef (working tree clean) · generated by `requimd context FR-012 --depth 1`
> Status: approved · prio: must · requirements/functional/format.md:130
> Rules: do not edit approved items without the analyst; never set status approved; never renumber IDs; cite IDs in code and tests as `// FR-012`.

## Item
<full Markdown of the item, metadata block included>

## Upstream (this item implements or refines)
- UC-001.s2 (step) "…step text…" · link `impl` · suspect: no
  <full text of depth-1 targets>

## Downstream (implements, refines or verifies this item)
- TC-012 "…" · link `ver` · status: approved · suspect: **yes**, the target changed since its last review
- NFR-009 "…" · link `ref`

## Mentions
- FR-300 mentions FR-012 (mentions are not links and are never suspect)

## Glossary terms used
- **Diagnostic**: definition … (aliases: …)

## Open diagnostics
- requirements/functional/format.md:134 warning weak-word "robust" (FR-110)

## Optional
- depth-2 neighbours: ID, title, status, file:line only
```

- The JSON form has the same sections plus `format_version`, so the MCP tool and any CI consumer share one schema with FR-402.
- Item ordering is by ID, and `--max-bytes` drops from "Optional" upwards.
- No section ever contains content from outside the project files.

#### 3. Interoperability with OpenSpec, Kiro and Spec Kit

- **Export, recommended:** `requimd diff <ref> --format delta` prints `## ADDED|MODIFIED|REMOVED|RENAMED Requirements` per file with `### <ID> <Title>` blocks. It adds a "Status changes" section and a "Suspect impact" section (links that will be suspect). Teams can paste it into an OpenSpec `proposal.md` or a PR description. OpenSpec folder generation is not the default.
- **Import: no** first-class import of OpenSpec, Kiro or Spec Kit requirements. They have no stable IDs (OpenSpec uses names; Spec Kit's FR-001 restarts per feature), so an import would violate BR-002 or produce unstable items. If needed, a one-shot `requimd init --from openspec` could convert to items with fresh IDs, as a migration and not a sync. That is an owner decision.
- **Validation:** keep the foreign formats out of `check`. Offer recipes instead:
  - A **Kiro hook** `.kiro/hooks/requimd-check.json` with `trigger: PostFileSave`, `matcher: "requirements/.*\\.md$"` and a `command` action running `requimd check --format json`.
  - A `PreToolUse` hook with matcher `@requimd` and `confirm`, for the write tools.
  - A **steering file** `.kiro/steering/requirements.md` with `inclusion: fileMatch` on the source paths, referencing `#[[file:requirements/README.md]]`.
  - A **Spec Kit preset** and an **OpenSpec project instruction** saying "reference requimd IDs; do not invent FR-### numbers; run `requimd check` after editing requirements".
  - A language-only lint of foreign Markdown is an open question.
- **Coverage across levels:** later, `requimd coverage --code <glob>` could scan for ID references (`// FR-012`, `Implements: FR-012`) in code, tests and SDD task files. That would be the deterministic counterpart of Spec Kit's E-pass "requirements with no task". It overlaps with DD-4.

#### 4. Generated AGENTS.md section

`requimd agents` prints it, and `requimd agents --write` updates only the text between markers, which makes the command idempotent. `requimd init` (FR-303) could offer it.

```markdown
<!-- requimd:begin (generated by requimd 0.1.0; edit requimd.toml, not this block) -->
## Requirements (requimd)

Requirements live in `requirements/**/*.md` and `acceptance/**/*.md` as requimd items: a heading
`## <ID> <Title>`, then a metadata block of `key: value` lines (links `impl`, `ref`, `ver`;
`status`, `prio`), a blank line, and the text. The format is in `requirements/functional/format.md`.

- IDs: UC, BR, FR (001–099 format, 100–199 checks, 200–299 LSP, 300–399 CLI, 400–499 export,
  500–599 editors), NFR, TC. New items take the next free number in their range
  (planned: a next-free-ID query in the CLI and MCP `requimd_project_info`; until then see `requirements/README.md`).
- Never renumber IDs or use-case step labels. Never set `status: approved`; `review` is the most you may propose.
- Do not edit items with `status: approved` unless the user asks; propose changes in a branch.
- Never run `requimd review`; reviewing suspect links is the analyst's decision.
- After editing requirements, run `requimd check` and fix all errors.
- Cite requirement IDs in code comments, test names and commit messages (`FR-012: …`).
- For context on one item, run `requimd context <ID>`; for impact, `requimd show <ID>` and `requimd diff`.
- Text in fenced code blocks is an example, not an item.
<!-- requimd:end -->
```

- The content comes from `requimd.toml` (types, prefixes, ranges if configured, link keys), so it stays correct for each project.
- Kiro and the agents listed on agents.md read AGENTS.md. Claude Code is not on that list, so the guide should show how to reference the block from `CLAUDE.md`. Verify Claude Code's current behaviour before documenting it.

#### 5. Fit with ADR-0011

- `requimd mcp` uses stdio only and opens no socket, so it is not a network command and needs no network ADR under point 3. Tools declare `openWorldHint: false`. `diff` uses local git only, as today.
- What changes is the **egress caveat**: the requirements requimd returns go to the agent host and possibly to a cloud LLM. requimd does not send them, the user's host does, under MCP's consent rules. The ADR-0011 consequence "never leave the machine through `check`, the language server or CI" stays true. The new ADR should state it explicitly and document a `--allow` list or `--exclude` patterns for confidential files if the owner wants that.
- **Recommend a new ADR**, "Agent interface: MCP over stdio, read-only by default". It covers the stdio-only transport, the tool list, write gating with `--allow-writes`, the human-only acts (review, approve, renumber, delete), lock-file behaviour of `check` under MCP, and the egress caveat.
- An HTTP transport, opt-in LLM commands (R-91) or any registry lookup would each need their own ADR.

#### 6. Suggested backlog mapping
- R-89 (AGENTS.md section): S.
- R-88 (`context`): S, extends FR-305 and FR-404.
- Phase 1's change proposal view (`diff --format delta`): S, extends FR-403.
- R-86 (read-only MCP): M.
- R-87 (write tools): M.
- Recipes: docs only.
- Spec changes needed: a no-lock-write mode for `check` (FR-116); a next-free-ID query; the MCP command FR in the 300 range; the context FR in the 400 range.

### Gaps
**Open questions for the product owner:**
1. **Write tools at all?** Should agents get `propose_item` and `propose_change` behind `--allow-writes`, or only read tools, with agents editing files directly and `check` validating them?
2. **Lock file under MCP:** should `check` gain a no-write mode (FR-116 change)? Or should first recording of new links become an explicit step for all callers?
3. **Dependency budget:** is rmcp plus tokio acceptable for NFR-005, or should requimd hand-write the stdio JSON-RPC loop? Should it support only MCP 2026-07-28, or the legacy `initialize` era too?
4. **Confidentiality:** should the MCP server honour an exclude list (for example `requirements/confidential/**`), given that its output reaches a possibly cloud-hosted LLM?
5. **Foreign formats:** offer a language-only lint for Kiro or Spec Kit Markdown (weak words, EARS, shall), or stay strictly with requimd items?
6. **OpenSpec:** is a `diff --format delta` export enough, or does a real team need `openspec/changes/` folders generated, or a one-shot migration import?
7. **AGENTS.md ownership:** should requimd write into AGENTS.md (marker block) or only print the text? Should it also generate a Kiro steering file and hook JSON?
8. **Analyst workflow:** will the analysts themselves use an agent with `requimd mcp` for impact questions? A small trial with one analyst would answer Böckeler's open question for requimd's users better than any published source found here.
9. **Size checks:** given the verbosity critique, does the owner want deterministic size checks, such as acceptance criteria per item, item length or items per change?
