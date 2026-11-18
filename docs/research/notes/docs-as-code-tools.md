# Open-source and docs-as-code requirements tools (requimd's closest relatives)

Scope: 16 tools or tool families that keep requirements as text files under version control, plus the ReqIF exchange format and executable-specification tools. Research date 2026-11-18. Public sources only; nothing was installed or run. Licences, star counts and versions are as shown on the cited pages on that date. Where a page shows no date, this is noted.

## Q1: What does each tool offer? (uniform profiles)

### Takeaway
The field splits into four groups. (a) Document-centric tools with their own markup and a rich data model: StrictDoc, Sphinx-needs/ubCode, Doorstop. (b) Tracing engines that read IDs from many file types and compute coverage: OpenFastTrace, LOBSTER, tracey, Reqflow. (c) Typed requirement languages: TRLC. (d) A new 2025-2026 wave of Rust, Markdown-first, git-native tools: tracey, SARA, Reqvire, mantra and cargo-ovft. Most new tools add an LSP and an MCP/AI server. Only StrictDoc and BASIL offer a web UI. Only StrictDoc offers serious ReqIF and Excel round trips among the text-file tools.

### Cited Findings

#### 1. Doorstop
- **Maintainer, licence, activity:** doorstop-dev organisation (founded by Jace Browning). LGPLv3, 697 stars, 2,853 commits, Python 3.10+, described as "under active development". The GitHub page shows no release date. — [GitHub](https://github.com/doorstop-dev/doorstop). A 2026 blog post calls the licence "MIT-style", which contradicts GitHub's LGPLv3. — [pistack.xyz, 2026-06-15](https://www.pistack.xyz/posts/2026-06-15-self-hosted-requirements-management-rmtoo-doorstop-strictdoc/)
- **Format and data model:**
  - One file per item: YAML, or Markdown with YAML front matter (`itemformat` per document). In Markdown items, the first H1 becomes `header` and the body becomes `text`.
  - Standard attributes: `active`, `derived`, `normative`, `level` (presentation order, e.g. `'1.10'`), `header`, `text`, `references` (external files with `path`, `type`, optional `keyword` and optional `sha256`), `links` (parent UID plus fingerprint) and `reviewed`.
  - Any number of custom "extended attributes", with defaults and publish lists set in `.doorstop.yml`. — [Item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)
  - A directory of items forms a document, and documents form a tree (`doorstop create SRD ./reqs/srd`, `doorstop link`). — [Docs home](https://doorstop.readthedocs.io/en/latest/)
- **Relations:** child-to-parent links only. Derived items need no parent. — [Item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)
- **Validation:** running `doorstop` checks the tree. The integrity checks include suspect links and unreviewed items. — [Docs home](https://doorstop.readthedocs.io/en/latest/); [Integrity checks v3.1](https://doorstop.readthedocs.io/en/v3.1/cli/validation.html)
- **Review and suspect links:** see Q2.
- **Publishing:** text, HTML (whole tree), Markdown, and LaTeX with a generated `compile.sh` that builds a PDF via pdflatex.
  - The traceability matrix follows the document tree by default. `--full-traceability` adds level-skipping and cross-branch links.
  - Custom HTML (CSS) and LaTeX (`.cls`) templates are supported. — [Publishing](https://doorstop.readthedocs.io/en/latest/cli/publishing.html)
- **Interchange:** export and import as YAML, CSV, TSV and XLSX ("Items can be created/updated from the export formats"). — [Interchange](https://doorstop.readthedocs.io/en/latest/cli/interchange.html)
- **APIs:** a REST interface and a Python API can query extended attributes. — [Item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)
- **Suitability for business analysts:** low to medium. The blog post calls it "intentionally minimal" and the most developer-centric of the tools it compares (needs Git and YAML), with no built-in web interface and no test management. — [pistack.xyz](https://www.pistack.xyz/posts/2026-06-15-self-hosted-requirements-management-rmtoo-doorstop-strictdoc/)
- **Ecosystem:** a fork-like tool, Jamb, reuses the same review and clear concept (`jamb review mark`, `jamb review clear`). — [Jamb FAQ](https://jamb.readthedocs.io/en/latest/faq.html). There is also a third-party Doorstop MCP server. — [Glama](https://glama.ai/mcp/servers/Jannled/DoorstopMCP)
- **Distinctive ideas:** per-link parent fingerprints; a configurable set of attributes that count toward the review fingerprint; spreadsheet round trip; a LaTeX/PDF template per document.

#### 2. StrictDoc (SDoc format, web UI, ReqIF)
- **Maintainer, licence, activity:** strictdoc-project, Apache-2.0. — [pistack.xyz](https://www.pistack.xyz/posts/2026-06-15-self-hosted-requirements-management-rmtoo-doorstop-strictdoc/)
  - Very active. Release 0.31.0 is dated "18 Nov" (the year is not shown, but a docs build of 2026-11-18 at 0.30.4-122 implies 2026). It added a table editor, link/anchor widgets for RST and Markdown fields, Ada and Modelica source parsing, and ReqIF export escaping fixes.
  - 0.30.x (Sep–Oct): the ReqIF importer reads Parent relations between node types, the Excel importer reads Parent relations, and a tree map was added.
  - 0.28.2 (Aug): move nodes across documents, custom UID hooks. — [Releases](https://github.com/strictdoc-project/strictdoc/releases); [User guide](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html)
- **Format and data model:**
  - `.sdoc` files with `[DOCUMENT]`, `[SECTION]`, `[TEXT]` and `[REQUIREMENT]`.
  - Default requirement fields: UID, LEVEL, STATUS, TAGS, TITLE, STATEMENT, RATIONALE, COMMENT, RELATIONS.
  - Custom element types are declared in `[GRAMMAR] ELEMENTS:` with TAG, PROPERTIES (`IS_COMPOSITE`, `PREFIX`, `VIEW_STYLE`) and FIELDS (TITLE, TYPE, REQUIRED). — [User guide](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html)
  - Field types: String, SingleChoice(A, B, ...), MultipleChoice(...) and Tag. — [User guide (TRACE)](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide-TRACE.html)
  - Markdown input is experimental, with grammars in `.gra.md` files. — [User guide (TRACE)](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide-TRACE.html)
- **Relations:**
  - Parent, Child and File, with optional ROLE (e.g. `Refines`) registered in the grammar and a REVERSE_ROLE declared once.
  - Cycles are validation errors.
  - MIDs (machine identifiers, `ENABLE_MID: True`) track nodes across moves and renames, separate from human UIDs. — [User guide](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html)
- **Authoring:**
  - A web UI (`strictdoc server`, 127.0.0.1:5111) edits and writes back to `.sdoc` and Markdown files. It "is work-in-progress" and "not yet hardened against unsafe use". Commits are still manual. — [User guide](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html); [User guide (TRACE)](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide-TRACE.html)
  - Syntax highlighting for VS Code, JetBrains, Eclipse, Sublime and Neovim. An LSP is on the roadmap (issue #577) and not available yet. — [User guide](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html)
- **Traceability:**
  - Source-code tracing with language-aware parsing for C/C++/Rust and others, plus file relations. — [User guide](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html); [Releases](https://github.com/strictdoc-project/strictdoc/releases)
  - Test-report import for JUnit XML, LLVM, Robot Framework and cargo-nextest (listed in the TOC only).
  - A diff/changelog between versions shows added, removed and changed content at a granular level. — [Feature map](https://strictdoc.readthedocs.io/en/stable/stable/docs/strictdoc_02_feature_map-DEEP-TRACE.html)
- **Search:** a Python-like query engine (e.g. `node.is_requirement` with AND/OR) in the web UI, plus project statistics. — [Feature map](https://strictdoc.readthedocs.io/en/stable/stable/docs/strictdoc_02_feature_map-DEEP-TRACE.html)
- **Publishing and interchange:**
  - Static HTML (incremental; 2,000–3,000 requirements in a few seconds), RST, ReqIF, PDF, JSON and Excel. — [User guide](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html)
  - PDF goes through HTML2PDF (Selenium, ChromeDriver and Chrome).
  - ReqIF support "is tool-specific" and may need adjustment. — [Feature map](https://strictdoc.readthedocs.io/en/stable/stable/docs/strictdoc_02_feature_map-DEEP-TRACE.html)
- **Suitability for business analysts:** rated the most accessible interface for non-developers among rmToo, Doorstop and StrictDoc, but with the most resource-intensive server. — [pistack.xyz](https://www.pistack.xyz/posts/2026-06-15-self-hosted-requirements-management-rmtoo-doorstop-strictdoc/)
- **Distinctive ideas:** grammar per document; relation roles with reverse names; MIDs; a web editor that writes back to text; composable documents (`DOCUMENT_FROM_FILE`).

#### 3. Sphinx-needs (with ubCode and sphinx-test-reports)
- **Maintainer and activity:** useblocks. Docs version 9.2.0, with 9.3.0 features already documented. — [Configuration](https://sphinx-needs.readthedocs.io/en/latest/configuration.html). (The licence was not verified in this session; see Gaps.)
- **Data model:** "needs" are directives in RST or MyST Markdown.
  - Types via `needs_types` (directive, title, prefix, color, style, `id_regex`, `inherits`, per-type `fields`, `links` and `back_links`). Default types: req, spec, impl, test, need.
  - Fields via `needs_fields`, which replaces `needs_extra_options` for new fields. Each field has a JSON-schema-like `schema`, `nullable`, `default`, `predicates` and `derive`, which computes the value.
  - Links via `needs_links`/`needs_extra_links`, with `incoming`/`outgoing` names, `allow_dead_links`, `target_types` (9.1.0) and `summary` shortcut links (9.2.0).
  - `severity` (violation, warning or info) on schema constraints.
  - Variants and dynamic functions.
  - Configuration in TOML via `needs_from_toml`.
  - Link `copy` is deprecated since 9.1.0 and replaced by `union` derive rules. — [Configuration](https://sphinx-needs.readthedocs.io/en/latest/configuration.html)
- **Validation:** Sphinx warnings, where `-W` fails the build; custom checks via `add_warning()`; schema validation; constraints. — [Configuration](https://sphinx-needs.readthedocs.io/en/latest/configuration.html)
- **Interchange:** the needs.json builder; `needimport` and `needs_external_needs` pull in other projects. — [Configuration](https://sphinx-needs.readthedocs.io/en/latest/configuration.html)
- **ubCode (useblocks, v0.39.0, 2026):**
  - Language server `ubc serve lsp` with diagnostics, hover, completion, navigation, quick-fixes and formatting.
  - VS Code and Open VSX extension with preview.
  - The standalone CLI `ubc` checks, queries, diffs and reports, and builds HTML and PDF "without Python or LaTeX".
  - MCP server; an experimental "Pharaoh" agent.
  - Free for open source; a commercial licence for private projects; an offline licence file for air-gapped use. — [ubCode](https://ubcode.useblocks.com/)
  - It replaces the deprecated sphinx-needs-vscode extension. — [useblocks tutorial](https://x-as-code.useblocks.com/tutorials/sphinx-needs-setup.html)
- **sphinx-test-reports (4.0.0):** turns JUnit XML and JSON results into needs. Directives: `test-file`, `test-suite`, `test-case`, `test-report`, `test-results` and `test-env`. Results are passed, failed or skipped, and the needs can be linked and filtered next to requirements. — [sphinx-test-reports](https://sphinx-test-reports.readthedocs.io/en/latest/)
- **Weaknesses:** one user with about 20,000 internal and 30,000 imported needs reports a persistently slow write phase and split the project into several. — [Discussion #1220 (mirror)](https://cf-workers-proxy-9e9.pages.dev/useblocks/sphinx-needs/discussions/1220). Analytical directives (needtable, needuml) do an O(N) scan of all needs. — [5.1.0 changelog](https://sphinx-needs.readthedocs.io/en/5.1.0/changelog.html). A third party calls Sphinx-needs "heavy-weight for some use-cases". — [sphinx-graph](https://awesome.ecosyste.ms/projects/github.com%2Fdanieleades%2Fsphinx-graph)
- **Distinctive ideas:** a JSON-schema per field; link-target type constraints; computed (derived) fields; one LSP and CLI binary that replaces the Python toolchain (ubCode); test results as first-class items.

#### 4. OpenFastTrace (OFT)
- **Maintainer, licence, activity:** itsallcode. GPL-3.0, 210 stars, version 4.10.0 in the README. Java 17+, runs with `java -jar` or JBang. An IntelliJ plugin exists. — [GitHub](https://github.com/itsallcode/openfasttrace)
- **Format:** spec items in Markdown.
  - ID line `` `req~tracing.defect-items~2` `` (type~name~revision) under a heading.
  - `Covers:` lists links to upstream items. `Needs: dsn` lists the artifact types required to cover the item.
  - Status values: approved, proposed, draft, rejected. — [OFT system requirements](https://github.com/itsallcode/openfasttrace/blob/main/doc/spec/system_requirements.md)
- **Code and test tracing:** a "tag importer" reads coverage tags such as `// [impl->dsn~validate-authentication-request~1]` from 25+ languages, plus JSON, TOML, YAML, HTML, PlantUML and Gherkin `.feature` files. See Q4. — [Input format support](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide/reference/input_format_support.md)
- **Link statuses:** outdated, predated, ambiguous, unwanted and orphaned. Also "deep coverage" and "transitive defects". — [OFT system requirements](https://github.com/itsallcode/openfasttrace/blob/main/doc/spec/system_requirements.md)
- **Reports:**
  - A single-file HTML report with a summary, progress bar and defect counts. — [OFT system requirements](https://github.com/itsallcode/openfasttrace/blob/main/doc/spec/system_requirements.md)
  - Console and XML reports, and Maven and Gradle integration. — [Reference index](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide/reference/reference.md)
- **Real-world use and derivatives:** Eclipse Ankaios uses `swdd`, `impl`, `utest`, `itest` and `stest` types and produces an HTML report in CI. — [Ankaios](https://eclipse-ankaios.github.io/ankaios/0.7/development/requirement-tracing). There is a Rust Cargo plugin, cargo-ovft. — [docs.rs](https://docs.rs/cargo-ovft)
- **Design-level support:** strong. `dsn` is a first-class artifact type in the chain feat → req → dsn → impl/utest. — [OFT system requirements](https://github.com/itsallcode/openfasttrace/blob/main/doc/spec/system_requirements.md)
- **Suitability for business analysts:** medium. The Markdown is readable, but the IDs and revisions are developer-flavoured.
- **Distinctive ideas:** a revision in the ID as an explicit "semantic change" signal; per-item `Needs:`; needed-coverage declared from inside a tag (`>>impl,test`); one tag syntax for every file type.

#### 5. TRLC (Treat Requirements Like Code, BMW)
- **Licence and activity:** GPL-3.0, 577 commits, Python reference implementation. CVC5 is needed for `--verify` (GNU/Linux only). Bazel integration exists. — [GitHub](https://github.com/bmw-software-engineering/trlc). Version 3.0.0 is on the Bazel registry. — [Bazel registry](https://registry-preview.bazel.build/modules/trlc/3.0.0)
- **Format:** a DSL "for writing (and linking) requirements with meta-data".
  - `.rsl` files hold user-defined types (records, tuples, enums, inheritance, packages), checks and warnings. `.trlc` files hold the record instances.
  - Processing order: all `.rsl` files are parsed first, then the `.trlc` files; references are resolved, then checks run.
  - The stated design goal is "No business logic should be encoded in the language design". — [TRLC LRM tracing report](https://bmw-software-engineering.github.io/trlc/tracing.html); [GitHub](https://github.com/bmw-software-engineering/trlc)
- **Validation:** static analysis of types plus user-defined check rules. A linter. A Python API for custom checks. A `--brief` CI mode. — [Tutorial index](https://github.com/bmw-software-engineering/trlc/blob/main/documentation/TUTORIAL.md); [search summary of TUTORIAL-CI](https://github.com/bmw-software-engineering/trlc/blob/main/documentation/TUTORIAL-CI.md)
- **Tooling:** the API supports HTML rendering, diffing, impact analysis and traceability tools. TRLC_RST converts for Sphinx. NewTec maintains converters. — [GitHub](https://github.com/bmw-software-engineering/trlc)
- **Suitability for business analysts:** low. It is programming-language-like.
- **Distinctive ideas:** a schema separate from data (`.rsl` vs `.trlc`); declarative checks ("if X then field Y required"); formal verification of the checks themselves.

#### 6. LOBSTER (BMW)
- **Licence and activity:** AGPL-3.0, 65 stars, 733 commits. — [GitHub](https://github.com/bmw-software-engineering/lobster)
- **Architecture:** converters extract trace tags into a common `.lobster` JSON interchange format. A report tool combines them under a tracing policy into `report.lobster`, and renderers produce HTML (with a graphviz policy diagram) or CI output.
  - Converters: lobster-trlc, -python, -cpp, -cpptest, -gtest, -codebeamer and -json.
  - Third-party converters: lobster-doxygen, MISS_HIT (MATLAB/Simulink), lobster-rust.
  - Planned: Java and Kotlin. — [GitHub](https://github.com/bmw-software-engineering/lobster)
- **Policy:** see Q4. The policy file (`lobster.yaml`) defines levels of kind requirements, implementation or activity, with `trace-to` and `requires` AND/OR groups. — [Config docs](https://github.com/bmw-software-engineering/lobster/blob/main/documentation/config_files.md)
- **Example report:** the TRLC project's own LOBSTER report shows coverage per level (Specification 72.7%, Code 49.0%, Test 66.0%). Issue types include "missing reference to Code", "missing up reference" and references to unknown requirements. — [TRLC tracing report](https://bmw-software-engineering.github.io/trlc/tracing.html)
- **Distinctive ideas:** a tool-neutral trace interchange format; declarative coverage policy with alternatives (unit test OR formal proof); "activity" as a kind that covers tests, proofs, analyses and reviews.

#### 7. rmToo
- **Format and output:** plain-text `*.req` files with key-value metadata. Requirements form a directed graph of topics. LaTeX pipeline to PDF or HTML. CLI only.
- **Licence and activity:** about 228 stars, custom open-source licence. "The last release was in 2022", and development has slowed. — [pistack.xyz](https://www.pistack.xyz/posts/2026-06-15-self-hosted-requirements-management-rmtoo-doorstop-strictdoc/)
- **Weaknesses:** the LaTeX pipeline intimidates teams without LaTeX skills, and the dependency graph is maintained by hand. — same source. (A low-depth secondary source; the GitHub repo could not be fetched.)
- **Distinctive idea:** a static, version-locked PDF specification as the primary deliverable.

#### 8. Reqflow
- **Licence and activity:** GPLv2+, Frédéric Hoerni. The last release is v1.6.0 (11 Mar 2019), so it is dormant. — [Reqflow](https://goeb.github.io/reqflow/)
- **Inputs:** docx, odt, HTML, PDF and txt. Regular expressions per document capture IDs (`-req`) and references (`-ref`), limited by `-start-after` and `-stop-after`; `define` sets constants.
- **Commands and outputs:**
  - `stat` reports coverage status; `-s -e` exits with code 2 when coverage is not 100%.
  - `trac` prints forward and reverse matrices as text, CSV or HTML.
  - `review` prints requirement text with traces. — [Reqflow](https://goeb.github.io/reqflow/)
- **Suitability for business analysts:** relevant because it reads Word and PDF documents analysts already have. The regex configuration is technical.
- **Distinctive ideas:** tracing across legacy Office or PDF documents without converting them; a CI gate by exit code.

#### 9. sphinxcontrib-traceability / mlx.traceability (Melexis)
- **Activity:** version 11.8.3 (no date shown); Sphinx plugin. — [Docs](https://melexis.github.io/sphinx-traceability-extension/)
- **Data model:**
  - `traceability_relationships` maps each forward relation to its reverse (e.g. `'validates': 'validated_by'`).
  - `ext_`-prefixed external relations need no reverse and resolve to URLs through templates (e.g. Polarion work items).
  - Attributes are validated by regex (e.g. `'asil': '^(QM|[ABCD])$'`).
  - Callbacks run per item.
  - An "undefined-reference" notification item.
- **Export:** JSON with IDs, captions, attributes, relations and an MD5 hash of each item's content. — [Configuration](https://melexis.github.io/sphinx-traceability-extension/configuration.html)
- **Other:** example sections cover requirements, design, implementation, unit tests, integration test report, a checklist and meeting notes. — [Docs](https://melexis.github.io/sphinx-traceability-extension/)
- **Distinctive ideas:** regex attribute schemas; external-link URL templates; a content hash in the JSON export for downstream change detection.

#### 10. ReqIF and open tools (Eclipse RMF/ProR, formalmind Studio, `reqif` Python package)
- **Data model** (as implemented by `reqif`):
  - SPEC-OBJECTS hold attribute values and reference a SPEC-OBJECT-TYPE.
  - SPEC-TYPES and ATTRIBUTE-DEFINITIONS define the attributes; DATATYPEs define the value types.
  - SPEC-RELATIONS hold the links.
  - SPECIFICATIONS hold a SPEC-HIERARCHY tree that references objects.
  - Rich text is XHTML. — [reqif](https://github.com/strictdoc-project/reqif)
- **The `reqif` package:** strictdoc-project, Apache-2.0, 59 stars.
  - Parse and unparse `.reqif` and `.reqifz`, format, optional XSD validation, anonymise for bug reports, `dump`.
  - Interoperability pain: tools write attributes in different orders, schemas from real tools are often incomplete, and indentation is inconsistent. The parser is tolerant, and validation is a separate step. — [reqif](https://github.com/strictdoc-project/reqif)
- **Eclipse RMF/ProR:** the Eclipse ReqIF reference implementation. ProR natively supports ReqIF 1.0.1 and 1.1. Formal Mind Studio is a third-party build. — [Eclipse RMF](https://eclipse.dev/rmf/); [formalmind](https://www.formalmind.com/?p=74). It is based on the old Eclipse Luna (4.4) per the RMF handbook, so it is effectively legacy. — [RMF docs source](https://eclipse.googlesource.com/rmf/org.eclipse.rmf.documentation/+/20400ce710b9fd1df9e09ec23f164983c3a15131/org.eclipse.rmf.documentation/rmf-latex/intro_intro.tex)
- **Suitability for business analysts:** ReqIF is the de facto door to customers' DOORS and Polarion systems, but the file format is not human-editable.

#### 11. Gherkin / Cucumber
- **Licence and activity:** Gherkin parser MIT, 4,105 commits, multiple language implementations; Rust is a "wished-for" implementation. — [cucumber/gherkin](https://github.com/cucumber/gherkin)
- **Format:** Feature, Rule, Scenario, Scenario Outline + Examples, and tags on every level. Tags are inherited into "pickles", the compiled executable scenarios, emitted as Cucumber Messages NDJSON. Keywords are translated into many spoken languages (`gherkin-languages.json`). — [cucumber/gherkin](https://github.com/cucumber/gherkin)
- **Link to requirement tools:** OFT's tag importer reads coverage tags from `.feature` files. — [OFT input formats](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide/reference/input_format_support.md)
- **Suitability for business analysts:** high. Given/When/Then is designed for business readers.
- **Distinctive ideas:** acceptance criteria that are executable; tags as the trace link (`@FR-012`); a standard event stream for results.

#### 12. Concordion and Gauge (Specification by Example)
- **Concordion:** HTML is the canonical specification format, with Markdown as an alternative. — [Concordion Markdown](https://concordion.org/documenting/java/markdown). The most recent release found is 4.0.1 (July 2023). This comes from search snippets on [Wikipedia](https://en.wikipedia.org/wiki/Concordion) and was not verified on the project site.
- **Gauge:** Go-based. Parses Markdown specifications into executable scenarios. Latest v1.6.33 (2026-06-18) and community-maintained, according to a single vendor page. — [dev.co](https://dev.co/testing/open-source/gauge)
- **Distinctive idea:** the specification document is plain Markdown that is also the test, so prose and examples stay in one file.

#### 13. BASIL (ELISA / Linux Foundation)
- **Maintainer, licence, activity:** elisa-tech, GPL-2.0, 63 stars. Started at Red Hat for its in-vehicle OS and moved to ELISA in 2023. The name comes from "ASIL B". — [GitHub](https://github.com/elisa-tech/BASIL); [FOSDEM 2025 slides](https://fosdem.org/2025/events/attachments/fosdem-2025-4798-basil-an-open-source-tool-that-supports-requirements-traceability-with-design-sbom/slides/237521/FOSDEM25_TYhePjt.pdf)
- **Architecture:** a Flask REST API with a React/PatternFly web app and containers.
- **Data model:** an upstream specification (or source code) is split into snippets. Work items are assigned per snippet: software requirements, test specifications, test cases, test runs, run configurations, justifications and documents. Each relationship carries a completeness percentage. Work items have lifecycle states (new, in progress, in review).
- **Test execution:** built in via tmt/fmf, or traced from KernelCI, Testing Farm, GitHub Actions, GitLab CI and LAVA.
- **Interchange:**
  - Export: SPDX 3 "design SBOM"; traceability matrices as SPDX, HTML and PDF.
  - Import: SPDX, YAML, JSON, CSV and XLSX.
- **Collaboration:** user accounts with granular permissions, comments and notifications. — [GitHub](https://github.com/elisa-tech/BASIL)
- **Recent material:** a 2026 ELISA seminar recap describes linking requirements, test specs and test cases, CI integration and matrix export via APIs. — [ELISA recap 2026-04-01](https://elisa.tech/ambassadors/2026/04/01/elisa-seminar-recap-notes-from-requirements-to-code-managing-end-to-end-traceability-with-basil/)
- **Access:** on the online instance, new accounts start as guests and need admin approval. — [search summary of LPC slides](https://lpc.events/event/19/contributions/2199/attachments/1738/4007/BASIL%20-%20Traceability%20as%20Code.pdf)
- **Distinctive ideas:** mapping requirements onto spans of an upstream document with completeness %; SPDX design SBOM; justifications as a work item type.

#### 14. Reqvire
- **Licence and activity:** Rust, Apache-2.0, 30 stars, version 1.5.2 (no date shown). — [GitHub](https://github.com/reqvire-org/reqvire)
- **Model:** Markdown, a layered model graph: ontology → capabilities → requirements → contracts → verification. Verification kinds: test, formal proof, analysis, inspection, demonstration. Relations include `verify` and `satisfiedBy`.
- **Commands:** `validate`, `lint`, `coverage`, `migrate`, a `serve` web explorer, and multi-repo `workspace` commands.
- **Impact analysis:** typed change impact against a PR merge-base, `--git-commit`, or `HEAD`.
- **AI:** an MCP server (stdio or HTTP) plus assistant skills for Claude Code and Codex. — [GitHub](https://github.com/reqvire-org/reqvire)
- **Distinctive ideas:** the five classic verification methods as types; impact relative to the merge-base; workspace across repositories.

#### 15. tracey
- **Licence and activity:** bearcove, MIT/Apache-2.0, 259 stars, Rust. Version 1.0.0 in late December 2025. — [GitHub](https://github.com/bearcove/tracey); [rust-digger](https://rust-digger.code-maven.com/crates/tracey)
- **Format:**
  - Requirements are marked in Markdown as `r[channel.id.allocation]` followed by the text. StrictDoc `.sdoc` and Typst are also read.
  - Code comments use `r[impl id]`, `r[verify id]`, `depends` and `related`. StrictDoc `@relation(...)` markers are also accepted.
- **Change detection:** versioned IDs (`r[auth.login+3]`) flag older references as stale. `tracey bump` updates versions for staged changes, and a pre-commit hook blocks un-bumped rule changes.
- **Queries:** `tracey query` (status, uncovered, untested, stale, unmapped, validate `--deny warnings`).
- **Interfaces:**
  - Web dashboard.
  - LSP with hover, go-to-definition, references, diagnostics, completion, rename, code lens, inlay hints and semantic tokens; a Zed extension.
  - MCP server and AI skill installation. — [GitHub](https://github.com/bearcove/tracey)
- **Distinctive ideas:** inlay hints and code lens showing coverage; a pre-commit hook that forces a version bump; `untested` vs `uncovered` as separate queries.

#### 16. SARA (and other Rust micro-tools)
- **SARA:** Rust, Apache-2.0, published January 2026.
  - Markdown with YAML front matter (`id`, `type`, `name`, relations).
  - Nine types: solution, use case, scenario, system requirement, system architecture, hardware and software requirement, hardware and software detailed design.
  - Relations: refines, derives_from, satisfies and depends_on. Each is written in one direction and the reverse is inferred.
  - Commands: `sara validate` (broken references, cycles, orphans), `sara query <ID>` (upstream and downstream), `sara report coverage|matrix`, `sara diff <REF1> <REF2>`, plus multi-repository aggregation. — [DEV article](https://dev.to/tumf/sara-a-cli-tool-for-managing-markdown-requirements-with-knowledge-graphs-nco)
- **Other Rust tools:**
  - mantra collects requirements from Markdown and traces them to implementation and tests. — [docs.rs](https://docs.rs/crate/mantra/0.6.0)
  - duvet (Cryspen, Apache-2.0) links implementations to specifications in both directions. — [GitHub](https://github.com/cryspen/duvet)
  - qa defines requirements as Rust code. — [docs.rs](https://docs.rs/crate/qa/latest)
- **Distinctive ideas:** design items (architecture, detailed design) in the same graph as requirements via `satisfies`; reverse links inferred, not written.

### Inferences
- requimd sits in the same niche as SARA, tracey and Reqvire: Markdown, Rust, a single binary, git-native. None of these three combines a glossary, language-quality checks for German and English, and a BA-oriented format. That combination looks like requimd's main differentiator.
- LSP support is now table stakes in the 2025-2026 tools (ubCode, tracey). StrictDoc still lacks one, which suggests requimd's LSP is a real advantage over the older Python tools.
- MCP servers appeared in ubCode, tracey and Reqvire within about a year. AI-agent access to the requirement graph is becoming expected.

### Gaps
- Sphinx-needs licence (believed MIT) and Doorstop's last release date were not confirmed from fetched pages.
- The rmToo GitHub repository could not be fetched (404). Its profile relies on one secondary blog post with known inaccuracies (it calls Doorstop "MIT-style", but GitHub shows LGPLv3).
- No primary source was read for Gauge's release status, Concordion's latest release, or German Gherkin keywords. The German keyword set (Funktionalität/Szenario/Angenommen/Wenn/Dann) is known to exist in `gherkin-languages.json` but was not verified in this session.
- The text of the BASIL LPC slides is image-only. The "traceability as code" content could not be read.

## Q2: How do Doorstop's "reviewed" stamps and suspect links compare to requimd's lock file?

### Takeaway
Doorstop stores two kinds of hashes inside the item files: a per-item `reviewed` fingerprint (detects unreviewed edits) and a per-link copy of the parent's fingerprint (detects suspect links). Both are cleared by explicit commands (`review`, `clear`). requimd keeps hashes in a separate lock file. The main design levers to borrow are a configurable hash scope, separate item-review and link-clear states, and per-link granularity. OFT and tracey take a different route: an explicit revision number in the ID, bumped by the author only for semantic changes.

### Cited Findings
- **The `reviewed` fingerprint:** "The value of the reviewed attribute indicates the fingerprint of the item when it was last reviewed".
  - By default the fingerprint covers the UID, `text`, `ref`/`references` and the UIDs in `links`. It does not cover `active`, `derived`, `normative`, `level` or `header`.
  - It is SHA256 in URL-safe Base64.
  - Extended attributes count only if listed under `attributes: reviewed:` in `.doorstop.yml`. Reordering that list changes all fingerprints. — [Item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)
- **Suspect links:** each link stores the parent UID plus "the fingerprint of the parent item". When the parent's fingerprint changes, the link is reported as suspect. "Only the UID part of the link contributes to the fingerprint of the item", so clearing a suspect link does not make the child unreviewed. — [Item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)
- **Clearing:** `doorstop clear` clears suspect links per item, per parent, per document or `all`. `doorstop review` stamps items. — [Quickstart v2.2](https://doorstop.readthedocs.io/en/v2.2/getting-started/quickstart/)
- **Audit caution:** Doorstop 2.0 moved to SHA256. The changelog warns that `doorstop review all` followed by `doorstop clear all` marks everything reviewed and clears every suspect link, so the audit trail depends on discipline and git history. — [Changelog v2.0](https://doorstop.readthedocs.io/en/v2.0/about/changelog/) (via search summary)
- **Referenced files:** with the `item_sha_required` extension, each file in `references` gets a `sha256`, so changes to referenced external files are detected too. — [Item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)
- **OFT alternative:** revisions in IDs are "intended to obsolete existing coverage links in case the content of a specification item semantically changed". Links to a higher revision are "outdated" and links to a lower one "predated". — [OFT user guide](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide/user_guide.md) (via search summary); [OFT system requirements](https://github.com/itsallcode/openfasttrace/blob/main/doc/spec/system_requirements.md)
- **tracey alternative:** `r[id+N]` versions, `tracey bump` for staged changes, and a pre-commit hook that blocks un-bumped rule changes. — [tracey](https://github.com/bearcove/tracey)
- **mlx.traceability:** puts an MD5 content hash per item in its JSON export, so external tools can detect changes. — [mlx configuration](https://melexis.github.io/sphinx-traceability-extension/configuration.html)
- **StrictDoc:** uses MIDs (stable machine IDs) to match nodes across moves and renames in diffs, not to flag suspects. — [StrictDoc user guide](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html)

### Inferences
- **Placement.** Doorstop's inline stamps make every review a visible diff in the item file. That is good for review in a PR, but noisy. requimd's separate lock file keeps item files clean and concentrates conflicts in one file. (CLAUDE.md already notes `requimd.lock` as a frequent rebase conflict.)
- **Hash scope.** A configurable field list, like Doorstop's `attributes: reviewed`, would avoid suspect cascades when only `status` or `prio` changes.
- **Hash vs revision.** Content hashes catch every change, including typo fixes, which produces false suspects. Revisions (OFT, tracey) catch only declared changes and can miss real ones. A hybrid looks attractive: hash by default, plus an "editorial change" escape that re-stamps without marking downstream links suspect. The escape should be visible in the diff or lock file for auditability.
- **Two separate states.** Doorstop separates "item changed since review" from "link to changed parent". If requimd only tracks link suspicion, an item-level "unreviewed" state could support review workflows. This depends on what the requimd lock file already records (not checked here).

### Gaps
- No public user complaints about Doorstop's stamp noise or merge conflicts were found. The Doorstop issue tracker was not searched in depth.

## Q3: How do StrictDoc and Sphinx-needs model custom item types, fields and relationship kinds, and how configurable is that?

### Takeaway
Both are fully configurable but differ in where configuration lives. StrictDoc declares a grammar inside each document (or in a `.gra.md` file) with typed fields and named relation roles. Sphinx-needs declares types, fields (with JSON schemas) and link types centrally in `conf.py` or TOML, with per-type constraints, target-type restrictions and severities. mlx.traceability and SARA show lighter variants: forward/reverse relation pairs, regex attribute validation and inferred reverse links.

### Cited Findings
- **StrictDoc grammar:**
  - `[GRAMMAR] ELEMENTS:` with TAG, PROPERTIES (`IS_COMPOSITE`, `PREFIX`, `VIEW_STYLE`) and FIELDS (TITLE, TYPE, REQUIRED).
  - Relations Parent, Child and File, with ROLE (e.g. `Refines`) registered in the grammar and REVERSE_ROLE declared once.
  - Parent relations are recommended over Child.
  - RELATIONS must be the last field. — [User guide](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html)
  - Field types: String, SingleChoice, MultipleChoice and Tag. MultipleChoice must be single-line and placed before the content field. — [User guide (TRACE)](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide-TRACE.html)
  - A File relation looks like `- TYPE: File / VALUE: /full/path/file.py`. — [User guide (TRACE)](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide-TRACE.html)
- **Sphinx-needs:**
  - Types: `needs_types` entries with `directive`, `title`, `prefix`, `color`, `style`, `id_regex`, `inherits`, `fields`, `links` and `back_links`. Setting `title` or `prefix` on any entry replaces the defaults.
  - Fields: `needs_fields` with a JSON-schema `schema`, `nullable`, `default`, `predicates`, and `derive`/`derive_by_type` for computed fields. Core fields such as `status` can only be tightened.
  - Links: `needs_links` with `incoming`/`outgoing` names, `allow_dead_links`, `target_types` and `summary` shortcuts.
  - Severity per schema constraint: violation, warning or info.
  - Configuration from TOML (`needs_from_toml`). — [Configuration](https://sphinx-needs.readthedocs.io/en/latest/configuration.html)
  - ubCode needs this declarative configuration to work without executing `conf.py`. A separate `needs-config-writer` tool exports it. — [needs-config-writer](https://pypi.org/project/needs-config-writer/)
- **mlx.traceability:** `traceability_relationships = {'validates': 'validated_by', ...}`, `ext_` external relations with URL templates, and `traceability_attributes` with a regex per attribute. — [mlx configuration](https://melexis.github.io/sphinx-traceability-extension/configuration.html)
- **SARA:** fixed types and four relation pairs. The user writes one direction and SARA infers the reverse. — [DEV article](https://dev.to/tumf/sara-a-cli-tool-for-managing-markdown-requirements-with-knowledge-graphs-nco)
- **TRLC:** types are a full language, with records, tuples, enums, inheritance, packages and user-defined checks. — [GitHub](https://github.com/bmw-software-engineering/trlc); [TRLC tracing report](https://bmw-software-engineering.github.io/trlc/tracing.html)
- **Doorstop:** no types. Item kind is implied by the document (directory), plus free-form extended attributes. — [Item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)

### Inferences
- requimd's fixed prefixes (UC, FR, NFR, BR, TC) are BA-friendly. A small project-level schema file could cover most needs without a DSL:
  - item types (prefix, required fields, allowed values);
  - link kinds (name, reverse name, allowed target prefixes, severity).
- Sphinx-needs' `target_types` and `severity` map directly onto requimd's typed links (e.g. `ver` must point to TC).
- Declaration in one central file (Sphinx-needs style) suits a git repository better than per-document grammars (StrictDoc style). The schema then drives LSP completion and hover.

### Gaps
- StrictDoc's section on supported field types and its custom validation (plugin) section were not fully readable in the fetched pages.

## Q4: How does OpenFastTrace trace requirements into code and tests via tags, and how does LOBSTER aggregate traces from many tools?

### Takeaway
OFT uses one tag grammar, `[<type>-><target-id>]`, inside comments of any file type. Coverage needs are declared on each item (`Needs:`) or inside the tag (`>>impl,test`). Revision numbers detect stale coverage. LOBSTER instead normalises extracts from many tools into one `.lobster` JSON format and evaluates a level-based policy with AND/OR coverage groups.

### Cited Findings
- **OFT item format:** an ID line `` `req~name~rev` ``, `Covers:` with links to upstream IDs, and `Needs: dsn` with the artifact types that must cover the item. An item that needs nothing terminates the chain. — [OFT system requirements](https://github.com/itsallcode/openfasttrace/blob/main/doc/spec/system_requirements.md); [OFT user guide](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide/user_guide.md) (via search summary)
- **OFT tag syntax:** `[ <covered-artifact-type> -> <list-of-ids> ]`, e.g. `// [impl->dsn~validate-authentication-request~1]`. Variants:
  - with the tag's own revision: `[impl~~2->dsn~...~1]`;
  - with its own name and revision: `[impl~validate-password~2->dsn~...~1]`;
  - with needed coverage: `[dsn->req~1password-login~1>>impl,test]`. — [Input format support](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide/reference/input_format_support.md)
- **OFT file types:**
  - Programming languages: Ada, C, C++, C#, Go, Java, JavaScript, Kotlin, Lua, Python, R, Rust, Swift, TypeScript, shell, SQL and more.
  - Data and markup: JSON, TOML, Protobuf, HTML, YAML.
  - PlantUML and Gherkin `.feature` files.
  - RST only through standalone comments (`.. [doc->req~user-guide~1]`).
  - XML is excluded because it collides with the SpecObject importer. — [Input format support](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide/reference/input_format_support.md)
- **OFT coverage rule:** an item is covered "when for each of the required artifact types at least one item exists that covers the original item". Deep coverage applies this recursively. A "transitive defect" is a defect only via downstream items and is marked in the HTML report. — [OFT system requirements](https://github.com/itsallcode/openfasttrace/blob/main/doc/spec/system_requirements.md)
- **In practice:** Ankaios places tags after the `use` statements in Rust files when the whole file implements a requirement, and uses `utest`, `itest` and `stest` for test levels. — [Ankaios](https://eclipse-ankaios.github.io/ankaios/0.7/development/requirement-tracing)
- **LOBSTER aggregation:**
  - Tool-specific converters write `.lobster` files. The report tool merges them under `lobster.yaml`; the older `lobster.conf` is deprecated.
  - Each level has `name`, `kind` (requirements, implementation or activity), `source` (the `.lobster` files), `trace-to` (the expected upward link target) and `requires` (a list of AND-groups of OR-alternatives).
  - Example: Requirements requires `["Code"]` and `["Unit Test", "Formal Proof"]`. — [LOBSTER config](https://github.com/bmw-software-engineering/lobster/blob/main/documentation/config_files.md); [LOBSTER GitHub](https://github.com/bmw-software-engineering/lobster)
- **LOBSTER issue vocabulary** (TRLC report): "missing reference to Code", "missing reference to Test", "missing up reference" (code without a requirement), and references to unknown IDs. — [TRLC tracing report](https://bmw-software-engineering.github.io/trlc/tracing.html)
- **Similar tag schemes:** tracey (`r[impl id]`, `r[verify id]`) and StrictDoc (`@relation(...)`). tracey accepts StrictDoc's markers, which hints at an emerging de-facto convention. — [tracey](https://github.com/bearcove/tracey)

### Inferences
- requimd could support a minimal, tool-neutral comment tag, such as `req: FR-012` or OFT-style `[impl->FR-012]`, scanned from configured globs. It could also read Gherkin tags (`@FR-012`). Both stay offline and text-only. Accepting OFT's syntax would give interoperability for free.
- A LOBSTER-style `requires` policy would let an analyst state "every FR needs a TC, and either a UC or a BR as source" in configuration, instead of hard-coding coverage rules.
- Ingesting other tools' JSON (e.g. `.lobster` or needs.json) as extra "activity" evidence would fit the single-binary constraint as plain file reading.

### Gaps
- The detailed `.lobster` JSON schema and LOBSTER's justification mechanism were not read.
- OFT's ASPEC report format was not examined.

## Q5: What do these tools offer non-technical stakeholders (web UI editing, Excel/ReqIF round trips, PDF documents)?

### Takeaway
Among file-based tools, StrictDoc offers the most for non-programmers: a local web editor that writes back to files, plus ReqIF, Excel and PDF. Doorstop offers spreadsheet export and import and LaTeX PDFs. BASIL offers a full multi-user web app, which is outside requimd's scope. ubCode now builds PDFs without LaTeX. Gherkin is the one format business readers routinely co-author.

### Cited Findings
- **StrictDoc:**
  - The web UI edits `.sdoc` and Markdown files and writes them back, with drag-and-drop images, a table editor and per-field undo (0.30.0 and 0.31.0). — [Releases](https://github.com/strictdoc-project/strictdoc/releases)
  - It is not hardened for public networks, and commits stay manual. — [User guide](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html)
  - ReqIF and Excel import read Parent relations (0.30.3). — [Releases](https://github.com/strictdoc-project/strictdoc/releases)
  - PDF requires Chrome and ChromeDriver. — [Feature map](https://strictdoc.readthedocs.io/en/stable/stable/docs/strictdoc_02_feature_map-DEEP-TRACE.html)
- **Doorstop:** CSV, TSV and XLSX export with re-import that creates or updates items. — [Interchange](https://doorstop.readthedocs.io/en/latest/cli/interchange.html). LaTeX PDF with templates. — [Publishing](https://doorstop.readthedocs.io/en/latest/cli/publishing.html)
- **BASIL:**
  - Web app with user permissions (e.g. read-only assessors), comments, notifications and to-dos.
  - Import from XLSX, CSV, YAML, JSON and SPDX; PDF and HTML matrices. — [GitHub](https://github.com/elisa-tech/BASIL)
- **ubCode:** HTML and PDF output from a standalone binary "without Python or LaTeX". — [ubCode](https://ubcode.useblocks.com/)
- **rmToo:** produces a static PDF, which suits "version-locked audit documents". — [pistack.xyz](https://www.pistack.xyz/posts/2026-06-15-self-hosted-requirements-management-rmtoo-doorstop-strictdoc/)
- **Reqflow:** reads stakeholders' existing docx, odt and PDF files directly. — [Reqflow](https://goeb.github.io/reqflow/)
- **ReqIF:** the exchange route to commercial tools. Interoperability problems are real: inconsistent attribute order and incomplete schemas from vendor tools. — [reqif](https://github.com/strictdoc-project/reqif). StrictDoc warns that ReqIF support is tool-specific. — [Feature map](https://strictdoc.readthedocs.io/en/stable/stable/docs/strictdoc_02_feature_map-DEEP-TRACE.html)

### Inferences
- For requimd's constraints (offline, single binary, no web app), the highest-value stakeholder features are:
  - an XLSX or CSV review round trip: export items, a reviewer fills in a comment or status column, re-import as a proposed change or report;
  - a print-ready site (CSS print stylesheet, so PDF via the browser without bundling Chrome);
  - one-way ReqIF export for customers on DOORS or Polarion.
- A local, read-only preview server with live reload is within requimd's constraints. A local editing UI like StrictDoc's is close to the "web app" exclusion and should be weighed carefully.

### Gaps
- No usability studies or BA-specific user feedback for any of these tools were found.

## Q6: What are the known weaknesses or user complaints?

### Takeaway
The recurring problems are build performance at scale (Sphinx-needs), Python toolchain weight and Chrome/LaTeX dependencies for PDFs, immature web UIs (StrictDoc), developer-centric formats (Doorstop YAML, TRLC DSL, OFT IDs), ReqIF dialect incompatibility, and dormant projects (Reqflow 2019, rmToo 2022, Eclipse RMF on Luna, Concordion 2023).

### Cited Findings
- **Sphinx-needs scale:** the write phase stays slow at about 20k internal plus 30k imported needs. The user split the project. — [Discussion #1220 (mirror)](https://cf-workers-proxy-9e9.pages.dev/useblocks/sphinx-needs/discussions/1220). Filters are an O(N) scan per directive. — [5.1.0 changelog](https://sphinx-needs.readthedocs.io/en/5.1.0/changelog.html). Assigning functions directly in `conf.py` disables incremental builds. — [Configuration](https://sphinx-needs.readthedocs.io/en/latest/configuration.html)
- **ubCode licensing:** the fast tooling is commercial for private projects. — [ubCode](https://ubcode.useblocks.com/)
- **StrictDoc:**
  - The web UI is "work-in-progress" and "not yet hardened". — [User guide (TRACE)](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide-TRACE.html)
  - There is no LSP yet. — [User guide](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html)
  - The server is resource-intensive. — [pistack.xyz](https://www.pistack.xyz/posts/2026-06-15-self-hosted-requirements-management-rmtoo-doorstop-strictdoc/)
- **Doorstop:**
  - One file per item "can be very restrictive", because editing means opening many files (StrictDoc FAQ, via search snippet). — [StrictDoc FAQ](https://strictdoc.readthedocs.io/en/latest/strictdoc_02_faq.html)
  - It is developer-centric, with no web UI and no matrix beyond the basic one. — [pistack.xyz](https://www.pistack.xyz/posts/2026-06-15-self-hosted-requirements-management-rmtoo-doorstop-strictdoc/)
  - Unquoted YAML levels are misparsed (`'1.10'`). — [Item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)
- **rmToo:** the LaTeX pipeline is intimidating, the dependency graph is manual, and the last release was in 2022. — [pistack.xyz](https://www.pistack.xyz/posts/2026-06-15-self-hosted-requirements-management-rmtoo-doorstop-strictdoc/)
- **Reqflow:** last release 2019. — [Reqflow](https://goeb.github.io/reqflow/)
- **TRLC:** `--verify` needs CVC5, available on GNU/Linux only. — [GitHub](https://github.com/bmw-software-engineering/trlc)
- **ReqIF tooling:** vendor files differ in attribute order and have incomplete schemas. — [reqif](https://github.com/strictdoc-project/reqif). ProR and formalmind Studio are based on Eclipse Luna. — [RMF docs source](https://eclipse.googlesource.com/rmf/org.eclipse.rmf.documentation/+/20400ce710b9fd1df9e09ec23f164983c3a15131/org.eclipse.rmf.documentation/rmf-latex/intro_intro.tex)
- **BASIL:** needs a server and database, and new users need admin approval. — [GitHub](https://github.com/elisa-tech/BASIL); [LPC slides (search summary)](https://lpc.events/event/19/contributions/2199/attachments/1738/4007/BASIL%20-%20Traceability%20as%20Code.pdf)
- **Doorstop audit risk:** `review all` plus `clear all` wipes all review evidence in one step. — [Changelog v2.0](https://doorstop.readthedocs.io/en/v2.0/about/changelog/)

### Inferences
- requimd's single Rust binary, LSP and offline design avoid the most common complaints: Python and Sphinx weight, slow builds, Chrome and LaTeX dependencies.
- Its main risk is the same as for its relatives: being too developer-centric for analysts. Glossary and language checks, plus an Excel round trip, would mitigate that.
- requimd should guard against "clear everything" anti-patterns, e.g. require a reason or a per-item scope when clearing suspects.

### Gaps
- Issue trackers (GitHub issues for Doorstop, StrictDoc and OFT) were not systematically mined. Complaints here come mainly from docs, changelogs, one discussion thread and one blog post.

## Q7: Ideas for requimd

### Takeaway
The highest-value, low-friction ideas are:
- a declarative coverage policy (LOBSTER, OFT);
- a configurable item and link schema with reverse names and target constraints (Sphinx-needs, StrictDoc, mlx);
- code, test and Gherkin tag scanning (OFT, tracey);
- JUnit result import (sphinx-test-reports, StrictDoc);
- refinements to suspect handling: hash scope, editorial-change escape, separate review state (Doorstop, OFT, tracey);
- an XLSX review round trip (Doorstop).

### Cited Findings
- Each idea in the table cites the tools whose sources appear in Q1–Q6 above.

### Inferences

| Idea | Seen in | Problem it solves for an analyst | Fit | Value | Effort |
|---|---|---|---|---|---|
| Declarative coverage policy per item type with AND/OR groups (e.g. FR requires [TC]; UC requires [FR] or [BR]) | LOBSTER `requires`, OFT `Needs:` | Defines "complete" per project instead of hard-coded rules; coverage report matches the analyst's process | fits | high | M |
| Per-item override of required coverage (`needs: TC, design`) | OFT `Needs:` | Some items legitimately need no test, or need a design item; avoids false gaps | fits | medium | S |
| Project schema file: item types, required fields, enum values, regex per field | Sphinx-needs `needs_types`/`needs_fields`, StrictDoc `[GRAMMAR]`, mlx attribute regex | Enforces team conventions (e.g. `prio` only must/should/could) and drives LSP completion | fits | high | M |
| Link kinds with registered reverse names, allowed target prefixes and severity | StrictDoc ROLE/REVERSE_ROLE, Sphinx-needs `target_types`/`severity`, mlx relationships, SARA inferred reverses | Hover and site show "verified by TC-3" on FR pages without the analyst writing back-links; catches `ver` pointing to a UC | fits | high | S |
| Code, test and file tag scanner (`[impl->FR-012]`, `req: FR-012`) over configured globs, OFT-compatible | OFT tag importer, tracey `r[impl id]`, StrictDoc `@relation` | Analyst sees which requirements developers implemented, without touching code | fits (offline text scan) | high | M |
| Gherkin `.feature` files as verification artifacts (scenario tags `@FR-012` become `ver` links; German keywords) | Gherkin/Cucumber, OFT `.feature` import, Gauge/Concordion | Acceptance criteria written in business language become traceable test cases | fits | high | M |
| JUnit XML result import: show passed/failed/skipped on TC items, site and coverage ("verified and passing") | sphinx-test-reports, StrictDoc test reports, BASIL | Answers "is this requirement currently met?", not just "is there a test?" | fits (reads local files) | high | M |
| Configurable fingerprint scope (which fields count for suspect detection; ignore status/prio) | Doorstop `attributes: reviewed` | Stops suspect cascades when only status or priority changes | fits | high | S |
| Editorial-change escape: re-stamp an item as "non-semantic change" without marking downstream links suspect, recorded in the lock file | OFT revisions (semantic change only), tracey `bump` + pre-commit hook | Typo fixes no longer force re-review of every linked TC; still auditable | fits | high | S |
| Separate states "item changed since last review" and "link suspect", with per-link and per-item clear (CLI and LSP code action) | Doorstop `review` / `clear` | Supports a review workflow and finer-grained impact handling | fits (if not already in the lock file) | medium | S |
| Guard against mass clearing (require a scope or reason; report cleared links in diff) | Doorstop changelog warning on `review all` + `clear all` | Keeps impact analysis trustworthy for audits | fits | medium | S |
| Richer link-status vocabulary: outdated, orphaned, ambiguous (duplicate ID), unwanted (wrong type), plus deep/transitive coverage | OFT link statuses, LOBSTER "missing up reference" | Clearer messages in check output and LSP diagnostics; distinguishes direct from inherited gaps | fits | medium | S |
| Declarative cross-field check rules (e.g. "prio: must ⇒ a `ver` link is required"; "status: approved ⇒ no weak words") | TRLC checks, Sphinx-needs constraints and predicates | Project-specific quality gates without code | fits (declarative, no scripting) | medium | M |
| CI gate: exit code when coverage < threshold or selected findings occur | Reqflow `stat -s -e`, ubCode `ubc check`, tracey `--deny warnings` | Prevents merging incomplete specifications | fits | medium | S |
| Up/downstream trace query (`requimd trace FR-012 --up --down`) and filter expressions | SARA `query`, StrictDoc query engine, tracey `query uncovered/untested/stale` | Quick impact answers in a review meeting | fits | medium | S |
| Impact relative to the PR merge-base (`--since merge-base`) | Reqvire | "What did this PR change, and what is now suspect?" in one command | fits (git is local) | medium | S |
| Stable machine IDs (MIDs) independent of display IDs, to follow moves and renames in diffs | StrictDoc MID | Diff shows "moved/renamed" instead of "deleted + added" | fits, but adds noise to files | low | M |
| XLSX/CSV review round trip (export with comment/status columns; re-import as proposed changes or a report) | Doorstop CSV/TSV/XLSX import/export, StrictDoc Excel, BASIL XLSX import | Lets non-technical stakeholders review in Excel | fits (needs an XLSX writer crate; still one binary) | high | M |
| ReqIF export (later import) with a simple attribute mapping | StrictDoc ReqIF, `reqif` package, Eclipse RMF | Exchange with customers on DOORS or Polarion | fits (file-based); dialect differences raise cost | medium | L |
| Print-ready site (print CSS, cover page, revision table) for PDF via the browser | Doorstop LaTeX, StrictDoc HTML2PDF, ubCode PDF, rmToo | Formal sign-off documents without bundling Chrome or LaTeX | fits | medium | S |
| Design-level item types (architecture/design, ADR) with a `satisfies` relation and an "implemented by" chain | SARA design types + `satisfies`, OFT `dsn` artifact type, Reqvire layered model | Path to requimd's design-level goal using proven relation names | fits | high | M |
| Verification-method attribute (test, analysis, inspection, demonstration) on `ver` links or TC items | Reqvire verification kinds, BASIL justifications | Not every requirement is tested; inspection or analysis evidence still counts toward coverage | fits | medium | S |
| Justification items: documented reason why an item needs no coverage | BASIL justifications, LOBSTER (justifications mentioned in docs, not read) | Removes false gaps in coverage reports with an auditable reason | fits | medium | S |
| MCP server (stdio) exposing the item graph, checks and coverage to AI assistants | ubCode MCP, tracey `mcp`, Reqvire MCP | Analysts using AI assistants get grounded, ID-correct answers | fits (local stdio, no network) | medium | M |
| LSP code lens or inlay hints showing coverage and suspect status next to each item | tracey LSP (code lens, inlay hints) | Analyst sees gaps while writing, without running a report | fits | medium | S |
| Multi-repo aggregation: import another repository's JSON export as external items | Sphinx-needs `needimport`/`needs_external_needs`, SARA multi-repo, Reqvire workspace | System-level requirements in one repo, component requirements in another | fits (offline file import) | medium | M |
| Read requirement IDs from legacy docx or PDF via regex, for tracing against stakeholder documents | Reqflow | Trace to a customer specification or law that stays a Word or PDF file | fits partially (parsing docx and PDF adds dependencies) | low | L |
| Snippet mapping: link items to text spans of an upstream source document, with completeness % | BASIL | Shows how much of an upstream spec or regulation is covered by requirements | fits partially (Markdown sources only) | medium | L |
| JSON export with a per-item content hash | mlx.traceability JSON export (MD5) | External tools can detect changed items without the lock file | fits | low | S |
| Local web UI that edits and writes back Markdown | StrictDoc server | Non-Markdown users can edit | breaks: "web app / graphical editor out of scope" (a read-only preview server fits) | medium | L |
| Variants / product-line filtering | Sphinx-needs variants | One specification for several product variants | fits, but complex | low | L |
| Formal verification of check rules (SMT) | TRLC `--verify` (CVC5) | Proves rule sets consistent | breaks: single-binary/few dependencies | low | L |

### Gaps
- Value and effort ratings are rough judgements by the researcher, not measured. They assume requimd's current architecture as described in the brief, not as read from its code.
- requimd's lock file was not inspected, so the ideas on separate review states and editorial changes may partly overlap with existing behaviour.
