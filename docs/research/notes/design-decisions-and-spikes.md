# Design decisions, design documents and spikes: practices and tools, with ideas for a requimd design level

Scope: how teams record decisions (ADRs), design documents and proposals (RFC/PEP/KEP/RFD), and exploratory work (spikes, hypotheses), and how these link to requirements. Research date: 18 November 2026. Dates of sources are given where the source states them. Tool calls were limited, so some named practices are listed under Gaps rather than profiled.

## Profiles of practices and tools

### Takeaway
Decision records share a common core (title, status, context, decision, consequences). Mature variants add decision drivers, considered options, and a "confirmation" step that says how the decision is verified. Proposal processes (PEP, KEP, Rust RFC, Oxide RFD) add explicit lifecycle states, supersede/replace links, goals and non-goals, and "unresolved questions" sections. Of the profiled tools, only Sphinx-needs (through custom types) and the Tyree/Akerman template ("Related requirements") link decisions to requirements as first-class data. Most ADR tooling handles files and rendering only.

### Cited Findings

#### Nygard ADR (2011)
- **What it is.** The original lightweight ADR, popularized by Michael Nygard's blog post of 15 Nov 2011. Fields: title, status, context, decision, consequences. — [adr.github.io ADR templates](https://adr.github.io/adr-templates/); [Nygard 2011](https://www.cognitect.com/blog/2011/11/15/documenting-architecture-decisions)
- **Practice (Fowler, 24 March 2026).** One file per ADR in the code repository, commonly `doc/adr`, in lightweight markup so records diff like code. Files are numbered in a monotonic sequence with a descriptive name (e.g. `0001-HTMX-for-active-web-pages`). Status is "proposed" while under discussion, "accepted" once adopted, and "superseded" with a link to the replacement. — [Fowler, ArchitectureDecisionRecord](https://martinfowler.com/bliki/ArchitectureDecisionRecord.html)
- **Immutability.** An accepted ADR is not reopened or edited. A changed decision gets a new ADR that supersedes the old one, which keeps a log of how long each decision governed the work. — [Fowler](https://martinfowler.com/bliki/ArchitectureDecisionRecord.html)
- **Brevity.** About one page, "inverted pyramid". List the serious alternatives with pros and cons. Record the **confidence level** and the **product context changes that should prompt re-evaluation**. ADRs are central to the "Advice Process" (Harmel-Law), where writing them elicits advice. — [Fowler](https://martinfowler.com/bliki/ArchitectureDecisionRecord.html)

#### MADR (Markdown Architectural Decision Records), 4.x
- **Structure.** Front matter: `status` (proposed, rejected, accepted, deprecated, superseded …), `date`, `decision-makers`, `consulted`, `informed`. Sections: Title (problem and chosen solution); Context and Problem Statement; **Decision Drivers** ("qualities, concerns, constraints, or forces"); **Considered Options**; Decision Outcome with **Consequences** and **Confirmation**; Pros and Cons of the Options (good/neutral/bad); More Information (evidence, agreement, **when to revisit**, related links). — [MADR template (develop branch)](https://github.com/adr/madr/blob/develop/template/adr-template.md)
- **Confirmation.** Says how compliance with the decision is checked: an automated or manual fitness function, a design or code review, or an architecture test (e.g. ArchUnit). The section is optional. — [MADR template](https://github.com/adr/madr/blob/develop/template/adr-template.md)
- **Variants.** adr.github.io links MADR 4.0.0. MADR ships full and minimal templates, each annotated or bare (page updated 2 Nov 2026). — [adr.github.io ADR templates](https://adr.github.io/adr-templates/)
- **Version support in tools.** Most MADR-based tools still support MADR 2.1.2; the Backstage ADR plugin supports 2.1.2 and 3.x. Tooling lags the template. — [adr.github.io tooling](https://adr.github.io/adr-tooling/)

#### Y-statements (Zimmermann; Zdun et al., "Sustainable Architectural Decisions")
- **What it is.** One structured sentence: *In the context of* <use case/user story>, *facing* <concern>, *we decided for* <option> *and neglected* <other options>, *to achieve* <quality>, *accepting* <downside>, (*because* <rationale>). There are short and long variants. A German "cards42" ADR card uses it. — [adr.github.io ADR templates](https://adr.github.io/adr-templates/); [Zimmermann, Y-statements (Medium)](https://medium.com/olzzio/y-statements-10eb07b5a177); [DPR Y-form template](https://socadk.github.io/design-practice-repository/artifact-templates/DPR-ArchitecturalDecisionRecordYForm)
- **Origin.** Presented by Olaf Zimmermann at SATURN 2012 and used in an enterprise transformation and an IoT cloud project. — [Zimmermann, Y-statements](https://medium.com/olzzio/y-statements-10eb07b5a177)
- **Notable idea.** The first clause ("in the context of <use case/user story>") puts the **requirement link inside the sentence**. The format captures context, addressed requirements, outcome and consequences in one line. — [Ozimmer, Architectural Decisions — The Making Of (2020-04-27)](https://ozimmer.ch/practices/2020/04/27/ArchitectureDecisionMaking.html)

#### Tyree/Akerman template (IEEE Software 2005, "Architecture Decisions: Demystifying Architecture")
- **Fields.** Issue, Decision, Status, Group, Assumptions, Constraints, Positions (the viable options), Argument (why one was selected), Implications (consequences), Related decisions, **Related requirements**, Related artifacts, Related principles, Notes. — [Wikimedia Phabricator discussion](https://phabricator.wikimedia.org/transactions/detail/PHID-XACT-TASK-4htqcee72xfczvn/); [Redfin ADR template wiki](https://github.com/redfin/react-server/wiki/Architecture-Decision-Record-Template)
- **Requirements link.** The Redfin adaptation says decisions should be tied to business objectives through "Related requirements". — [Redfin](https://github.com/redfin/react-server/wiki/Architecture-Decision-Record-Template)
- **Critique.** Practitioners call the template not lightweight enough to reuse easily. — [Wikimedia Phabricator](https://phabricator.wikimedia.org/transactions/detail/PHID-XACT-TASK-4htqcee72xfczvn/)
- The original PDF was not retrieved, so exact section wording is unverified.

#### ecADR: Definition of Done for decisions (Zimmermann, 2020)
- **Five criteria.** A decision is "DONE-done" when all five hold:
  - **E**vidence: the design will work, for example shown by a proof of concept, a spike or an expert endorsement.
  - **C**riteria: at least two alternatives were compared against decision drivers. Do not invent fake alternatives.
  - **A**greement: peers and the team have challenged and accepted it.
  - **D**ocumentation: recorded in a lean format such as a Y-statement or MADR.
  - **R**ealization and review plan: implementation is scheduled, the check that it was implemented as intended is set (e.g. ArchUnit), and a future review is planned.

  "ADs age." Technology changes and stakeholder feedback can require revisiting a decision, and the review should ask when the decision should be revisited or expire. — [Ozimmer, A Definition of Done for Architectural Decision Making](https://ozimmer.ch/practices/2020/05/22/ADDefinitionOfDone.html)
- **Notable idea.** Spikes are named as the **evidence** for a decision. This connects exploratory work to decisions.

#### adr.github.io (community hub) and its tool landscape
- **Definitions.** An AD is "a justified design choice that addresses a functional or non-functional requirement that is architecturally significant". An ASR is "a requirement with a measurable effect on the architecture and quality of a system". A project's ADRs together form its decision log. — [adr.github.io](https://adr.github.io/)
- **Tool list (page updated 23 Sep 2026).**
  - CLIs: adr-tools (npryce, Bash, Nygard format), dotnet-adr (.NET global tool), pyadr (lifecycle propose/accept/reject/deprecate/supersede), ADG (Go; Nygard, MADR, QOC), Talo (ADRs, RFCs and custom design docs), and Rust and other ports (e.g. `adrs`).
  - Rendering: Log4brains (CLI and rendering), adr-viewer.
  - Editors: ADR Manager (web, form-based via GitHub) and its VS Code extension.
  - Portal: Backstage ADR plugin (search across organizations and repositories).
  - Web apps: ReflectRally (workflows, ownership, review), adr.zone (generates Nygard, MADR, Y-statement and 42010-style records), Loqbooq (Slack).
  - Code-linked: e-adr (Java annotations), `architectural-decision` (PHP attributes).
  - Labelled unmaintained: adr-log, ADMentor, eadlsync, SE Repo.

  The page itself says the list is "rather inclusive". — [adr.github.io ADR tooling](https://adr.github.io/adr-tooling/)
- **Requirements linkage.** None of the listed tools is described as linking decisions to requirements. Only e-adr and `architectural-decision` link decisions to code. — [adr.github.io ADR tooling](https://adr.github.io/adr-tooling/)

#### Structurizr (C4 model as code)
- **ADR support.** `!adrs <path> [importer]` attaches Markdown or AsciiDoc ADRs to the workspace, a software system or a container. Built-in importers are `adrtools` (the default), `madr` and `log4brains`; custom `DocumentationImporter` plugins are possible. Decisions are therefore scoped to **model elements**, not to requirements. — [Structurizr DSL docs: ADRs](https://docs.structurizr.com/dsl/adrs) (docs nav shows binaries v2026.09.19)
- **Notable idea.** **Import existing ADR folders as-is** through format-specific importers instead of forcing a new format.

#### arc42 section 9 (Architecture Decisions)
- **Scope.** Records important, expensive, large-scale or risky decisions with their rationale, so that stakeholders understand *why* and not only *what*. Three forms: one ADR per decision (Nygard fields; status proposed/accepted/deprecated/superseded), a table ordered by importance, or a detailed section per decision. Document only architecturally significant decisions. Section 4 (solution strategy) holds the key decisions, and section 9 should refer to it rather than duplicate it. — [docs.arc42.org section 9](https://docs.arc42.org/section-9/)
- **Tips.** Document the decision criteria (Tip 9-2), the reasons (9-3) and the rejected alternatives (9-6). — [docs.arc42.org section 9](https://docs.arc42.org/section-9/)
- **Sphinx-needs and arc42.** The useblocks how-to models arc42 decisions as a custom `arch-decision` need type with an `ADR_` prefix, "with full traceability between architecture decisions, components, and requirements". — [useblocks x-as-code: arc42](https://x-as-code.useblocks.com/how-to-guides/arc42/index.html)

#### ISO/IEC/IEEE 42010:2022
- **Contents.** The 2022 edition has clauses on architecture decisions and rationale (5.2.12), AD element correspondences (5.2.11, 6.9) and recording decisions and rationale (6.10). — [iTeh sample of ISO/IEC/IEEE 42010:2022 (ToC)](https://cdn.standards.iteh.ai/samples/74393/fc7b7f103d8446a4b87a3261e31370d3/ISO-IEC-IEEE-42010-2022.pdf)
- **Conceptual model (based on the 2011 edition).** A Decision *affects* architecture description elements and *pertains to* one or more Concerns, and making a decision can raise new Concerns. Correspondences express "consistency, traceability, dependencies, obligations" between elements, enforced by correspondence rules. — [quality.arc42.org on ISO 42010](https://quality.arc42.org/standards/iso-42010); [TRAK on ISO 42010](https://trak.sourceforge.io/iso_42010.html)
- **Caveat.** The normative 2022 wording was not retrieved; the summaries above are secondary.

#### Twin Peaks (Nuseibeh, IEEE Computer 34(3), March 2001)
- **What it is.** Requirements and architecture are specified concurrently and iteratively, with "equal status". Stable yet adaptable architectures are developed in the presence of changing requirements, as a partial, simplified spiral model. — [Open University ORO record](https://oro.open.ac.uk/2213); DOI 10.1109/2.910904
- **Later work.** Frames the model as discovering requirements progressively while exploring alternative architectural decisions concurrently. A series of Twin Peaks workshops followed (e.g. at ICSE 2014). — [Twin Peaks workshop site](https://www.se.rit.edu/~mehdi/twinpeaks/ICSE14/)
- **For requimd.** This motivates **bidirectional** links: design can raise new requirements and questions, not only answer them.

#### Python PEPs (PEP 1)
- **Preamble.** Author, Sponsor, Status, Type (Standards Track/Informational/Process), Created, **Requires**, **Replaces**, **Superseded-By**, **Resolution** (link to the acceptance or rejection post), **Discussions-To**, Post-History. — [PEP 1](https://peps.python.org/pep-0001/)
- **Statuses.** Draft, Active, Accepted, Provisional, Deferred, Rejected, Withdrawn, Final, Superseded.
  - Draft → Accepted/Rejected/Withdrawn by pronouncement.
  - Accepted → Final once the implementation is merged.
  - Provisional means accepted pending user feedback.
  - Deferred is used when no progress is made.
  - Active is for never-completed Informational or Process PEPs. — [PEP 1](https://peps.python.org/pep-0001/)
- **Sections.** Motivation, Rationale (alternatives and evidence of consensus), **Rejected Ideas**, **Open Issues**, Backwards Compatibility, Security Implications, How to Teach This, Reference Implementation. — [PEP 1](https://peps.python.org/pep-0001/)
- **Notable idea.** A separate **Final** state ("implemented") after Accepted. That is the decision-level analogue of "verified".

#### Rust RFCs
- **Template.** Front matter: Feature Name, Start Date, RFC PR, Rust Issue. Sections: Summary, Motivation, Guide-level explanation, Reference-level explanation, Drawbacks, **Rationale and alternatives** ("Why is this design the best in the space of possible designs?"), **Prior art**, **Unresolved questions** (split into those to resolve before merge and those to resolve during implementation before stabilization), **Future possibilities**. — [rust-lang/rfcs 0000-template.md](https://raw.githubusercontent.com/rust-lang/rfcs/master/0000-template.md)
- **Notable idea.** Unresolved questions are tied to a **gate**: resolved before acceptance, or before stabilization. Open questions therefore get a deadline milestone.

#### Kubernetes KEPs
- **kep.yaml.** title, kep-number, authors, owning-sig, participating-sigs, **status** (provisional, implementable, implemented, deferred, rejected, withdrawn, replaced), creation-date, reviewers, approvers, **see-also**, **replaces**, **stage** (alpha/beta/stable; or deprecated/disabled/removed), latest-milestone, **milestone** per stage, feature-gates, disable-supported, metrics. Some fields are required Production Readiness Review (PRR) answers at alpha or beta. — [KEP template kep.yaml](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/kep.yaml)
- **README sections.** Summary; Motivation with **Goals** ("How will we know that this has succeeded?") and **Non-Goals** ("helps to focus discussion"); Proposal (User Stories, Notes/Constraints/Caveats, Risks and Mitigations); Design Details (**Test Plan**, **Graduation Criteria**, upgrade and version-skew strategy); PRR questionnaire; Implementation History; Drawbacks; Alternatives. — [KEP README template](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/README.md)
- **Release signoff checklist.** Status `implementable` set by approvers, design details, test plan, graduation criteria, PRR approved, docs, links to supporting design docs and PRs. — [KEP README template](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/README.md)
- **Notable idea.** Machine-readable metadata sits **next to** the prose (a separate YAML file) and is validated by tooling. Readiness is a **checklist gate**.

#### Oxide RFDs (Requests for Discussion)
- **What it is.** "Timely rather than polished" documents for technical designs, API changes, processes and company-level ideas. — [Oxide RFD 1](https://rfd.shared.oxide.computer/rfd/0001)
- **States.** prediscussion, **ideation** (topic and scope only: a scratchpad others can pick up), discussion (open PR), published (merged, still updatable), **committed** (fully implemented; it now explains how the system works), abandoned. — [Oxide RFD 1](https://rfd.shared.oxide.computer/rfd/0001)
- **Metadata.** AsciiDoc attributes `authors`, `state`, `discussion` (link to the PR), `labels`. — [Oxide RFD 1](https://rfd.shared.oxide.computer/rfd/0001)
- **Storage and flow.** One git branch per number (`0042`) with `rfd/0042/README.adoc`. Nothing is pushed directly to main. A bot fixes state, title and discussion link. Comments run for about 3 to 5 business days. An RFD is not merged if nobody else has read it. Changes to a published RFD go in their own branch and PR. — [Oxide RFD 1](https://rfd.shared.oxide.computer/rfd/0001)
- **Tooling.** CSV export, chat-bot lookup, short URLs, rendered site. — [Oxide RFD 1](https://rfd.shared.oxide.computer/rfd/0001)
- **Date.** The document gives no date; it references RFD 480 (2024).

#### Google-style design docs (Malte Ubl, 6 July 2020)
- **Structure.** Context and scope ("not a requirements doc"); **Goals and non-goals** (non-goals are reasonable possibilities deliberately excluded, not negated goals); the actual design (focused on trade-offs); **Alternatives considered**; cross-cutting concerns (security, privacy, observability). Mini design docs of 1 to 3 pages suit incremental work. — [Design Docs at Google](https://www.industrialempathy.com/posts/design-docs-at-google/)
- **Lifecycle.** Creation, review, implementation and iteration, maintenance and learning. People rarely update documents, so changes end up as **amendments**, which should be **linked from the original**. Authors should revisit a doc 1 to 2 years later. — [Design Docs at Google](https://www.industrialempathy.com/posts/design-docs-at-google/)

#### Hypothesis-driven development and experiment cards (spikes)
- **Thoughtworks format.** "We believe <capability> will result in <outcome>. We will know we have succeeded when <measurable signal>". The signal is defined **before** the test to avoid biased reading of results. — [Thoughtworks, How to implement hypothesis-driven development](https://www.thoughtworks.com/insights/articles/how-implement-hypothesis-driven-development)
- **Pip Decks variant.** "Because we saw <insight>, we believe <assumption> … We will know we're right/wrong when we see <KPI change>". Applied to the riskiest unproven assumptions. — [Pip Decks hypothesis statement](https://pipdecks.com/pages/hypothesis-statement)
- **Experiment reports.** Hypothesis, method, learning; validated or invalidated; next step; persist or pivot. Kanban states: new, active, stopped. — [Kanban boards for hypothesis-driven development](https://medium.com/lean-product-development/kanban-boards-for-hypothesis-driven-development-8a977bb2c8c)
- **Spikes.** No canonical spike-record template was found. The ecADR "Evidence" criterion names a spike or proof of concept as evidence for a decision. — [Ozimmer ecADR](https://ozimmer.ch/practices/2020/05/22/ADDefinitionOfDone.html)

#### Kiro (AWS) specs
- **Files.** `requirements.md` (user stories and acceptance criteria in structured notation; or `bugfix.md`), `design.md` ("technical architecture, sequence diagrams, and implementation considerations", component design, data flow, error handling, testing strategy) and `tasks.md` (trackable tasks, in progress or completed; run in dependency "waves"). "Analyze Requirements" catches inconsistencies before design. Page updated 12 Nov 2026. — [Kiro docs: Specs](https://kiro.dev/docs/specs/)
- **Traceability.** The official page does not describe how design.md references requirement IDs. Community templates (cc-sdd) require tasks to reference numeric requirement IDs ("1.1", "3.3") exactly as defined, without inventing new ones. — [cc-sdd kiro-spec-tasks template](https://cdn.jsdelivr.net/npm/cc-sdd@2.0.0/templates/agents/codex/commands/kiro-spec-tasks.md); [Kiro docs: Specs](https://kiro.dev/docs/specs/)

#### Sphinx-needs (design items next to requirements)
- **Model.** Default need types are req, spec, impl and test. Links can target any need type. Custom link types come from `needs_extra_links`. There is no built-in "decision" type or "satisfies" convention; teams define e.g. `arch-decision` (useblocks arc42 how-to). Recent versions allow filtered links. — [Sphinx-needs need directive](https://sphinx-needs.readthedocs.io/en/stable/directives/need.html); [useblocks arc42 how-to](https://x-as-code.useblocks.com/how-to-guides/arc42/index.html)

#### 2025-2026 tools: ADR linting and AI-assisted ADRs
- **Archgate.** Turns ADRs into executable checks: a companion rules file per decision, with `archgate check` blocking violations. AI coding agents read ADRs before coding and capture new patterns back into ADRs. — [Archgate README (npm 0.48.1)](https://cdn.jsdelivr.net/npm/archgate@0.48.1/README.md)
- **ADR Kit.** Python, MADR format, MCP tools for agents. On approval, it generates lint configs, e.g. `disallow: [axios]` becomes an ESLint rule pointing to the ADR, plus Ruff rules. — [ADR Kit on GitHub](https://github.com/kschlt/adr-kit); [PyPI adr-kit](https://pypi.org/project/adr-kit/0.2.5/)
- **ADR Guard.** .NET CLI that validates and indexes ADRs, with AI drafting. It **forces the status of generated ADRs to Proposed** and never accepts a decision on the team's behalf. — [NuGet RodriOliveira.AdrGuard](https://www.nuget.org/packages/RodriOliveira.AdrGuard)
- **Equal Experts (27 Oct 2025).** Generates ADRs from one-sentence "kernel of truth" decisions via metaprompts. AI frequently hallucinated references (non-existent APIs, pages, features). — [Equal Experts: Accelerating ADRs with generative AI](https://www.equalexperts.com/blog/ai/accelerating-architectural-decision-records-adrs-with-generative-ai/)
- Adoption and maintenance of these tools were not verified.

### Inferences
- The common denominator of decision formats is small (title, status, context, decision, consequences). The valuable extras for traceability are **decision drivers** (MADR), **related requirements** (Tyree/Akerman), the **"in the context of" clause** (Y-statement) and **confirmation** (MADR, ecADR R). Each maps directly onto a typed link in requimd.
- Proposal processes all distinguish "accepted" from "implemented or verified" (PEP Final, KEP implemented, Oxide committed). Decision status therefore has two axes: agreement (proposed → accepted) and realization (accepted → implemented/confirmed).
- Supersession is universal (Nygard/Fowler superseded, PEP Replaces/Superseded-By, KEP replaces, Google amendments). It is the decision-level counterpart of requimd's "never renumber, never delete" rules (BR-002, BR-003).

### Gaps
- **Not researched (tool budget):** Go proposals (golang/proposal; reportedly issue labels such as "likely accept/decline"), the IETF RFC lifecycle (Internet-Draft → RFC, Obsoletes/Updates headers), Uber/Squarespace RFC practices, Confluence decision templates, StrictDoc design modelling, and SysML v2/Capella treatment of decisions.
- **Not verified:** Log4brains maintenance status in 2026; Backstage ADR plugin feature details; the exact release date of MADR 4.0.0 (only "4.0.0 linked" was confirmed).
- **Not covered:** ATAM utility trees and quality attribute scenarios (source, stimulus, environment, artifact, response, response measure) and arc42 section 10 quality scenarios. arc42 section 9 does not describe its relation to sections 2, 10 and 11.

## What item types, metadata and states would a design level need?

### Takeaway
Established practice supports five design-level item kinds:
- **decision**: the core
- **design document/proposal**: a container with goals, non-goals and alternatives
- **open question**: unresolved questions and open issues
- **assumption or hypothesis**: Tyree/Akerman assumptions, experiment cards
- **spike or experiment**: evidence for decisions

Constraints and risks already appear as sections inside decisions and proposals (Tyree/Akerman "Constraints", KEP "Risks and Mitigations"), so they can be a requirement kind or a section rather than a new design type.

### Cited Findings
- **Decision states.**
  - MADR: proposed, rejected, accepted, deprecated, superseded. — [MADR template](https://github.com/adr/madr/blob/develop/template/adr-template.md)
  - arc42 and Nygard: proposed, accepted, deprecated, superseded. — [docs.arc42.org section 9](https://docs.arc42.org/section-9/)
  - pyadr lifecycle: propose, accept, reject, deprecate, supersede. — [adr.github.io tooling](https://adr.github.io/adr-tooling/)
- **Proposal states.**
  - PEP: Draft, Active, Accepted, Provisional, Deferred, Rejected, Withdrawn, Final, Superseded. — [PEP 1](https://peps.python.org/pep-0001/)
  - KEP: provisional, implementable, implemented, deferred, rejected, withdrawn, replaced. — [kep.yaml](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/kep.yaml)
  - Oxide RFD: prediscussion, ideation, discussion, published, committed, abandoned. — [Oxide RFD 1](https://rfd.shared.oxide.computer/rfd/0001)
- **Decision metadata beyond status.**
  - MADR: date, decision-makers, consulted, informed (a RACI-like set). — [MADR](https://github.com/adr/madr/blob/develop/template/adr-template.md)
  - Fowler: confidence level and re-evaluation triggers. — [Fowler](https://martinfowler.com/bliki/ArchitectureDecisionRecord.html)
  - ecADR: a review or expiry date. — [ecADR](https://ozimmer.ch/practices/2020/05/22/ADDefinitionOfDone.html)
  - PEP Resolution and Discussions-To: links to where the decision was made. — [PEP 1](https://peps.python.org/pep-0001/)
- **Open questions as a first-class section.** PEP "Open Issues", Rust "Unresolved questions" (with a gate: before merge, or before stabilization), Tyree/Akerman "Assumptions". — [PEP 1](https://peps.python.org/pep-0001/); [Rust RFC template](https://raw.githubusercontent.com/rust-lang/rfcs/master/0000-template.md); [Phabricator on Tyree/Akerman](https://phabricator.wikimedia.org/transactions/detail/PHID-XACT-TASK-4htqcee72xfczvn/)
- **Hypotheses and spikes.**
  - Fields: hypothesis ("we believe"), success signal defined up front ("we will know"), result (validated/invalidated), next step. — [Thoughtworks](https://www.thoughtworks.com/insights/articles/how-implement-hypothesis-driven-development); [Kanban HDD](https://medium.com/lean-product-development/kanban-boards-for-hypothesis-driven-development-8a977bb2c8c)
  - Spikes serve as decision evidence. — [ecADR](https://ozimmer.ch/practices/2020/05/22/ADDefinitionOfDone.html)
- **Design document contents.** Goals and non-goals, alternatives considered, cross-cutting concerns. — [Design Docs at Google](https://www.industrialempathy.com/posts/design-docs-at-google/); [KEP README](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/README.md)
- **Design elements.**
  - Kiro design.md: components, data flow, error handling, testing strategy. — [Kiro docs](https://kiro.dev/docs/specs/)
  - Sphinx-needs: `spec` and `impl` need types. — [Sphinx-needs](https://sphinx-needs.readthedocs.io/en/stable/directives/need.html)
  - Structurizr: C4 elements to which ADRs attach. — [Structurizr ADRs](https://docs.structurizr.com/dsl/adrs)

### Inferences
- **Minimal state set for a requimd decision (DEC-)** that covers MADR, Nygard and arc42: `proposed → accepted | rejected`, then `deprecated`, `superseded`. An optional realization flag (`implemented` or "confirmed") mirrors PEP Final, KEP implemented and Oxide committed. This could reuse requimd's existing status machinery, with BR-004 (never set approved automatically) carried over: tools and AI may only set `proposed`, as ADR Guard does.
- **Open question (Q-).** States `open → answered | obsolete`. Links: `about` (the items it concerns), `answered-by` (a DEC- or a requirement change), and an optional gate, e.g. `before: review`, so `requimd check` can block a status change while questions are open. This follows Rust's "resolve before merge".
- **Assumption (ASM-) and spike (SPK-).** An assumption has states `assumed → validated | invalidated`. A spike carries the hypothesis, a timebox, the success signal and a verdict (validated/invalidated/inconclusive). It links `tests: ASM-` and `informs: DEC-`.
- **Constraints and risks.** Model them as requirement-level items or decision sections rather than new design types, to keep the model small.
- **Design element (DES-).** Optional and later. Analysts mostly need decisions and questions; design elements matter for architects and for coverage ("requirements without design").

### Gaps
- No public source defines a standard "open question" or "assumption register" item format with states. RAID logs (Risks, Assumptions, Issues, Dependencies) were not researched in this session.
- No canonical spike record template was found. The spike fields above are a synthesis from the hypothesis-card sources.

## Which relationship kinds link design to requirements, and how is a decision verified?

### Takeaway
Sources use these link kinds:
- decision **addresses** a requirement or concern (ADR definition, 42010 "pertains to", Y-statement "in the context of")
- decision **driven by** qualities and constraints (MADR drivers)
- **supersedes / replaces** (universal)
- **requires / depends on** (PEP Requires)
- **see also** (KEP)
- **affects** design elements (42010)
- **implements / satisfies** (Sphinx-needs custom links)

A decision is verified in two steps. *Done* means evidence, criteria, agreement, documentation and a review plan (ecADR). *Confirmed* means checked in the system by a review or by an automated fitness function such as ArchUnit or lint rules (MADR Confirmation, Archgate, ADR Kit).

### Cited Findings
- **Decisions address requirements.** An AD "addresses a functional or non-functional requirement that is architecturally significant". — [adr.github.io](https://adr.github.io/)
- **42010 relations.** A Decision *pertains to* Concerns and *affects* AD elements. Correspondences cover "traceability, dependencies, obligations". — [quality.arc42.org on ISO 42010](https://quality.arc42.org/standards/iso-42010); [TRAK](https://trak.sourceforge.io/iso_42010.html)
- **Drivers and requirement fields.** MADR "Decision Drivers" are qualities, concerns, constraints or forces. Tyree/Akerman have "Related requirements", "Related decisions", "Related artifacts" and "Related principles". — [MADR](https://github.com/adr/madr/blob/develop/template/adr-template.md); [Redfin template](https://github.com/redfin/react-server/wiki/Architecture-Decision-Record-Template)
- **Dependency and replacement links.** PEP Requires, Replaces and Superseded-By; KEP see-also and replaces. — [PEP 1](https://peps.python.org/pep-0001/); [kep.yaml](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/kep.yaml)
- **Implementation links.** Sphinx-needs creates any link type via `needs_extra_links`. A Continuum example links sw_impl items to architecture specs with a custom `implements` field. — [Sphinx-needs search results / need directive](https://sphinx-needs.readthedocs.io/en/stable/directives/need.html); [RT-RK Continuum](https://www.rt-rk.com/?p=19863)
- **Verification of decisions.**
  - MADR "Confirmation": a fitness function, design or code review, or ArchUnit test. — [MADR](https://github.com/adr/madr/blob/develop/template/adr-template.md)
  - ecADR "R": schedule the check that the decision was implemented as intended, possibly via ArchUnit, plus a review. — [ecADR](https://ozimmer.ch/practices/2020/05/22/ADDefinitionOfDone.html)
  - Archgate and ADR Kit turn decisions into lint or CI rules. — [Archgate](https://cdn.jsdelivr.net/npm/archgate@0.48.1/README.md); [ADR Kit](https://github.com/kschlt/adr-kit)
  - KEP Test Plan and Graduation Criteria are verification gates for a design. — [KEP README](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/README.md)
- **Decision-to-code links.** e-adr (Java annotations) and `architectural-decision` (PHP attributes) embed decisions in code. — [adr.github.io tooling](https://adr.github.io/adr-tooling/)

### Inferences
Proposed requimd link vocabulary, kept small and mapped to sources:
- `addresses`: DEC → requirement (Y-statement context, AD definition)
- `driven-by`: DEC → NFR/BR/constraint (MADR drivers). This is the link that should drive suspect analysis.
- `supersedes`: DEC → DEC, with the inverse computed (PEP, KEP, Fowler)
- `depends-on`: DEC → DEC (PEP Requires)
- `satisfies`: DES → requirement (Sphinx-needs style)
- `ver`: DEC → TC- (MADR Confirmation). This reuses requimd's existing `ver` link so decisions are "verified on their own level".
- `evidence`: DEC → SPK- (ecADR E)
- `answers`: DEC → Q- (open question)

For a decision, `ver` can point to a TC- that is a review checklist or a fitness-function test. requimd only needs the link and the TC status; it does not need to run the test. This keeps the single-binary, language-agnostic constraint.

### Gaps
- No empirical research found (in the time available) on which link kinds practitioners actually use most. Architectural knowledge management literature beyond Twin Peaks and 42010 was not searched.

## How could suspect-link impact analysis extend to decisions?

### Takeaway
Practice already expects decisions to be re-evaluated when their context changes:
- Fowler: record "product context changes that should prompt the team to reevaluate".
- ecADR: "ADs age"; plan a review or expiry.
- Google: amendments, revisit after 1 to 2 years.

No tool found automates this from requirement changes. requimd's lock file could do it: a change to a `driven-by` or `addresses` target makes the decision suspect ("needs re-evaluation").

### Cited Findings
- **Re-evaluation triggers.** Record the confidence level and the context changes that should trigger re-evaluation. — [Fowler](https://martinfowler.com/bliki/ArchitectureDecisionRecord.html)
- **Review and expiry.** Plan a review and decide when the decision should be revisited or expire; ADs age with technology and stakeholder feedback. — [ecADR](https://ozimmer.ch/practices/2020/05/22/ADDefinitionOfDone.html)
- **"When to revisit".** MADR's "More Information" section includes when to revisit the decision. — [MADR](https://github.com/adr/madr/blob/develop/template/adr-template.md)
- **Immutability.** An accepted ADR is not edited; a changed decision is a new ADR that supersedes the old. — [Fowler](https://martinfowler.com/bliki/ArchitectureDecisionRecord.html)
- **Amendments.** Google design docs drift, and changes are linked from the original as amendments. — [Design Docs at Google](https://www.industrialempathy.com/posts/design-docs-at-google/)
- **No requirements linkage in ADR tools.** None of the ADR tools listed on adr.github.io links decisions to requirements, so none can detect staleness from requirement changes. — [adr.github.io tooling](https://adr.github.io/adr-tooling/)
- **Twin Peaks.** Architecture must stay stable yet adaptable as requirements change. This motivates propagation in both directions. — [ORO record of Nuseibeh 2001](https://oro.open.ac.uk/2213)

### Inferences
- **Forward propagation.** If any `driven-by` or `addresses` target of an accepted DEC changes content (lock-file hash differs), report the DEC as **suspect: re-evaluate**. Clearing it would mean either re-confirming (updating the hash, as for current suspect links) or writing a superseding DEC.
- **Backward propagation.** When a DEC becomes `superseded` or `deprecated`, requirements that `satisfies`/`addresses` links point back to, and TC- items reached via `ver`, become suspect. Design elements realizing the old decision need a look.
- **Immutability check.** The lock file already stores content hashes, so `requimd check` can warn when an **accepted** decision's text changed without a new superseding DEC. This enforces Fowler's and Nygard's immutability rule cheaply.
- **Time-based staleness.** Optional `revisit:` dates on DECs and `before:` gates on Q- items. A check against the system date fits offline use but makes output date-dependent; this could be an opt-in flag.
- **Assumption invalidation.** Setting an ASM- to `invalidated` should mark every DEC that depends on it as suspect. This is the strongest impact signal for analysts, because assumptions are often the business side of a decision.

### Gaps
- No academic or tool source was found that automatically detects stale decisions from requirement changes; this appears to be an open niche (absence of evidence within the limited search).

## What works well for analysts in recording decisions and open questions?

### Takeaway
Lightweight, one-page, sentence-like formats work best for non-architects: Y-statements, Nygard, minimal MADR, mini design docs. The "timely rather than polished" culture (Oxide) matters too, as does explicit separation of goals and non-goals and of open questions. Heavy templates (Tyree/Akerman) are judged too heavy to reuse. AI drafting works only with a human acceptance gate and with checks for hallucinated references.

### Cited Findings
- **Y-statement.** Captures context, addressed requirement, outcome and consequences in one structured sentence. A German "cards42" card variant exists, which is relevant for German-speaking analysts. — [Ozimmer 2020](https://ozimmer.ch/practices/2020/04/27/ArchitectureDecisionMaking.html); [adr.github.io templates](https://adr.github.io/adr-templates/)
- **Brevity.** Keep ADRs to one page and link to detail. — [Fowler](https://martinfowler.com/bliki/ArchitectureDecisionRecord.html)
- **Minimal templates.** MADR offers minimal and bare variants. — [adr.github.io templates](https://adr.github.io/adr-templates/)
- **Mini design docs.** 1 to 3 pages for incremental work. — [Design Docs at Google](https://www.industrialempathy.com/posts/design-docs-at-google/)
- **Oxide RFD.** "Timely rather than polished". The "ideation" state captures a topic and scope only, as a scratchpad others can pick up. — [Oxide RFD 1](https://rfd.shared.oxide.computer/rfd/0001)
- **Non-goals.** Listing non-goals focuses discussion. — [KEP README](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/README.md); [Design Docs at Google](https://www.industrialempathy.com/posts/design-docs-at-google/)
- **Heavy templates.** Tyree/Akerman is not lightweight enough to be easily reusable. — [Phabricator](https://phabricator.wikimedia.org/transactions/detail/PHID-XACT-TASK-4htqcee72xfczvn/)
- **Real alternatives only.** Do not invent fake alternatives to satisfy a template; a single option can be justified. — [ecADR](https://ozimmer.ch/practices/2020/05/22/ADDefinitionOfDone.html)
- **Form-based editing.** ADR Manager edits ADRs through forms on GitHub, aimed at people less comfortable with raw Markdown. — [adr.github.io tooling](https://adr.github.io/adr-tooling/)
- **AI drafting.** ADR Guard forces AI output to Proposed. Equal Experts report frequent hallucinated references in AI-generated ADRs. — [ADR Guard](https://www.nuget.org/packages/RodriOliveira.AdrGuard); [Equal Experts](https://www.equalexperts.com/blog/ai/accelerating-architectural-decision-records-adrs-with-generative-ai/)
- **Advice Process.** Writing an ADR is the vehicle for gathering advice, which suits analysts who facilitate rather than decide. — [Fowler](https://martinfowler.com/bliki/ArchitectureDecisionRecord.html)
- **Hypothesis cards.** Make business assumptions explicit and testable. — [Thoughtworks](https://www.thoughtworks.com/insights/articles/how-implement-hypothesis-driven-development); [Pip Decks](https://pipdecks.com/pages/hypothesis-statement)

### Inferences
- **Business decisions too.** For analysts, the design level should start with **business and requirements decisions** (scope, rule choices) and **open questions**, not only architecture. MADR itself notes that ADR use extends beyond architecture (adr.github.io: AKM "can extend to design and other decisions"). — [adr.github.io](https://adr.github.io/)
- **Language-server support.** A code action "new decision from this requirement" (pre-filling `addresses:`) and "new open question about this item" would lower the barrier. Hover on a requirement could show its open questions and decisions.
- **Language checks for decisions.** For example: a decision with no `addresses` or `driven-by` link; a "Considered options" section with only one option and no justification; Y-statement clauses missing. These extend requimd's existing language-quality checks.

### Gaps
- No user study was found on analysts (as opposed to architects) writing ADRs. Recommendations here are inferred from practitioner sources.

## Ideas for requimd

### Takeaway
The highest-value, best-fitting ideas reuse what requimd already has: IDs, typed links, the lock file, `ver` links and language checks. They are: a DEC- item type with a small lifecycle; `driven-by` and `addresses` links feeding suspect analysis; an immutability check for accepted decisions; and open-question and assumption items that can block status changes. Code-level enforcement of decisions (Archgate-style) does not fit a requirements-centric, language-agnostic single binary.

### Cited Findings
- All "seen in" entries refer to sources cited in the sections above. The table itself is synthesis.

### Inferences

| Idea | Seen in | Problem it solves for an analyst | Fit | Value | Effort |
|---|---|---|---|---|---|
| Decision item type `DEC-` with states proposed / accepted / rejected / deprecated / superseded (tools and AI may only set proposed) | Nygard, MADR, arc42 §9, pyadr, ADR Guard | Decisions scattered in mails and meetings get IDs, a status and a place next to the requirements | fits (same file and metadata-line model; BR-004 analogue) | high | M |
| `addresses:` and `driven-by:` typed links from DEC to requirements, NFRs and constraints | Y-statement "in the context of", MADR decision drivers, Tyree/Akerman "Related requirements", 42010 "pertains to" | Shows why a requirement is realized the way it is; coverage reports "ASR without decision" | fits | high | S |
| Suspect "re-evaluate" status on a DEC when a `driven-by`/`addresses` target changes (lock-file hash) | Fowler (re-evaluation triggers), ecADR (ADs age); no tool automates it | Changed requirements no longer silently invalidate past decisions | fits (extends ADR-0007 lock file) | high | M |
| Immutability check: warn when an accepted DEC's content hash changes without a superseding DEC | Fowler, Nygard, Google amendments | Keeps the decision log trustworthy and auditable | fits (lock file) | high | S |
| `supersedes:` link with computed inverse, a check that new links do not target superseded DECs, and a supersession chain in site and Mermaid | PEP Replaces/Superseded-By, KEP replaces | See which decision is current; avoid referencing dead decisions | fits | high | S |
| `ver:` from DEC to TC- (review checklist or fitness-function test) as decision "confirmation"; realization flag after verification | MADR Confirmation, ecADR R, PEP Final, KEP implemented, Oxide committed | "Verified on their own level" from the vision, without running code | fits (reuses `ver` and coverage) | high | S |
| Open-question item `Q-` (open / answered / obsolete) with `about:` and `answered-by:`; optional gate that blocks `review` status of linked items while questions are open | PEP Open Issues, Rust Unresolved questions (with gate), Oxide ideation | Analysts' biggest daily need: tracking unresolved points per requirement until they are settled | fits | high | S |
| Assumption item `ASM-` (assumed / validated / invalidated); invalidation makes dependent DECs and requirements suspect | Tyree/Akerman Assumptions, hypothesis cards, RAID logs (not researched) | Makes business assumptions explicit and shows their impact when they turn out wrong | fits | high | M |
| Spike/experiment item `SPK-`: "we believe … we will know when …", timebox, verdict validated / invalidated / inconclusive; `evidence:` link from DEC | Thoughtworks HDD, Pip Decks, ecADR Evidence | Records what a spike or prototype found and which decision it supports, instead of losing it in tickets | fits | medium | S |
| Considered options as labelled sub-items (`O1`, `O2`, like step labels) with pros and cons; check for single-option decisions without justification | MADR, Tyree/Akerman Positions, ecADR Criteria | Structured comparison; rejected options stay findable and referenceable | fits (labels like use-case steps, BR-003 style) | medium | S |
| Y-statement as an allowed compact decision form, with a language check for missing clauses (German and English variants) | Zimmermann/Zdun, cards42 (German) | One-sentence decisions are easy for analysts to write and review | fits (language-check engine) | medium | S |
| Goals and non-goals sections in a design document item, referencing requirement IDs; warn when a non-goal is also addressed by a DEC/DES | KEP, Google design docs | Scope discussions get explicit, checkable boundaries | fits | medium | S |
| `revisit:` date and `confidence:` metadata on DECs; opt-in check `--as-of <date>` lists overdue reviews | Fowler (confidence), ecADR (review/expiry), MADR "when to revisit", Google (revisit 1-2 years) | Regular decision hygiene without a separate tracker | fits (deterministic with an explicit date flag) | medium | S |
| Decision log page in the static site: filter by status, link graph (addresses, supersedes), JSON and CSV matrix columns for DEC | adr-log, Log4brains, Backstage ADR plugin, Oxide CSV export | One place for stakeholders to browse decisions with their requirements | fits (extends M5 site/matrix/JSON) | medium | M |
| Importer or reader for existing MADR or Nygard ADR folders (e.g. `docs/decisions/`) as DEC items, without rewriting them | Structurizr `!adrs` importers (adrtools, madr, log4brains) | Teams with existing ADRs can trace them at once; requimd's own ADRs could be included | breaks (partly): CommonMark-plus-metadata-lines format; MADR uses YAML front matter, so an adapter is needed | medium | M |
| `discussion:` and `resolution:` link metadata (URL to PR, issue or meeting minutes) | PEP Discussions-To/Resolution, Oxide `discussion` | Traces where and by whom a decision was made | fits (links only; no network access needed) | low | S |
| Decision-maker roles `decided-by` / `consulted` / `informed` | MADR front matter | Accountability and stakeholder involvement visible for analysts | fits | low | S |
| Design element item `DES-` with `satisfies:` to requirements, plus coverage "requirement without design" | Sphinx-needs spec/impl, Kiro design.md, Structurizr elements | Requirement-to-design coverage for architects; less central for analysts | fits | medium | M |
| Code-comment scan for item IDs (e.g. `// FR-006`, `// DEC-003`) to report "realized in" locations | e-adr annotations, `architectural-decision` attributes; requimd's own CLAUDE.md convention | Shows whether a decision or requirement was implemented, without a code model | fits (plain text scan, offline); scope creep risk | medium | M |
| Generate lint or architecture rules from decisions (Archgate, ADR Kit style) | Archgate, ADR Kit | Enforces decisions in code | breaks: language-agnostic single binary, analyst focus (NFR-005 scope) | low | L |
| Built-in AI drafting of decisions | adr.zone, Equal Experts, ADR Guard | Faster drafting | breaks: NFR-006 offline, if built in. External agents can draft via the LSP and files; requimd should only enforce "AI drafts stay proposed" and check that references resolve | low | S (check only) |
| Bidirectional "raises" link: a DEC or spike can create new requirements or questions (`raises: Q-, FR-`) | Twin Peaks, 42010 ("making a decision may raise new concerns") | Captures the iterative requirement ↔ design loop instead of a one-way waterfall | fits | medium | S |

### Gaps
- Value and effort ratings are rough judgments, not measured.
- Fit judgments assume requimd's constraints as described in the assignment (CommonMark, offline single binary, LSP, analyst audience). They should be checked against the actual FR/NFR texts (e.g. NFR-005, NFR-006) before use.
