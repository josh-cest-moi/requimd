# AI-assisted requirements engineering and spec-driven development: a scan for requimd

Scan date: 18 November 2026. These notes cover 17 web searches and fetches. Most spec-driven tool details come from secondary write-ups, because the official GitHub READMEs and the kiro.dev pages for steering and hooks were not opened. The key primary sources read in full were Böckeler's Fowler article (Oct 2025, now more than a year old), the Jama OpenSpec guide (Jul 2026), Birk's tool survey (Feb 2026) and Hey and Frattini's TLR paper (Jun 2026). Vendor AI claims are marketing unless stated otherwise.

## What do spec-driven development (SDD) tools do that a requirements tool for analysts could offer?

### Takeaway
SDD tools share one structure. Markdown files for requirements, design and tasks sit in the repo, next to a standing "memory bank" (constitution, steering, AGENTS.md), and slash commands make an agent move from each level to the next. The parts worth borrowing are the mechanical ones:
- requirement-to-task coverage cross-checks (spec-kit `/analyze`)
- explicit change deltas that merge into a living spec (OpenSpec)
- a project-rules file that agents read
- machine-readable context for agents

Critics say the generated Markdown is verbose, agents ignore the specs anyway, and most tools are "spec-first" rather than keeping specs alive.

### Cited Findings

**Framing and critique (Böckeler, Thoughtworks, 15 Oct 2025)**
- Böckeler defines three levels:
  - **spec-first:** the spec is written before the task and may be thrown away afterwards.
  - **spec-anchored:** the spec is kept and used to evolve the feature.
  - **spec-as-source:** humans edit only the spec, never the code.
  
  All the tools she found are spec-first. Only Tessl aims at spec-anchored or spec-as-source. — [Böckeler, martinfowler.com](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)
- She separates *specs* (scoped to the task that creates or changes a feature) from the *memory bank*: cross-session context such as rules files and product or architecture overviews. — [Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)
- Her criticisms:
  - Kiro turned a small bug into 4 user stories with 16 acceptance criteria ("like using a sledgehammer to crack a nut").
  - spec-kit produced many verbose, repetitive files ("I'd rather review code than all these markdown files").
  - Agents ignored instructions, or over-applied constitution articles.
  - Research notes were treated as new specs, which produced duplicate classes.
  - It is unclear whether developers or product people are meant to write the specs.
  - SDD risks combining model-driven development's "inflexibility *and* non-determinism". Parseable model-driven-development specs at least gave validity and consistency checking, which natural language loses.
  
  — [Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)
- Even per-file Tessl specs produced different code across runs. Making specs more specific improved repeatability, which shows how hard unambiguous specification is. — [Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)

#### Profile: GitHub Spec Kit
**Maker:** GitHub. Licence: open source; MIT is assumed and was not verified in this scan.

**What it is:** a CLI that sets up workspace files and slash commands for many coding agents. Templates and scripts live under `.specify/`, in the subfolders `memory/`, `scripts/` and `templates/` ([Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)).

