# DD-5: StrictDoc and Sphinx-needs/ubCode, schema models, ReqIF and Excel exchange

Research date 2026-11-18. Public sources only; nothing was installed, downloaded or run. This deep dive builds on the phase 1 report (R-01 to R-08, R-73, R-80) and `notes/docs-as-code-tools.md`; it does not repeat their findings. StrictDoc facts come from its own user guide source (SDoc file on GitHub, release 0.31.0 era) and its exporter source code; Sphinx-needs facts from the 9.2.0 docs (released 04.11.2026) and its changelog; ubCode facts from ubcode.useblocks.com (version 0.39.0).

Reference point: requimd already has `[types.<name>]` (prefix, `meta`, `body`, `required` with `a|b`), `[links.<key>]` (`name`, `to`), `[values]`, `[fields] meta`, `[defaults]` and `[checks]` severity overrides by check ID (`C:/Users/grafj/projects/reqmd/requimd.toml`; FR-013, FR-014, FR-015, FR-103 to FR-105 in `requirements/`). Several things the two tools offer are therefore already partly present; the gaps are typed fields beyond value sets, inverse names, source-type constraints, cardinality, declarative rules and exchange formats.

## StrictDoc: grammar model, MIDs, composable documents, Markdown, validations, web UI, roadmap

### Takeaway
StrictDoc's schema is a per-document `[GRAMMAR]` (inline or imported from a `.sgra` file) with element TAGs, three properties, four field types and registered relations (Parent, Child, File) with ROLE and REVERSE_ROLE. It is richer per document than requimd but scattered across documents; MIDs exist mainly for diff and ReqIF identity; Markdown, composable documents, diff and HTML2PDF are all marked experimental or incomplete, and there is still no LSP.

