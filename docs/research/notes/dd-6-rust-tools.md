# DD-6: The Rust, Markdown-first, git-native requirements tools (tracey, SARA, Reqvire and neighbours)

Compiled 18 November 2026 from public READMEs, documentation sites, source files on GitHub, the crates.io API and the GitHub API. No software was installed or run. Phase 1 findings are not repeated unless a newer or more precise fact replaces them.

Tools covered in depth: tracey (bearcove), SARA (cledouarec), Reqvire (reqvire-org). Covered briefly: mantra, cargo-ovft (Open Very Fast Trace), duvet, flashtrace and other small 2026 repositories under the GitHub `requirements-traceability` topic.

## 1. File format, item types and design-level types

### Takeaway
The three tools use three very different formats. tracey has rule markers with no metadata. SARA uses one Markdown file per item with YAML front matter. Reqvire uses heading-based elements with reserved `#### Metadata` and `#### Relations` subsections and a heavy semantic layer (ontologies, SHACL, SKOS). None has use cases with numbered, linkable steps. requimd's heading-plus-one-metadata-line format, with many items per file, is the lightest format for an analyst to read. The closest in shape is mantra (`# \`req-1\`: Title` plus property bullets).

### Cited Findings
**tracey**
- A requirement is defined by a marker such as `r[auth.login]` on its own line. The marker must be its own paragraph, at column 0 or inside a blockquote. An inline mention such as "When implementing r[auth.login] ..." does not define a requirement. — [writing-specs.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/writing-specs.md)
- Requirement text runs from the marker to the "next blank line, heading, or another marker". The blockquote form `> r[...]` holds multi-paragraph requirements. — [writing-specs.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/writing-specs.md)
- IDs are dot-separated segments and "must contain at least one dot". The guide recommends that IDs mirror the heading structure, e.g. `auth.password.min-length`. — [writing-specs.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/writing-specs.md)
- The prefix (`r`, `h2`, `req`, `api`) is inferred from the spec files rather than configured. Different specs may reuse the same ID because their prefixes differ. — [writing-specs.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/writing-specs.md)
- There are no item attributes such as status, priority or level. The writing guide documents none. — [writing-specs.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/writing-specs.md)
- Versions use a `+N` suffix (`r[auth.login+3]`). `r[auth.login]` equals `r[auth.login+1]`. — [versioning.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/versioning.md)
- Other spec formats: StrictDoc `.sdoc` (the `UID:` becomes the ID) and Typst (`#PREFIX("id")[body]`). Markdown, Typst and `.sdoc` files can be mixed in one `include` glob. — [README](https://github.com/bearcove/tracey); [writing-specs.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/writing-specs.md)
- Code annotations have the form `// r[VERB REQ]`. The verbs are `impl` (the default), `verify`, `depends` and `related`. The StrictDoc form `// @relation(CH-001, scope=function, role=Verifies)` is also accepted. Comments are scanned in about 20 languages, among them Rust, TypeScript, Go, C/C++, Java, Kotlin, C#, Zig, PHP, YAML and JSON5. — [README](https://github.com/bearcove/tracey)
- Configuration lives in `.config/tracey/config.styx`. It lists `specs`, each with `impls` that have `include`, `exclude`, `test_include` and `source_url` fields. — [README](https://github.com/bearcove/tracey)
- There are no item types and no design level. tracey only traces spec rules to code and tests. — [README](https://github.com/bearcove/tracey)

**SARA**
- Each item is one Markdown file. Its YAML front matter holds `id`, `type`, `name` and relation lists such as `derives_from`, `is_satisfied_by` and `depends_on`. — [README](https://github.com/cledouarec/sara)
- There are ten built-in types: `solution`, `use_case`, `scenario`, `system_requirement`, `system_architecture`, `hardware_requirement`, `software_requirement`, `hardware_detailed_design`, `software_detailed_design` and `architecture_decision_record`. ADRs carry `status`, `deciders`, `justifies` and `supersedes`. — [README](https://github.com/cledouarec/sara)
- The hierarchy is Solution → Use Case → Scenario → System Requirement → System Architecture → HW/SW Requirement → HW/SW Detailed Design. ADRs link into the architecture and design levels. — [README](https://github.com/cledouarec/sara)
- There are six relation pairs, each declared in one direction with the reverse inferred: `refines`/`is_refined_by`, `derives_from`/`derives`, `satisfies`/`is_satisfied_by`, `justifies`/`justified_by`, `depends_on`/`is_required_by` (peer) and `supersedes`/`superseded_by` (peer). The README recommends linking bottom-up. — [README](https://github.com/cledouarec/sara)
- Since 0.9.0 (12 June 2026), a custom YAML schema can replace the built-in model entirely: types, fields, relations and allowed links. Since 0.10.0 (13 August 2026), IDs are generated from `id_format` templates and non-interactive `init` suggests the next unused ID. — [sara-cli CHANGELOG](https://github.com/cledouarec/sara/blob/main/sara-cli/CHANGELOG.md)
- The use case template has Actors, Pre-conditions, Main Success Outcome, Key Functional Scope and Post-conditions. It has no numbered steps and no alternative or exception flows. — [use_case.tera](https://github.com/cledouarec/sara/blob/main/sara-core/templates/use_case.tera)
- Item bodies come from Tera templates per type (`adr.tera`, `system_requirement.tera` and others). — [repository tree](https://github.com/cledouarec/sara/tree/main/sara-core/templates)

**Reqvire**
- In a multi-element file, an element is a `###` heading under `# Elements`, with `####` subsections. A single-element file starts with `# Element` and uses `##` subsections. Element names must be globally unique. — [Modeling language](https://www.reqvire.org/modeling-language)
- The reserved subsections are Metadata, Relations, Details, Reused Contract Context, ontology blocks, Shapes (fenced Turtle), queries and Concept References. Entries are bullets indented by two spaces, e.g. `  * type: requirement` and `  * specify: [API Authentication](#api-authentication)`. — [Modeling language](https://www.reqvire.org/modeling-language)
- Links are standard Markdown links to file anchors. Source files can be relation targets: `satisfiedBy: [auth_middleware.rs](../src/auth_middleware.rs)`. — [Modeling language](https://www.reqvire.org/modeling-language)
- The model is a graph: Ontology → Capabilities → Requirements → Contracts → Verification. There are five verification types: `test-verification`, `formal-proof-verification`, `analysis-verification`, `inspection-verification` and `demonstration-verification`, derived from `verification-objective` elements. — [README](https://github.com/reqvire-org/reqvire)
- Relations seen in the docs: `specify`/`specifiedBy`, `constrainedBy`/`constrain`, `verifiedBy`/`verify`, `satisfiedBy`, `derivedFrom`, `bindContract`, `define` and `use`. The docs give no complete list. — [Modeling language](https://www.reqvire.org/modeling-language); [User guide](https://www.reqvire.org/user-guide)
- "Semantic contracts" are reusable SHACL profiles bound to ontology terms. Ontologies hold OWL/RDF classes and properties. — [README](https://github.com/reqvire-org/reqvire); [MCP server](https://www.reqvire.org/mcp-server)
- `README.md`, `CHANGELOG.md` and `LICENSE.md` are reserved and skipped as model files. `.reqvireignore` excludes files from parsing but still lets file relations point at them. — [User guide](https://www.reqvire.org/user-guide)

**Others**
- mantra: a Markdown requirement is a heading with the ID in backticks, e.g. `# \`req-1\`: Requirement Title`, followed by an optional bullet list of properties (Parents, Manual, Deprecated, Exclude, Optional, Replaces). JSON5 and TOML are also accepted. Code traces use Rust macros (`req_satisfied`, `req_test`, `assert_req!`). — [lib.rs/mantra](https://lib.rs/crates/mantra)
- cargo-ovft uses OpenFastTrace Markdown (`type~name~version` IDs, `Needs:`, `Covers:`) and `// [impl->feat~login-process~1]` tags in code. — [lib.rs/cargo-ovft](https://lib.rs/crates/cargo-ovft)
- duvet links code to specifications such as IETF RFCs through `//=` (source and section) and `//#` (quoted requirement text) comments. — [awslabs/duvet](https://github.com/awslabs/duvet)

### Inferences
- requimd's format (ID in the heading, one metadata line, many items per file, plain prose) differs from all three. SARA needs one file per item and YAML. Reqvire needs nested bulleted subsections and a model vocabulary. tracey has no metadata, so it cannot carry status, priority or check type.
- SARA's design vocabulary (`satisfies`, `justifies`, `supersedes`, architecture and detailed-design types, ADRs as items) is the most directly reusable model for requimd's planned design level (R-67, F3). It confirms phase 1's suggestion with concrete relation names and inverse names.
- Reqvire's link to a source file as a `satisfiedBy` target is a lightweight alternative to scanning code tags. A requirement points at the implementing file, and the file's content hash feeds impact analysis (see section 4).

### Gaps
- Reqvire's full lists of element types and relations were not found. The docs show only examples.
- tracey's Typst and StrictDoc parsing rules were not examined in detail.

## 2. Checks and validation; glossary, language checks and German

### Takeaway
All checks in the three tools are structural: references, duplicates, cycles, orphans, staleness and graph redundancy. None checks wording. None has weak-word, modal-verb, "shall", story-pattern or acceptance-criteria checks, and none mentions German. Reqvire is the one exception on glossary: it has a SKOS thesaurus of `concept` elements with `altLabel` synonyms and explicit "Concept References". It does not scan prose for terms or warn about synonyms.

### Cited Findings
**tracey**
- LSP diagnostics: broken references (Error, with "Did you mean..."), unknown prefix (Error), unknown verb (Warning), stale references (Warning), duplicate definitions (Error), and an `impl` annotation in a test file (Error). — [editor-integration.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/editor-integration.md)
- `tracey query validate` checks for "broken refs, naming issues". Warnings fail the run only with `--deny warnings`. — [README](https://github.com/bearcove/tracey)
- The `tracey_validate` MCP tool describes checks for "circular dependencies and unknown references". — [mcp.rs](https://github.com/bearcove/tracey/blob/main/crates/tracey/src/bridge/mcp.rs)
- The query set is `status`, `uncovered`, `untested`, `stale`, `unmapped` (code units without a requirement) and `rule <id>`. — [README](https://github.com/bearcove/tracey)

**SARA**
- `sara check` reports broken references, circular dependencies, orphan items (configurable as warning or error), relations not allowed for the item type, duplicate IDs and ID format mismatches. `--strict` turns non-critical issues into errors. — [README](https://github.com/cledouarec/sara)
- Validation is implemented through a `ValidationRule` trait (0.5.0, 3 February 2026). Missing parents became a warning instead of an error in 0.10.0. — [sara-cli CHANGELOG](https://github.com/cledouarec/sara/blob/main/sara-cli/CHANGELOG.md)
- The changelog has no entries for glossary, language or HTML features. — [sara-cli CHANGELOG](https://github.com/cledouarec/sara/blob/main/sara-cli/CHANGELOG.md)

**Reqvire**
- `reqvire validate` checks "model structure, relations, Contract Bindings, ontology, and semantic contracts". Mutating operations validate before they write. — [User guide](https://www.reqvire.org/user-guide)
- `reqvire lint` implements these rules. Auto-fixable: `redundant_verify_relations` (a verification links directly to a requirement it already reaches through the hierarchy) and `safe_redundant_hierarchical_relations`. Auditable: `cross_submodel_hierarchical_relation`, `multi_branch_convergence` and `concept_authoring_warning` (missing definition, isolated concept). A fourth auditable rule, `maybe_redundant_hierarchical_relations`, exists but its detector always returns an empty list. The module has no wording checks. — [lint.rs](https://github.com/reqvire-org/reqvire/blob/main/crates/reqvire-core/src/lint.rs)
- `reqvire format` previews canonical formatting, ordering and relation layout. `--fix` applies the changes. — [User guide](https://www.reqvire.org/user-guide)
- Concepts are Markdown elements with `type: concept`. A concept has a body (the definition), `#### Labels` with `altLabel` entries, a Scope Note, Examples, Mappings and taxonomy relations (`derivedFrom`). Reqvire generates SKOS triples from them. Requirements reference a concept with a `#### Concept References` link list. The docs tell authors not to add `pref_label` or language metadata. They mention no language tags or multilingual support. — [Ontologies](https://www.reqvire.org/ontologies)
- `concept.rs` holds only a name-to-identifier helper. Nothing in it scans requirement prose for concept labels. — [concept.rs](https://github.com/reqvire-org/reqvire/blob/main/crates/reqvire-core/src/concept.rs)

**Others**
- mantra, cargo-ovft and duvet document only trace and coverage checks. Their pages mention no wording or glossary checks. — [lib.rs/mantra](https://lib.rs/crates/mantra); [lib.rs/cargo-ovft](https://lib.rs/crates/cargo-ovft); [awslabs/duvet](https://github.com/awslabs/duvet)

**requimd, for comparison**
- FR-114 reports an alias of a glossary term in an item's title or text and names the preferred term. — local spec `requirements/functional/checks.md` (FR-114)

### Inferences
- The language-check differentiator (weak words, modal verbs, required verb, story pattern, acceptance criteria, in German and English) is confirmed against all tools examined.
- The glossary differentiator needs sharper wording. "Has a glossary" is no longer unique, since Reqvire has a thesaurus with synonyms. requimd's glossary is unique in being part of the writing loop: it detects aliases in prose (FR-114), shows term hover in running text, and needs no explicit "Concept References" list.
- SARA's checks that requimd may lack: cycle detection, orphan items as a configurable severity, and "relation not allowed for this type". Reqvire's redundant-link lint is a cheap check requimd could add, e.g. a TC that `ver`s both FR-012 and a UC step FR-012 already covers.

### Gaps
- I could not confirm whether duvet extracts RFC 2119 keywords (MUST, SHOULD) from specifications. The README page I read does not say.
- Reqvire's `reqvire concepts validate` rules are not documented on the pages I read.

## 3. Language server and MCP

### Takeaway
tracey is the only one of the three with a language server. Its feature list is almost equal to requimd's, but it targets code comments as much as spec files. It adds a hover that shows a word diff between rule versions, a "Did you mean" rename fix for unknown IDs, document highlight and multi-root workspaces. SARA and Reqvire have no LSP. All three have MCP servers. SARA (8 tools, stdio, with create and update) and tracey (11 tools, stdio, read-mostly) are small. Reqvire's is large and semantic (about 28 tools plus prompts, Streamable HTTP, mutations opt-in). requimd has no MCP server.

### Cited Findings
**tracey LSP** (`tracey lsp`, built on tower-lsp)
- Advertised capabilities: completion (triggers `[` and space), hover, definition, implementation, references, document highlight, document and workspace symbols, code actions, execute command (`tracey.renameUnknownRequirement`), code lens, inlay hints, rename with prepare, and full semantic tokens (`namespace`, `keyword`, `variable`). It uses full text sync and workspace folders with change notifications. There is no formatting provider. — [lsp.rs](https://github.com/bearcove/tracey/blob/main/crates/tracey/src/bridge/lsp.rs)
- Hover shows the full requirement text, the spec name and source URL, and "for versioned requirements, a diff of changes between versions". — [editor-integration.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/editor-integration.md)
- Go to definition jumps from a code reference to the spec, and from a spec definition to its implementations (with a picker). Find references groups results by impl, verify, depends and related. — [editor-integration.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/editor-integration.md)
- Completion fuzzy-matches IDs after `r[` and shows the requirement text. It also completes the verbs. — [editor-integration.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/editor-integration.md)
- Rename updates the definition and all references, and checks naming conventions and conflicts. — [editor-integration.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/editor-integration.md)
- Code lens shows coverage counts on spec definitions, e.g. "3 impls, 1 test". — [editor-integration.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/editor-integration.md)
- Code action titles and inlay hint labels come from the daemon. The LSP bridge does not hard-code them. — [lsp.rs](https://github.com/bearcove/tracey/blob/main/crates/tracey/src/bridge/lsp.rs)
- Editors: a Zed dev extension (`tracey-zed`, not in the Zed registry), plus Neovim via `vim.lsp.config` with `cmd = { 'tracey', 'lsp' }`. For VS Code the docs only say to use a generic LSP client. There is no dedicated VS Code extension. — [editor-integration.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/editor-integration.md)
- The repository has tests for the diagnostic lifecycle and for multi-root LSP. — [repository tree](https://github.com/bearcove/tracey/tree/main/crates/tracey/tests)

**SARA and Reqvire LSP**
- SARA has crates `sara-cli`, `sara-core` and `sara-mcp`, and no LSP. — [repository tree](https://github.com/cledouarec/sara)
- Reqvire's docs mention no LSP or editor integration, and the visible source tree has no `lsp` paths. Its local UI is `reqvire serve` (the Explorer, `--host`, `--port`). — [User guide](https://www.reqvire.org/user-guide); [repository tree](https://github.com/reqvire-org/reqvire)

**MCP**
- tracey has 11 stdio tools, each with a required `cwd`: `tracey_status`, `tracey_uncovered`, `tracey_untested`, `tracey_stale`, `tracey_unmapped`, `tracey_rule`, `tracey_config`, `tracey_reload`, `tracey_validate`, `tracey_config_exclude` and `tracey_config_include`. The last two change the configuration. There is no bump tool. — [mcp.rs](https://github.com/bearcove/tracey/blob/main/crates/tracey/src/bridge/mcp.rs)
- `tracey ai` registers the MCP server and installs a skill for Codex and Claude. The skill lives in `crates/tracey/skill/SKILL.md`. — [README](https://github.com/bearcove/tracey); [repository tree](https://github.com/bearcove/tracey/tree/main/crates/tracey/skill)
- SARA (`sara mcp`, stdio, added in 0.12.0 on 2 November 2026) has 8 tools: `sara_search_items`, `sara_get_item`, `sara_trace_item`, `sara_analyze_impact`, `sara_validate`, `sara_describe_model`, `sara_create_item` and `sara_update_item`. Read tools never modify files. Write tools write only Markdown inside configured repositories. — [README](https://github.com/cledouarec/sara); [sara-mcp README](https://github.com/cledouarec/sara/blob/main/sara-mcp/README.md); [CHANGELOG](https://github.com/cledouarec/sara/blob/main/sara-cli/CHANGELOG.md)
- Reqvire's MCP read tools include `reqvire.read_element`, `reqvire.search`, `reqvire.model`, `reqvire.traces`, `reqvire.coverage`, `reqvire.change_impact`, `reqvire.lint`, `reqvire.format`, `reqvire.semantic.sparql` (read-only SPARQL), `reqvire.semantic.export`, the concept tools (`reqvire.concepts.list` and others) and the workspace and branch tools (`reqvire.branch.create` and others). Mutations (add, remove, move, rename, merge, link, unlink, relink) need `--enable-mutations`, return structured diffs and mostly support `dry_run`. Prompts include `reqvire.workflow.plan_change` and `reqvire.workflow.verify_coverage`. — [MCP server](https://www.reqvire.org/mcp-server); [README](https://github.com/reqvire-org/reqvire)
- Reqvire's MCP transport is Streamable HTTP at `/mcp` (protocol 2025-11-25) and rejects non-loopback origins. The README also lists stdio. — [MCP server](https://www.reqvire.org/mcp-server); [README](https://github.com/reqvire-org/reqvire)
- In v1.5.1 (3 November 2026), "typed MCP tools replace the removed CLI authoring commands". Reqvire is moving authoring to AI agents. — [Reqvire releases](https://github.com/reqvire-org/reqvire/releases). The user guide still lists CLI mutating commands (`add`, `rm`, `mv`, `rename`, `link`). — [User guide](https://www.reqvire.org/user-guide). The two sources conflict.

**requimd LSP, for comparison** (local spec `requirements/functional/lsp.md`)
- The language server covers: diagnostics (FR-202), reference completion (FR-203), key and value completion (FR-204), hover with glossary (FR-205), definition (FR-206), references (FR-207), implementation (FR-208), inlay hints (FR-209), document symbols (FR-210), workspace symbols (FR-211), assign ID (FR-212), create stub (FR-213), rename (FR-214), code lens on steps (FR-215), semantic tokens (FR-216), external file changes (FR-217), mark as reviewed (FR-218) and language-check code actions (FR-219).

### Inferences
- requimd's LSP matches tracey's item for item. requimd also has key and value completion, glossary hover, the assign-ID and create-stub actions, mark as reviewed and language fixes. tracey has three things requimd lacks: a version diff in hover, a "Did you mean" fix for unknown IDs, and document highlight. requimd also has a dedicated VS Code extension, which tracey lacks.
- The MCP tool sets converge on a common core: get item, search, trace up and down, impact, validate, coverage or uncovered, and describe the model. SARA's 8-tool set is the most transferable template for a requimd MCP server (R-86). Reqvire's pattern of read-only by default, mutations behind a flag, and `dry_run` is the safest default for analysts.

### Gaps
- tracey's actual code action titles and inlay hint texts are produced in the daemon, which I did not read.
- I did not determine whether requimd's LSP supports document highlight and multi-root workspaces. Check against FR-201.

## 4. Change and impact

### Takeaway
There are three models. tracey uses explicit revisions in the IDs, enforced by `tracey bump` and a pre-commit hook. It compares rule text byte for byte against HEAD and has no editorial escape. Reqvire computes hash-based change impact per run against the PR merge-base. It outputs a checklist of "invalidated verifications" and keeps no persistent review state. SARA's `diff` lists added, removed and modified items and relations between two refs, with no propagation. Only requimd keeps a persistent per-link review state (the lock file) that stays suspect until someone reviews it.

### Cited Findings
**tracey**
- A code reference is stale when its version is older than the spec version. Stale requirements do not count as covered. Stale references appear in the dashboard, the LSP, MCP and `tracey query stale`. — [versioning.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/versioning.md)
- To clear a stale reference, review the change (hover shows a word-level diff), update the code, then bump the annotation. The guide insists on that order. — [versioning.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/versioning.md)
- `bump` reads staged spec files (`git diff-index --cached HEAD`), parses the HEAD and index versions, and flags a rule when `raw` text differs and the version is unchanged. It compares without normalising whitespace. It rewrites the markers to `+1` and re-stages the file. — [bump.rs](https://github.com/bearcove/tracey/blob/main/crates/tracey/src/bump.rs)
- bump has no editorial escape. The only bypass is `git commit --no-verify`. bump also writes index content to the working tree, which can overwrite unstaged edits in the same file. — [bump.rs](https://github.com/bearcove/tracey/blob/main/crates/tracey/src/bump.rs) (my reading of the source)
- `tracey pre-commit` fails when staged rule text changed without a version bump. The repository ships `hooks/pre-commit`, `hooks/pre-push` and `hooks/install.sh`. — [versioning.md](https://github.com/bearcove/tracey/blob/main/docs/content/guide/versioning.md); [repository tree](https://github.com/bearcove/tracey/tree/main/hooks)

**Reqvire**
- `reqvire change-impact --git-commit origin/main` turns repository diffs into a review queue. The output lists changed requirements, affected descendants, linked verifications, bound contracts, satisfied implementation artifacts and governance context. `--json` is available. — [User guide](https://www.reqvire.org/user-guide)
- When PR context is available, the comparison uses the merge-base. Otherwise the caller must give a baseline. `HEAD` reviews pending changes. Analysis never fetches. — [README](https://github.com/reqvire-org/reqvire)
- An element counts as changed when its `hash_impact_content` differs. File bindings use a stored `content_hash`. Impact walks forward relations recursively, with per-relation filters (e.g. `use` never traverses). Every verification in an impact tree is listed under `invalidated_verifications` and printed as an unchecked `- [ ]` box for manual review. Relocated elements (same name, new ID) are detected. Nothing persists between runs. — [change_impact.rs](https://github.com/reqvire-org/reqvire/blob/main/crates/reqvire-core/src/change_impact.rs)
- v1.5.1 added cross-repository change impact across the declared workspace repositories. — [Reqvire releases](https://github.com/reqvire-org/reqvire/releases)
- The repository has a `.github/workflows/change_impact.yml` workflow. I did not read its contents. — [repository tree](https://github.com/reqvire-org/reqvire)

**SARA**
- `sara diff <REF1> <REF2>` compares graphs between git refs. It was added in 0.3.1 (20 January 2026). Since 0.10.0 it fails instead of comparing the working tree with itself. — [CHANGELOG](https://github.com/cledouarec/sara/blob/main/sara-cli/CHANGELOG.md)
- `GraphDiff::compute` reports items added, removed and modified (by name, description, attributes, file path and joined upstream and downstream ID lists), plus added and removed relations. It does not propagate impact, so dependents are not flagged. — [diff.rs](https://github.com/cledouarec/sara/blob/main/sara-core/src/graph/diff.rs)
- The `sara_analyze_impact` MCP tool "lists the items a change may affect". The README does not document the algorithm. — [sara-mcp README](https://github.com/cledouarec/sara/blob/main/sara-mcp/README.md)

**requimd, for comparison**
- FR-116 stores a content hash of each link target as of the last review in the lock file. For an item the hash covers title, metadata except `status`, and body. For a step it covers the step text. HTML comments are excluded. — local spec `requirements/functional/checks.md` (FR-116)

### Inferences
- requimd's lock file gives durable, auditable, per-link review state. tracey keeps the review state in the code annotation. Reqvire keeps none: its checklist is recomputed per PR and disappears after the merge. requimd's model fits analysts best. Reqvire's merge-base checklist is the best presentation for a PR review.
- tracey's byte-exact comparison has the same false-positive problem phase 1 found for content hashes (typo fixes force a bump). tracey offers no escape either, so the "editorial change" idea (R-35, DD-3) is still unaddressed by every neighbour.
- tracey's version diff in hover needs the old text. tracey gets it from git history. requimd could do the same if the lock file recorded the commit of the last review next to the hash.

### Gaps
- How Reqvire detects "PR context" for the merge-base (environment variables, a CI provider) is not in the code I read.
- How SARA's `sara_analyze_impact` traverses the graph is not documented.

## 5. Multi-repo, workspaces and scale

### Takeaway
SARA aggregates several repository paths into one graph through configuration. Reqvire has the most elaborate model: a tracked `reqvire-workspace.toml` with pinned commits, workspace commands, coordinated branches and PRs, and cross-repo impact. tracey supports several specs per config and multi-root LSP workspaces, but not cross-repo links. None publishes scale benchmarks.

### Cited Findings
- SARA: `sara.toml` takes `[repositories] paths`, which aggregates documents from several git repositories into one graph. — [README](https://github.com/cledouarec/sara)
- Reqvire: `reqvire-workspace.toml` declares component repositories with URLs, refs and exact commit pins. The commands are `workspace status`, `add`, `remove`, `sync`, `freeze` and `migrate`, the last including `--from repo` for Google Repo. — [README](https://github.com/reqvire-org/reqvire)
- Reqvire v1.5.1 adds coordinated component and root branches and creates or reuses linked pull requests. — [Reqvire releases](https://github.com/reqvire-org/reqvire/releases)
- Reqvire's global `--workspace` option runs commands and the MCP server against a repository from outside it. — [User guide](https://www.reqvire.org/user-guide)
- tracey runs one persistent daemon per workspace over a Unix socket. The web, LSP and MCP front ends connect to it. It rebuilds on file changes and exits after 10 minutes idle. — [README](https://github.com/bearcove/tracey)
- tracey has multiple specs per config, each with its own `impls`. — [README](https://github.com/bearcove/tracey)

### Inferences
- requimd's R-33 idea (import another repository's JSON export as read-only external items) is lighter than Reqvire's workspace and needs no network (NFR-006). SARA's local `paths` list is the simplest proven variant.
- tracey's Unix-socket daemon, together with release binaries only for macOS ARM and Linux (section 7), suggests weak Windows support. requimd's single offline binary on Windows is a practical advantage for the owner's audience.

### Gaps
- None of the projects publishes performance figures or tested model sizes.
- I could not confirm whether tracey runs on Windows. The docs do not say.

## 6. Publishing: HTML, diagrams and matrices

### Takeaway
None of the three produces a static, shareable HTML site the way requimd does. tracey has a live local web dashboard. Reqvire has a local "Explorer" server plus a published explorer of its own model. SARA has text, JSON, CSV matrices and Mermaid only. mantra and cargo-ovft produce static HTML reports.

### Cited Findings
- `tracey web` starts a browser UI with three views: Spec (rendered with inline status), Coverage (a filterable table) and Sources (a file tree with coverage badges and annotated code). It has Cmd+K search. — [README](https://github.com/bearcove/tracey)
- SARA's outputs are text, JSON, CSV (`sara report matrix --format csv`) and Mermaid (`sara query --format mermaid`, since 0.11.0). The README does not mention HTML. — [README](https://github.com/cledouarec/sara); [CHANGELOG](https://github.com/cledouarec/sara/blob/main/sara-cli/CHANGELOG.md)
- SARA reports: `sara report coverage` and `sara report matrix`. — [README](https://github.com/cledouarec/sara)
- Reqvire: `reqvire model` (Mermaid by default, `--json`), `reqvire traces` (verification trace trees as Mermaid), `reqvire coverage` (verification and implementation coverage), and `reqvire semantic export` to Turtle or JSON-LD. `reqvire serve` starts the local Explorer. — [User guide](https://www.reqvire.org/user-guide)
- Reqvire publishes its own model at reqvire-org.github.io/reqvire ("Explore the model"). — [README](https://github.com/reqvire-org/reqvire)
- mantra's `mantra report` writes a static HTML site (Jinja2 templates over JSON) or JSON. — [lib.rs/mantra](https://lib.rs/crates/mantra)
- cargo-ovft writes `requirements_report.html` or JSON. — [lib.rs/cargo-ovft](https://lib.rs/crates/cargo-ovft)

### Inferences
- requimd's static site, CSV matrix, JSON and Mermaid exports, published offline through releases (FR-400 to FR-407), are broader than any one neighbour's. tracey's Sources view, showing code annotated with requirements, is the one view requimd has no counterpart for.

### Gaps
- I did not determine whether Reqvire's `html` module in `reqvire-core` produces a static export or only serves the Explorer.

## 7. Project health and adoption

### Takeaway
All three are young, effectively single-maintainer projects. tracey has the most stars, 19 contributors and the backing of a well-known Rust author, but its releases stalled in May 2026. SARA releases very often (40 crates.io versions in ten months) and has modest uptake. Reqvire is the most active by commits but is a one-person project that closed code PRs, and it has pivoted towards ontology-driven "semantic engineering".

### Cited Findings
**tracey**
- Repository created 28 December 2025, last push 28 October 2026. 259 stars, 41 forks, 31 open issues. The API reports Apache-2.0 — [GitHub API](https://api.github.com/repos/bearcove/tracey); the README fetch reported dual Apache-2.0 and MIT — [README](https://github.com/bearcove/tracey). The two sources conflict on the licence.
- 19 contributors. `fasterthanlime` made 203 contributions, and the next contributor made 10. — [GitHub API contributors](https://api.github.com/repos/bearcove/tracey/contributors)
- crates.io: first version 0.1.0 on 28 December 2025, 1.0.0 on 30 December 2025, latest 1.3.0 on 4 March 2026. 576 downloads in total. — [crates.io API](https://crates.io/api/v1/crates/tracey)
- The latest GitHub release is v1.4.0 (15 May 2026), with binaries for macOS Apple Silicon and Linux ARM64 and x86_64. crates.io lags at 1.3.0. — [tracey releases](https://github.com/bearcove/tracey/releases)
- A fork exists at kraln/tracey. — [kraln/tracey](https://github.com/kraln/tracey)

**SARA**
- Repository created 10 January 2026, last push 18 November 2026. 107 stars, 9 forks, Apache-2.0. — [GitHub API](https://api.github.com/repos/cledouarec/sara)
- 6 contributors, 2 of them bots. `cledouarec` made 152 contributions. — [GitHub API contributors](https://api.github.com/repos/cledouarec/sara/contributors)
- `sara-cli` on crates.io: 0.1.0 on 16 January 2026, latest 0.12.2 on 13 November 2026, 40 versions, 2,883 downloads (1,942 recent). — [crates.io API](https://crates.io/api/v1/crates/sara-cli)
- The Show HN post got 3 points and 2 comments. The author has an embedded-systems background (automotive, medical, avionics, CMMI). — [Hacker News](https://news.ycombinator.com/item?id=46752826)

**Reqvire**
- Repository created 27 December 2024, last push 13 November 2026. 30 stars, 9 forks, Apache-2.0. Its topics include `mbse`, `ontologies` and `vibe-engineering`. — [GitHub API](https://api.github.com/repos/reqvire-org/reqvire)
- 4 contributors. `ilijaljubicic` made 752 contributions. A `claude` account made 4. — [GitHub API contributors](https://api.github.com/repos/reqvire-org/reqvire/contributors)
- 11 GitHub releases, from v0.15.1 (5 June 2026) to v1.5.3 (13 November 2026). 1.0.0 came out on 19 June 2026. — [GitHub API releases](https://api.github.com/repos/reqvire-org/reqvire/releases). Reqvire is not on crates.io (the API returns 404). — [crates.io API](https://crates.io/api/v1/crates/reqvire)
- "Code, model, and specification pull requests are closed." Suggestions go to Discussions. — [README](https://github.com/reqvire-org/reqvire)

**Others**
- mantra: MIT, by Manuel Hatzl. 0.9.0 came out on 1 September 2026, with 18 releases and about 42 downloads a month. An Infineon Developer Community blog post covers it. — [lib.rs/mantra](https://lib.rs/crates/mantra); [Infineon blog](https://community.infineon.com/t5/Blogs/Requirements-Traceability-with-mantra/ba-p/864822)
- cargo-ovft: MIT/Apache-2.0. 0.5.0 came out on 30 October 2026, with about 298 downloads a month. — [lib.rs/cargo-ovft](https://lib.rs/crates/cargo-ovft)
- duvet (awslabs): Apache-2.0, 174 stars, 126 commits. CI runs the Verus verifier on its coverage model. — [awslabs/duvet](https://github.com/awslabs/duvet). Phase 1 cited a cryspen/duvet repository. The awslabs repository is the one read here.
- Small 2026 Rust repositories under the topic include flashtrace (6 stars, Apache-2.0, "agent-native"), forkzero/lattice, kolanski/FDML and spec-dna. All have fewer than 10 stars. — [GitHub topic](https://github.com/topics/requirements-traceability?l=rust); [flashtrace](https://github.com/flashtrace/flashtrace)

### Inferences
- None of the neighbours has more than one substantial maintainer. Adoption signals are weak: stars in the tens to low hundreds, crates.io downloads in the hundreds to low thousands. The niche is real but immature, and none of them is established in the business-analyst audience.
- Reqvire is moving towards ontology-driven systems engineering with AI agents. SARA is moving towards configurable MBSE-like schemas for embedded systems. tracey is moving towards developer spec coverage. None is moving towards analysts writing prose requirements.

### Gaps
- Real users of tracey beyond the bearcove projects, and of SARA or Reqvire, could not be verified from public sources.
- tracey's commit activity after May 2026 (releases versus pushes) was not analysed.

## 8. What they do better, what only requimd does, comparison, verdict, ideas and open questions

### Takeaway
requimd is the only tool in this group built for business analysts writing prose: language checks in two languages, a glossary in the writing loop, use case steps as link targets, a persistent per-link review state, a static site and a dedicated VS Code extension. The neighbours are ahead in four places. tracey leads on code-to-spec tracing. SARA leads on design-level types and a configurable schema. Reqvire leads on merge-base PR impact and verification methods. All three offer MCP access for AI agents.

### Cited Findings
Feature comparison. Sources are those cited in sections 1 to 7. requimd values come from the local specification (`requirements/`).

| Feature | requimd | tracey | SARA | Reqvire | Others |
|---|---|---|---|---|---|
| Item syntax | ID in heading, one metadata line, many items per file | `r[id]` marker paragraph, no metadata | One file per item, YAML front matter | `###` element plus `#### Metadata` and `#### Relations` bullets | mantra: `` # `id`: Title `` plus bullets; OFT `type~name~v` |
| Item types | UC, FR, NFR, BR, US, TC (fixed) | None (rules only) | 10 built-in, custom YAML schema | Capability, requirement, contract, five verification types, ontology, concept | OFT: user types |
| Design level (architecture, ADR) | Planned (R-67) | No | Yes: system architecture, detailed design, ADR | Contracts and ontologies; files as `satisfiedBy` targets | OFT `dsn` |
| Use case step labels as link targets | Yes (FR-006, FR-007) | No | No (template without steps) | No | No |
| Inferred or named reverse links | Backlinks on the site; named inverses planned (R-02) | n/a | Yes, six pairs | Paired relation names | – |
| Structural checks | Links, duplicates, steps, placeholders and more | Broken refs, prefix, verb, stale, duplicates, impl in test file | Broken refs, cycles, orphans, invalid relation, duplicates, ID format | Structure, contracts, ontology; redundant-link lint with auto-fix | – |
| Glossary | Yes, with alias warnings in prose and hover | No | No | SKOS thesaurus with `altLabel`; explicit Concept References; no prose scan | No |
| Language checks (weak words, modal verbs, story pattern, acceptance criteria) | Yes, German and English | No | No | No | No |
| LSP | Yes (FR-200 to FR-219) | Yes (tower-lsp) | No | No | No |
| Hover | Item text and glossary | Text, coverage, version diff | – | – | – |
| Code lens and inlay hints | Steps (FR-215), inlay (FR-209) | Coverage counts, inlay | – | – | – |
| Code actions | Assign ID, stub, mark reviewed, language fixes | Daemon-defined, plus rename of unknown ID | – | – | – |
| Dedicated editor extension | VS Code | Zed (dev extension) | – | – | – |
| MCP server | No (R-86 candidate) | 11 tools, stdio | 8 tools, stdio, create and update | About 28 tools plus prompts, HTTP, mutations opt-in | – |
| Code annotation scanning | No (`// FR-006` convention only) | Yes, about 20 languages | No | Source files as relation targets | OFT, mantra macros, duvet |
| Change detection | Lock file, per-link content hash, persistent | Versioned IDs, `bump`, pre-commit hook, byte-exact | None (diff only) | Per-run element hashes | OFT revisions |
| Diff between refs | `requimd diff` (FR-403) | No | `sara diff` (items, relations) | `change-impact --git-commit` | – |
| Merge-base PR impact | No (R-41) | No | No | Yes, with an invalidated-verification checklist | – |
| Review state and mark as reviewed | Yes (FR-218, FR-307) | Bump the annotation | No | No (per-run checklist) | – |
| Multi-repo | No (R-33) | Multi-spec, multi-root LSP | `[repositories] paths` | Workspace manifest with commit pins, cross-repo impact | – |
| Static HTML site | Yes (FR-400) | No (live dashboard) | No | Local Explorer, published model | mantra, cargo-ovft |
| Matrix (CSV) | Yes (FR-401) | Coverage table in the dashboard | CSV matrix | Coverage reports | – |
| Mermaid | Yes (FR-404) | No | `query --format mermaid` | Model and trace diagrams | – |
| JSON export | Yes (FR-402) | Via MCP and queries | Yes | Yes, plus Turtle and JSON-LD | mantra, ovft |
| Verification method | Planned (R-52) | No | No | Five verification types | – |
| Windows binaries | Yes (NFR-004) | Not shown (macOS and Linux only) | Via cargo | Not determined | – |
| Licence | – | Apache-2.0 (MIT dual in README) | Apache-2.0 | Apache-2.0 | MIT and Apache |
| First and latest release | Alpha on GitHub | 28 Dec 2025 / v1.4.0, 15 May 2026 | 16 Jan 2026 / 0.12.2, 13 Nov 2026 | v0.15.1, 5 Jun 2026 / v1.5.3, 13 Nov 2026 | – |

### Inferences
**Verdict on requimd's differentiators**
- **German and English language checks: confirmed.** No neighbour has any wording check. Reqvire's lint is graph-only, as its source shows. tracey and SARA document none. No neighbour mentions German anywhere.
- **Glossary: confirmed with qualification.** Reqvire has a glossary-like SKOS thesaurus with synonyms. requimd is unique in using the glossary while the analyst writes: alias detection in prose (FR-114), hover on terms, and no separate "Concept References" list. Phrase the claim as "a glossary that checks the prose", not "a glossary".
- **Analyst-oriented format: confirmed.** tracey has no metadata. SARA needs YAML and one file per item. Reqvire needs nested subsections plus an ontology vocabulary, and its authoring is moving to AI agents (v1.5.1). Only mantra comes close in shape, but it targets Rust developers.
- **Use case step labels: confirmed.** No tool makes use case steps addressable. SARA's use case template has no steps at all.
- **Additional differentiators found:** a persistent per-link review state with "mark as reviewed" (no neighbour has one), a static site in releases, a dedicated VS Code extension, and Windows as a first-class platform.

**What the neighbours do better (by tool)**
- tracey: code and test tracing in about 20 languages, version diff in hover, a "Did you mean" fix for unknown IDs, a pre-commit enforcement hook, a live dashboard with annotated sources, AI skill installation, StrictDoc and Typst input.
- SARA: design-level types and ADRs in the same graph, a custom schema without recompiling, inferred reverse links, cycle and orphan checks, multi-repo aggregation, Mermaid trace queries, an MCP server with write tools.
- Reqvire: merge-base change impact with a checklist of invalidated verifications, cross-repo impact, five verification methods, a redundant-link lint with auto-fix, a canonical formatter, read-only SPARQL, MCP prompts and the opt-in-mutation pattern.

**Ideas worth adopting, with concrete examples**
1. **PR impact checklist (from Reqvire).** `requimd diff --since merge-base --format md` prints Markdown the owner can paste into the PR description. It fits the repository's no-`gh` workflow, where the owner pastes a prepared description. Example output: `- [ ] TC-413 verifies FR-400 (FR-400 changed)` and `- [ ] FR-012 impl UC-003.s2 (step text changed)`. This extends FR-403 and R-41.
2. **Diff in hover on a suspect link (from tracey).** Record the commit of the last review in `requimd.lock` next to the hash. Hovering a suspect `ver: FR-012` then shows a word diff of FR-012 since that commit, computed through local git with no network. This supports FR-116 and FR-218.
3. **"Did you mean" fix for unknown IDs (from tracey).** For `impl: UC-01.s3`, report "Unknown item UC-01. Did you mean UC-001?" with a quick fix that replaces the reference. This extends FR-202 and FR-219.
4. **Coverage code lens on items (from tracey).** A code lens such as "2 impl · 1 ver · 1 suspect" on each FR heading. This extends FR-215 from steps to items, as R-31 proposes.
5. **Cycle and orphan checks (from SARA), redundant links (from Reqvire).** Examples: "FR-012 → FR-020 → FR-012 forms an `impl` cycle"; "TC-007 `ver` FR-012 is redundant: TC-007 already verifies UC-003.s2, which FR-012 implements". The redundant-link check should be informational and fixable.
6. **Named inverse links (from SARA).** Hover and site show "verified by TC-413, TC-414" and "implemented by FR-400" without back-links in the source. This is R-02, now with SARA's naming convention (`is_satisfied_by`, `justified_by`) as a reference.
7. **Read-only MCP server modelled on SARA's tool set.** For example `requimd_get_item`, `requimd_search`, `requimd_trace`, `requimd_suspects`, `requimd_check`, `requimd_glossary` (term, aliases, definition) and `requimd_describe_format`, over stdio. Any write tool stays behind a flag, as in Reqvire's `--enable-mutations`, and supports `dry_run`. This is R-86. The glossary tool would show off a differentiator to AI users.
8. **Design-level vocabulary (from SARA).** DES and ADR items with `satisfies` up to FR and `justifies` from ADR to DES, plus `supersedes` between ADRs. This gives R-67 a tested relation set.
9. **Verification method on TC (from Reqvire).** `method: inspection | analysis | demonstration | test`, with coverage split by method. This is R-52.
10. **Tag compatibility with tracey, if code scanning comes (R-26).** Accept `// r[impl FR-006]` and StrictDoc's `@relation(FR-006, role=Implements)` alongside requimd's `// FR-006`, so teams that already use tracey need not re-tag.

**Open questions for the product owner**
1. Should requimd ever read code (R-26)? If so, should it accept tracey's `r[impl ...]` syntax, or keep the analyst-only boundary and leave code coverage to tracey or OFT?
2. Is "glossary that checks the prose" the right public wording for the differentiator now that Reqvire has a thesaurus? Is a SKOS export of the glossary worth anything to your users? It is probably low priority.
3. Should the lock file record the commit of the last review (for diffs in hover and audit), at the cost of more lock-file churn and merge conflicts (see DD-3)?
4. Should a PR impact checklist in Markdown (idea 1) become part of the PR workflow for this repository and for users?
5. Should a requimd MCP server be read-only at first? Should it expose language and glossary checks so that AI assistants write requirements that already pass them?
6. For the design level, adopt SARA's relation names (`satisfies`, `justifies`, `supersedes`), or define requimd-specific short keys in the style of `impl`, `ref` and `ver`?
7. Is multi-repo aggregation (R-33) needed for any expected user, or can it wait?

### Gaps
- Real-world scale and performance were not compared, because no project publishes figures.
- The comparison rests on documentation and partial source reading. Features implemented but undocumented, such as tracey's daemon-defined code actions, may be missing.
- I did not check whether requimd's language server supports document highlight and multi-root workspaces against its implementation. The table reflects the specification only.