**Workflow:** Constitution → Specify → Plan → Tasks → Implement. Optional quality gates are `clarify`, `checklist` and `analyze` ([developertoolkit.ai](https://developertoolkit.ai/en/ecosystem/frameworks/spec-kit/); [matsen walkthrough, Feb 2026](https://matsen.fhcrc.org/general/2026/02/10/spec-kit-walkthrough.html)).
- The constitution (`.specify/memory/constitution.md`) holds immutable project principles. Every spec, plan and task is checked against it ([fundesk.io](https://www.fundesk.io/spec-driven-development-github-spec-kit-guide); [Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)).

**Structure:** one spec is many files: spec, plan, tasks, research, data-model, api and component. Each spec gets its own branch, so its lifetime follows a change request rather than a feature ([Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)).

**Linking levels:**
- `/speckit.analyze` is a read-only cross-check of spec.md, plan.md and tasks.md. It reports requirements with no task, tasks with no requirement, and conflicts with the constitution ([developertoolkit.ai](https://developertoolkit.ai/en/ecosystem/frameworks/spec-kit/)).
- Checklists act as "unit tests for requirements quality", not as tests of the code ([sam-solutions](https://sam-solutions.com/blog/spec-driven-development-github-spec-kit/)).

**Caveat:** command names changed during 2026, so early-2026 tutorials no longer work ([vibecoding.app review](https://vibecoding.app/blog/spec-kit-review)).

**Notable for requimd:** the analyze-style coverage cross-check, and the constitution as a named, checkable rules file.

#### Profile: AWS Kiro
**Maker:** AWS. Licence: proprietary, assumed and not verified.

**What it is:** an agentic IDE based on Code OSS, plus a CLI ([Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)).

**Spec files:** each feature spec has three files ([kiro.dev specs](https://kiro.dev/docs/specs/); [dev.to](https://dev.to/jubinsoni/aws-kiro-the-agentic-ide-that-makes-specs-the-unit-of-work-3eko)):
- `requirements.md`: user stories with acceptance criteria in EARS-style "WHEN … THE SYSTEM SHALL …". Böckeler saw GIVEN/WHEN/THEN.
- `design.md`: architecture, data flow, data models, error handling and testing strategy.
- `tasks.md`: a checklist traced to requirement numbers.

**Workflow variants:** Requirements-first, Design-first, and a "Quick Spec" that produces all three files in one pass without approval gates ([kiro.dev feature specs](https://kiro.dev/docs/specs/feature-specs/)).

**Sync and task execution:**
- After you edit requirements.md or design.md, "Sync Files" on tasks.md creates tasks that map to the new requirements ([kiro.dev best practices](https://kiro.dev/docs/specs/best-practices/)).
- You start a task by clicking it, and its status updates automatically ([kiro.dev first project](https://kiro.dev/docs/getting-started/first-project/)).

**Steering:** files in `.kiro/steering/`, by default product.md, tech.md and structure.md, give durable project context ([Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html); [dev.to](https://dev.to/jubinsoni/aws-kiro-the-agentic-ide-that-makes-specs-the-unit-of-work-3eko)).

**Hooks:** event automation on file save or commit, for example to update tests or docs. This is from secondary sources only ([dev.to](https://dev.to/jubinsoni/aws-kiro-the-agentic-ide-that-makes-specs-the-unit-of-work-3eko)).

**Notable for requimd:** EARS as the default requirement form, and tasks traced to requirement numbers.

#### Profile: Tessl
**Maker:** Tessl. In Oct 2025 the framework was in private beta. The CLI creates `.tessl/framework` and doubles as an MCP server ([Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)).

**Spec files:** single `.spec.md` files with tags such as `@generate` and `@test`, plus an API section that fixes the interfaces other code may use.

**Generated code:** generated files carry the header "GENERATED FROM SPEC - DO NOT EDIT". The mapping was one spec to one file.

**Commands:** `tessl document --code` reverse-engineers a spec from existing JavaScript, and `tessl build` generates code from a spec. The memory bank includes KNOWLEDGE.md and AGENTS.md ([Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)).

**Registry:** Tessl also runs a registry of "skills" and "tiles", which lists OpenSpec skills among others ([Tessl registry: openspec-propose](https://tessl.io/registry/skills/github/Fission-AI/OpenSpec/openspec-propose)).

**Notable for requimd:**
- It is the only one aiming at spec-as-source.
- It reverse-engineers specs from code.
- The CLI is itself an MCP server.

Tessl's state as of Nov 2026 was not verified.

#### Profile: OpenSpec
**Maker:** Fission-AI. Licence: MIT. npm package `@fission-ai/openspec` ([search summary of repo and mirrors](https://www.gitlink.org.cn/mirrors/OpenSpec); [newreleases v1.0.0](https://newreleases.io/project/github/Fission-AI/OpenSpec/release/v1.0.0)).

**Structure:** an `openspec/` root with two folders ([Jama blog, 30 Jul 2026](https://www.jamasoftware.com/blog/openspec-guide/)):
- `specs/`: the current source of truth, organised by domain.
- `changes/<name>/`: proposed changes. Each change folder holds `proposal.md` (intent and scope), *delta specs* marked **ADDED / MODIFIED / REMOVED / RENAMED**, `design.md` and `tasks.md`.

**Commands (v1.0):** `/opsx:explore`, `/opsx:propose`, `/opsx:apply`, `/opsx:sync` and `/opsx:archive`. An extended profile adds `/opsx:new`, `/opsx:continue`, `/opsx:verify` and `/opsx:bulk-archive`. The old `/openspec:*` commands were removed ([Jama blog](https://www.jamasoftware.com/blog/openspec-guide/); [newreleases](https://newreleases.io/project/github/Fission-AI/OpenSpec/release/v1.0.0)).

**Archive and sync:**
- Archiving checks that artifacts and tasks are complete. It then offers to merge the deltas into `specs/` and moves the folder to `changes/archive/YYYY-MM-DD-<name>/` ([Jama blog](https://www.jamasoftware.com/blog/openspec-guide/); [codemyspec](https://codemyspec.com/blog/openspec-explained)).
- `sync` merges the deltas without archiving the change ([newreleases](https://newreleases.io/project/github/Fission-AI/OpenSpec/release/v1.0.0)).

**Notable for requimd:**
- It is the clearest "spec-anchored" model: a living spec plus reviewable deltas.
- Jama positions OpenSpec as a *planning layer*. In Jama's view, proposals and deltas "are not a governed requirements record", and the RM platform stays the system of record ([Jama blog](https://www.jamasoftware.com/blog/openspec-guide/)).

#### Profile: BMAD Method
**Maker:** the bmad-code-org community, on GitHub. Licence: not verified.

**What it is:** agile roles packaged as agents: analyst, PM, architect, Scrum Master, developer and QA. Newer versions install them as skills in Claude Code, Codex and Cursor ([developertoolkit.ai](https://developertoolkit.ai/en/ecosystem/frameworks/bmad-method/)).

**Workflow:** analyst brief → PM agent writes the PRD → architect agent writes the architecture from the PRD → Scrum Master agent breaks both into "context-embedded" story files (`{epicNum}.{storyNum}.{storyTitle}.md`) → developer agents implement each story in a fresh chat ([GitHub discussion #113](https://github.com/bmad-code-org/BMAD-METHOD/discussions/113); [augmentcode](https://www.augmentcode.com/guides/bmad-method-ai-development)).

**Purpose:** preserving decisions in PRD, architecture and story files counters cross-session "drift" ([augmentcode](https://www.augmentcode.com/guides/bmad-method-ai-development)).

**Notable for requimd:**
- An explicit *analyst* role sits upstream of the coding agents.
- Stories carry embedded context, which is a form of context export.

**Caveat:** agent names and templates differ between versions.

#### Profile: AGENTS.md (with CLAUDE.md and similar conventions)
**Steward:** AGENTS.md is a plain-Markdown file of agent instructions with no required fields. It is stewarded by the Agentic AI Foundation (AAIF) under the Linux Foundation, founded 9 Dec 2025 alongside MCP and goose. The AGENTS.md site claims use in more than 60k open-source projects ([agents.md](https://agents.md/); [OpenAI announcement](https://openai.com/index/agentic-ai-foundation/); [The Register](https://www.theregister.com/2025/12/09/linux_foundation_agentic_ai_foundation/)).

**Role:** these files are the "memory bank" or steering level of SDD ([Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)).

**Notable for requimd:** a requimd project could ship or generate an AGENTS.md section that explains its own format and rules to any agent.

#### Agent OS (Builder Methods)
No sources were found in this scan, so there is no profile. See Gaps.

### Inferences
- **The level structure maps onto requimd.** The common SDD structure is requirements, design, tasks and code, plus a rules or memory layer. requimd already covers the requirements level with typed links, and its planned design level corresponds to Kiro's design.md and spec-kit's plan.md.
- **requimd's unique asset is determinism.** Böckeler's central complaint is that natural-language specs lose the validity and consistency checking that parseable model-driven-development specs had. requimd restores exactly that (link checks, suspect links, language rules) without needing an LLM.
- **requimd is already "spec-anchored".** Items persist, and git plus the lock file give change tracking. OpenSpec-style deltas largely coincide with `requimd diff` between git refs. The missing piece is presenting a diff as a *change proposal* that a reviewer can approve.
- **Analysts can use the coverage-gap idea now.** spec-kit's `/analyze` (requirements without tasks, tasks without requirements) is a deterministic check that requimd's coverage report could extend to design items, task items or code references.

### Gaps
- **Not opened in this scan:** the official spec-kit README and docs, kiro.dev steering and hooks pages, the OpenSpec repo README and the Tessl docs. Some facts come from secondary blogs, and licences (spec-kit, BMAD, Kiro) were not verified.
- **Agent OS (Builder Methods):** no results. It reportedly uses spec folders with standards files, but that is unverified background knowledge, so I do not report it as a finding.
- **Tessl's 2026 status:** whether it is GA and whether it still does spec-as-source was not checked.
- **Long-term evidence:** none was found that SDD specs stay in sync with code over months. Böckeler notes the lack of long-term usage reports.

## Which AI tasks do requirements management (RM) tools actually perform, and how are results presented?

### Takeaway
Commercial RM tools mostly offer the following:
1. Rule-based or NLP quality scoring against INCOSE and EARS, with suggested rewrites. This is the oldest and most mature feature, and much of it is "traditional NLP", not LLMs.
2. LLM generation of test cases and acceptance criteria from requirements.
3. More recently, MCP servers that expose the RM repository to agents.

Independent evidence of usefulness is almost absent. A Nov 2025 survey of 21 RM tool homepages found that 11 did not mention AI at all.

### Cited Findings

**Survey of RM tool AI (Birk, Feb 2026)**
- Andreas Birk reviewed 21 RM tool homepages as of Nov 2025. 11 did not mention AI, 6 gave it medium emphasis and 4 high emphasis ([makingofsoftware.com, 18 Feb 2026](https://makingofsoftware.com/2026/02/ai-support-in-requirements-tools/)). His observations:
  - Most AI in RM tools is "traditional NLP" quality assurance that has existed for a long time.
  - GenAI in RE is "still in its early stages" and lags behind testing and project management.
  - The challenges he names are shadow AI and IT, privacy and security, and the expertise needed to self-host models.

**Vendor tools**

- **Jama Connect Advisor (Jama Software, commercial add-on for Jama Connect Cloud)**
  - NLP scores each requirement against INCOSE rules and/or EARS notation, with the user choosing the rule set. It gives recommendations while authoring, batch analysis and downloadable reports ([Jama datasheet](https://www.jamasoftware.com/datasheet/jama-connect-advisor); [Jama Advisor page](https://www.jamasoftware.com/platform/jama-connect/jama-connect-advisor/)).
  - Jama also claims generated revisions and test cases derived from requirements ([Jama AI page](https://www.jamasoftware.com/solutions/artificial-intelligence/)).
  - Birk reads Advisor as mainly traditional NLP ([Birk](https://makingofsoftware.com/2026/02/ai-support-in-requirements-tools/)).
  - Jama's July 2026 positioning: OpenSpec and agents act as the planning layer, while Jama is the governed record. The pieces are its Traceability Information Model, an MCP server, suspect flagging of downstream items affected by AI-assisted changes, baselines and e-signatures ([Jama OpenSpec blog](https://www.jamasoftware.com/blog/openspec-guide/)).
- **QVscribe (QRA Corp)**
  - Continuous assessment of requirements against domain standards. The source is QRA's own blog; no independent detail was found ([qracorp.com](https://qracorp.com/requirement-software-tools-comparison/)). See Gaps.
- **Copilot4DevOps (Modern Requirements, an Azure DevOps extension)**
  - Generates test cases with steps and expected results, and test data scenarios, from approved requirements or user stories. Execution stays in Azure Test Plans. The vendor claims "up to 60% faster" test case creation ([copilot4devops.com](https://copilot4devops.com/ai-test-case-generation-how-to/); [Microsoft Marketplace](https://marketplace.microsoft.com/en-us/product/web-apps/2101440ontarioinc.copilot4devops_official?tab=overview); [product page](https://copilot4devops.com/product/)).
- **IBM Requirements Quality Assistant / Engineering AI Hub**
  - RQA 3.1 (2023) used Watson to detect 10 quality issues based on INCOSE guidance ([Jazz blog, Mar 2023](https://jazz.net/blog/index.php/2023/03/13/rqa-3-1-improve-requirements-with-ibm-watson/)).
  - A reseller page says the original RQA product was discontinued ([requisis](https://requisis.com/en/saas-doors-dng/rqa-saas.html)).
  - In 2025 IBM published a DIY "Requirements AI assistant for DOORS Next" demo on watsonx.ai, with an example repo ([Jazz blog, Feb 2025](https://jazz.net/blog/index.php/2025/02/14/try-it-yourself-requirements-ai-assistant-for-doors-next/); [IBM/doors-next-ri](https://github.com/IBM/doors-next-ri)).
  - Search results say IBM Engineering AI Hub scores requirements against standards, proposes rewording, lets the user choose the model provider (watsonx.ai, Bedrock, Azure) and adds "governed MCP tools" in v1.4. The underlying page was not opened (see Gaps).
- **PTC Codebeamer AI**
  - A Requirements Assistant detects INCOSE-aligned quality issues and rewrites requirements. A Test Case Assistant derives structured test cases from requirements and project data. The source is a PTC partner, not PTC ([3HTi](https://3hti.com/alm/codebeamer-ai-capabilities/)).
  - Jama, a competitor, claims Codebeamer has no official MCP server ([Jama vs Codebeamer](https://www.jamasoftware.com/solutions/why-choose-jama-connect-over-codebeamer/)).
- **Atlassian Rovo**
  - A general assistant for Jira and Confluence, not an RE tool. Birk rates it among the most advanced, citing its semantic model, multi-LLM strategy, MCP integration and transparency ([Birk](https://makingofsoftware.com/2026/02/ai-support-in-requirements-tools/)).
- **Visure and Siemens Polarion:** only generic statements were found ("AI-assisted authoring and quality checks"; "Polarion X adds AI features"). See Gaps ([thenextweb roundup](https://thenextweb.com/news/best-ai-requirements-management-software-2026)).

**Research on LLMs for RE (NLP4RE)**
- An SLR of 74 primary studies (2023–2024) on LLMs for RE finds that most use LLMs for elicitation and validation. Defect detection and classification dominated pre-LLM work ([arXiv 2509.11446, Sep 2025](https://arxiv.org/abs/2509.11446)).
- A prompt-engineering review covers 35 studies (2018–2025, from 867 screened). It names prompt uncertainty and limited controllability as barriers to trustworthy use in RE ([arXiv 2507.07682, Jul 2025](https://arxiv.org/abs/2507.07682)).
- Trace link recovery (TLR) with LLMs:
  - Tracer reaches about 80.9% average recall, but only about 17.8% average precision, with a recall-oriented prompt. Adding self-correction raises precision but cuts recall to 66.5% ([arXiv 2611.09162](https://arxiv.org/html/2611.09162)).
  - R2Code reports F1 0.73 (precision 0.76, recall 0.71) in its best configuration ([arXiv 2604.22432](https://arxiv.org/html/2604.22432v1)).
  - The numbers vary strongly across datasets and are not directly comparable.
- Hey and Frattini (KIT and Chalmers, Jun 2026) annotated 28 quality factors in 189 use cases (eTour, iTrust) and ran five TLR approaches, including a GPT-4o-based one (LiSSA). Their findings ([arXiv 2606.11834](https://arxiv.org/html/2606.11834)):
  - Scattered requirements, long requirements, anaphora, negatives and sentences that start without a noun phrase hurt TLR.
  - Some textbook "defects" (tangled requirements, alternatives) *helped* it.
  - So quality rules and machine traceability can pull in different directions.

### Inferences
- **Deterministic rules are the core.** The commercially mature "AI" feature (INCOSE/EARS quality scoring with rule-by-rule explanation) is largely deterministic NLP. requimd's existing language checks are in the same category and can be extended without an LLM. The value lies in rule coverage, rule IDs, explanations and a per-item score.
- **Present results like lint output.** Vendors show results as per-item scores, rule hits with recommendations, batch reports and suggested rewrites. An LSP gives the same experience natively through diagnostics, hover and code actions.
- **Generated tests and trace links are suggestions.** LLM test generation and trace link suggestion are where generative AI is used. Precision figures (about 18% at high recall for Tracer) mean a human must accept or reject every suggestion, not apply them automatically.

### Gaps
- **QVscribe:** no primary documentation of its rule set, scoring method or 2026 GenAI features was read.
- **Visure, Polarion and Rovo:** no primary documentation was found on Visure AI, Polarion AI (Polarion X) or a Rovo requirements-specific agent.
- **IBM Engineering AI Hub:** the MCP and v1.4 claim surfaced in a search summary whose source page was not opened, so it needs verification.
- **Evidence of usefulness:** no independent, controlled evidence (user studies, field data) was found on whether quality scoring or AI test generation in commercial RM tools improves outcomes. The vendor percentages are unverified.
- **Unsearched topics:** duplicate or conflict detection and change summarisation were not specifically searched. There is no sourced vendor claim for these.

## Do requirements tools expose MCP servers or agent interfaces?

### Takeaway
Yes, and this is the clearest 2025–2026 trend. Jama has an official MCP server and markets itself as "first". IBM Engineering AI Hub reportedly adds MCP tools. Rovo integrates MCP. Tessl's CLI is an MCP server. Small community MCP servers exist for the git-based open-source tools StrictDoc and Doorstop. No MCP server was found for Polarion, Codebeamer, DOORS or Sphinx-Needs.

### Cited Findings
- **Jama:** an official MCP server lets LLMs "securely access and interact with Jama Connect in real time". Jama claims to be the first engineering management software with an official MCP server ([Jama vs Codebeamer page](https://www.jamasoftware.com/solutions/why-choose-jama-connect-over-codebeamer/); [Jama vs Polarion](https://www.jamasoftware.com/solutions/choosing-jama-connect-over-polarion/)).
- **Jama's use of the server:** it exposes "governed context" (the Traceability Information Model, approved requirements, verification evidence) to coding agents that use OpenSpec ([Jama OpenSpec blog, Jul 2026](https://www.jamasoftware.com/blog/openspec-guide/)).
- **Rovo:** Atlassian Rovo has MCP integration ([Birk](https://makingofsoftware.com/2026/02/ai-support-in-requirements-tools/)).
- **Tessl:** the Tessl CLI "doubles as an MCP server" ([Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)).
- **StrictDoc:** a community server, dapperfu/strictdoc-mcp, exposes StrictDoc's CLI as MCP tools over stdio, with tools only and no resources or prompts ([GitHub](https://github.com/dapperfu/strictdoc-mcp)).
- **Doorstop:** a community server, jannled/doorstopmcp, lets agents create, retrieve and manage YAML requirements in git with parent-child links. It is explicitly not production-ready: no auth, and race conditions under concurrency ([agentseal listing](https://agentseal.org/mcp/https-githubcom-jannled-doorstopmcp)).
- **Governance:** MCP is now stewarded by the Linux Foundation's Agentic AI Foundation together with AGENTS.md ([OpenAI](https://openai.com/index/agentic-ai-foundation/); [The Register](https://www.theregister.com/2025/12/09/linux_foundation_agentic_ai_foundation/)).

### Inferences
- **An MCP server fits requimd's constraints.** A local stdio MCP server needs no network: the agent client launches the binary as a subprocess.
- **Suggested tools:**
  - Read-only tools such as `get_item`, `search`, `trace(up/down)`, `suspect_links`, `check`, `coverage` and `glossary_lookup`.
  - Optionally `propose_item` or `propose_change`, which write files that `requimd check` then validates.

  These would give coding agents traceable, governed context in the same way Jama's server does, but against plain files.
- **Context exported for agents.** "Governed context for agents" is Jama's 2026 pitch. requimd can offer the same thing to small teams for free, from git.

### Gaps
- **Polarion, Codebeamer, DOORS and Sphinx-Needs:** no MCP servers were found. Absence in search results is not proof that none exist.
- **Jama MCP tool list:** the exact tools, resources and permissions were not read.

## Which AI features could requimd offer while staying offline by default?

### Takeaway
The highest-value, lowest-risk ideas are deterministic:
- extended INCOSE/EARS rule scoring
- coverage and gap cross-checks across levels, including code references
- an agent context export
- a local stdio MCP server exposing the requirement graph
- a generated AGENTS.md section

LLM generation (rewrites, tests, trace suggestions) should be opt-in at most. A better route is to emit prompts or context for the user's own agent rather than embedding a model client.

### Cited Findings
- Kiro requires editing requirements and then running "Sync Files" to update tasks ([kiro.dev](https://kiro.dev/docs/specs/best-practices/)), and spec-kit's analyze reports orphans in both directions ([developertoolkit.ai](https://developertoolkit.ai/en/ecosystem/frameworks/spec-kit/)). Both show demand for level-to-level consistency checks.
- Jama's AI story for 2026 combines governance with agent context: suspect flags for AI-induced changes and MCP-exposed traceability ([Jama blog](https://www.jamasoftware.com/blog/openspec-guide/)). requimd already has suspect links (lock file).
- Shadow AI, privacy and the self-hosting expertise gap are named obstacles ([Birk](https://makingofsoftware.com/2026/02/ai-support-in-requirements-tools/)). That supports offline-by-default.

### Inferences — Ideas for requimd

| Idea | Seen in | Problem it solves for an analyst | Fit | Value | Effort |
|---|---|---|---|---|---|
| Local stdio MCP server (`requimd mcp`) with read-only tools: item, search, trace up/down, suspect links, check, coverage, glossary | Jama MCP, Tessl CLI, StrictDoc and Doorstop MCP | Developers' coding agents read the *approved* requirements and links instead of guessing; the analyst's spec becomes the agents' context | Fits (stdio subprocess, no network; Rust MCP over JSON-RPC is similar to the existing LSP plumbing) | High | M |
| MCP write tools: draft a new item or change with the next free ID, validated by `check`; never set `status: approved` | Doorstop MCP, OpenSpec `/opsx:propose` | Agents and developers can propose requirement changes in the correct format; the analyst reviews them as a PR | Fits if writes stay as files in a branch (BR-004 respected) | Medium | M |
| Agent context export (`requimd context FR-012 --depth 2`): item plus linked items, glossary terms used and verification links, as Markdown or JSON | BMAD "context-embedded stories", Jama "governed context" | Gives a coding agent or reviewer a self-contained, traceable brief per feature | Fits | High | S |
| Generated AGENTS.md / CLAUDE.md section describing the requimd format, ID ranges, rules (no renumbering, never approve) and how to run `check` | AGENTS.md (AAIF), Kiro steering, spec-kit constitution | Agents editing the repo follow the analyst's conventions | Fits | Medium | S |
| Cross-level coverage cross-check: requirements with no design or task, design items with no requirement, plus code scanning for `FR-xxx` references (unknown IDs, unimplemented musts) | spec-kit `/analyze`, Kiro tasks traced to requirement numbers | Shows which requirements are not designed, implemented or tested, and where code cites deleted or unknown IDs | Fits (local file scan) | High | M |
| Design level: design items (DD-/ADR) traced to requirements, shown in the matrix and in suspect analysis | Kiro design.md, spec-kit plan.md, BMAD architecture doc | The planned long-term goal; matches the SDD "design" layer | Fits | High | L |
| Change proposal view: render `requimd diff <ref>` as an OpenSpec-like delta (ADDED/MODIFIED/REMOVED/RENAMED per item), with suspect-link impact, for PR review | OpenSpec delta specs, Jama suspect flagging | Reviewers see *what changed in the requirements*, not raw line diffs | Fits (builds on existing diff and lock file) | High | S |
| Project rules file ("constitution"): named, checkable project principles (e.g. required links per type, forbidden words, required EARS for FR) that `check` enforces | spec-kit constitution, Kiro steering | Turns team writing conventions into enforced checks | Fits | Medium | M |
| Per-item quality score with INCOSE rule IDs (e.g. R7 vague terms, R19 combinators) in diagnostics, hover and reports, with an expanded rule catalogue for DE and EN | Jama Connect Advisor, QVscribe, IBM RQA, Codebeamer Requirements Assistant | Reviewers and analysts get an explainable score and find weak items fast | Fits (deterministic) | High | M |
| EARS pattern detection, plus code actions and snippets that rewrite into the EARS templates | Kiro requirements.md, Jama Advisor (EARS) | Helps analysts write testable requirements consistently | Fits | Medium | S |
| Deterministic acceptance test skeletons: generate TC- items from EARS clauses or GIVEN/WHEN/THEN, with `ver` links prefilled | Copilot4DevOps, Codebeamer Test Case Assistant, Kiro acceptance criteria | Speeds up writing verification items and keeps coverage high | Fits (template-based, no LLM) | Medium | M |
| Offline trace link *suggestions* (TF-IDF or glossary-term overlap) shown as hints or code actions, never auto-added | NLP4RE TLR research (Tracer, R2Code, LiSSA) | Finds forgotten links between requirements and tests or design | Fits; low precision is likely, so it must remain a suggestion | Medium | M |
| Near-duplicate detection (lexical similarity across items) | NLP4RE research; vendor claims not verified here | Catches copy-paste and drift between similar requirements | Fits | Medium | M |
| Prompt export (`requimd prompt rewrite FR-012` / `prompt tests FR-012`) that prints an LLM-ready prompt with context and rules, for the user's own agent | Pattern behind Jama, Copilot4DevOps and Codebeamer rewrite and test features | LLM help without requimd sending data anywhere | Fits | Medium | S |
| Opt-in LLM commands calling a configured endpoint (local Ollama or a company gateway) for rewrites, tests and summaries | Jama Advisor, Copilot4DevOps, IBM AI Hub (choice of model provider) | One-click rewrite and test drafts | Breaks the default only if not opt-in. Allowed under the explicit-network rule (NFR-006 / ADR-0011) but adds dependencies and binary size (NFR-005) | Medium | L |
| Git pre-commit hook or CI recipe running `check` and suspect reporting | Kiro hooks | Keeps the spec valid with no editor discipline needed | Fits | Medium | S |
| Embedding-based semantic search | Rovo semantic model | Search by meaning across a large requirement set | Breaks: single offline binary (model weights) and few dependencies | Low | L |
| Spec-to-code generation markers ("GENERATED FROM SPEC") / spec-as-source | Tessl | Not an analyst problem | Out of scope (code generation) | Low | L |

### Gaps
- Value and effort ratings are my own judgement, not sourced. Effort assumes the current Rust architecture (LSP, lock file, diff) described in the brief.
- No source evaluated whether analysts (as opposed to developers) benefit from SDD tools. Böckeler flags this as an open question.

## Where do AI features in RM and SDD tools fail or face criticism?

### Takeaway
The criticisms fall into five groups:
- verbosity and review overload
- agents ignoring or over-applying specs
- non-determinism
- unclear target users
- one workflow size for all problems

RM-tool AI adds three more: vendor-only evidence, privacy and shadow-AI concerns, and low precision in trace suggestions. Quality rules and automated traceability can also conflict.

### Cited Findings
- **Review overload:** spec-kit produced verbose, repetitive Markdown, partly containing code. Kiro also proved too verbose for small tasks ([Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)).
- **Sizing:** one workflow for all problem sizes over-specifies small fixes. Böckeler suggests that SDD may fit neither very large unclear features nor trivial tasks ([Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)). Kiro's "Quick Spec" mode, which skips approval gates, partly responds to this ([kiro.dev](https://kiro.dev/docs/specs/feature-specs/)).
- **False sense of control:** checklists are interpreted by the AI, so compliance isn't guaranteed. Agents ignored notes or over-applied the constitution ([Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)).
- **Non-determinism:** the same spec generated different code across runs ([Böckeler](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html)).
- **Fast churn:** spec-kit tutorials from early 2026 no longer worked months later ([vibecoding.app](https://vibecoding.app/blog/spec-kit-review)), and OpenSpec removed its old commands in v1.0 ([newreleases](https://newreleases.io/project/github/Fission-AI/OpenSpec/release/v1.0.0)).
- **Governance gap:** SDD change artifacts are not a governed record. This is Jama's view and it is self-interested, but it points at a real gap: approval, baselines and signatures ([Jama blog](https://www.jamasoftware.com/blog/openspec-guide/)).
- **Immaturity, privacy and marketing evidence:**
  - RM GenAI is early stage.
  - Shadow AI, privacy and security risks are real.
  - Self-hosting needs scarce expertise.
  - The survey looked only at marketing claims; no measured outcomes exist ([Birk](https://makingofsoftware.com/2026/02/ai-support-in-requirements-tools/)).
- **Controllability:** prompt uncertainty and limited controllability are barriers to trustworthy LLM use in RE ([arXiv 2507.07682](https://arxiv.org/abs/2507.07682)).
- **Trace suggestion precision:** high-recall LLM tracing gave about 18% precision ([arXiv 2611.09162](https://arxiv.org/html/2611.09162)).
- **Quality versus traceability:** some "defects" in quality rules make automated TLR *easier*, and some good-practice choices make it harder ([arXiv 2606.11834](https://arxiv.org/html/2606.11834)).
- **Product churn and opaque positioning:** IBM's RQA product was reportedly discontinued and its function moved into a platform (AI Hub / watsonx) ([requisis](https://requisis.com/en/saas-doors-dng/rqa-saas.html)).

### Inferences
- **Keep the CLI deterministic.** For requimd the strongest positioning is the deterministic, reviewable layer under any AI: checks, links, suspect flags and diffs that make AI-written or AI-consumed specs verifiable. That answers Böckeler's "lost validity checking" concern directly.
- **Generative features as suggestions.** Any generative feature should produce reviewable suggestions (code actions, draft files in a branch) and never modify status or links silently. That fits BR-004 and the PR workflow.
- **Concise output.** requimd's output for agents (context export, MCP) should be compact, which mitigates review overload and verbosity.

### Gaps
- No independent user study was found comparing analysts' productivity or quality with and without RM AI features.
- No critical review of Jama's MCP server or of Kiro hooks in practice was found.