### Cited Findings
- **Grammar elements.** A grammar lists `ELEMENTS`, each with `TAG`, `PROPERTIES` and `FIELDS` (each field has `TITLE`, `TYPE`, `REQUIRED`). The default SECTION element is shown with `IS_COMPOSITE: True`, `PREFIX: None`, `VIEW_STYLE: Narrative`. — [StrictDoc user guide (rendered)](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html), §6.2.3
- **Element properties.** `IS_COMPOSITE` "controls whether a document node will be treated as a leaf node"; `PREFIX` "determines the prefix that will be automatically added to a UID when creating a new requirement"; `VIEW_STYLE` selects the rendering template and falls back to the document-level VIEW_STYLE. — [User guide source](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc)
- **Field types: exactly four.** `String` ("Simple String"), `SingleChoice` ("Enum-like behavior, one choice is possible. Must be single-line"), `MultipleChoice` ("comma-separated words with fixed options. Must be single-line") and `Tag` ("comma-separated list of tags/key words"). SingleChoice drives auto-completion in the web editor. There is no integer, date or regex-pattern type. — [User guide source](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc)
- **Relations.** Supported relation types are Parent, Child and File; the default grammar defines Parent and File; relations must be registered in the grammar before use. A role is registered as `RELATIONS: - TYPE: Parent  ROLE: Refines`, and a requirement then writes `ROLE: Refines` on its Parent relation. — [User guide (rendered)](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html), §6.2.4.10
- **Reverse names.** "The ``REVERSE_ROLE`` field is only defined in the grammar"; "If ``REVERSE_ROLE`` is omitted, StrictDoc displays ``ROLE`` in both directions." Example: `REVERSE_ROLE: Refined by`. — [User guide source](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc)
- **Grammar files.** A grammar can live in a separate `.sgra` file referenced with `IMPORT_FROM_FILE: grammar.sgra`, "as if it was declared" inline; aliases such as `@my_grammar` resolve relative to the project configuration file. — [User guide source](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc)
- **MIDs.** Enabled per document with `OPTIONS: ENABLE_MID: True`; the default grammar lacks a MID field, so an explicit grammar is needed. MIDs are written on save by the web server or by `strictdoc export --formats sdoc`; in Markdown they activate when a `.gra.md` grammar declares a MID field. Purpose: identify nodes independently of UID or TITLE for portability and diff/changelog matching; MID and UID coexist. — [User guide (rendered)](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html), §6.2.4.1
- **Diff depends on MIDs.** The diff/changelog is experimental (`DIFF` feature, `--generate-diff-git="HEAD^..HEAD"`); without MIDs "StrictDoc relies on change tracking based on node UIDs, which is less reliable." — [User guide source, second half](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc)
- **Composable documents.** "The composable documents is an early feature with only 50%+ of the implementation complete"; "a document can be included to no more than one including document." JSON export does not include included documents by default. — [User guide source](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc)
- **Markdown (experimental).** Supports sections, requirements, custom types, metadata and relations; metadata uses bold field names such as `**UID**`; headings of seven or more `#` are a non-CommonMark extension that other tools render as plain text. — [User guide source, second half](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc); status "Markdown support (experimental)" in §23.1 of the [rendered guide](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html)
- **Validations.** Relation cycles (Req-A ⇒ Req-B ⇒ Req-C ⇒ Req-A) produce validation errors; links display only if both ends have a UID. — [User guide (rendered)](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html), §6.2.4.10. Custom validations are Python plugins implementing `traceability_index_build_finished` and calling `validation_index.add_issue(...)`, optionally per field; "not yet fully supported in the editable web interface." — [User guide source, second half](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc)
- **Web UI limits.** Server plus git commit is a "hybrid, manual workflow". Missing: a commit/push screen, editing `LEVEL: None` sections, deleting documents, recursive section deletion, project settings, editing File relations, moving TOC nodes of documents with includes, editing `.sgra` files. — [User guide source, second half](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc). The server is "not yet hardened against unsafe use", uses pickle caching in /tmp, and shared deployment is "impractical"; `--watch` reloads only on external file changes. — [User guide (rendered)](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html), §4.2–4.3
- **LSP.** "The StrictDoc LSP is on StrictDoc's long-term roadmap" (issue #577); TextMate/Sublime grammars give highlighting only. — [User guide (rendered)](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html), §5. The backlog describes an LSP that "can enable editing of SDoc files in IDEs" but it is unscheduled. — [StrictDoc Backlog](https://strictdoc.readthedocs.io/en/latest/strictdoc_28_Backlog.html) (via search snippet)
- **2026 plan.** A search snippet of the development plan lists 2026-Q1: parse Rust code, prepare the server for concurrent use, WHAT-WHY-HOW tutorial, Markdown support; 2026-Q2: more Markdown, C macros. — [Development plan (search snippet)](https://strictdoc.readthedocs.io/en/stable/stable/docs/strictdoc_24_development_plan.html). The fetched page (built 2026-11-18 from 0.31.0) shows the roadmap only as diagram images, so the snippet could not be confirmed in text. — same URL

### Inferences
- StrictDoc's ROLE/REVERSE_ROLE pair is the cleanest precedent for R-02: the inverse name lives only in the schema, never in the item. requimd's `[links.<key>] name` already holds the forward name; an `inverse` key is the obvious complement.
- StrictDoc's per-document grammar is a poor fit for requimd: one project-wide `requimd.toml` gives one schema for the LSP, the site and exports. StrictDoc itself drifted toward shared `.sgra` files, which supports central configuration.
- Four untyped-ish field types are enough for most BA metadata (enum, multi-enum, tags, text). StrictDoc has no date, number or pattern type; Sphinx-needs has them (below). A middle ground is enough for requimd.
- MIDs solve a problem requimd largely does not have: BR-002 forbids renumbering, and requimd IDs are human-stable. MIDs mainly matter to StrictDoc because its UIDs are optional and its ReqIF identifiers are otherwise random (next section). R-08 can stay low priority.
- StrictDoc's Markdown dialect (bold field labels, deep headings) is not plain CommonMark, so requimd's "readable on GitHub without the tool" property remains a differentiator.

### Gaps
- The exact Markdown grammar specification (`SDOC_MARKDOWN_SPEC`) was not read.
- Whether StrictDoc checks UID uniqueness, unknown relation targets and required fields as errors was not stated in the fetched text (very likely, but unconfirmed).
- The draw.io roadmap diagram could not be read, so 2026-Q3/Q4 plans are unknown.

## StrictDoc: ReqIF profile, Excel and PDF

### Takeaway
StrictDoc ships one ReqIF profile, `p01_sdoc` ("native"), which maps its own grammar one-to-one into ReqIF and uses random UUID identifiers unless MIDs are on; vendor dialects (the ProSTEP/PSI implementation guide) are only planned. Excel is one-way in practice ("SDoc -> Excel -> SDoc" round trip "not yet supported"), and PDF needs Chrome plus chromedriver and is experimental.

### Cited Findings
- **Profiles.** `ReqIFProfile.P01_SDOC = "p01_sdoc"` and `ALL = [P01_SDOC]` — the only profile in code. — [sdoc_reqif_fields.py](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/strictdoc/backend/reqif/sdoc_reqif_fields.py). The guide states that only native SDoc-to-ReqIF conversion is supported and that the PSI ReqIF Implementation Guide format is planned; the workflow "is therefore tool-specific." — [User guide source, second half](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc)
- **Options.** `--reqif-multiline-is-xhtml`, `--reqif-import-markup={RST,HTML,Text}` (default RST) and `--reqif-enable-mid` (requires MIDs), also settable in the project config. Polarion is named as an example of a tool that expects XHTML. — [User guide source, second half](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc)
- **Reserved attribute mapping.** UID → `ReqIF.ForeignID`; STATEMENT → `ReqIF.Text`; COMMENT → `NOTES`; CREATED_BY → `ReqIF.ForeignCreatedBy`; TITLE → `ReqIF.ChapterName` for composite nodes, else `ReqIF.Name`. — [sdoc_reqif_fields.py](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/strictdoc/backend/reqif/sdoc_reqif_fields.py)
- **What the exporter writes** (from [sdoc_to_reqif_converter.py](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/strictdoc/backend/reqif/p01_sdoc/sdoc_to_reqif_converter.py)):
  - Datatypes: single-line STRING (`max_length="10000"`); multi-line as XHTML when `multiline_is_xhtml`, otherwise STRING; one ENUMERATION per (node type, choice field), enum values with `ENUM-VALUE-<uuid4>` identifiers, `KEY` = option index, `LONG-NAME` = option text.
  - One SPEC-OBJECT-TYPE per grammar element, identifier `<tag>_<uuid4>`; attribute definitions `<object type id>_<field>`; MID is not exported as an attribute.
  - Multi-line XHTML is built by splitting on blank lines and wrapping escaped paragraphs in `<xhtml:p>` inside `<xhtml:div>`; no RST-to-XHTML rendering happens there.
  - One SPECIFICATION per document (identifier = MID if enabled, else `SPECIFICATION-<uuid4>`) with a SPEC-HIERARCHY that nests children under composite nodes.
  - One SPEC-RELATION-TYPE per (relation type, role), named by the role if present; only Parent and Child references are exported, File relations are skipped, and source is always the node that carries the reference.
  - Spec objects use the MID as identifier only if MIDs are enabled; otherwise `<node_type>-<uuid4>`, so "Without MIDs, identifiers change on every export."
  - Header: `REQ-IF-TOOL-ID`/`SOURCE-TOOL-ID` "strictdoc", `REQ-IF-VERSION` "1.0", title "Documentation export by StrictDoc", ReqIF 1.2 namespace `http://www.omg.org/spec/ReqIF/20110401/reqif.xsd`.
- **Release activity on ReqIF and Excel.** 0.30.x made the ReqIF importer and the Excel importer read Parent relations; 0.31.0 fixed ReqIF export escaping. — [Releases](https://github.com/strictdoc-project/strictdoc/releases) (from phase 1 notes)
- **Excel.** Import reads XLS/XLSX (xlrd/openpyxl), maps sheet-1 headers to fields "best-effort", one row per requirement; export writes XLSX via XlsxWriter with `--fields` to choose columns; "A roundtrip "SDoc -> Excel -> SDoc" is not yet supported." — [User guide source, second half](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc)
- **PDF.** HTML2PDF is experimental and off by default because "the implementation has not been completed yet"; CLI export and the "Export to PDF" button need Chrome and chromedriver, which StrictDoc downloads on demand unless `--chromedriver` is given; table splitting across pages is unresolved. A separate PDF route via Sphinx/LaTeX exists. — [User guide source, second half](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc); [rendered guide §8.3](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html)
- **The `reqif` library (StrictDoc's own).** Parse/unparse, format, anonymise, validate; "ReqIF files produced by various tool often have incomplete schemas"; inconsistent indentation; tools do not follow the library's alphabetical attribute order; `passthrough` aims for output "identical to the contents of the input file"; the official OMG XSD check is opt-in (`--use-reqif-schema`). 59 stars, no published releases on GitHub. — [strictdoc-project/reqif](https://github.com/strictdoc-project/reqif)

### Inferences
- The `p01_sdoc` profile is a "self-describing dump": every target tool must map StrictDoc's types and attributes by hand on first import. That is acceptable for one-off hand-over, but random identifiers without MIDs defeat every re-import, because DOORS Next, Codebeamer and Jama match on identifiers or `ReqIF.ForeignID` (see the ReqIF section). requimd's stable human IDs give it stable identifiers for free; it should derive every ReqIF IDENTIFIER deterministically.
- StrictDoc's decision to emit plain paragraph XHTML rather than rendered markup is a pragmatic minimum: rich lists and tables are where vendor importers differ most. requimd could start the same way and add lists and emphasis later.
- Even the most feature-rich open tool has no working Excel round trip. A careful CSV/XLSX review round trip (proposal below) would be a real differentiator, not a catch-up item.

### Gaps
- No public report was found of a StrictDoc ReqIF file being imported into DOORS Next, Polarion, Codebeamer or Jama and how it fared.
- The StrictDoc import path (`reqif_to_sdoc_converter.py`) was not read.

## Sphinx-needs: types, fields, links, schema validation, dynamic functions, needs.json, visualisations, performance

### Takeaway
Since 2025 Sphinx-needs has moved from Python-in-`conf.py` to declarative, typed configuration in `ubproject.toml`: `needs_types`, `needs_fields` (JSON-schema types, defaults, predicates, derived values), `needs_links` (outgoing/incoming names, `target_types`, back-link counts) and a JSON-schema rule engine with three severities. The needs.json export embeds its own schema and is moving from a version-keyed history file to a flat, source-annotated format (2a1).

### Cited Findings
- **Timeline.**
  - 5.0.0 (18.02.2025) and 5.1.0 (06.03.2025): no schema work.
  - 6.0.0 (28.09.2025) introduced "a fast, declarative and versatile JSON schema based need validation", wrote `schema_violations.json`, and split validation into local and network parts so that IDE extensions such as ubCode can "validate-on-type for need-local changes".
  - 6.1.0 (31.10.2025) removed `needs_schema_severity` in favour of the warning types `sn_schema_info`, `sn_schema_warning` and `sn_schema_violation`.
  - 6.2.0 (28.11.2025) made validation "~3 times faster" by moving to `jsonschema-rs`.
  - 6.3.0 (15.12.2025) deprecated `needs_extra_links` for `needs_links` and resolved paths relative to the TOML file.
  - 7.0.0 introduced `needs_fields`, replacing `needs_extra_options`, and moved `needs_statuses` and `needs_tags` checks into schema validation.

  — [Changelog (7.0.0 docs)](https://sphinx-needs.readthedocs.io/en/7.0.0/changelog.html)
- **Recent releases.** 9.1.0 (22.10.2026) added per-type fields and `target_types` and replaced `copy = true` with a derived `union`; 9.2.0 (04.11.2026) added `severity` for field and link constraints, `back_links` counts (`minItems`/`maxItems`) and reporting of unknown keys. — [Changelog (latest)](https://sphinx-needs.readthedocs.io/en/latest/changelog.html)
- **Licence and scale.** MIT; 327 stars. The repo is now a uv workspace with sphinx-needs, sphinx-codelinks, sphinx-test-reports and others. — [GitHub](https://github.com/useblocks/sphinx-needs)
- **needs_types.** Entry keys: `directive`, `title`, `prefix`, `color`, `style`, `inherits`, `fields`, `links` (others ignored). Since 9.1.0 it can be a dictionary keyed by directive name, and a type can carry an `id_regex`. TOML example: `[needs.types.req]` with `title = "Requirement"`, `prefix = "R_"`, `color = "#BFD8D2"`. — [Configuration](https://sphinx-needs.readthedocs.io/en/latest/configuration.html)
- **needs_fields** (7.0.0+):
  - `description` is "output in the schema of the needs.json, and can be used by other tools".
  - `schema` defaults to `{"type": "string"}`.
  - `nullable`, `default` and `predicates` are evaluated in order for needs that do not set the field.
  - `derive` (9.0.0): "A derived field cannot be set in a need or by a `needextend`".
  - Unknown keys in an entry are ignored, and 9.2.0 now reports them.

  — [Configuration](https://sphinx-needs.readthedocs.io/en/latest/configuration.html); [Changelog](https://sphinx-needs.readthedocs.io/en/latest/changelog.html)
- **needs_links.** Keys:
  - `outgoing` (default `<name>`) and `incoming` (default `<name> incoming`).
  - `allow_dead_links` (default False; when set, dead links log at INFO).
  - `target_types` (9.1.0: links may point to listed types or types inheriting from them).
  - `summary` (9.2.0, for shortcut links across the trace).
  - `copy` (deprecated in 9.1.0, removed in 10.0).
  - `style`, `style_part`, `style_start` and `style_end` set the PlantUML colour, line style and arrows of needflow diagrams.

  Verbatim example: `"tests": {"incoming": "is tested by", "outgoing": "tests", "copy": False, "style_start": "<-", "style_end": "down-->", "style": "#00AA00", "style_part": "dotted,#00AA00"}`. — [Configuration](https://sphinx-needs.readthedocs.io/en/latest/configuration.html)
- **Schema rules.** Defined in `schemas.json` (set via `schema_definitions_from_json` in `[needs]` of `ubproject.toml`, which the docs recommend) or `needs_schema_definitions`. Fields are typed under `[needs.fields.<name>]` with `schema.type`, `schema.enum`, `schema.minimum` and similar keys. — [Schema validation](https://sphinx-needs.readthedocs.io/en/latest/schema/index.html)
  - Each rule has `id`, `severity` (`violation` default, `warning`, `info`), `message`, `select` (a JSON Schema filter on local properties) and `validate` with `local`, `network` and/or `network_back`.
  - Network rules use `items`, `contains`, `minContains` and `maxContains` on linked needs; they nest up to 4 levels ("Maximum network validation recursion level 4 reached."). `network_back` checks incoming links, e.g. "every requirement is linked by at least one test".
  - Types: string (`minLength`, `maxLength`, `pattern`, `format` date/date-time/email/uri/uuid, `enum`, `const`), integer and number (min/max/multipleOf), boolean, and array (`items` required, plus `uniqueItems` and min/max counts).
  - Pattern syntax is limited to what "Python, Rust, and SQLite" all support: no lookarounds and no backreferences.
  - Severity mapping: violation is shown as ERROR, warning and info as WARNING. Field and link constraints from `needs_fields`, `needs_links` and type entries are always `violation` unless the type declaration lowers the severity.
  - Output: console `WARNING|ERROR: <message> [type.subtype]` with subtypes `field_fail`, `link_fail`, `local_fail`, `network_missing_target`, `network_contains_too_few`, `network_contains_too_many` and `network_items_fail`. `schema_violations.json` has a summary and findings grouped by need ID; suppressed findings still appear there.
  - Limitations: schema rules do not follow `inherits`; cross-field and multi-hop conditions need schema rules rather than type entries; legacy `needs_constraints` and `needs_warnings` remain during migration.
- **Legacy constraints.** `needs_constraints` are Python filter strings (`"'critical' in tags"`) with a `severity` key mapped to `on_fail` actions `warn`, `break` or a `style`; `needs_warnings` are filter strings such as `"type == 'req' and not status"`. Setting `needs_warnings` "will deactivate incremental build support". — [Configuration](https://sphinx-needs.readthedocs.io/en/latest/configuration.html)
- **Dynamic functions.** Written as `[[copy("xxx")]]` in defaults or values. Custom ones are registered with `add_dynamic_function(app, fn)`, and setting `needs_functions` deactivates incremental builds. Values computed by dynamic functions or variants are written to the needs.json schema default as `[]`. — [Configuration](https://sphinx-needs.readthedocs.io/en/latest/configuration.html); [Builders](https://sphinx-needs.readthedocs.io/en/latest/builders.html)
- **Variants.** `needs_variants` maps variant names to filter strings, e.g. `"var_a": "'var_a' in build_tags"`. — [Configuration](https://sphinx-needs.readthedocs.io/en/latest/configuration.html)
- **needs.json format 1.** Top level: `created`, `current_version`, `project`, `versions`. Each version holds `created`, `needs_schema`, `needs_defaults_removed` and the `needs` map. Raising the version stores the new needs beside the old, "so one file keeps a history". — [Builders](https://sphinx-needs.readthedocs.io/en/latest/builders.html)
  - `needs_schema` is a JSON Schema (draft-07) with one property per field, each with `description`, `field_type` (`core`, `links`, `backlinks`, `extra`) and `type`.
  - Options: `needs_json_remove_defaults`, `needs_json_exclude_fields`, `needs_reproducible_json` (no timestamps) and `needs_build_json`. A `needs_id` builder writes one JSON file per need with percent-escaped names.

  — [Builders](https://sphinx-needs.readthedocs.io/en/latest/builders.html); [Configuration](https://sphinx-needs.readthedocs.io/en/latest/configuration.html)
- **needs.json format 2a1 (experimental).**
  - No `versions` key; it has `format_version`, a `project` object, a `creator` object and a `needs` list.
  - Each record has `id`, `type`, `fields`, `links`, `content` and `source`.
  - Readers older than 9.3.0 refuse it, and 8.5.0 aborts the build.
  - `needimport` and `needs_external_needs` now read through the `ub-needs-exchange` library "under its published exchange contract".

  — [Builders](https://sphinx-needs.readthedocs.io/en/latest/builders.html); [Changelog](https://sphinx-needs.readthedocs.io/en/latest/changelog.html)
- **Visualisations.** ubCode previews and builds needlist, needtable, needflow, needsequence, needpie and needbar. — [ubCode roadmap](https://ubcode.useblocks.com/development/roadmap.html). Analytical directives scan all needs, an O(N) cost per directive, and one user with about 20,000 local and 30,000 imported needs split the project because of slow builds. — phase 1 notes, citing the [5.1.0 changelog](https://sphinx-needs.readthedocs.io/en/5.1.0/changelog.html) and [discussion #1220 (mirror)](https://cf-workers-proxy-9e9.pages.dev/useblocks/sphinx-needs/discussions/1220)

### Inferences
- The field model has converged on a "JSON Schema subset in TOML" (`[needs.fields.x] schema.type/enum/pattern`). requimd can adopt the same vocabulary (`type`, `values`, `pattern`, `min`/`max`) without a JSON-schema engine, and keep regexes to what Rust's `regex` crate supports, as Sphinx-needs already does.
- `target_types` arrived only in 9.1.0 (October 2026); requimd's `to = [...]` has had the same constraint since M1. requimd is ahead here. It lacks source-type constraints, cardinality (`back_links` counts) and inverse names.
- Sphinx-needs needed three generations (filter-string constraints, `needs_warnings`, then JSON-schema rules) because Python expressions blocked incremental builds and IDE validation. A declarative, non-Turing-complete rule format is the lesson; requimd should not add an expression language.
- The `local` versus `network` split matters for an LSP: local rules can run on every keystroke, network rules after the project index updates. requimd's checks can be classified the same way.
- needs.json has abandoned the "history of versions inside one file" design in favour of a flat list with `source`. requimd, whose history lives in git, should start flat.
- `copy` (copying a link into another link kind) was deprecated for explicit derivation. requimd should keep links explicit and only compute inverses.

### Gaps
- The full need record of format 1 (`docname`, `lineno`, `sections`, `links_back` and others) is not listed on the builders page; the `ub-needs-exchange` contract itself was not found by search.
- The built-in dynamic functions (`check_linked_values`, `links_from_content` and others) were not covered by the fetched excerpts.
- No benchmark of 9.x builds at 10,000+ needs was found.

## ubCode: what it does, licensing, and the market signal

### Takeaway
ubCode is useblocks' proprietary single binary (`ubc`) that indexes Sphinx-needs projects without Python: LSP, live preview, checks, diff and impact analysis, HTML and PDF builds, an MCP server and an experimental agent. It is free for public open-source repositories and needs a commercial key otherwise; unlicensed private use is capped at 5 source files or 100 needs for most CLI commands. The open engine (Sphinx-needs, MIT) and the fast, editor-grade tooling (ubCode) are now separate products.

### Cited Findings
- **Features.**
  - `ubc` is one standalone binary needing no Python, LaTeX or `conf.py`.
  - Commands: `ubc quickstart`, `ubc check` (can fail a PR on selected findings), `ubc build html` and `ubc build pdf`, `ubc serve lsp`, `ubc agent`, plus query, diff and report.
  - LSP: diagnostics as you type, hover, completion, navigation, quick-fixes and formatting.
  - VS Code extension, also on Open VSX for Cursor, Kiro and VSCodium, with preview, needs graph, diff and impact analysis.
  - An MCP server for AI assistants, and "Pharaoh", an experimental agent whose engine "holds" a stage until it passes.
  - "No telemetry", and every network request is listed.

  — [ubCode](https://ubcode.useblocks.com/)
- **Roadmap state (0.39.0).**
  - Done: schema validation (0.21.0), impact analysis via `ubc diff` (0.21.0), the needs graph (0.19.0), source-code tracing via Codelinks for C/C++, Python, Rust, C# and YAML (0.29.0), and initial MyST Markdown parsing (0.30.0).
  - Planned: MyST editor features, Markdown formatting, workspace support for multiple projects, and detection of unsupported Sphinx-needs features.
  - The roadmap has no ReqIF or Excel item.

  — [ubCode roadmap](https://ubcode.useblocks.com/development/roadmap.html)
- **Licensing.**
  - "ubCode is proprietary software developed and owned by useblocks GmbH"; the source is not public.
  - The open-source mode is free when the project's git remotes are publicly reachable on GitHub, GitLab, Bitbucket, Codeberg or Gitea, and the terms require an OSI licence. Everything else needs a commercial key requested by e-mail; no prices are published.
  - Key types: user key, offline `ActivationFile.skm`, and CI systems licences (CLI only).
  - Keys are validated online against `api.cryptolens.io` and cached for 3 days.
  - "ubCode has no feature tiers". Unlicensed limits: most CLI build and check commands are capped at 5 source files, some at 100 needs, and `ubc format`, `query`, `diff` and coverage are refused. The editor LSP works on any size, but previews are capped and MCP project tools need a licence.

  — [ubCode licensing](https://ubcode.useblocks.com/usage/ublicense.html)
- **Exchange is a separate product.** useblocks documents ReqIF under ubConnect, with per-object-type mapping blocks that replace the global mapping and XHTML-typed attributes (search snippet; the fetched page returned 404). — [ubConnect ReqIF (search result)](https://ubconnect.useblocks.com/stable/features/reqif.html)
- **Shared core.** Sphinx-needs restricts schema regexes to what "Python, Rust, and SQLite" support and uses `jsonschema-rs` and minijinja. This suggests that the open Python package and the closed binary share a non-Python core. — [Schema validation](https://sphinx-needs.readthedocs.io/en/latest/schema/index.html); [Changelog 7.0.0 docs](https://sphinx-needs.readthedocs.io/en/7.0.0/changelog.html)

### Inferences
- The market signal: a vendor with the most popular open docs-as-code requirements engine decided that the editor experience (LSP, preview, fast checks, MCP) is what companies pay for. That validates requimd's bet on LSP plus checks in one binary. It also means requimd competes directly with a paid product in private corporate repositories, which is where business analysts work. ubCode's free tier covers only public repositories, and the size caps make a free trial on a real private spec impossible without a key.
- requimd's open licence plus no network calls at all (NFR-006) is a sharper offer than ubCode's online key check, which needs an offline file in air-gapped settings. This is a selling point for regulated or internal teams.
- ubCode is still tied to RST and MyST directive syntax and to Sphinx semantics. Its Markdown is MyST and only partly supported; there is no glossary or language checking and no BA-oriented item types. requimd's niche (plain CommonMark, BA vocabulary, German language checks) remains uncontested by it.
- useblocks sells ReqIF as a separate connector product (ubConnect). That suggests ReqIF is an enterprise-integration revenue line, not a commodity feature, and that a free minimal ReqIF export in requimd would stand out.

### Gaps
- ubCode's implementation language is not stated on its pages (Rust is plausible from the evidence above but unconfirmed).
- No prices, customer numbers or adoption figures are public.
- The ubConnect ReqIF page could not be fetched (404), so its mapping format and tested tools are unconfirmed.

## ReqIF in practice: what DOORS Next, Polarion, Codebeamer and Jama need

### Takeaway
No vendor documents a "minimum" ReqIF. In practice the target tools need stable identifiers on every element (objects, types, attribute definitions, enum values), the ProSTEP conventional attributes (`ReqIF.ForeignID`, `ReqIF.Name`, `ReqIF.ChapterName`, `ReqIF.Text`), XHTML for rich text, and a SPECIFICATION hierarchy. Everything else is mapped interactively by the importing user. Identity stability decides whether a second delivery updates or duplicates.

### Cited Findings
- **ProSTEP guidance.** The current ProSTEP iViP ReqIF Implementation Guideline is V1.10 (January 2024, referring to OMG ReqIF 1.2). — [ProSTEP guide PDF](https://www.prostep.org/fileadmin/prod-download/ProSTEP_iViP_ImplementationGuide_ReqIF_V1-10_aw.pdf) (title via search; the PDF was not read). Importers describe the conventional defaults as headline `ReqIF.ChapterName`, short description `ReqIF.Name` and XHTML content `ReqIF.Text`. — [reqteam ReqXML to ReqIF](https://documentation.reqteam.com/reqxml-to-reqif/). ReqView maps ChapterName to heading and Text to text, falling back to Name. — [ReqView import](https://reqview-homepage.netlify.app/doc/import-reqif) (via search summary)
- **DOORS Next.**
  - ReqIF IDs "must remain unique and unchanged across all ReqIF related interactions".
  - Imports are additive: "Imports are non-destructive - it does NOT remove/delete any resource."
  - A changed attribute ID shows up as a new, duplicate attribute. Same-named types with different IDs from other scopes are created with a `[DNG-Renamed-n]` suffix.
  - Enumeration values are matched and missing ones inserted. Removing and re-adding values risks data loss.
  - Mapping contexts match by name when there are no RDF URIs.

  — [jazz.net ReqIF usage best practices](https://jazz.net/wiki/bin/view/Deployment/ReqIFUsageBestPractices?rev=2)
- **DOORS Next and XHTML.** IBM's own ideas portal says DOORS Next's XHTML `<object>` usage is nonstandard. A Polarion-exported ReqIF failed XML schema validation on DNG import, and pictures were lost on DOORS import (forum reports). — [IBM community](https://community.ibm.com/community/user/discussion/picture-attributes-are-lost-during-reqif-import); [IBM ideas ENGRMDN-I-725](https://ibm-ai-apps.ideas.ibm.com/ideas/ENGRMDN-I-725) (via search summary; individual claims not verified on the pages)
- **Codebeamer.**
  - Mapping happens per specification, item type and relation type; every attribute is mapped to a tracker field or ignored.
  - "ReqIF has no reference fields": Codebeamer converts reference fields to relations on export and binds relation types to reference fields on import.
  - Imports trigger no workflow actions, and import into existing projects needs 9.3 or newer.

  — [Codebeamer import of ReqIF data](https://support.ptc.com/help/codebeamer/r2.1/en/codebeamer/user_guide/ug_import_reqif_data.html) (via search summary)
  - For round trips "the ReqIF.ForeignID must be the same" in both files. Choice values are matched by name, and values unknown in the target are dropped, not created. Ignoring an attribute loses the mapping.

  — [Codebeamer round trip](https://support.ptc.com/help/codebeamer/r3.3/en/codebeamer/user_guide/ug_FieldValueManagement_RoundTrip_ReqIF_ExportImport.html)
- **Jama.**
  - Jama Connect Interchange matches items to existing items through a specified ReqIF ID field, and Jama recommends a dedicated custom field for ReqIF identifiers.
  - Pick lists can import blank if their values are not mapped.
  - Spec folders do not map (hierarchy flattened in a Visure case).
  - Data types map Boolean, Date and Integer to the native types, but Real to text.
  - Maximum file size is about 500 MB.

  — [Jama Visure exchange](https://support.jamasoftware.com/hc/en-us/articles/39529450230413-Visure-Data-Exchange-with-Jama-Connect-via-ReqIF); [Pick list issue](https://support.jamasoftware.com/hc/en-us/articles/46518022753165-Pick-List-Field-Not-Populating-After-ReqIF-Import-in-Jama-Connect-Interchange); [Data Exchange guide](https://support.jamasoftware.com/hc/en-us/articles/24585785107597-Data-Exchange-User-Guide) (via search summaries)
- **Polarion.** No official Polarion mapping documentation was found. MathWorks notes that Polarion and DOORS "require a specific set of ReqIF data type, attribute, and SpecObject type definitions". Siemens describes baseline-then-ReqIF "roundtrip" back to the sender. — [MathWorks](https://au.mathworks.com/help/slrequirements/ug/import-requirements-from-reqif-files.html); [Siemens Polarion blog](https://blogs.sw.siemens.com/polarion/part-1-how-to-exchange-data-from-ibm-rational-doors-classic-to-siemens-polarion/) (via search summary)
- **Capella.** Capella reportedly breaks when `ReqIF.ForeignID` values contain non-numeric characters (forum). — [Capella forum](https://forum.mbse-capella.org/t/problems-on-reqif-import/11185/2) (via search summary)

### Inferences
- A minimal ReqIF that every one of these tools can ingest is a ReqIF 1.2 file with deterministic identifiers on all elements, the four ProSTEP attributes, enumerations for value sets, one SPEC-OBJECT-TYPE per item type, one SPEC-RELATION-TYPE per link kind, and one SPECIFICATION per source file. The importing user does the mapping, and identity stability makes the second import update instead of duplicate.
- Rich text is the riskiest area. A conservative XHTML subset (paragraphs, lists, emphasis, links, no `<object>`, no images in a first version) avoids the documented failures.
- The non-numeric ForeignID problem (Capella) means `FR-012` may not survive everywhere. An option to also export the numeric part, or to put the requimd ID in a separate attribute, may be needed. Treat this as a test point, not a design driver.
- Import is much harder than export: every vendor writes its own dialect, and the `reqif` library's experience shows incomplete schemas and tool-specific ordering. A one-off "convert a customer's ReqIF into requimd Markdown" is the realistic scope; synchronising back is not.

### Gaps
- The ProSTEP V1.10 guide itself (normative attribute list, XHTML subset, tool extension rules) was not read; it should be read before implementing.
- No first-hand Polarion or DOORS Next documentation on minimum required elements was found; claims rest on forums and vendor help pages of varying versions.
- Which XHTML elements each tool accepts in `ReqIF.Text` was not established.

## Excel round trips for reviewers

### Takeaway
Among the open tools, only Doorstop round-trips spreadsheets. It does so naively: the row's UID selects the item, the old item is deleted and recreated from the row, the file always wins, and there is no conflict detection. StrictDoc exports only. A safe design needs a content hash per row (optimistic concurrency), separate verdict and comment columns, and an import that produces a review record or proposed changes instead of overwriting files.

### Cited Findings
- **Doorstop export.**
  - Columns: `uid` first, then `level`, `text`, `ref` (dropped if unused), `references`, `links`, then custom attributes sorted.
  - Links are joined with newlines.
  - XLSX formatting: bold header, top-left wrap alignment, auto filter over the header, column width from the longest line (capped at 65), and frozen header row.

  — [doorstop exporter.py](https://raw.githubusercontent.com/doorstop-dev/doorstop/develop/doorstop/core/exporter.py)
- **Doorstop import.**
  - The header row is lower-cased. A `uid` or `id` column selects the item, and `links` are split on whitespace, semicolons or commas.
  - Rows with text but no UID get the next number.
  - An existing item is deleted (`_delete_existing`) and recreated from the row only, which is a replacement, not a merge.
  - Rows missing from the file leave items untouched.
  - Errors are logged and skipped, but a failure after the delete can leave the item removed.
  - "There is no conflict detection, merging, or timestamp check": the file wins.

  — [doorstop importer.py](https://raw.githubusercontent.com/doorstop-dev/doorstop/develop/doorstop/core/importer.py)
- **StrictDoc.** Best-effort header-to-field import and `--fields` export; no SDoc → Excel → SDoc round trip. — [User guide source](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc)
- **Rust crates within a single binary.**
  - `rust_xlsxwriter` (a port of Python's XlsxWriter) writes XLSX with data-validation dropdowns and cell locking and sheet protection. It cannot modify existing files, and its protection "is very easy to deactivate".
  - `calamine` is pure Rust and read-only. It reads xls, xlsx, xlsb and ods but not formatting or data-validation rules.

  — [rust_xlsxwriter data validation](https://rustxlsxwriter.github.io/examples/data_validation.html); [rust_xlsxwriter protection](https://rustxlsxwriter.github.io/worksheet/protection.html); [calamine](https://docs.rs/calamine/)

### Inferences
- Doorstop shows the failure modes to avoid: overwriting by row, losing attributes absent from the sheet, and silently clobbering edits made in git since the export. A content hash column, which requimd already computes for FR-116 and FR-403, gives cheap optimistic concurrency. If the hash in the row differs from the current item hash, the row is a conflict and is reported, not applied.
- Reviewers mostly give verdicts and comments, rarely rewritten text. Making verdict and comment the primary columns and "proposed text" optional keeps the import simple and honours BR-004 (no tool sets `approved`).
- CSV alone works without any crate. Excel-friendly CSV for German locales typically needs a UTF-8 BOM and a semicolon delimiter. This is common practice, not verified from a source in this session. XLSX adds dropdowns and locked ID and hash columns at the cost of two pure-Rust crates.

### Gaps
- No source was found on how commercial tools (Jama, Polarion, ReqView) structure their Excel round-trip conflict handling; this session did not reach their docs.
- The binary-size cost of `rust_xlsxwriter` plus `calamine` was not measured.

## Proposal for requimd

### Takeaway
Extend `requimd.toml` in place, without a new file or an expression language: typed keys with value sets and patterns, link kinds with inverse names, source types and cardinality, and a small list of declarative rules with requimd's existing severities. Mirror needs.json's embedded schema and flat source-annotated records in the JSON export. Add a one-way, deterministic ReqIF 1.2 export following the ProSTEP attribute conventions, and defer import to a one-off converter. Offer a CSV-first, hash-guarded review round trip whose import writes a review report, never item status.

### 1. Schema extension for `requimd.toml`
Design principles drawn from the evidence:
- one central schema, not per-document grammars (StrictDoc's drift to `.sgra`);
- a JSON-Schema-like vocabulary without a JSON-Schema engine (Sphinx-needs `[needs.fields.x] schema.*`);
- regexes limited to the Rust `regex` crate, as Sphinx-needs limits them to Python, Rust and SQLite;
- rules split into local (per item, run on every keystroke in the LSP) and network (after indexing), as Sphinx-needs 6.0 did;
- no Python-style filter strings, which cost Sphinx-needs incremental builds.

Example (new keys are commented; existing ones keep their meaning, so FR-014 defaults stay valid):

```toml
# --- Typed keys (extends [values]; [values] stays as shorthand for type = "enum") ---
[keys.prio]
type = "enum"                       # enum | list | text | int | number | date | bool
values = ["must", "should", "could", "wont"]
ordered = true                      # enables "child prio not above parent" (R-21)

[keys.src]
type = "text"
pattern = '^(BGB|HGB|DSGVO) (§|Art\.) ?\d+'   # Rust regex syntax, no lookaround
help = "Legal source, e.g. 'BGB § 312g'"         # shown in hover and completion

[keys.due]
type = "date"                       # ISO 8601, checked offline

[keys.tags]
type = "list"                       # comma-separated (FR-004), optional value set
values = ["legal", "ux", "security"]
open = true                         # true: values are suggestions, not a closed set

[keys.must]
type = "text"                       # Planguage level stays free text for now

# --- Link kinds: inverse names, source types, cardinality ---
[links.ver]
name = "verifies"
inverse = "verified by"             # hover, show, site backlinks (R-02)
from = ["test"]                     # allowed source types (new; `to` already exists)
to = ["functional", "quality", "story"]
max = 0                             # 0 = unbounded targets per item (default)

[links.ref]
name = "refines"
inverse = "refined by"
acyclic = true                      # today fixed by FR-015 for ref; makes it a property (R-03)
max = 1                             # single parent

[links.impl]
name = "implements"
inverse = "implemented by"
to = ["usecase", "step"]

# --- Declarative rules with severities (R-19, R-24; DD-4 owns coverage policy) ---
[[rules]]
id = "must-needs-test"              # becomes a check id, overridable in [checks]
severity = "warning"                # error | warning | info | off (existing sev values)
select = { type = ["functional", "quality"], prio = "must", status = ["review", "approved"] }
incoming = { ver = { min = 1 } }    # network rule, depth 1 only
message = "A must requirement in review needs at least one verifying test."

[[rules]]
id = "src-required-for-rules"
severity = "error"
select = { type = "rule" }
require = ["src"]                   # local rule; same semantics as `required`, but conditional

[checks]
invalid-value = "error"             # existing: severity override by check id
must-needs-test = "info"            # rule ids are overridable like built-in checks
```

Notes:
- `select` matches only key equality or membership. `require` checks key presence. `incoming` and `outgoing` count links by kind, optionally with `types = [...]` on the other end.
- Rules go one hop only; anything deeper is DD-4's coverage policy.
- Unknown keys in any table are reported (Sphinx-needs 9.2 learned this late).
- Severities reuse the `sev` vocabulary (error, warning, info, off) and the existing `[checks]` override mechanism. Built-in schema checks (FR-103, FR-104, FR-105) stay `error` by default, like Sphinx-needs' "violation" default.

### 2. What of needs.json to mirror in `requimd export json` (FR-402)
- **Flat record list with source**, like format 2a1: `format_version`, `project` (name, root, config hash), `creator` (`requimd` and version), `items[]` with `id`, `type`, `title`, `meta`, `links` (by kind), `body`, `source` (file, line, byte range). Do not store a history of versions in one file; git and `requimd diff` cover that.
- **Embedded schema** (`schema`: item types, keys with type and values, link kinds with `name`, `inverse`, `from` and `to`). Consumers can then validate and render without parsing `requimd.toml`. This is the most reused part of needs.json (`needs_schema` with `field_type` core, links, backlinks or extra).
- **Computed backlinks**, keyed by kind and labelled with the inverse name, so the site, other repositories (R-33) and AI tools need not invert the graph.
- **Reproducible by default** (no timestamps; Sphinx-needs needs `needs_reproducible_json` for this). Add a per-item content hash (R-76) and the lock file's suspect state.
- **Optional `--omit-defaults`**, matching `needs_json_remove_defaults`.
- **A documented, versioned contract** (a JSON Schema file for the export in `docs/`), as useblocks did with `ub-needs-exchange`. Readers refuse unknown major versions, as Sphinx-needs does with 2a1.
- Possibly later, an `export needs-json` adapter that writes Sphinx-needs format 1. That would let Sphinx-needs projects `needimport` requimd items. It is cheap if the internal model is stable, but optional.

### 3. Minimal ReqIF export (R-73); import only as a later one-off converter
`requimd export reqif [-o file.reqif] [--spec <glob>]` would write a single ReqIF 1.2 XML file, with no zip and no images in v1:

- **Header:** `REQ-IF-HEADER` with an IDENTIFIER derived from the project name, `REQ-IF-TOOL-ID` and `SOURCE-TOOL-ID` `requimd <version>`, and `REQ-IF-VERSION` `1.0`. `CREATION-TIME` comes from the HEAD commit time, which keeps output deterministic and offline.
- **DATATYPES:** one `DATATYPE-DEFINITION-STRING` (max length, e.g. 10000), one `DATATYPE-DEFINITION-XHTML`, and one `DATATYPE-DEFINITION-ENUMERATION` per value set, with ENUM-VALUE identifiers derived from key and value (e.g. `requimd-enum-prio-must`), not random. DOORS Next matches by ID and inserts unknown values; Codebeamer drops values unknown in the target.
- **SPEC-TYPES:**
  - One `SPEC-OBJECT-TYPE` per item type (`requimd-type-functional`, LONG-NAME `functional`), plus one for headings that are not items.
  - Attributes per type: `ReqIF.ForeignID` (STRING, the requimd ID), `ReqIF.Name` (title), `ReqIF.Text` (XHTML body), `ReqIF.ChapterName` (heading objects only), and one attribute per allowed meta key (ENUMERATION for value sets, else STRING). Attribute identifiers are stable, e.g. `requimd-attr-functional-prio`.
  - One `SPEC-RELATION-TYPE` per link kind (LONG-NAME = `name`, e.g. "verifies") and one `SPECIFICATION-TYPE`.
- **SPEC-OBJECTS:** IDENTIFIER `requimd-FR-012`; use-case steps become objects `requimd-UC-001.s2`. This must be checked against the xsd:ID rules: `.` and `-` are allowed in NCNames. `LAST-CHANGE` comes from the last commit that touched the item, or from HEAD for simplicity.
- **SPEC-RELATIONS:** source is the item that carries the link, target is the linked item or step, and the type is the link kind. This matches StrictDoc's convention.
- **SPECIFICATIONS:** one per Markdown file, LONG-NAME = the file's H1. The SPEC-HIERARCHY follows the heading tree, with stable hierarchy identifiers (path-derived).
- **XHTML:** a conservative subset rendered from pulldown-cmark: `p`, `br`, `ul`/`ol`/`li`, `strong`/`em`, `a`, and inline code as plain text. Tables and images come later, after testing against the ProSTEP guide.
- **Dependencies:** hand-written XML output (escaping only) or `quick-xml`. Optional validation against the OMG XSD stays out of the binary; document a `reqif` library or ProR check in CI instead.

Import: not needed for the first version. The realistic case is a customer delivering a DOORS or Polarion export once. Offer later `requimd import reqif <file> --map map.toml`, which generates new Markdown files and items with new IDs, keeps the foreign ID in a `src`-like key, and never merges into existing files. Bidirectional sync is out of scope (dialects, NFR-006 spirit, single source of truth).

### 4. Review round trip within the single binary (R-80)
- **Export:** `requimd export review [--since <ref>] [--type ...] [--status review] -o review.csv|review.xlsx`. Columns, in order:
  - `ID`, `Type`, `Title`, `Text` (plain text, steps inlined), `Status`, `Prio`, links (one column per kind, newline-joined), `Changed since` (from `requimd diff` when `--since` is given) and `Hash` (short content hash per FR-116 rules);
  - reviewer columns: `Reviewer`, `Verdict` (dropdown: ok / change / reject / question), `Comment`, `Proposed text` (optional).
- **CSV first** (no dependency): UTF-8 with BOM, with a `--delimiter ';'` option for German Excel.
- **XLSX second** via `rust_xlsxwriter` (write) and `calamine` (read), both pure Rust. The ID and hash columns are locked, the verdict has a dropdown, the header row is frozen, and the column filter and widths follow Doorstop.
- **Import:** `requimd import review review.xlsx [-o reviews/R-2026-11-18.md]`.
  - Rows are matched by ID.
  - If the row's `Hash` is not the item's current hash, the row is a conflict: the item changed since export. It is reported and never applied.
  - Unknown IDs are errors, and missing rows count as "not reviewed".
  - Output is a review record: a Markdown file with reviewer, date, verdict and comment per item and the hash reviewed. It is checkable (stale when the item changes), which links to DD-2's R-45.
  - `Proposed text` is never written into requirement files automatically. `--suggest` prints a unified diff or writes LSP-visible suggestions the analyst accepts.
  - The import never sets `status` (BR-004); at most it lists items that are candidates for `review`.
- **Avoid Doorstop's pattern:** no delete-and-recreate, no "file wins", no silent drop of columns.

### 5. Open questions for the product owner
1. Typed keys: is `[keys.<name>]` acceptable as a new section beside `[values]`, or should `[values]` grow into tables? Which types do analysts need: date, number, open tag lists, ordered enums?
2. Inverse names: one inverse per link kind in English only, or localised (`inverse = { en = "verified by", de = "verifiziert durch" }`) for German sites?
3. Should `from` (source types) and `max` (cardinality) be enforced by default for `ver` (only tests verify) and `ref` (single parent), or only when configured?
4. Declarative `[[rules]]`: in scope now, or wait for DD-4's coverage policy so that one mechanism covers both?
5. ReqIF: is there a concrete customer or partner exchange (DOORS Next, Polarion, Codebeamer, Jama) to test against? Without one, ReqIF should stay behind spreadsheet review on the roadmap, as phase 1 already suggests.
6. ReqIF granularity: one SPECIFICATION per Markdown file, or one per item type? Should use-case steps be separate objects or part of the use case's text?
7. Is a one-off ReqIF import (customer spec to Markdown) a real need, or is export enough?
8. Review round trip: CSV only first (no new dependency), or XLSX from the start? Does adding two crates fit NFR-005?
9. Should a review import be allowed to write proposed text changes (behind a flag, hash-guarded), or only produce a report and suggestions?
10. JSON export: should requimd also publish a Sphinx-needs-compatible needs.json adapter for mixed projects, or keep only its own documented contract?

### Gaps
- The proposal has not been checked against the remaining FRs that read `[values]` and `[links]` (FR-015's fixed names), nor against DD-3 (suspect scope per link kind) and DD-4 (coverage policy), which overlap with `[[rules]]` and link properties.
- The ReqIF identifier rules (xsd:ID for step IDs with `.`) and the XHTML subset must be confirmed against the OMG schema and the ProSTEP guide before implementation.
