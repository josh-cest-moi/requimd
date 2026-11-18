# Backlog/ALM platforms and model-based (MBSE) requirements tools: profiles and ideas for requimd

Scan date: 2026-11-18. Public sources only, no accounts, no software installed or run. Sources are from 2024 to 2026 unless marked otherwise. Many profile fields (licence, pricing) could not be checked within this scan's budget; these are marked "not verified here" and also listed under Gaps.

## How do these tools represent requirements? (tool profiles)

### Takeaway
Backlog/ALM platforms model requirements as work items (issues) with a type, a parent-child hierarchy and generic links. Traceability to tests and releases comes from add-ons (Jira) or from built-in test/pipeline links (Azure DevOps, GitLab). MBSE tools model requirements as typed model elements with a fixed set of semantic relations (derive, refine, satisfy, verify, trace, copy, containment). SysML v2 adds a standard textual notation and a standard REST API, which is the part closest to requimd's text-first approach.

### Cited Findings

#### Group A: backlog and ALM platforms

**Jira + RTM (Requirements and Test Management for Jira)**
- Maker: Deviniti (Jira Marketplace app). Licence: commercial (Marketplace); pricing not verified here.
- Data model: requirements, test cases, test plans and executions are Jira issues managed in a "Test Management" area. Deviniti says many customers work with multi-layer requirements (system, subsystem, component) — [Deviniti RTM product page](https://deviniti.com/products/atlassian-apps/jira-requirements-test-management/).
- Traceability: a Traceability report (Apps > Test Management > Reports > Traceability) gives visibility over linked test cases, requirements and defects; issue links can be toggled into the table — [RTM Cloud docs: traceability](https://deviniti.com/support/addon/cloud/requirements-test-management/latest/traceability/), [RTM Server docs](https://deviniti.com/support/addon/server/requirements-test-management/latest/traceability/).
- Reporting: traceability report, coverage report and test execution summary; the Requirement Test Coverage report shows whether requirements are covered by test cases, test plans, executions and defects — [Deviniti blog: requirement coverage report](https://deviniti.com/blog/application-lifecycle-management/report-requirement-test-coverage-jira/).
- Plain Jira has no traceability matrix report by default; community answers point to apps such as RTM — [Atlassian Community: Generate a RTM in JIRA](https://community.atlassian.com/forums/Jira-questions/Generate-a-RTM-in-JIRA/qaq-p/1427193).
- Not verified here: baselines, review/approval workflow.
- Distinctive ideas: requirement tree with levels inside a backlog tool; one coverage report spanning requirement, test, execution and defect.

**R4J – Requirements Management for Jira (now "easeRequirements for Jira")**
- Maker: easeSolutions. Commercial Marketplace app — [easeSolutions product page](https://www.easesolutions.com/r4j).
- Data model: a requirements tree over Jira issues; documentation-style hierarchy (folder/tree) — [R4J documentation](https://easesolutions.atlassian.net/wiki/x/RwCrh), [R4J linking features](https://easesolutions.atlassian.net/wiki/x/AYBXUw).
- Baselines: snapshots of current data for milestones, releases or reviews; baselines can be compared over the lifecycle — [R4J documentation](https://easesolutions.atlassian.net/wiki/x/RwCrh) (as summarized by search; page not fetched in full).
- Reporting: a two-dimensional traceability matrix between item types (e.g. initiatives to epics) and a coverage view down a chain (initiative > epic > story > bug > subtask) — [R4J documentation](https://easesolutions.atlassian.net/wiki/x/RwCrh).
- Pain point from a user review: performance degrades with thousands of requirements across interconnected projects, especially for traceability reports and baselines — [Capterra reviews](https://www.capterra.ca/reviews/164178/requirements-management-for-jira-r4j).
- Distinctive ideas: baseline per release/review; matrix between any two item types; multi-level coverage chain.

**Xray (test management for Jira)**
- Maker: Xblend/Xpand IT (Marketplace app) — [Atlassian Marketplace: Xray](https://marketplace.atlassian.com/plugins/com.xpandit.plugins.xray/overview).
- Data model: "coverable issues" (requirements, stories) are covered by Test issues; tests run in Test Executions, optionally per Test Environment — [Xray Cloud: Coverable issues](https://confluence.getxray.app/display/XRAYCLOUD/Coverable+issues).
- Requirement status is computed per version: for requirement X and version V, Xray evaluates the statuses of related tests executed on version V; a "Requirement Coverage Strategy" setting decides whether versioned test executions or versioned test sets count, so one requirement can have a different status per Fix Version — [Xray: Understanding coverage and the calculation of Test and requirement statuses](https://confluence.getxray.app/display/XRAY35/Understanding+coverage+and+the+calculation+of+Test+and+requirement+statuses).
- Environment analysis answers "how is this requirement doing on environment X?"; executions without an environment count as an "empty" environment in the aggregate — [Xray: Test Coverage](https://confluence.getxray.app/display/XRAY/Test+Coverage).
- Overall Requirement Coverage report: stacked bar per analysis version (or test plan), test environment and grouping field — [Xray: Overall Requirement Coverage Report](https://confluence.getxray.app/display/XRAY/Overall+Requirement+Coverage+Report).
- Pain point: users report the "Requirement Status" field and report values disagreeing — [Atlassian Community thread](https://community.atlassian.com/t5/Marketplace-Apps-Integrations/One-requirement-multiple-test-versions/qaq-p/978210).
- Note: sources mix Xray doc versions (Server 3.2/3.5, Cloud), so exact labels may differ by edition.
- Distinctive ideas: requirement status as a derived value per (version, environment); configurable coverage strategy; status rolled up from the latest execution.

**Confluence requirements pages**
- Not researched in depth in this scan (see Gaps). Practitioner articles argue that Jira lacks "document-like views", so requirement context is scattered — [Atlassian Community: Jira and Requirements – The Missing Piece](https://community.atlassian.com/forums/App-Central-articles/Jira-and-Requirements-The-Missing-Piece/ba-p/3116387).

**Azure DevOps Boards**
- Maker: Microsoft. Commercial SaaS (Azure DevOps Services) and on-premises (Azure DevOps Server).
- Data model: work items of process-specific types (User Story in Agile, PBI in Scrum, Requirement in CMMI). Parent-child links support "Backlog Overview, Stories Overview, and Requirements Overview reports" — [Link types reference (ms.date 2025-10-08, updated 2026-11-03)](https://learn.microsoft.com/en-us/azure/devops/boards/queries/link-type-reference).
- Relations: system link types Parent/Child (tree, one parent only, no cycles), Related (network, non-directional), Predecessor/Successor (dependency, cycles rejected), Duplicate/Duplicate Of; process-defined Tested By/Tests (test case to requirement/story/PBI), Affects/Affected By (CMMI only: change request to requirement), shared steps/parameters; remote links across organizations (Consumes From/Produced For) — same source.
- Every link type carries machine-readable attributes: `directional`, `acyclic`, `singleTarget`, `topology` (tree, dependency, network), `remote` — same source.
- Traceability to code and delivery: external links to Branch, Commit, Pull Request, Build, Found in build, Integrated in build, "Integrated in release environment" (created when a release reports deployment status), Tag, Test Result, Wiki; GitHub Commit/Issue/Pull Request links for connected repos — same source.
- Not verified here: Acceptance Criteria field, CMMI Requirement Type field, Impact Assessment field (the CMMI guidance page returned 404).
- Distinctive ideas: link types with declared topology constraints; "Integrated in release environment" as an automatic requirement-to-release link; change requests linked with "Affects".

**GitLab requirements management and test cases**
- Maker: GitLab. Tier: Ultimate (GitLab.com, Self-Managed, Dedicated) — [GitLab docs: Requirements](https://docs.gitlab.com/user/project/requirements/).
- Data model: a requirement has title and description; it can be archived/reopened; status filter satisfied/failed/missing. Requirements and test cases are being migrated to "work items" — same source.
- Verification from CI: a (manual) CI job writes `requirements.json` as a `requirements` report artifact; `{"*":"passed"}` marks all open requirements satisfied; specific IIDs can get individual `passed`/`failed`; the key becomes `requirements_v2` once work items are enabled; `CI_HAS_OPEN_REQUIREMENTS` lets the job run only when needed — same source.
- Import/export: CSV import needs `title` and `description` headers; CSV export (emailed) includes ID, title, description, author, timestamps, state — same source.
- Dates: minimum role changed to Planner in GitLab 17.7; screenshots date from 13.1/13.5, so parts of the feature are older (2020 era) — same source.
- Distinctive ideas: requirement "satisfied" state set by a CI artifact; a very thin requirement model (title + description) that still supports CI verification.

**GitHub Issues and Projects**
- Maker: GitHub (Microsoft). SaaS plans Free/Team/Enterprise.
- Sub-issues, issue types and advanced search became generally available in April 2025 — [GitHub changelog 2025-04-09](https://github.blog/changelog/2025-04-09-evolving-github-issues-and-projects/), [InfoQ](https://www.infoq.com/news/2025/04/github-subissues-journey).
- Limits raised to 100 sub-issues per parent and 25 issue types per organization (December 2024) — [GitHub changelog 2024-12-12](https://github.blog/changelog/2024-12-12-github-issues-projects-close-issue-as-a-duplicate-rest-api-for-sub-issues-and-more/).
- Issue fields (structured metadata pinned to issue types; single select, text, number, date): public preview March 2026, all organizations May 2026, GA on 2026-07-02 — [changelog 2026-03-12](https://github.blog/changelog/2026-03-12-issue-fields-structured-issue-metadata-is-in-public-preview/), [changelog 2026-05-21](https://github.blog/changelog/2026-05-21-issue-fields-are-now-in-public-preview-for-all-organizations/), [changelog 2026-07-02](https://github.blog/changelog/2026-07-02-issue-fields-are-now-generally-available).
- Not verified here: typed issue-to-issue relations beyond sub-issues and duplicates; built-in test linking (none known).
- Distinctive ideas: fields defined per issue type (a schema per item kind); hierarchy limited to parent/sub-issue.

**Aha!**
- Maker: Aha! Labs. Commercial SaaS.
- Data model: requirements are children of features; release views group requirements into features — [SourceForge: Aha vs Linear](https://sourceforge.net/software/compare/Aha-vs-Linear/) (aggregator; treat as weak).
- Acceptance criteria: custom fields on requirements, e.g. a Note field for acceptance criteria — [Aha! blog: requirements custom fields](https://www.aha.io/blog/requirements-custom-fields); an AI library prompt generates acceptance criteria (main flow and edge cases) from the feature definition — [Aha! AI library: acceptance criteria](https://www.aha.io/ai-library/request-acceptance-criteria).
- Distinctive ideas: release > feature > requirement hierarchy; generating acceptance criteria from a feature description.

**Productboard**
- Not researched in this scan (budget). See Gaps.

**Linear**
- Maker: Linear Orbit. Commercial SaaS.
- Reviewers describe each requirement as an issue tied to status, owner and release planning, with custom issue types/fields/statuses; document-style formatting for long specs is limited — [ONES tool guide](https://ones.com/blog/?p=1170409) (competitor-written, weak source).
- Not verified here: Linear Documents, project specs, relations.

#### Group B: model-based requirements

**SysML v2 (OMG)**
- Maker: OMG (Object Management Group); open standard. SysML v2.0, KerML 1.0 and the Systems Modeling API and Services 1.0 were approved for final adoption (press release 2025-07-21; the release repo says formally adopted 2025-06-30) — [OMG press release](https://www.omg.org/news/releases/pr2025/07-21-25.htm), [sysml-v2-release on GitHub](https://github.com/systems-modeling/sysml-v2-release). Formal document number formal/2025-09-03 (September 2025); editorial update March 2026 for ISO submission — same GitHub repo, [Wikipedia](https://en.wikipedia.org/wiki/Systems_modeling_language).
- API: Systems Modeling API and Services 1.0 with a platform-independent model and machine-readable files — [OMG spec page](https://www.omg.org/spec/SystemsModelingAPI).
- Data model: `requirement def` (reusable template) and `requirement` usages that bind values; `doc` comments for prose; `subject` (what the requirement is about); `attribute` parameters with units; `require constraint` (guaranteed) and `assume constraint` (taken for granted) — [Ran Wei SysML v2 tutorial, Module 5](https://wrwei.github.io/tutorials/sysml/SysMLv2_Module5_Requirements_EN.html).
- Relations: `satisfy … by …` (design element meets requirement), `specializes` for flow-down, nesting for hierarchy, `#derivation` connections (one original requirement to one or more derived ones, from the RequirementDerivation library) — [Module 5](https://wrwei.github.io/tutorials/sysml/SysMLv2_Module5_Requirements_EN.html), [Dassault/No Magic SysML v2 docs: derive requirement](https://docs.nomagic.com/SYSML2P/2026x/derive-requirement-254423094.html). Graphical notation for derivation is still an open OMG issue — [OMG issue SYSML21-655](https://issues.omg.org/issues/SYSML21-655).
- Verification: `verification def` with `subject`, an `objective { verify requirement … }` and a `return verdict : VerdictKind` (pass/fail/…); verification is a V&V-time assertion distinct from design-time `satisfy` — [Module 9](https://wrwei.github.io/tutorials/sysml/SysMLv2_Module9_Verification_EN.html).
- Distinctive ideas: requirement as a constraint over a subject; assume/require split; definition/usage reuse; verification cases with verdicts; standard textual notation plus REST API.

**Eclipse Capella / Arcadia**
- Maker: Eclipse Foundation project (originally Thales); licence not verified here (believed EPL; see Gaps).
- Requirements: via the Requirements Viewpoint add-on, with ReqIF import; users add relation types such as "satisfies" or "is allocated to" — [Capella forum: Requirements VP](https://forum.mbse-capella.org/t/requirements-vp/3758), [forum: ReqIF import datatypes](https://forum.mbse-capella.org/t/requirements-viewpoint-reqif-import-datatypes/3995).
- Known issue: links created after ReqIF import were lost on re-import (reported as a bug) — [Capella forum](https://forum.mbse-capella.org/t/requirements-links-created-after-initial-reqif-import-break-after-reimport/2964).
- Model traceability: Arcadia creates "realization" links between levels (Realized Function/Exchange/Component) and many hidden semantic links (e.g. "Capability Involvements"); a validation function catches missed allocations — [Capella forum: Traceability options](https://forum.mbse-capella.org/t/capella-traceability-options/6926).
- Scripting: capellambse (Python, headless) reads requirements and their traces and exports to Excel or ReqIF — [capellambse ReqIF extension](https://dsd-dbs.github.io/py-capellambse/code/capellambse.extensions.reqif.html); a December 2025 webinar showed Python-generated traceability coverage reports flagging missing links — [Capella webinar 2025-12-11](https://mbse-capella.org/webinar_20251211.html).
- Distinctive ideas: realization links between abstraction levels (operational > system > logical > physical); model validation rules for missing allocations; ReqIF as exchange format.

**Sparx Enterprise Architect**
- Maker: Sparx Systems. Commercial desktop tool (editions).
- Specification Manager gives a Word-processor or spreadsheet view of requirements in a package, with traceability (Traceability window, Relationship Matrix), change management (baselines, auditing), testing and risk — [Sparx user guide: specification tracking (17.2)](https://sparxsystems.com/enterprise_architect_user_guide/17.2/specification_tracking.html), [Specification Manager PDF 16.1](https://sparxsystems.com/resources/user-guides/16.1/modeling/specification-manager.pdf).
- Relationship Matrix shows relations within a specification package or against another package — [specification traceability (v11, older)](https://sparxsystems.com/enterprise_architect_user_guide/11/requirement_models/specification_traceability.html).
- Baselines: any package can be baselined, compared with the current model, stored as compressed XML or exported to a file; baselines/auditing need Corporate or higher editions — [Sparx guide v10 baselines (older)](https://sparxsystems.com/enterprise_architect_user_guide/10/projects_and_teams/baselines.html), [specification tracking 17.2](https://sparxsystems.com/enterprise_architect_user_guide/17.2/specification_tracking.html).
- Derivation guidance exists ("deriving a requirement from another") — [Sparx 17.1 guide](https://www.sparxsystems.com/enterprise_architect_user_guide/17.1/guide_books/deriving_a_requirement_from_another.html).
- Distinctive ideas: document view and model view of the same items; relationship matrix between two packages; package-level baselines diffed against the live model.

**Dassault Cameo / Magic Systems of Systems Architect (Cameo Requirements Modeler)**
- Maker: Dassault Systèmes (No Magic). Commercial.
- Suspect links track changes to requirements linked to design elements by satisfy, derive, refine or verify, and flag unlinked and deprecated requirements — [Cameo Requirements Modeler 2024x docs](https://docs.nomagic.com/x/NS8mC), [2021x docs](https://docs.nomagic.com/x/gYx9B).
- Dependency matrix templates for Derive, Verify, Satisfy and Refine; matrices also remove redundant or create missing dependencies — same sources.
- SysML v2 plugin (2026x) supports `#derivation` in a textual editor — [No Magic SysML v2 docs](https://docs.nomagic.com/SYSML2P/2026x/derive-requirement-254423094.html).
- SysML v1 requirement tables typically show name, id, text and satisfying elements, with optional verify columns (shown for PTC Modeler, a comparable SysML tool) — [PTC Modeler: working with requirements](https://support.ptc.com/help/modeler/r10.1/en/Modeler/sysml/Overview_of_working_with_requirements.html); «deriveReqt» is a stereotyped UML dependency — [PTC Modeler: Derive reqt](https://support.ptc.com/help/modeler/r10.1/en/Modeler/sysml/SysML_Derive_relationship.html).
- Distinctive ideas: suspect links scoped to specific relation kinds; matrix per relation kind; "unlinked requirement" as a flagged state.

**Lightweight text-based modeling: Mermaid requirement diagrams, Structurizr DSL**
- Mermaid `requirementDiagram`: requirement kinds `requirement`, `functionalRequirement`, `interfaceRequirement`, `performanceRequirement`, `physicalRequirement`, `designConstraint`; attributes `id`, `text`, `risk` (Low/Medium/High), `verifymethod` (Analysis/Inspection/Test/Demonstration); `element` with `type` and `docref`; relations `contains`, `copies`, `derives`, `satisfies`, `verifies`, `refines`, `traces` (the SysML v1 set) — [Mermaid docs](https://mermaid.js.org/syntax/requirementDiagram.html). Docs inconsistently write `docref`/`docRef`.
- PlantUML: not researched (see Gaps).
- Structurizr DSL: a `workspace` holds a `model` (elements, relationships written as `a -> b "Uses"`, implied relationships) and separate `views` that include/exclude elements by expression; the separation of content and presentation improves diffs; the CLI exports to PlantUML, Mermaid or a static HTML site — [Structurizr DSL language reference](https://docs.structurizr.com/dsl/language), [tutorial](https://docs.structurizr.com/dsl/tutorial), [Why as code](https://docs.structurizr.com/as-code).
- Distinctive ideas: one model, many views; view definitions as code; export to several diagram dialects.

### Inferences
- Backlog tools are strong at hierarchy (epic > story > task) and at links to commits, builds and releases, but weak at typed requirement relations. MBSE tools are the reverse.
- The clearest overlap between both worlds is "verification status per requirement": Xray (per version/environment), GitLab (CI artifact), SysML v2 (verdict), Cameo (verify relation + suspect). requimd already has `ver` links and coverage, so result import is the natural next step.
- SysML v2 is the first MBSE standard with a normative textual syntax; it validates the text-first approach but its constraint/parameter semantics target engineers, not business analysts.

### Gaps
- Confluence requirements blueprint, Jira native acceptance-criteria handling and Zephyr were not researched within budget.
- Productboard and Linear Documents/specs were not researched from primary sources; Linear and Aha! findings partly rely on aggregator or competitor content.
- Licences/pricing for most commercial tools, and Capella's licence (believed EPL-2.0), were not verified.
- Azure DevOps Acceptance Criteria field and CMMI requirement fields: the guidance page returned 404; not verified.
- GitLab test case relation to requirements: the docs page did not describe it.
- PlantUML requirement support was not checked.

## Which relationship kinds do these tools define, and which matter most in practice?

### Takeaway
SysML v1's set (containment, derive, refine, satisfy, verify, trace, copy) is the de facto vocabulary, reproduced in Mermaid and in Cameo/Sparx/PTC. SysML v2 reduces it to satisfy, verify, derivation connections, specialization and nesting. Backlog tools use generic relations (parent/child, related, predecessor, duplicate, tested by, affects). Tool features (suspect links, matrices, coverage) are built around four kinds: derive, refine, satisfy, verify.

### Cited Findings
- Mermaid relation set: contains, copies, derives, satisfies, verifies, refines, traces — [Mermaid docs](https://mermaid.js.org/syntax/requirementDiagram.html).
- Cameo builds suspect links and matrix templates only for satisfy, derive, refine and verify — [Cameo Requirements Modeler 2024x](https://docs.nomagic.com/x/NS8mC).
- SysML v2: `satisfy … by`, `verify requirement` in verification objectives, `specializes` for flow-down, nesting, `#derivation` connections from one original to many derived requirements — [Module 5](https://wrwei.github.io/tutorials/sysml/SysMLv2_Module5_Requirements_EN.html), [Module 9](https://wrwei.github.io/tutorials/sysml/SysMLv2_Module9_Verification_EN.html), [No Magic SysML v2](https://docs.nomagic.com/SYSML2P/2026x/derive-requirement-254423094.html). The tutorial claims there is no separate `derive` keyword; derivation exists as a library connection — the two sources describe different mechanisms, not a contradiction.
- Azure DevOps: Parent/Child (tree), Related (network), Predecessor/Successor (dependency), Duplicate, Tested By/Tests, Affects/Affected By (CMMI change request to requirement), plus links to commits/PRs/builds/releases; link types declare `acyclic`, `directional`, `singleTarget`, `topology` — [Azure DevOps link types](https://learn.microsoft.com/en-us/azure/devops/boards/queries/link-type-reference).
- Capella: "satisfies" and "is allocated to" as user-defined ReqIF relation types; Arcadia realization links between levels — [Capella forum](https://forum.mbse-capella.org/t/requirements-vp/3758), [forum: traceability options](https://forum.mbse-capella.org/t/capella-traceability-options/6926).
- R4J/RTM/Xray reports are built on hierarchy plus requirement-test-defect links — [R4J docs](https://easesolutions.atlassian.net/wiki/x/RwCrh), [RTM coverage](https://deviniti.com/blog/application-lifecycle-management/report-requirement-test-coverage-jira/), [Xray coverage](https://confluence.getxray.app/display/XRAY35/Understanding+coverage+and+the+calculation+of+Test+and+requirement+statuses).

### Inferences
- In practice, the relations that drive features are: hierarchy (containment/parent), derive (requirement to requirement across levels), satisfy (design to requirement), verify (test to requirement), and a generic trace/related. Refine (use case or model element refining a requirement) is used less but is the natural link from UC to FR.
- "Conflict" is not a standard relation in any of the sources found; Azure's "Affects" (change request to requirement) is the nearest ALM-specific relation and fits requimd's diff/suspect work.
- Link-type constraints (acyclic, single target, allowed source/target kinds) are a cheap way to give a text tool model-like rigor.

### Gaps
- No usage statistics were found on which relation kinds practitioners actually use most; the "most important" ranking above is inferred from which kinds tools build features on.
- SysML v2 "refinement" syntax for requirements was not found in the sources fetched.

## How do these tools handle acceptance criteria, Definition of Done, and the link from requirement to release/version?

### Takeaway
Backlog tools treat acceptance criteria as a text field (or custom field) on the story; no source showed AC as individually identified, linkable items. The link to releases is via version fields (Jira Fix Version, used by Xray to compute status per version) or via pipeline links (Azure "Integrated in release environment"). MBSE tools replace acceptance criteria with verification cases and constraints with verdicts. Definition of Done was not represented as data in any source found.

### Cited Findings
- Aha!: acceptance criteria held in custom fields (e.g. Note field) on requirements; AI generates AC from the feature definition — [Aha! blog](https://www.aha.io/blog/requirements-custom-fields), [Aha! AI library](https://www.aha.io/ai-library/request-acceptance-criteria).
- Linear: teams tailor custom issue types/fields/statuses to acceptance criteria (review source) — [ONES guide](https://ones.com/blog/?p=1170409).
- Xray: requirement status per Fix Version (versioned test executions or versioned test sets) and per test environment — [Xray coverage calculation](https://confluence.getxray.app/display/XRAY35/Understanding+coverage+and+the+calculation+of+Test+and+requirement+statuses).
- Azure DevOps: "Integrated in build" and "Integrated in release environment" links, the latter created automatically when a release definition reports deployment status to work items — [Azure link types](https://learn.microsoft.com/en-us/azure/devops/boards/queries/link-type-reference).
- GitLab: requirement satisfied/failed via a CI artifact (`requirements.json`) — [GitLab requirements](https://docs.gitlab.com/user/project/requirements/).
- R4J: baselines as snapshots for milestones, releases or reviews — [R4J docs](https://easesolutions.atlassian.net/wiki/x/RwCrh).
- SysML v2: verification cases with objectives and `return verdict`; `require constraint` expresses a checkable condition — [Module 9](https://wrwei.github.io/tutorials/sysml/SysMLv2_Module9_Verification_EN.html), [Module 5](https://wrwei.github.io/tutorials/sysml/SysMLv2_Module5_Requirements_EN.html).
- Mermaid's `verifymethod` (Analysis/Inspection/Test/Demonstration) records how a requirement is to be verified — [Mermaid docs](https://mermaid.js.org/syntax/requirementDiagram.html).

### Inferences
- An opportunity for requimd: acceptance criteria as numbered sub-elements of an item (like use case step labels), so that TC items can `ver`-link to a single criterion and coverage can be reported per criterion. None of the scanned tools does this natively.
- Release linkage in requimd fits as git tags/refs plus an optional `release:` metadata value; "status by release" can be computed from git history without a database.
- Definition of Done is team-level policy, not item data; a checklist file plus a lint rule ("item in status X must have ver link") is the text-tool equivalent.

### Gaps
- Jira's and Azure DevOps' native acceptance-criteria fields were not verified from primary docs in this scan.
- No source described Definition of Done as structured data in any of these tools.

## What can a text-first tool learn from SysML v2's textual notation and requirement semantics?

### Takeaway
SysML v2 shows that requirements can be standard, diffable text with formal semantics: a requirement has a subject, assumptions and required constraints, optional parameters with units, prose `doc`, and typed relations (satisfy, verify, derivation). Verification is a first-class case with an objective and verdict. A text-first tool can adopt the vocabulary and structure (subject, assume/require, satisfy/verify/derive, verification method and verdict) without adopting the full constraint language.

### Cited Findings
- `requirement def` vs `requirement` usage with `:>>` value binding; `subject vehicle : Vehicle;` recommended on every requirement; `doc /* … */` for prose — [Module 5](https://wrwei.github.io/tutorials/sysml/SysMLv2_Module5_Requirements_EN.html).
- `assume constraint` (not verified by the system) vs `require constraint` (guaranteed only when assumptions hold) — same source.
- `assert satisfy brakingReq by brakes;` (design-time) vs `verify requirement` in a verification objective (V&V-time) — [Module 5](https://wrwei.github.io/tutorials/sysml/SysMLv2_Module5_Requirements_EN.html), [Module 9](https://wrwei.github.io/tutorials/sysml/SysMLv2_Module9_Verification_EN.html).
- Verification cases compose (`first test3; then test2; …`) and can embed analyses compared against thresholds — [Module 9](https://wrwei.github.io/tutorials/sysml/SysMLv2_Module9_Verification_EN.html).
- `#derivation` connection from the RequirementDerivation library — [No Magic SysML v2 docs](https://docs.nomagic.com/SYSML2P/2026x/derive-requirement-254423094.html).
- Standard API (Systems Modeling API and Services 1.0) adopted with SysML v2 — [OMG](https://www.omg.org/spec/SystemsModelingAPI), [OMG press release](https://www.omg.org/news/releases/pr2025/07-21-25.htm).
- Caution: the spec's graphical notation for shorthand usages like satisfy/verify is reported as inconsistent in open OMG issues — [OMG issue SYSML2_-256](https://issues.omg.org/issues/SYSML2_-256) (as summarized by search), [SYSML21-655](https://issues.omg.org/issues/SYSML21-655).

### Inferences
- requimd's typed links already mirror SysML's relation idea; adopting SysML names (`sat`, `ver`, `der`, `ref`) and their direction conventions would make a later SysML v2 export straightforward.
- "Subject" maps to requimd's planned design level: a requirement names the system/component it constrains; design items `sat`-link back. This gives the design level a standard-backed shape.
- Assume/require is useful for analysts in plain language: an "Assumptions" metadata line or section, linted for weak words, separates context from the obligation.
- Parameters with units and executable constraints are low value for business analysts and costly to implement; a light form (named values like `max_response_time = 2 s` referenced in text) may still help consistency checks.
- A one-way export to SysML v2 textual notation (requirement usages with `doc`, `satisfy`, `verify`) would let requimd feed MBSE tools without becoming one.

### Gaps
- The official OMG spec text and pilot implementation library files were not fetched; syntax comes from a tutorial and vendor docs. Exact keyword forms (e.g. `verification def` vs older `verification case def`) should be checked against formal/2025-09-03.
- Requirement `concern`/stakeholder and `refinement` syntax were not covered by the fetched sources.

## Which reporting views (dashboards, coverage, status by release) do teams rely on?

### Takeaway
The recurring views are: traceability matrix between two item kinds, multi-level coverage chain, requirement test coverage (covered/uncovered/passed/failed) per version and environment, baseline comparison, and model-validation reports of missing links. Dashboards in ALM tools are mostly built from these.

### Cited Findings
- R4J: 2D traceability matrix between item types plus coverage view down the hierarchy — [R4J docs](https://easesolutions.atlassian.net/wiki/x/RwCrh).
- RTM: traceability report, coverage report (requirement covered or uncovered by test cases, plans, executions, defects), test execution summary — [Deviniti blog](https://deviniti.com/blog/application-lifecycle-management/report-requirement-test-coverage-jira/), [RTM docs](https://deviniti.com/support/addon/cloud/requirements-test-management/latest/traceability/).
- Xray: Overall Requirement Coverage (stacked bars by version or test plan, environment, grouping field) — [Xray report](https://confluence.getxray.app/display/XRAY/Overall+Requirement+Coverage+Report).
- Azure DevOps: Backlog/Stories/Requirements Overview reports depend on parent-child links; on-premises SQL reports depend on Tested By links — [Azure link types](https://learn.microsoft.com/en-us/azure/devops/boards/queries/link-type-reference).
- Sparx: Relationship Matrix; baseline comparison against the current model — [Sparx 17.2](https://sparxsystems.com/enterprise_architect_user_guide/17.2/specification_tracking.html).
- Cameo: dependency matrices per relation kind; suspect and unlinked requirement lists — [Cameo 2024x](https://docs.nomagic.com/x/NS8mC).
- Capella: Python-generated traceability coverage reports flagging missing links (webinar Dec 2025) — [Capella webinar](https://mbse-capella.org/webinar_20251211.html).
- GitLab: list filter by satisfied/failed/missing — [GitLab docs](https://docs.gitlab.com/user/project/requirements/).

### Inferences
- requimd already covers matrix (CSV), coverage, diff and suspect links. The missing views are: verification status (pass/fail) per item and per release, a matrix filtered by relation kind, and a baseline report between two named tags.
- All of these can be static HTML generated by the CLI; no live dashboard is needed.

### Gaps
- No survey data on which reports teams use most; the list reflects what vendors document.

## Why do analysts move from backlog tools to dedicated RM tools, or the reverse?

### Takeaway
Analysts leave Jira-style backlogs mainly for missing traceability and missing document context (requirements read in isolation, no baselines, no matrix). Teams stay or return because backlog tools are where developers work and a separate RM tool creates a "truth gap". Most sources are RM vendors, so the evidence is biased.

### Cited Findings
- Jama: Jira excels at task management but alone falls short for requirements management — [Jama Software](https://www.jamasoftware.com/blog/leveraging-jama-connect-and-jira-for-enhanced-requirements/) (vendor).
- Perforce: no traceability in Jira without an add-on or integrated tool — [Perforce](https://perforce.com/node/1162136) (vendor).
- Atlassian community: without document-like views "the DNA is scattered"; flat issue lists make engineers read requirements in isolation and miss constraints defined higher up; separating RM from development creates a "truth gap" — [Jira and Requirements – The Missing Piece](https://community.atlassian.com/forums/App-Central-articles/Jira-and-Requirements-The-Missing-Piece/ba-p/3116387), [Spreadsheets vs enterprise giants](https://community.atlassian.com/forums/App-Central-articles/Spreadsheets-vs-enterprise-giants-Finding-the-middle-ground-for/ba-p/3197171) (Marketplace partner articles).
- Academic: Jira is widely adopted but its native functionality does not fully address the RM lifecycle (Rozenberga and Kirikova, 2025) — [CEUR-WS paper](https://ceur-ws.org/Vol-4171/paper_69.pdf).
- R4J users report performance problems at thousands of requirements — [Capterra](https://www.capterra.ca/reviews/164178/requirements-management-for-jira-r4j).
- Capella users report broken links after ReqIF re-import, a typical cost of moving requirements between tools — [Capella forum](https://forum.mbse-capella.org/t/requirements-links-created-after-initial-reqif-import-break-after-reimport/2964).

### Inferences
- requimd sits between both camps: documents (Markdown) with item-level IDs and links, living next to code in git. Its pitch against Jira is document context plus traceability; against heavy RM tools it is the developer workflow (PRs, diffs) without a sync gap.
- Round-tripping through exchange formats (ReqIF, CSV) is fragile; stable IDs in text avoid the re-import link loss seen in Capella.

### Gaps
- Could not fetch the article "Why Requirements Engineers Struggle with Jira" (only the title loaded): [link](https://community.atlassian.com/forums/App-Central-articles/Why-Requirements-Engineers-Struggle-with-Jira-and-What-the/ba-p/3230575).
- No neutral (non-vendor) user survey on migrations was found.

## Ideas for requimd

### Takeaway
The highest-value, constraint-compatible ideas are: a SysML-aligned relation vocabulary with declared constraints, verification results imported from files (status per item and per release), acceptance criteria as addressable sub-elements, a `subject`/`sat` link to design items for the planned design level, and relation-kind-aware suspect links and matrices. Anything requiring a web app, database or live sync breaks requimd's constraints.

### Cited Findings
- Sources for each idea are in the sections above; the "seen in" column names the tool.

### Inferences

| Idea | Seen in | Problem it solves for an analyst | Fit | Value | Effort |
|---|---|---|---|---|---|
| Fixed relation vocabulary aligned with SysML (contain, derive, refine, satisfy, verify, trace, copy), with direction and inverse names shown in hover | SysML v1/v2, Mermaid, Cameo, Sparx | Clear meaning of each link; easier exchange with MBSE teams | fits | high | S |
| Declared link-type constraints in config: acyclic, single target, allowed source/target item kinds | Azure DevOps link type attributes | Catches wrong links (e.g. TC deriving an FR, cycles in derive) at check time | fits | high | M |
| Import verification results from a file (e.g. JUnit XML or a `results.json` keyed by TC/FR ID) to set passed/failed per item | GitLab `requirements.json`, Xray, SysML v2 verdicts | Shows which requirements are actually verified, not just linked | fits (local file, no network) | high | M |
| Status and coverage by release, computed from git tags/refs (and an optional `release:` metadata value) | Xray per Fix Version, Azure "Integrated in release environment", R4J baselines | Answers "what was in release 2.3 and was it verified?" | fits | high | M |
| Acceptance criteria as labelled sub-elements (like step labels) that TC items can `ver`-link individually | Gap in Aha!/Linear/Jira (AC only as text fields) | Coverage per criterion, not per story; finds untested criteria | fits | high | M |
| `subject` metadata and `sat` links from design items to requirements | SysML v2 subject and satisfy, Capella realization, Cameo | Foundation for the design level; shows which component meets which requirement | fits | high | M |
| Assumptions separate from the requirement statement (`assume` section or metadata), linted like requirement text | SysML v2 assume/require constraints | Makes hidden preconditions visible in reviews | fits | medium | S |
| Verification method metadata (Analysis, Inspection, Test, Demonstration) with coverage split by method | Mermaid `verifymethod`, SysML | Plans verification for items that cannot be tested automatically | fits | medium | S |
| Suspect-link policy per relation kind (e.g. only derive/satisfy/verify become suspect) | Cameo suspect links | Fewer false alarms after edits to loosely related items | fits | medium | S |
| Matrix per relation kind (derive matrix, verify matrix) with "missing" highlighting in the site | Cameo dependency matrices, Sparx Relationship Matrix, R4J | Spot gaps between two levels at a glance | fits | medium | S |
| Named baselines (git tag + label) and a baseline-to-baseline report | R4J, Sparx package baselines | Review sign-off and audit snapshots without a database | fits (builds on existing diff) | medium | S |
| Change request items linked with `affects` to requirements, with diff showing the requirement changes per CR | Azure DevOps CMMI Affects link | Impact analysis per change request | fits | medium | S |
| View definitions as code (include/exclude by kind, tag, relation) driving site pages and Mermaid diagrams | Structurizr DSL views | Tailored views per audience (business, test, design) from one source | fits | medium | M |
| One-way export to SysML v2 textual notation (requirement usages with `doc`, `satisfy`, `verify`, `#derivation`) | SysML v2, Systems Modeling API | Hand-over to MBSE/systems engineering teams | fits (export only) | low-medium | M |
| ReqIF import/export | Capella Requirements VP, capellambse, Sparx | Exchange with DOORS/Polarion/Capella users | fits (file-based); round-trip fragility | medium | L |
| CSV import/export for issue trackers (title, description, ID, status) | GitLab requirements CSV | Bulk move items from Jira/GitLab into requimd or back | fits | medium | S |
| Item kinds with their own required metadata (schema per kind) | GitHub issue fields pinned to issue types | Ensures e.g. every NFR has a measure and every TC a method | fits (if not already present) | medium | S |
| Parameters with units referenced in requirement text (`max_response = 2 s`) and checked for consistency | SysML v2 attributes and constraints | One place to change a number used in many requirements | fits | low-medium | M |
| Validation report of missing allocations/realizations between levels (UC > FR > design) | Capella validation, Capella Python coverage reports | Shows unrealized requirements per level | fits (extension of coverage) | medium | S |
| Live dashboard with per-user views and comments | Jira/Azure dashboards, Aha! | Team-wide status at a glance | breaks: web app with user management, database storage | low | L |
| Two-way live sync with Jira/Azure DevOps/GitHub issues | R4J, Jama-Jira integrations | Requirements and backlog stay aligned | breaks: no network at runtime (NFR-006, except explicit request), single source of truth | medium | L |
| Graphical requirement/model editor | Cameo, Capella, Sparx | Diagram-first modelling | breaks: graphical editor out of scope | low | L |

### Gaps
- Value and effort ratings are judgements based on the cited tool behavior and the requimd context given, not measured data.
- Whether requimd already supports per-kind required metadata, parent/child hierarchy or named baselines was not checked against its code in this scan.
