# Own the design level, close the review gap

requimd can take over almost everything enterprise requirements tools build on a database. Git, the lock file and the static site already supply snapshots, change detection and publishing. Only a few ideas break its constraints: hosted review apps with reviewer accounts, live sync with Jira or Azure DevOps, embedded language models and semantic search, a local editing UI, and bundled Word or PDF converters. The comparison shows three gaps that matter most to an analyst. There is **no recorded review and approval tied to exact content**, **no named baselines**, and **coverage stops at "a test is linked"** rather than "the test passes". It also shows an open niche: **none of the decision-record tools listed on adr.github.io links decisions to requirements** ([adr.github.io](https://adr.github.io/adr-tooling/)), and no tool found warns that a decision needs re-evaluation when the requirement behind it changes. requimd's lock file could do this with little new machinery. The field is moving toward requimd's model. A 2025–2026 wave of Rust, Markdown-first tools (tracey, SARA, Reqvire) ships language servers and MCP servers. Spec-driven development puts Markdown specs into repositories. Jama now calls AI-written specs a "planning layer" beneath a governed record of requirements ([Jama](https://www.jamasoftware.com/blog/openspec-guide/)). Against that wave, requimd stands out by its focus on analysts: glossary, German and English language checks, and use cases with stable step labels. The ideas worth adopting first build on that focus. They are the design level, review packages and review records, named baselines, test-result import, a configurable coverage policy, language rules that cite INCOSE rule IDs, German sentence-template checks, and a local MCP server. The catalogue below lists 97 ideas (R-01 to R-97). About 15 of them match an entry already planned or specified, and some 40 more extend an existing spec item. The report ends with eight proposed deep dives for phase 2.

## Charter

**Goal.** Collect ideas that make requimd more capable for business analysts, at the requirements level and at the design level. Every idea is rated for fit against requimd's concept. Ideas that break a constraint stay in the catalogue, marked with the constraint they break and what that would cost.

**Lifecycle covered.** Elicit, write, structure, trace, review and approve, baseline and change, verify, publish and report. At the design level: design decisions, design items, spikes and proposals.

**Research questions.**

1. What do tools offer at each lifecycle stage, design included?
2. How do text- and git-based tools solve what enterprise tools solve with a database and a web UI?
3. Which ideas fit requimd's constraints, which break them, and at what cost?
4. Where is the field heading?
5. Which gaps does the comparison reveal? The owner has no fixed pain points yet.

**Out of scope.** Pure product-discovery tools, hardware-only PLM, price comparisons, and hands-on trials of commercial tools.

**Method.** Public sources only. Phase 1 is this broad scan across six areas. It is followed by an owner checkpoint, then deep dives and a synthesis. The notes were compiled on 18 November 2026. They are in [`notes/`](notes/), one file per area. Phase 2, the deep dives, is [requirements-and-design-tools-deep-dives-2026.md](requirements-and-design-tools-deep-dives-2026.md).

Fit is rated against these constraints:

| Constraint | Source |
|---|---|
| Plain CommonMark that renders on GitHub and GitLab | NFR-008 |
| Git is the storage | Vision |
| Single binary with few dependencies | NFR-005 |
| No network at runtime unless a command explicitly opts in | NFR-006, ADR-0011 |
| Out of scope so far: graphical editor, web app with user management, database, real-time collaboration | Vision |
| The tool never sets `status: approved` | BR-004 (a business rule, noted where an idea touches it) |

Each idea was compared with `requirements/` (README and item headings), `docs/feature-report.md` and the "After v0.1.0" section of `docs/roadmap.md`. The column "Existing" names a matching ID (FR-xxx, or B/F entries of the feature report) where the idea is already specified or planned.

## Six tool families share one data model

Every family in the scan models the same things: **typed items with IDs and attributes, typed links, a hierarchy, change history, baselines, reviews and exports**. They differ in where that model lives. Enterprise tools keep it in a server database. Docs-as-code tools keep it in text files under version control. Backlog tools keep it in issues. Modeling tools keep it in a model repository. Spec-driven AI tools keep it in Markdown that an agent reads. Methods and standards describe what the model must contain, and decision practices extend it to the design level. The tables give each tool or method one line and its most borrowable idea. The research notes hold the full profiles.

### Enterprise requirements and ALM tools

| Tool | What it is | Most borrowable idea |
|---|---|---|
| IBM DOORS Next / DOORS 9 ([IBM GCM](https://www.ibm.com/docs/SSYMRC_7.0.3/com.ibm.rational.gcapp.doc/topics/c_overview_gcapp.html)) | Server RM in the IBM ELM suite; streams, baselines, change sets | Baseline sets (one named set of per-module baselines); quality score aligned to the INCOSE guide (RQA) |
| Jama Connect ([licence types](https://help.jamasoftware.com/en/reference/permissions/license-types-26811.html)) | SaaS RM with Review Center and Advisor | A review as a versioned record with signatures; reviewers who see only the review; rule-by-rule INCOSE/EARS report |
| Siemens Polarion ([2512](https://blogs.sw.siemens.com/polarion/polarion-alm-2512-whats-new-and-noteworthy/)) | ALM with LiveDoc documents made of work items | Copilot "Consistency Check" that follows links to find contradictions; branch and merge of documents with links |
| PTC Codebeamer ([engineering.com](https://www.engineering.com/ptc-updates-alm-portfolio-with-codebeamer-and-pure-variants/)) | ALM with product-line variants (pure::variants) and Review Hub | Review shown as a diff against the last reviewed version; separate moderator and reviewer roles |
| Visure Requirements ALM ([Visure](https://visuresolutions.com/features/ai-requirements-management/)) | Commercial RM with Quality Analyzer and ReqIF | Quality checks while writing, positioned as the core feature |
| ReqView ([change management](https://reqview-homepage.netlify.app/doc/change-management)) | Desktop RM whose projects are files kept in git | Suspect rules chosen per link type; clearing suspects in bulk; three-way compare with a base |
| SPEC Innovations Innoslate ([NLP quality](https://specinnovations.com/innoslate/nlp-quality-analytics)) | Cloud RM plus MBSE | Quality score per requirement, with a shareable quality report |
| Modern Requirements4DevOps ([review](https://www.modernrequirements.com/products/modern-requirements4devops-features/review/)) | RM add-on inside Azure DevOps | Review a document or baseline as one package, then baseline exactly what was approved |
| Perforce ALM ([blog](https://www.perforce.com/blog/use-testtrack-more-bugs)) | Requirements, tests and issues in one tool | Reviews happen inside the document; the separate review mode was removed |
| Inflectra SpiraTeam ([baselining](https://inflectra.com/SpiraTeam/Highlights/Baselining.aspx)) | ALM with release-bound baselines and an AI assistant | Baseline bound to a release; "ai-augmented" tag on AI-changed content |
| Altium Requirements & Systems Portal (Valispace) ([Valispace](https://www.valispace.com/ai/)) | Requirements with live engineering parameters | Named values with units, defined once and referenced in requirement text |

### Docs-as-code and open-source tools

| Tool | What it is | Most borrowable idea |
|---|---|---|
| Doorstop ([item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)) | One YAML or Markdown file per item, Python CLI | A configurable set of fields counts toward the review fingerprint; per-link parent fingerprints; spreadsheet round trip |
| StrictDoc ([user guide](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html)) | SDoc format, static HTML, local web editor, ReqIF and Excel | Relation roles with a reverse name declared once; ReqIF and Excel import and export |
| Sphinx-needs / ubCode ([config](https://sphinx-needs.readthedocs.io/en/latest/configuration.html), [ubCode](https://ubcode.useblocks.com/)) | Requirement directives in Sphinx; ubCode adds an LSP, CLI and MCP in one binary | Field schemas, link `target_types` and a severity per constraint; test results as items (sphinx-test-reports) |
| OpenFastTrace ([input formats](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide/reference/input_format_support.md)) | Java tracer of Markdown spec items and code tags | One tag syntax (`[impl->dsn~id~1]`) in 25+ file types; per-item `Needs:` coverage |
| TRLC ([GitHub](https://github.com/bmw-software-engineering/trlc)) | BMW requirements language with typed records | Schema kept separate from data; declarative check rules |
| LOBSTER ([config](https://github.com/bmw-software-engineering/lobster/blob/main/documentation/config_files.md)) | BMW trace aggregator over a JSON interchange format | Coverage policy per level with AND/OR groups ("Code" and "Unit Test or Formal Proof") |
| rmToo ([pistack](https://www.pistack.xyz/posts/2026-06-15-self-hosted-requirements-management-rmtoo-doorstop-strictdoc/)) | Text-file RM with a LaTeX pipeline, slowing since 2022 | A version-locked PDF as the primary deliverable |
| Reqflow ([Reqflow](https://goeb.github.io/reqflow/)) | Regex tracer over docx, odt and PDF, dormant since 2019 | Trace against stakeholders' Word and PDF files without converting them; exit code as a CI gate |
| mlx.traceability ([config](https://melexis.github.io/sphinx-traceability-extension/configuration.html)) | Sphinx traceability plugin | Forward and reverse relation pairs; regex attribute schemas; a content hash per item in the JSON export |
| ReqIF tools: Eclipse RMF/ProR, `reqif` ([reqif](https://github.com/strictdoc-project/reqif)) | Reference editor (legacy Eclipse) and a Python ReqIF library | A tolerant parser with validation as a separate step, given messy vendor files |
| Gherkin / Cucumber ([gherkin](https://github.com/cucumber/gherkin)) | Given/When/Then acceptance tests, translated into many spoken languages | Executable acceptance criteria; tags such as `@FR-012` as trace links |
| Concordion / Gauge ([Concordion](https://concordion.org/documenting/java/markdown)) | Markdown specifications that are also tests | Prose and examples in one file that runs |
| BASIL ([GitHub](https://github.com/elisa-tech/BASIL)) | ELISA web app for traceability with a design SBOM | Mapping requirements to spans of an upstream document, with completeness %; "justification" items |
| Reqvire ([GitHub](https://github.com/reqvire-org/reqvire)) | Rust, Markdown model graph with an MCP server | Change impact against the PR merge-base; five verification methods as types |
| tracey ([GitHub](https://github.com/bearcove/tracey)) | Rust, Markdown rules traced to code, with LSP and MCP | Code lens and inlay hints showing coverage; versioned IDs with a pre-commit hook that forces a version bump |
| SARA, mantra, duvet ([SARA](https://dev.to/tumf/sara-a-cli-tool-for-managing-markdown-requirements-with-knowledge-graphs-nco)) | Rust Markdown requirement graphs and spec-to-code tracers | Design item types (architecture, detailed design) in the same graph via `satisfies`; reverse links inferred |

### AI-assisted RE and spec-driven development

| Tool or source | What it is | Most borrowable idea |
|---|---|---|
| GitHub Spec Kit ([developertoolkit](https://developertoolkit.ai/en/ecosystem/frameworks/spec-kit/)) | CLI and agent commands: constitution, specify, plan, tasks | `/analyze`, a read-only cross-check: requirements with no task, tasks with no requirement |
| AWS Kiro ([specs](https://kiro.dev/docs/specs/)) | Agentic IDE with requirements.md, design.md and tasks.md | EARS as the default requirement form; tasks traced to requirement numbers |
| Tessl ([Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)) | Spec-as-source framework whose CLI is also an MCP server | Reverse-engineers specs from existing code |
| OpenSpec ([Jama guide](https://www.jamasoftware.com/blog/openspec-guide/)) | A living spec plus change folders with delta specs | Change deltas marked ADDED / MODIFIED / REMOVED / RENAMED, merged into the spec when the change is archived |
| BMAD Method ([developertoolkit](https://developertoolkit.ai/en/ecosystem/frameworks/bmad-method/)) | Agile roles as agents, analyst first | Story files that carry their own context |
| AGENTS.md ([agents.md](https://agents.md/)) | Plain-Markdown agent instructions, stewarded by the Linux Foundation's AAIF | A standard place to tell any agent the project's rules |
| Jama Connect Advisor and MCP ([Advisor results](https://help2.jamasoftware.com/ah/en/jama-connect-advisor-/interpreting-the-results.html)) | NLP scoring against INCOSE and EARS; an official MCP server | "Governed context" for coding agents; a rule-by-rule report |
| IBM RQA / Engineering AI Hub ([RQA](https://www.ibm.com/products/requirements-quality-assistant)) | Watson-based quality scoring, now inside an AI platform | Ten INCOSE-based quality issues detected per requirement |
| QVscribe ([QRA](https://qracorp.com/requirement-software-tools-comparison/)) | Requirement quality analysis against domain standards | Continuous assessment while writing (vendor claim only) |
| Copilot4DevOps ([product](https://copilot4devops.com/ai-test-case-generation-how-to/)) | LLM test-case generation in Azure DevOps | Test steps and data scenarios generated from approved requirements |
| Codebeamer AI ([3HTi](https://3hti.com/alm/codebeamer-ai-capabilities/)) | Requirements and test-case assistants | INCOSE-aligned rewrite suggestions |
| Atlassian Rovo ([Birk](https://makingofsoftware.com/2026/02/ai-support-in-requirements-tools/)) | General assistant for Jira and Confluence | Semantic model with MCP integration |
| NLP4RE trace-link research ([Tracer](https://arxiv.org/html/2611.09162), [Hey & Frattini](https://arxiv.org/html/2606.11834)) | LLM trace link recovery and quality-factor studies | Suggested links must stay suggestions: about 18% precision at 81% recall |

### Backlog, ALM and modeling tools

| Tool | What it is | Most borrowable idea |
|---|---|---|
| Jira + RTM ([Deviniti](https://deviniti.com/blog/application-lifecycle-management/report-requirement-test-coverage-jira/)) | Requirements and tests as Jira issues | One coverage report across requirement, test, execution and defect |
| R4J / easeRequirements ([docs](https://easesolutions.atlassian.net/wiki/x/RwCrh)) | Requirement tree over Jira issues | Matrix between any two item types; baselines per release or review |
| Xray ([coverage](https://confluence.getxray.app/display/XRAY35/Understanding+coverage+and+the+calculation+of+Test+and+requirement+statuses)) | Test management for Jira | Requirement status computed per version and test environment |
| Azure DevOps Boards ([link types](https://learn.microsoft.com/en-us/azure/devops/boards/queries/link-type-reference)) | Work items with typed links | Link types that declare `acyclic`, `singleTarget` and topology; "Affects" links from change requests |
| GitLab requirements ([docs](https://docs.gitlab.com/user/project/requirements/)) | Thin requirement items in GitLab Ultimate | A CI job's `requirements.json` artifact marks requirements satisfied or failed |
| GitHub Issues and Projects ([issue fields](https://github.blog/changelog/2026-07-02-issue-fields-are-now-generally-available)) | Issue types, sub-issues, typed fields (GA July 2026) | Fields defined per issue type, i.e. a schema per item kind |
| Aha! ([blog](https://www.aha.io/blog/requirements-custom-fields)) | Roadmapping with features and requirements | Acceptance criteria generated from a feature description |
| Linear | Issue tracker (weak, competitor-written sources only) | Requirement as an issue tied to a release |
| SysML v2 ([OMG](https://www.omg.org/news/releases/pr2025/07-21-25.htm), [tutorial](https://wrwei.github.io/tutorials/sysml/SysMLv2_Module5_Requirements_EN.html)) | OMG modeling standard adopted 2025, with textual notation and a REST API | Requirement with a subject, assume and require constraints; satisfy versus verify |
| Eclipse Capella ([forum](https://forum.mbse-capella.org/t/capella-traceability-options/6926)) | Arcadia MBSE workbench | Validation report of missing realizations between levels |
| Sparx Enterprise Architect ([guide 17.2](https://sparxsystems.com/enterprise_architect_user_guide/17.2/specification_tracking.html)) | UML/SysML modeler with Specification Manager | Document view and model view of the same items; package baselines compared with the live model |
| Cameo Requirements Modeler ([2024x](https://docs.nomagic.com/x/NS8mC)) | SysML modeler | Suspect links only for derive, satisfy, refine and verify; one matrix per relation kind |
| Mermaid requirementDiagram ([docs](https://mermaid.js.org/syntax/requirementDiagram.html)) | Text diagrams with SysML v1 relations | `verifymethod` attribute; the seven SysML relation names |
| Structurizr DSL ([language](https://docs.structurizr.com/dsl/language)) | C4 architecture as code | One model, many views defined as code |

### RE methods, writing standards and regulation

| Method or standard | What it is | Most borrowable idea |
|---|---|---|
| ISO/IEC/IEEE 29148:2018 ([iTeh](https://standards.iteh.ai/catalog/standards/iso/8cf2bc2b-8b5e-4907-a82a-d1c5676c9e85/iso-iec-ieee-29148-2018)) | RE standard: characteristics, language criteria, information items | Separate checks for individual requirements and for the set as a whole |
| INCOSE GtWR v4, 2023 ([Jama summary](https://www.jamasoftware.com/requirements-management-guide/writing-requirements/incose-requirements-writing-guide/)) | 42 writing rules mapped to 15 characteristics | A stable rule ID (R7 vague terms, R19 combinators) to cite in every finding |
| EARS ([Mavin](https://alistairmavin.com/ears/)) | Five sentence patterns with a fixed clause order | Deterministic pattern parsing: zero or more preconditions, at most one trigger, one system name |
| SOPHIST / Rupp template ([LMU handout](https://www.pst.ifi.lmu.de/Lehre/fruhere-semester/sose-2009/seprakt/Formulierungsregel_DE_Rupp_Schablone.pdf)) | German sentence template taught via IREB | Modal verb as bindingness (muss, soll, wird); fixed activity phrases; infinitive at the end |
| Planguage ([Modern Analyst](https://modernanalyst.com/Resources/Articles/tabid/115/ID/2926/Specifying-Quality-Requirements-With-Planguage.aspx)) | Gilb's notation for quality requirements | Scale, Meter, Past, Tolerable, Goal, Wish as checkable fields |
| Volere (background, unverified) | Robertson template with "snow cards" | Fit criterion, rationale, originator and a `conflicts` link per requirement |
| BABOK v3 ([IIBA task cards](https://www.iiba.org/contentassets/a8cf501643fb4feaac7c308e152eb03f/all-task-cards-req-and-designs-life-cycle-mgmt.pdf)) | IIBA lifecycle tasks: trace, maintain, prioritise, assess changes, approve | Approval as a distinct, auditable lifecycle step covering designs too |
| IREB CPRE Foundation Level ([downloads](https://www.ireb.org/en/downloads/)) | The curriculum most German-speaking BAs know | Its vocabulary (Satzschablone, Glossar, Baseline) for docs and rule names |
| ISO/IEC 25010:2023 ([ISO](https://www.iso.org/standard/78176.html)) | Product quality model with nine characteristics | NFR coverage by quality characteristic |
| Use cases, user stories, Gherkin, story and impact mapping (background) | Cockburn, INVEST, BDD, Patton, Adzic | Extensions keyed to step numbers; acceptance criteria as Given/When/Then |
| Automotive SPICE 4.0 ([UL](https://www.ul.com/sis/insights/automotive-spice-system-requirements-analysis-sys2)) | Automotive process assessment model | Bidirectional traceability plus consistency evidence |
| ISO 26262, DO-178C / DO-330 ([Rapita](https://www.rapitasystems.com/products/features/tool-qualification-do-178-and-iso-26262)) | Safety standards with tool qualification | A tool-confidence kit built from deterministic output and fixtures |
| IEC 62304 and FDA QMSR ([Greenlight Guru](https://www.greenlight.guru/blog/iec-62304-software-dhf)) | Medical software lifecycle; QMSR in force since 2 Feb 2026 | The standards do not prescribe how records are stored, so git-based records are acceptable in principle |
| EU CRA / IEC 62443 (background, unverified) | Cybersecurity regulation and standard | External clause catalogues that project requirements `satisfy` |
| ReqIF 1.2 and ProSTEP guide ([ProSTEP](https://www.prostep.org/fileadmin/prod-download/ProSTEP_iViP_ImplementationGuide_ReqIF_V1-10_aw.pdf)) | XML exchange format for DOORS and Polarion | Follow the ProSTEP profile to avoid vendor dialect problems |
| OSLC RM/QM, CycloneDX (background, unverified) | Linked-data protocol; SBOM with attestations | OSLC link names (satisfies, validatedBy) as a naming reference |
| Inspections and perspective-based reading (background) | Fagan roles; reading from a user, designer or tester view | A review checklist per perspective for the rules a tool cannot check |

### Design decisions, proposals and spikes

| Practice or tool | What it is | Most borrowable idea |
|---|---|---|
| Nygard ADR / Fowler ([Fowler](https://martinfowler.com/bliki/ArchitectureDecisionRecord.html)) | One-page numbered decision files in the repo | Accepted ADRs are immutable; a change is a new ADR that supersedes the old one; re-evaluation triggers |
| MADR 4 ([template](https://github.com/adr/madr/blob/develop/template/adr-template.md)) | Markdown ADR template with front matter | Decision drivers, considered options and "Confirmation" (how compliance is checked) |
| Y-statements ([Zimmermann](https://medium.com/olzzio/y-statements-10eb07b5a177)) | A decision as one structured sentence | "In the context of <requirement>" places the requirement link inside the sentence |
| Tyree/Akerman ([Redfin](https://github.com/redfin/react-server/wiki/Architecture-Decision-Record-Template)) | Full decision template, 2005 | An explicit "Related requirements" and "Assumptions" section |
| ecADR ([Ozimmer](https://ozimmer.ch/practices/2020/05/22/ADDefinitionOfDone.html)) | Definition of done for decisions | A spike or proof of concept as the evidence for a decision; "ADs age" |
| adr.github.io tools ([tooling](https://adr.github.io/adr-tooling/)) | adr-tools, Log4brains, ADR Manager, Backstage plugin | Lifecycle commands (propose, accept, supersede); no tool links decisions to requirements |
| Structurizr ADRs ([docs](https://docs.structurizr.com/dsl/adrs)) | ADR folders attached to C4 model elements | Import existing ADR folders as they are, through format importers |
| arc42 §9 ([arc42](https://docs.arc42.org/section-9/)) | Architecture template section for decisions | Record only architecturally significant decisions, with criteria and rejected options |
| ISO/IEC/IEEE 42010:2022 ([arc42 summary](https://quality.arc42.org/standards/iso-42010)) | Architecture description standard | A decision *pertains to* concerns and can *raise* new ones |
| Twin Peaks ([Nuseibeh 2001](https://oro.open.ac.uk/2213)) | Requirements and architecture evolve together | Links in both directions: design raises requirements |
| Python PEP 1 ([PEP 1](https://peps.python.org/pep-0001/)) | Proposal process with a rich status set | "Final" (implemented) as a separate state after "Accepted"; Open Issues; Replaces/Superseded-By |
| Rust RFCs ([template](https://raw.githubusercontent.com/rust-lang/rfcs/master/0000-template.md)) | Proposal template | Unresolved questions tied to a gate (before merge or before stabilization) |
| Kubernetes KEPs ([template](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/README.md)) | Proposals with machine-readable kep.yaml | Goals and non-goals; graduation criteria as gates |
| Oxide RFDs ([RFD 1](https://rfd.shared.oxide.computer/rfd/0001)) | "Timely rather than polished" design discussions in git | An "ideation" state: topic and scope only, which others can pick up |
| Google design docs ([Ubl](https://www.industrialempathy.com/posts/design-docs-at-google/)) | Design docs of 1 to 3 pages | Non-goals; amendments linked from the original |
| Hypothesis cards ([Thoughtworks](https://www.thoughtworks.com/insights/articles/how-implement-hypothesis-driven-development)) | "We believe … we will know when …" | The success signal is fixed before the experiment |
| Archgate, ADR Kit, ADR Guard ([ADR Guard](https://www.nuget.org/packages/RodriOliveira.AdrGuard)) | 2025–2026 ADR linting and AI drafting | AI-drafted decisions are forced to "proposed" |

## Every lifecycle stage has a file-based answer

The answer to research questions 1 and 2 is that **enterprise tools mostly re-implement git inside a database, then add review and reporting surfaces on top**. DOORS Next has streams, baselines and change sets. Once configuration management is enabled it cannot be turned off, and global configurations need their own server ([IBM](https://www.ibm.com/docs/SSYMRC_7.0.3/com.ibm.rational.gcapp.doc/topics/c_overview_gcapp.html)). One DOORS Next user asked for a "graphical history/branch view … like git" ([Jazz forum](https://jazz.net/forum/questions/291047/feature-request-graphical-historybranch-view-for-streams-baselines-in-doors-next-global-configuration-like-git)). Polarion added automatic merge of branched documents in 2404 and carried links through merges in 2512 ([Polarion 2404](https://blogs.sw.siemens.com/polarion/polarion-alm-2404-whats-new-and-noteworthy/), [2512](https://blogs.sw.siemens.com/polarion/polarion-alm-2512-whats-new-and-noteworthy/)). ReqView is the only enterprise tool that stores projects in git. Its JSON files do not merge well, so it locks a document's file before editing ([ReqView](https://reqview-homepage.netlify.app/doc/git-collaboration)). requimd's line-based Markdown avoids that problem.

The table maps each stage to the enterprise answer, the text-and-git answer seen in the scan, and what requimd has today.

| Stage | Enterprise tools (database, web UI) | Text- and git-based answer | requimd today |
|---|---|---|---|
| Elicit | AI drafting of requirements (Innoslate, ValiAssistant); ReqIF import of customer specs | Reqflow reads Word and PDF; StrictDoc imports ReqIF and Excel; story and impact maps as text | Item templates and stubs (FR-212, FR-213); no import |
| Write | Quality assistants scoring against INCOSE and EARS (Jama, IBM, Polarion, Visure, Innoslate); Word-like LiveDoc editing | LSPs (ubCode, tracey); StrictDoc's local web editor | LSP with completion, hover and code actions; weak words, modal verbs, "shall", story pattern, glossary aliases (FR-110 to FR-121) |
| Structure | Typed items, custom fields, link roles, document hierarchy | StrictDoc grammars, Sphinx-needs schemas, TRLC types, Doorstop directories | Configurable types, keys, value sets and link types (FR-014, BR-006) |
| Trace | Live traceability, trace scores, matrices per relation kind | OFT and tracey code tags, LOBSTER coverage policies, SARA queries | Typed links, step coverage, step matrix, inlay hints, code lens (FR-106, FR-209, FR-215, FR-401) |
| Review and approve | Review Center, Review Hub, e-signatures, free reviewer licences | Pull requests, Doorstop review stamps, ReqView status workflow, Oxide's RFD pull requests | Pull requests, status values, BR-004; reviews of suspect links only (FR-218, FR-307) |
| Baseline and change | Streams, baselines, change sets, suspect links, impact analysis | Git tags and branches, Doorstop fingerprints, OFT and tracey revisions, OpenSpec deltas | Lock-file suspect links (FR-116); `requimd diff` between refs (FR-403) |
| Verify | Test management; status per version and environment (Xray); CI artifacts (GitLab) | sphinx-test-reports and StrictDoc import JUnit; Gherkin; Concordion and Gauge | `ver` links with a `method` key (F2); unverified-requirements check planned (B10) |
| Publish and report | Word and PDF, dashboards, ReqIF, Jira sync | Static HTML (StrictDoc, Doorstop, OFT), PDF via LaTeX or Chrome, ubCode PDF | Static site, CSV matrix, JSON, Mermaid, changelog (FR-400 to FR-407) |
| Design level | Requirements and MBSE in one database (Innoslate, Codebeamer, Polarion item types); Cameo satisfy links | ADR folders, Sphinx-needs `arch-decision` types, SARA design types, OFT `dsn`, Kiro design.md, PEP/KEP/RFD | Long-term goal; traceability-model ADR planned (B9, F3) |

Review is where the database still earns its keep. Jama makes **reviewer licences free, gives anyone invited to a review a reviewer licence automatically, and creates temporary 30-day accounts for outside reviewers** ([Jama licences](https://help.jamasoftware.com/en/reference/permissions/license-types-26811.html)). Under 21 CFR Part 11 the review itself is the electronic record: each signature carries the signer's name, the time and its meaning, and a signed review cannot be deleted ([Jama Part 11](https://www.jamasoftware.com/media/2023/12/jama-connect-and-fda-21-cfr-part-11.pdf)). Modern Requirements goes one step further: approval moves the item status, an audit report is produced when the review closes, and the approved set becomes a baseline in one step ([Modern Requirements](https://www.modernrequirements.com/products/modern-requirements4devops-features/review/)). Git hosting gives requimd comments and approvals on pull requests, but **analysts' stakeholders often cannot use a pull request, and the approval lives on the hosting platform rather than in the repository**. The text-based answer is a review package rendered from a git range, plus a review record committed to the repository. The regulated domains point the same way. IEC 62304 and the FDA's QMSR demand traceability but **do not prescribe how records are stored** ([Greenlight Guru](https://www.greenlight.guru/blog/iec-62304-software-dhf)). Whether assessors accept git-based evidence remains unproven: the notes found no case study either way.

## Only five idea families break the constraints, and each has a cheaper substitute

Research question 3 has a reassuring answer. Of the 97 ideas in the catalogue, **only about ten break a constraint**, and they fall into five families. Each family has a version that fits.

**Hosted collaboration** breaks the "no web app with user management, no real-time collaboration" scope. This covers reviewer accounts, notifications, @mentions, live dashboards and real-time review (Jama, Codebeamer, Polarion). The substitute is a static review package plus a review record file, with the git host's notifications doing the rest. The cost is the convenience of "click a link in an email".

**Live sync** breaks NFR-006 and the single source of truth. This covers two-way synchronisation with Jira, Azure DevOps or GitHub issues, and live OSLC providers. Static issue keys (B1, already planned) and CSV import and export give most of the value without a network.

**Embedded AI** costs binary size and dependencies (NFR-005). It breaks NFR-006 unless it is an explicit opt-in command, which ADR-0011 allows. This covers LLM rewrites, embedding-based semantic search and LLM conflict detection. The evidence argues for restraint. A November 2025 survey of 21 RM tool homepages found **11 that did not mention AI at all**, and judged most RM "AI" to be traditional NLP ([Birk](https://makingofsoftware.com/2026/02/ai-support-in-requirements-tools/)). The cheaper path is to expose requimd's deterministic graph to the user's own agent through a local MCP server, a context export and a prompt export.

**Graphical editing** breaks the "no graphical editor or web app" scope. StrictDoc's local web editor writes back to text files, but its own documentation calls it "work-in-progress" and "not yet hardened against unsafe use" ([StrictDoc](https://strictdoc.readthedocs.io/en/latest/latest/docs/strictdoc_01_user_guide.html)). A read-only preview with live reload, or the spreadsheet round trip, covers non-Markdown users.

**Heavy converters and verifiers** break NFR-005. These are bundled Chrome or LaTeX for PDF output, spelling dictionaries, and SMT solvers for formal checks (TRLC's `--verify` needs CVC5 on Linux). Print CSS on the site, the editor's own spell checker, and declarative rules without proofs are the substitutes. The owner has already decided that PDFs come from a separate converter outside requimd (FR-405, deprecated).

Two ideas fit the constraints but conflict with project rules, and they are marked as such in the catalogue. One is setting `approved` automatically after a review, which BR-004 forbids. The other is generating code lint rules from decisions (Archgate, ADR Kit), which moves requimd from analyst tooling into code enforcement.

## The field converges on LSP, MCP and deterministic checks under AI

Research question 4 points in a clear direction, with five signals.

**First, text-first tools now ship editor and agent interfaces by default.** The 2025–2026 Rust tools tracey, SARA and Reqvire are Markdown-first and git-native. tracey reached 1.0 in December 2025 with an LSP and an MCP server ([tracey](https://github.com/bearcove/tracey)). Reqvire ships an MCP server plus skills for coding assistants ([Reqvire](https://github.com/reqvire-org/reqvire)). ubCode puts an LSP, CLI and MCP server into one binary for Sphinx-needs, but it is commercial for private projects ([ubCode](https://ubcode.useblocks.com/)). StrictDoc, the most feature-rich open tool, still has no LSP. MCP and AGENTS.md have both been stewarded by the Linux Foundation's Agentic AI Foundation since December 2025 ([OpenAI](https://openai.com/index/agentic-ai-foundation/)). Jama markets an official MCP server as a first for its market ([Jama](https://www.jamasoftware.com/solutions/why-choose-jama-connect-over-codebeamer/)).

**Second, spec-driven development makes Markdown specs in the repository mainstream, but without validity checks.** Böckeler found all the tools she tried to be "spec-first", meaning the spec is written before the work and may be dropped afterwards. She reports verbose output (Kiro turned a small bug into **four user stories with 16 acceptance criteria**). She also warns that SDD risks combining model-driven development's "inflexibility *and* non-determinism", while losing the validity and consistency checking that parseable specs gave ([Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)). This is exactly the layer requimd provides. Jama draws the same line from the vendor side: OpenSpec proposals "are not a governed requirements record" ([Jama](https://www.jamasoftware.com/blog/openspec-guide/)).

**Third, AI in RM tools keeps a deterministic core.** The mature feature everywhere is scoring against INCOSE rules and EARS: Jama Advisor, IBM RQA, Polarion Copilot Content Validation, Visure, Innoslate ([Jama](https://help2.jamasoftware.com/ah/en/jama-connect-advisor-/interpreting-the-results.html), [IBM](https://www.ibm.com/products/requirements-quality-assistant)). Generative features cluster around test-case drafting, and no independent evidence of their benefit was found. LLM trace recovery reaches **about 81% recall at only 18% precision** with a recall-oriented prompt ([Tracer](https://arxiv.org/html/2611.09162)). Writing quality and machine traceability can also pull in opposite directions: some textbook "defects" helped automated tracing ([Hey and Frattini](https://arxiv.org/html/2606.11834)).

**Fourth, enterprise tools move toward git's model**: branching, merging and diff-based review (Polarion merges, Codebeamer's Review Hub diffs). requimd already lives there.

**Fifth, standards are being renewed.** SysML v2 was adopted in 2025 with a normative textual notation and a standard API ([OMG](https://www.omg.org/news/releases/pr2025/07-21-25.htm)). INCOSE's GtWR v4 dates from 2023, and a January 2026 webinar mentions a "GfWR", possibly a renamed edition ([INCOSE webinar](https://www.incose.org/wp-content/uploads/2026/01/webinar_144_the_incose_gfwr_raising_the_ante.pdf)). The FDA's QMSR took effect on 2 February 2026. The direction is clear: **analysts will be asked for verifiable, text-based, agent-readable specifications, with a governance layer on top**. That favours requimd's model, provided it adds the governance pieces.

## What requimd already does well / differentiators

requimd occupies the same niche as tracey, SARA and Reqvire: Rust, Markdown, a single binary, git-native. The notes found **none of them combining a glossary, German and English language checks and a format built for analysts**. That combination is requimd's clearest differentiator. All commercial quality checkers in the scan describe English INCOSE and EARS checks. Whether any supports German could not be confirmed, so German support may be a second differentiator, though that is unverified. German-speaking analysts are trained on IREB and the SOPHIST templates. SOPHIST co-founded IREB and runs more than 300 trainings a year ([SOPHIST](https://www.sophist.de/fileadmin/user_upload/Bilder_zu_Seiten/Training/SOPHIST_Katalog_Offene_Trainings_2024ALT3.pdf)). A tool that checks those templates speaks their language.

The **language server is a real advantage** over the older Python tools. StrictDoc's LSP is still a roadmap issue, and the Sphinx-needs LSP is part of the commercial ubCode. The single binary also avoids the most common complaints about docs-as-code tools. One Sphinx-needs user with **about 20,000 internal and 30,000 imported needs** reports a persistently slow build and split the project in several ([discussion](https://cf-workers-proxy-9e9.pages.dev/useblocks/sphinx-needs/discussions/1220)). StrictDoc's PDF output needs Chrome and ChromeDriver, and rmToo's needs LaTeX.

requimd's **suspect-link design is already more refined than most**. Doorstop writes fingerprints into each item file, so every review becomes noise in the item diff. Doorstop's own changelog also warns that `review all` followed by `clear all` wipes every review in one step ([Doorstop changelog](https://doorstop.readthedocs.io/en/v2.0/about/changelog/)). requimd keeps hashes in a separate lock file and leaves `status`, HTML comments and re-wrapped whitespace out of the hash (FR-116). That already removes the false suspects that Doorstop users avoid by tuning their fingerprint fields.

**Use case steps with stable labels** (BR-003) give requimd a finer trace target than the backlog tools offer: they link whole issues, and keep acceptance criteria as plain text fields ([Aha!](https://www.aha.io/blog/requirements-custom-fields)). Determinism answers Böckeler's central complaint about spec-driven development. Offline-by-default matches the privacy and shadow-AI concerns that Birk names as the main obstacles to AI in RM tools.

## Gaps revealed by the comparison

The owner named no fixed pain points, so research question 5 matters. Set against the table stakes of the field, requimd has nine gaps, in order of how often the notes raise them.

**Review and approval evidence** is the largest gap. Every enterprise tool records who approved which content, when, and with what meaning. requimd has pull requests and status values, but no record of who reviewed exactly which text. Its review state covers links, not items.

**Named baselines** come next. Analysts think in "approved spec v1.2", and git tags do the job technically. But requimd has no baseline command and no frozen evidence bundle. Its only "since baseline" view is the planned site changelog (B13).

**Verification results**: coverage means "a `ver` link exists", never "the test passed on release 2.3". Xray computes requirement status per version and environment ([Xray](https://confluence.getxray.app/display/XRAY35/Understanding+coverage+and+the+calculation+of+Test+and+requirement+statuses)). GitLab marks requirements satisfied from a CI artifact ([GitLab](https://docs.gitlab.com/user/project/requirements/)).

**Coverage beyond use case steps**: coverage today means steps with an `impl` link. LOBSTER and OpenFastTrace let a project declare what "complete" means for each level ([LOBSTER](https://github.com/bmw-software-engineering/lobster/blob/main/documentation/config_files.md)). Spec Kit's `/analyze` checks requirements against tasks in both directions.

**The design level**: it is planned but not specified, and the field leaves it open. ADR tools handle files and rendering, and Sphinx-needs supports decisions only through custom types.

**Interchange**: there is no ReqIF, which is the door to customers on DOORS and Polarion. There is also no spreadsheet round trip for stakeholders who live in Excel.

**Agent access**: there is no MCP server, although three of the new tools ship one.

**Rule provenance and scoring**: findings do not cite INCOSE or EARS rule IDs, and there is no per-item score or quality trend.

**Relation vocabulary and code tracing**: only `impl`, `ref` and `ver` are defined, with no derive, satisfy, conflicts or mitigates. Nothing scans code or tests for the `// FR-006` comments that requimd's own repository uses by convention.

## Sixteen ideas earn a place on the roadmap first

These ideas are recommended for adoption because they close the largest gaps, build on what requimd already has, and fit every constraint.

**For the design level**, adopt decision items (R-54), their `addresses` and `driven-by` links (R-55), and decision re-evaluation via the lock file (R-56). Add open questions with a review gate (R-60) and assumptions whose invalidation propagates (R-61). These should feed the planned traceability-model ADR (B9, F3) rather than wait for it. Open questions are the cheapest and probably the most useful item for an analyst's daily work. Re-evaluation fills the niche no other tool fills.

**For governance**, adopt named baselines (R-37), a review package for a git range (R-44) and review records with content hashes (R-45). Together they turn git into the review-and-baseline evidence that enterprise tools sell.

**For verification**, adopt a configurable coverage policy (R-24), which generalises B10. Add test-result import (R-49), which answers B2's open question when combined with a code and test tag scanner (R-26).

**For writing quality**, adopt rule provenance (R-09) and German EARS and Rupp template checks (R-11, R-12), which strengthen the differentiator.

**For agents**, adopt a local stdio MCP server with a context export (R-86, R-88).

**Cheap additions** worth doing alongside: suspect scope per link type (R-34), named inverse links (R-02), the immutability and supersession checks for decisions (R-57, R-58), and reasons recorded when suspects are cleared (R-35). ReqIF export (R-73) has high value but large effort and fragile dialects ([reqif](https://github.com/strictdoc-project/reqif)), so it should go through a deep dive first.

## Idea catalogue

Value and effort are judgements from the notes, adjusted to what the spec says requimd already does. Effort: S is up to a day, M a few days, L a week or more. "Existing" names a spec item or feature-report entry the idea extends or matches; "–" means nothing is planned.

### Data model and schema

| ID | Idea | What it does | Seen in | Problem it solves for an analyst | Fit | Value | Effort | Existing |
|---|---|---|---|---|---|---|---|---|
| R-01 | Standard relation vocabulary | Default link types derive, refine, satisfy, verify, trace, depends, conflicts, with fixed directions | SysML v1/v2, Mermaid, Cameo, BABOK, OSLC | Every link has a clear meaning; exchange with MBSE teams; basis for the design level | fits | high | S | F3/F4 traceability-model ADR |
| R-02 | Named inverse links | Each link type declares a reverse name ("verified by") shown in hover, `show` and site backlinks | StrictDoc REVERSE_ROLE, mlx, Sphinx-needs, SARA | Readers see "verified by TC-3" on the FR without writing back-links | fits | medium | S | extends FR-104, FR-400 backlinks |
| R-03 | Link topology constraints | Per link type: acyclic, single target, allowed source types | Azure DevOps link attributes | Catches derive cycles or two parents at check time | fits | medium | S | extends FR-108 (ref only) |
| R-04 | Field patterns | Regex or format rule per metadata key | mlx.traceability, Sphinx-needs schema | Enforces conventions beyond value sets (e.g. source IDs) | fits | medium | S | extends FR-103, FR-105 |
| R-05 | Addressable acceptance criteria | Labelled criteria (`ac1`, `ac2`) like step labels; TCs `ver`-link to one criterion; coverage per criterion | Gap in Jira, Aha!, Linear (AC are text fields) | Finds untested criteria, not just untested stories | fits | high | M | extends FR-121, FR-006, BR-003 |
| R-06 | Named values with units | Define `max_response = 2 s` once; reference it in text; hover and rename | Valispace/Altium RSP, SysML v2 attributes | One change of a limit updates every requirement; no inconsistent numbers | fits (adds format) | medium | M | – |
| R-07 | Variant tags | `variant:` key plus filtered check, export and site per variant | Polarion with Teamcenter, Codebeamer with pure::variants, Sphinx-needs | One spec for several product variants without long-lived branches | fits | low | M | – |
| R-08 | Stable machine IDs | Hidden IDs that follow items across renames in diffs | StrictDoc MID | Diff shows "renamed" instead of removed plus added | fits (noise in files) | low | M | FR-214 rename, BR-002 |

### Text quality and language

| ID | Idea | What it does | Seen in | Problem it solves for an analyst | Fit | Value | Effort | Existing |
|---|---|---|---|---|---|---|---|---|
| R-09 | Rule provenance | Every language finding cites its source rule (INCOSE R7, EARS, SOPHIST, 29148) in message, hover and docs | Jama Advisor, IBM RQA, GtWR v4 | Findings are defensible in reviews and audits; analysts learn the rule | fits | high | S | extends FR-110, FR-118, ADR-0005 |
| R-10 | Deterministic INCOSE subset (EN, DE) | Escape clauses, open-ended lists, absolutes, pronouns, combinators, "and/or", purpose phrases, number without unit, decimal format | GtWR v4, INCOSE workbench, Jama Advisor | Catches most sloppy wording before review, in both languages | fits | high | M | partly FR-110 list; B5 |
| R-11 | EARS with German keywords | Pattern per FR, clause order, one trigger, "If" without "then", system name from glossary; Solange/Wenn/Falls…dann/Sofern with verb-second order; skeleton snippets | EARS, Kiro, Jama Advisor | Testable requirements in a consistent form, in German too | fits | high | M | FR-112, A2 (later) |
| R-12 | Rupp/SOPHIST template check | Modal verb (muss/soll/wird) cross-checked with `prio`; activity phrases; infinitive last; wenn/falls condition first | SOPHIST, IREB, LMU handout | Matches the template German BAs are trained on; catches priority-text mismatch | fits | high | M | extends FR-118, ADR-0009 |
| R-13 | German passive and nominalisation | Flags werden + Partizip II and -ung nouns with "erfolgt"; disambiguates "wird" | SOPHIST deletion, GtWR R2 | Finds hidden actors and vague processes, the most common German defects | fits | high | M | B5 (passive) |
| R-14 | German-aware glossary matching | Inflected forms, compounds and gender-inclusive forms match a term | GtWR R4/R36, IREB glossary | Term checks stop missing "Kunden" or "Kundenkonto" | fits (changes FR-110's no-stemming rule) | high | M | extends FR-114 |
| R-15 | Rule profiles | Named presets (light, INCOSE-strict, 29148, ASPICE, IEC 62304, German BA) selecting checks and required keys | 29148, INCOSE, ASPICE | One tool serves a startup BA and a regulated team without noise | fits | high | M | extends FR-117 |
| R-16 | Quality score and report | Per-item score from rule hits; project summary in JSON and site; trend between baselines | Jama Advisor, Innoslate, IBM RQA | Shows weak items fast and whether quality improves over releases | fits | medium | M | builds on FR-301 |
| R-17 | Planguage levels | Adds past, tolerable, goal, wish to NFRs; checks units and numeric ordering | Planguage | Makes "measurable" a mechanical check | fits | medium | S | extends FR-014 (scale, meter, must, plan) |
| R-18 | ISO 25010 characteristic | `quality:` value set on NFRs and a coverage-by-characteristic report | ISO/IEC 25010:2023 | Shows blind spots (no security or safety NFRs) | fits | medium | S | – |
| R-19 | Declarative cross-field rules | "prio: must requires a ver link"; "status: review allows no weak words" | TRLC, Sphinx-needs constraints, Spec Kit constitution | Project quality gates without code | fits | medium | M | – |
| R-20 | Near-duplicate detection | Lexical n-gram similarity across items | Polarion Copilot Similarity, 29148 set consistency | Catches copy-paste requirements that drift apart | fits | medium | M | – |
| R-21 | Consistency along links | Child prio above parent, approved item verified only by draft TC, unit mismatches between linked items | Polarion Copilot Consistency Check | Contradictions across files an analyst misses | fits | medium | M | extends FR-109 |
| R-22 | Use case and story structure | Use case level and primary actor present, actor and story role in glossary, extensions refer to existing steps | Cockburn, INVEST | Use cases stay well-formed as steps change | fits | medium | S | extends FR-006, FR-007, FR-113 |
| R-23 | Bundled spell check | DE and EN dictionaries in the binary | GtWR R13 | Fewer typos | breaks NFR-005 (size); leave to the editor | low | M | – |

### Traceability, coverage and impact

| ID | Idea | What it does | Seen in | Problem it solves for an analyst | Fit | Value | Effort | Existing |
|---|---|---|---|---|---|---|---|---|
| R-24 | Coverage policy | Config per item type with AND/OR groups ("FR needs TC"; "NFR needs TC or inspection"; "UC step needs FR") | LOBSTER `requires`, OFT `Needs:`, Spec Kit `/analyze` | Defines "complete" for the team's own process | fits | high | M | generalises FR-106, FR-306, B10 |
| R-25 | Justified exceptions | Per-item override (`needs:` or `no-ver: reason`) that removes a gap with a recorded reason | OFT per-item `Needs:`, BASIL justifications | No false gaps; the reason is auditable | fits | medium | S | – |
| R-26 | Code and test tag scanner | Scans configured globs for IDs in comments (`// FR-006`, OFT `[impl->FR-012]`); unknown IDs; "realized in" on hover | OFT, tracey, StrictDoc `@relation`, e-adr | Analysts see what developers implemented without reading code | fits (offline text scan) | high | M | B2 open question; NFR-010 convention |
| R-27 | Trace query | `requimd trace FR-012 --up --down --depth n` with filters (uncovered, unverified, suspect) | SARA `query`, tracey `query`, StrictDoc query | Quick impact answers in a meeting | fits | medium | S | extends FR-305, FR-404 |
| R-28 | Matrix per relation kind | Matrix between any two item types or one link kind, with missing cells highlighted | Cameo, Sparx, R4J | Gaps between two levels at a glance | fits | medium | S | extends FR-401 (steps only) |
| R-29 | Coverage numbers | Percentages per type and link kind in check output and site | Jama trace score, LOBSTER report | One health indicator for reviews and management | fits | medium | S | extends FR-306 |
| R-30 | Orphan and derived reports | Tests without requirement, requirements without parent, transitive gaps | OFT, LOBSTER, DO-178C | Answers standard audit questions directly | fits | medium | S | – |
| R-31 | Coverage in the editor | Code lens or inlay hint per item: "2 impl, 1 ver, 1 suspect" | tracey | Gaps visible while writing | fits | medium | S | extends FR-209, FR-215, B7 |
| R-32 | Offline link suggestions | Glossary-term overlap suggests missing links as code actions, never auto-added | NLP4RE trace research | Finds forgotten links | fits (low precision expected) | low | M | – |
| R-33 | Multi-repo aggregation | Imports another repository's JSON export as read-only external items | Sphinx-needs `needimport`, SARA, Reqvire workspace, DOORS GCM | System and component specs in separate repos stay traceable | fits | medium | M | uses FR-402 |

### Baselines, change and suspect links

| ID | Idea | What it does | Seen in | Problem it solves for an analyst | Fit | Value | Effort | Existing |
|---|---|---|---|---|---|---|---|---|
| R-34 | Suspect scope per link type | Chooses per link type which parts count (text, title, keys), or "never suspect" | ReqView, Doorstop `attributes: reviewed`, Cameo | Fewer false suspects from cosmetic or loose links | fits | high | S | extends FR-116 (fixed scope, status excluded) |
| R-35 | Review with a reason | `requimd review FR-012 --reason "editorial"`; `--all` requires a reason; cleared links appear in diff output | Doorstop changelog warning, OFT/tracey semantic revisions | Impact decisions are documented; mass clearing stays visible | fits | medium | S | extends FR-307, FR-218 |
| R-36 | Item review state | "Changed since last review" per item, separate from link suspicion | Doorstop `reviewed` fingerprint | Supports item-level review workflows | fits | medium | M | extends ADR-0007 |
| R-37 | Named baselines | `requimd baseline <name>`: tag plus record (description, state) and frozen bundle (hashes, matrix, coverage, open suspects, status counts) | DOORS baseline sets, Spira, R4J, Sparx, ASPICE | Analysts think in baselines, not git refs; audit-ready snapshot | fits | high | M | builds on FR-403, B13 |
| R-38 | Baseline-to-baseline report with impact | "What changed since baseline X" plus suspect impact of each change | Modern Requirements, ReqView compare, BABOK | Answers change-board questions in one page | fits | high | S | FR-403, B13 (planned; impact new) |
| R-39 | Per-item history | Who changed an item, when, and with which commit message, from git log, in `show` and on the site | ISO 26262, IEC 62304, ReqView history report | Item-level audit trail without a database | fits | high | M | same git calls as B6 |
| R-40 | Status transition rules | Allowed transitions checked against a base ref in CI | BABOK approve | Prevents status jumps such as draft to approved without review | fits | medium | S | extends FR-109, BR-004 |
| R-41 | Impact of a pull request | `--since merge-base`: what this PR changed and what is now suspect | Reqvire | One command for PR review | fits | medium | S | extends FR-403 |
| R-42 | Change-request items | CR items with `affects` links; diff grouped per CR | Azure DevOps CMMI | Impact analysis per change request | fits | medium | S | – |
| R-43 | Status by release | Per tag: which items existed, which were verified and passing | Xray per Fix Version, Azure release links | "What was in release 2.3, and was it verified?" | fits | medium | M | needs R-49 |

### Review and approval

| ID | Idea | What it does | Seen in | Problem it solves for an analyst | Fit | Value | Effort | Existing |
|---|---|---|---|---|---|---|---|---|
| R-44 | Review package | Static page for a git range: changed items only, word-level differences, suspect impact, and a checklist of judgement rules per perspective (user, designer, tester); printable | Codebeamer Review Hub, Jama Review Center, Modern Requirements, OpenSpec deltas, PBR | Stakeholders review only what changed, in a browser or on paper | fits | high | M | builds on B13, FR-403 |
| R-45 | Review records in the repository | File with review ID, items with content hashes, reviewers, verdict per item; check marks it stale when an item changes | Jama Part 11 record, Modern Requirements audit report, ASPICE | Proves the released text is the reviewed text; reviews outlive the git host | fits | high | M | – (honours BR-004) |
| R-46 | Signed approvals | Verifies that a baseline tag or commit is signed and lists signers | Part 11 practice, Polarion and Jama e-signatures | Evidence of who approved, without a server | fits (local git); identity management would break scope | medium | M | – |
| R-47 | Automatic approval status | Review result sets `status: approved` | Modern Requirements | Avoids forgotten status updates | conflicts with BR-004; propose `review` only | low | S | BR-004 |
| R-48 | Hosted review app | Reviewer accounts, notifications, @mentions, real-time review | Jama, Codebeamer, Polarion | Large-team coordination | breaks: web app with user management, real-time collaboration | low | L | – |

### Verification and test results

| ID | Idea | What it does | Seen in | Problem it solves for an analyst | Fit | Value | Effort | Existing |
|---|---|---|---|---|---|---|---|---|
| R-49 | Test result import | Reads JUnit XML (or a results file keyed by ID); pass, fail or skipped on TCs, requirements, site and coverage | sphinx-test-reports, StrictDoc, GitLab, Xray | "Is this requirement met now?", not just "is there a test?" | fits (local files) | high | M | B2 (later) |
| R-50 | Gherkin feature files | `.feature` scenarios with `@FR-012` tags count as `ver` records; EN and DE keywords; listed on the site | Gherkin, OFT `.feature` import, Gauge | Business-language acceptance criteria become traceable tests | fits | high | M | – |
| R-51 | Test skeletons | Code action creates a TC stub from a requirement, EARS clause or criterion, with `ver` prefilled | Copilot4DevOps, Inflectra.ai (LLM versions) | Faster verification items, higher coverage | fits (template, no LLM) | medium | S | like FR-213 |
| R-52 | Verification by method | Coverage report split by `method` (test, inspection, analysis, demonstration) | Reqvire, Mermaid `verifymethod`, SysML | Plans verification for items that cannot be tested automatically | fits | low | S | F2 `method` key, B10 |
| R-53 | CI coverage gate | Exit code when coverage drops below a threshold or selected checks fire | Reqflow, ubCode, tracey `--deny warnings` | Incomplete specs cannot be merged | fits | low | S | extends FR-302 |

### Design level: decisions, open questions, assumptions, spikes

| ID | Idea | What it does | Seen in | Problem it solves for an analyst | Fit | Value | Effort | Existing |
|---|---|---|---|---|---|---|---|---|
| R-54 | Decision items (DEC) | States proposed, accepted, rejected, deprecated, superseded; tools and AI set only proposed | Nygard, MADR, arc42, pyadr, ADR Guard | Decisions buried in mails and meetings get an ID, status and place | fits | high | M | B9, F3 |
| R-55 | Decision links | `addresses` and `driven-by` from DEC to requirements, NFRs and rules; report "significant requirement without decision" | Y-statements, MADR drivers, Tyree/Akerman, ISO 42010 | Shows why a requirement is realised the way it is | fits | high | S | B9 |
| R-56 | Decision re-evaluation | A DEC turns suspect when a `driven-by` or `addresses` target changes | Fowler re-evaluation triggers, ecADR "ADs age"; no tool automates it | Changed requirements no longer silently invalidate past decisions | fits (lock file) | high | M | extends FR-116, ADR-0007 |
| R-57 | Immutable accepted decisions | Warns when an accepted DEC's hash changes without a superseding DEC | Fowler, Nygard, Google amendments | Keeps the decision log trustworthy | fits | high | S | – |
| R-58 | Supersession chain | `supersedes` link, computed inverse, warning on links to superseded DECs, chain on site | PEP 1, KEP, Fowler | Shows which decision is current | fits | high | S | – |
| R-59 | Decision confirmation | `ver` from a TC (review checklist or fitness test) to a DEC; "implemented" as a second status axis | MADR Confirmation, ecADR, PEP Final, KEP implemented, Oxide committed | Decisions "verified on their own level", as the vision asks | fits | high | S | vision long-term goal |
| R-60 | Open questions (Q) | States open, answered, obsolete; `about`, `answered-by`; optional gate blocks `review` while open; hover shows a requirement's questions | PEP Open Issues, Rust RFC unresolved questions, Oxide ideation | Tracks unresolved points per requirement until they are settled | fits | high | S | – |
| R-61 | Assumptions (ASM) | States assumed, validated, invalidated; invalidation makes dependent DECs and requirements suspect | Tyree/Akerman, SysML v2 `assume`, hypothesis cards | Business assumptions become explicit, with visible impact when wrong | fits | high | M | – |
| R-62 | Spikes (SPK) | Hypothesis, timebox, success signal, verdict; `evidence` link from DEC | Thoughtworks, Pip Decks, ecADR Evidence | Spike findings stop disappearing into tickets | fits | medium | S | – |
| R-63 | Considered options | Labelled options (`o1`, `o2`) with pros and cons; flags single-option decisions without justification | MADR, Tyree/Akerman, ecADR | Rejected options stay findable and referenceable | fits (label mechanism of BR-003) | medium | S | – |
| R-64 | Y-statement form | One-sentence decision with a clause check, EN and DE | Zimmermann, cards42 | Easy decisions for analysts to write and review | fits | medium | S | – |
| R-65 | Proposals with goals and non-goals | Proposal item referencing requirement IDs; warns when a non-goal is addressed | KEP, Google design docs, OpenSpec proposal.md | Scope boundaries become explicit and checkable | fits | medium | S | – |
| R-66 | Revisit dates and confidence | `revisit:` and `confidence:` keys; opt-in `--as-of <date>` lists overdue reviews | Fowler, ecADR, MADR | Decision hygiene without a separate tracker | fits (explicit date keeps output deterministic) | medium | S | – |
| R-67 | Design elements (DES) | DES items with `satisfies` and a `subject`; coverage "requirement without design" | SysML v2, SARA, OFT `dsn`, Kiro design.md, Capella | Requirement-to-design coverage for architects | fits | medium | M | F3 |
| R-68 | `raises` link | A DEC or spike raises new requirements or questions | Twin Peaks, ISO 42010 | Captures the loop between requirements and design | fits | medium | S | – |
| R-69 | Decision log on the site | Status filter, link graph, DEC columns in matrix and JSON | Log4brains, adr-log, Backstage plugin, Oxide CSV | One place to browse decisions with their requirements | fits | medium | M | extends FR-400 to FR-402 |
| R-70 | Read existing ADR folders | Reads MADR or Nygard ADRs (e.g. `docs/decisions/`) as DEC items | Structurizr `!adrs` importers | Teams with existing ADRs trace them at once | partly breaks requimd's metadata-line format (MADR uses YAML front matter; needs an adapter) | medium | M | – |
| R-71 | Decision accountability | `decided-by`, `consulted`, `informed`; `discussion` and `resolution` links | MADR, PEP 1, Oxide | Shows who decided and where | fits | low | S | – |
| R-72 | Decisions as code rules | Generates lint or architecture rules from decisions | Archgate, ADR Kit | Enforces decisions in code | breaks: scope and NFR-005 (language-specific) | low | L | – |

### Publishing and interchange

| ID | Idea | What it does | Seen in | Problem it solves for an analyst | Fit | Value | Effort | Existing |
|---|---|---|---|---|---|---|---|---|
| R-73 | ReqIF export, later import | ReqIF 1.2 following the ProSTEP profile; import as a one-off conversion | Polarion, Visure, DOORS, StrictDoc, Capella | Exchange with customers and suppliers on DOORS or Polarion | fits (file transform) | high | L | – |
| R-74 | Tracker CSV import and export | Title, description, ID, status to and from Jira or GitLab CSV | GitLab requirements CSV, Doorstop | Moves items in bulk between a backlog and requimd | fits | medium | S | – |
| R-75 | SysML v2 textual export | One-way export of requirement usages with `doc`, `satisfy`, `verify` | SysML v2 | Hand-over to systems engineering teams | fits | low | M | – |
| R-76 | Content hash in JSON export | Per-item hash in the JSON output | mlx.traceability | Downstream tools detect changes without the lock file | fits | low | S | extends FR-402 |
| R-77 | Linked-data and attestation export | Static JSON-LD with OSLC terms; CycloneDX attestation file | OSLC, CycloneDX 1.6 | Machine-readable compliance evidence (CRA) | fits (static file) | low | M | – |
| R-78 | Live sync | Two-way sync with Jira, Azure DevOps or GitHub; live OSLC | R4J, Jama–Jira, Polarion connectors | Backlog and spec stay aligned | breaks NFR-006 and the single source of truth | medium | L | B1 (static keys) fits |
| R-79 | Bundled Word or PDF converter | Chrome, LaTeX or docx writer inside the tool | StrictDoc HTML2PDF, Doorstop LaTeX | Contract documents | breaks NFR-005; owner chose a separate converter (FR-405) | low | L | FR-405 deprecated |

### Stakeholder reach

| ID | Idea | What it does | Seen in | Problem it solves for an analyst | Fit | Value | Effort | Existing |
|---|---|---|---|---|---|---|---|---|
| R-80 | Spreadsheet review round trip | Exports items with comment and verdict columns (CSV, later XLSX); re-imports as a review record or report | Doorstop, StrictDoc Excel, BASIL | Non-technical stakeholders review in Excel | fits (XLSX needs a crate) | high | M | builds on FR-401, D4 |
| R-81 | Print-ready site | Print CSS, cover page, revision table, so a browser prints a sign-off document | Doorstop, StrictDoc, ubCode, rmToo | Formal documents without bundling Chrome or LaTeX | fits | low | S | extends FR-400 |
| R-82 | Document-order export | Site or single page ordered by the 29148, Volere or Lastenheft/Pflichtenheft outline | 29148, Volere, German practice | Produces the document shape a contract expects | fits | medium | M | – |
| R-83 | Audience views | Configured views (include or exclude by type, tag, link kind) driving site pages and diagrams | Structurizr views | Tailored views for business, test and design readers | fits | medium | M | extends FR-400 `--status`, FR-404 |
| R-84 | Workshop views | Story map and impact map generated as Mermaid from `activity:` or `goal:` links | Patton, Adzic | Visual overviews for workshops without another tool | fits | low | M | extends FR-404 |
| R-85 | Local web editor | Browser UI that edits and writes back Markdown | StrictDoc server | Non-Markdown users can edit | breaks: graphical editor or web app (a read-only preview server fits) | medium | L | – |

### AI and agent support

| ID | Idea | What it does | Seen in | Problem it solves for an analyst | Fit | Value | Effort | Existing |
|---|---|---|---|---|---|---|---|---|
| R-86 | Local MCP server | `requimd mcp` over stdio: get item, search, trace, suspects, check, coverage, glossary | Jama MCP, ubCode, tracey, Reqvire, Tessl | Coding agents read the analyst's approved requirements instead of guessing | fits (subprocess, no network) | high | M | – |
| R-87 | MCP propose tools | Drafts an item or change with the next free ID into files; validated by check; never approves | Doorstop MCP, OpenSpec propose, ADR Guard | Agents propose changes in the right format for review as a PR | fits (BR-004) | medium | M | – |
| R-88 | Context export | `requimd context FR-012 --depth 2`: item, linked items, glossary terms, verification, as Markdown or JSON | BMAD story files, Jama "governed context" | A self-contained, traceable brief per feature | fits | high | S | extends FR-305, FR-402 |
| R-89 | Agent instructions file | Generates an AGENTS.md section on the format, ID ranges, BR-002, BR-004 and how to run check | AGENTS.md, Kiro steering, Spec Kit constitution | Agents editing the repo follow the analyst's conventions | fits | medium | S | extends FR-303 |
| R-90 | Prompt export | Prints an LLM-ready prompt with context and rules for a rewrite or test draft | Pattern behind Jama, Copilot4DevOps, Codebeamer | LLM help without requimd sending data anywhere | fits | medium | S | – |
| R-91 | Opt-in LLM commands | Rewrite, test drafts, ambiguity review against a configured endpoint | Jama Advisor, Copilot4DevOps, IBM AI Hub | One-command drafts | allowed by ADR-0011 opt-in; costs NFR-005 | medium | L | B4 (on demand) |
| R-92 | AI provenance marker | `origin: ai` key; check keeps such items at draft until reviewed | Inflectra "ai-augmented", ADR Guard | Reviewers know which text needs extra care | fits | low | S | – |
| R-93 | Embedding search | Semantic search with bundled model weights | Atlassian Rovo | Search by meaning | breaks NFR-005 and NFR-006 | low | L | – |

### Regulated-domain evidence

| ID | Idea | What it does | Seen in | Problem it solves for an analyst | Fit | Value | Effort | Existing |
|---|---|---|---|---|---|---|---|---|
| R-94 | Integrity level attribute | `asil`, `dal` or safety class with an inheritance check (child at least as strict unless decomposition is noted) | ISO 26262, DO-178C, IEC 62304 (background) | Catches integrity mistakes in decomposition | fits | medium | M | – |
| R-95 | Risk control links | `mitigates` from requirements to risk items, plus risk-control coverage | IEC 62304, ISO 14971 | Every hazard has a verified control | fits | medium | S | – |
| R-96 | External requirement catalogues | Read-only items for regulation or standard clauses (CRA Annex I, IEC 62443) with `satisfies` and a compliance matrix | CRA, IEC 62443, ASPICE (background) | Compliance mapping with the same machinery as internal traces | fits | medium | M | – |
| R-97 | Tool qualification kit | Rule catalogue, deterministic outputs, fixture corpus with expected diagnostics, versioned release notes | DO-330, ISO 26262 tool confidence | Regulated users can argue tool confidence | fits (documentation and tests) | medium | M | builds on `examples/broken/EXPECTED.md` |

## Candidates for deep dives

These eight deep dives follow from the evidence. They are ordered by how much each would change requimd's next decisions.

**DD-1. Design-level practice for the traceability-model ADR.** Covers MADR, ecADR, Y-statements, PEP, KEP, Rust RFCs, Oxide RFDs, and the subject and satisfy semantics of SysML v2. This is the open niche, and the planned ADR (B9, F3) needs it. The deep dive should answer five questions. Which item types come first (decisions, questions, assumptions, spikes, design elements)? Which status values? Which link names? Should agreement and realisation be two axes? How should re-evaluation propagate through the lock file? Should analysts start with business decisions rather than architecture?

**DD-2. Review and approval evidence: Jama Review Center, Modern Requirements, Codebeamer Review Hub.** It should define the minimum content of a review package and a review record, compared with what Part 11 expects of a signature (name, time, meaning). It should also say how reviewers without git take part, and whether signed tags or `Approved-by:` trailers are enough evidence for any assessor.

**DD-3. Suspect-link and review-state mechanics: Doorstop, ReqView, OpenFastTrace and tracey revisions.** It should decide which refinements FR-116 and ADR-0007 need. Candidates are a per-link-type scope, an item-level review state, recorded reasons, and an "editorial change" escape. It should also look at how to keep `requimd.lock` merge conflicts, already a known rebase pain point, under control as the file grows.

**DD-4. Coverage policy and code and test tracing: OpenFastTrace, LOBSTER, sphinx-test-reports.** It should answer B2's open question of how tests are matched to items. That includes whether to adopt OFT's tag syntax for interoperability, and what a policy configuration looks like in `requimd.toml`. A further question is whether to read `.lobster` or JUnit files as evidence.

**DD-5. StrictDoc and Sphinx-needs/ubCode.** These are the two feature-rich open relatives. ubCode is also the one commercial single-binary competitor with an LSP and MCP. The deep dive should study their schema models (grammar versus central config, field schemas, link `target_types`, severities), StrictDoc's ReqIF and Excel round trips and what its ReqIF profile teaches, and how ubCode's licensing shapes the market for an open tool.

**DD-6. The Rust git-native wave: tracey, SARA, Reqvire.** These are requimd's direct neighbours, and reading their docs and code is within scope. The deep dive should confirm the differentiator claim (no glossary and no German language checks). It should also compare their LSP features and MCP tool sets, and look at their design types, multi-repo workspaces and merge-base impact.

**DD-7. Spec-driven development and agents: Kiro, OpenSpec, Spec Kit, MCP, AGENTS.md.** It should define requimd's role as the deterministic, governed layer beneath agent-written specs. That means the MCP tool list, the context export format and whether OpenSpec-style deltas map onto `requimd diff`. It should also say whether analysts, not only developers, benefit. Böckeler leaves that last question open.

**DD-8. German quality rules: INCOSE GtWR v4 (or the 2026 "GfWR"), SOPHIST/Rupp, IREB.** It should verify the authoritative rule list and numbering, which the scan could not read. It should map each deterministic rule to German, validate a German weak-word list with the owner, and check whether any commercial checker supports German. If none does, requimd's second differentiator is confirmed.

Two candidates the charter named are not recommended as separate deep dives. Polarion and Codebeamer offer mostly server-side features that do not transfer; their diff-based review is covered by DD-2 and their variants rate low. SysML v2 matters mainly for its relation vocabulary, which DD-1 absorbs. ReqIF (R-73) sits inside DD-5. A separate ReqIF deep dive would make sense only if a real customer exchange is on the horizon.

## Open points and caveats

**Vendor bias is strong.** Most enterprise and AI-tool claims come from vendor pages and partner blogs. Examples include Copilot4DevOps' "up to 60% faster" test creation, a reported 30% cut in Codebeamer review cycles that PTC's own material does not confirm, and Jama's "first official MCP server". The way Jama computes its trace score was never confirmed. No independent benchmark of any AI quality checker was found. No user study compares analysts' results with and without these features. No usability study of business analysts using any of the text-based tools was found either.

**Several primary sources could not be read.** Codebeamer's 3.3 release page returned HTTP 403. The INCOSE GtWR v4 summary sheet also returned 403, so the full R1–R42 rule list and numbering are unverified. ISO 29148, ISO 26262, DO-178C and ISO 42010:2022 are paywalled, so their clause details are secondary. The official Spec Kit README, Kiro's steering and hooks pages, the OpenSpec README and the Tessl docs were not opened, so some SDD details come from secondary blogs. The rmToo repository returned 404, and its profile rests on one blog post that also misstates Doorstop's licence (it says "MIT-style"; GitHub shows LGPLv3). BASIL's conference slides are image-only.

**Several statements are marked background and unverified.** These include the Volere fields, the Cockburn template details, the OSLC property names, the CRA dates and obligations, the ISO 25010:2023 characteristic names, Polarion's Subversion backend and the German Gherkin keywords.

**Some areas were not researched.** These are the Confluence requirements blueprint, Productboard, Linear Documents, Zephyr, PlantUML requirement support, Go and IETF proposal processes, RAID logs and Agent OS (no sources found). Licences were not verified for Sphinx-needs, Spec Kit, BMAD, Kiro and Capella.

**No evidence was found either way on whether assessors accept git-based evidence** in ASPICE, FDA or DO-178C audits. The scan also found no long-term evidence that SDD specs stay in sync with code over months.

**Ratings are judgement, not measurement.** The researchers rated value and effort without reading requimd's code, and this report adjusted them only against the spec. One notes file proposed a configurable fingerprint scope; FR-116 already excludes `status`, HTML comments and whitespace, so R-34 is narrowed to per-link-type scope. The owner should calibrate the ratings at the checkpoint.

## Conclusion

The scan changes the framing of requimd's next steps. The question is no longer which enterprise features a text tool can afford. Nearly all of them reduce to files, hashes and git history, and the few that need a server have workable substitutes. The real decision is where requimd should lead rather than follow. The evidence points to two places that reinforce each other. One is the design level, where decision-record tooling has never been connected to requirements and where requimd's lock file can turn "ADs age" from advice into a check. The other is review and baseline evidence, where enterprise tools sell governance that git can provide once requimd writes it into the repository.

The AI wave strengthens this position rather than threatening it. As agents write more specifications, the scarce resource becomes a deterministic, offline layer that says whether a spec is valid, complete, reviewed and current. Jama is positioning its database as that layer for large companies. requimd can be the same layer for teams who keep their requirements in git, and its German and English checks for analysts are something neither Jama nor the Rust newcomers offer.
