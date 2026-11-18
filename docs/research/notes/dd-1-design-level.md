# DD-1: Design-level practice for requimd's traceability-model decision

Scope: how design decisions, open questions, assumptions, spikes and design elements should be modelled next to requirements in requimd. This builds on phase 1 (`docs/research/requirements-and-design-tools-2026.md`, R-54 to R-72; `notes/design-decisions-and-spikes.md`; the SysML v2 part of `notes/backlog-and-modeling.md`) and does not repeat its profiles. It adds primary-source field lists and lifecycles, the research literature on decision decay and assumptions, a check of requimd's own lock-file mechanics (ADR-0007, FR-116, FR-109, FR-015), and evidence from requimd's own decision practice (`docs/decisions/`, `docs/status.md`). Research date: 18 November 2026. Public sources only.

Local evidence from the requimd repository, cited by path (read-only):

- `requirements/functional/checks.md` FR-116: the hash of a link target covers "the title, the metadata except `status`, and the body". FR-109 (`status-link`): an `approved` item that links to a `draft` or `deprecated` item gets a diagnostic.
- `docs/decisions/0007-lock-file.md`: the lock file has two sections, `[ids]` and `[links]`. Each `[links]` key is `<source> <key> <target>` and its value is the target hash at the last review. Only a review (FR-218, FR-307) changes an existing hash.
- `requirements/functional/format.md` FR-015: the status values `draft`, `review`, `approved` and `deprecated` have a fixed meaning that FR-109, FR-116, FR-400, FR-403 and FR-404 read. "The traceability model of a later design level replaces it" (decision F4).
- `docs/decisions/0010-html-comments-outside-the-hash.md`: "Status: **proposed** on 2026-10-17. Amends ADR-0007." `0007` says "Amended by ADR-0010 (proposed)". The project already uses an *amends* relation besides *supersedes*.
- `docs/status.md` decision log: a table of date, decision and "where recorded", with about 77 dated rows. Most rows are product, scope and policy decisions the owner made as an analyst or product owner, for example F1 (languages), F2 (what counts as a verification record), F3 (design level later) and C2 (publishing). Few are architecture decisions.

---

## Which item types come first, and what fields and lifecycles do the primary sources use?

### Takeaway
All the sources share a small core: ID, title, status, context or problem, the decision, alternatives, and consequences. On top of that they add three things requimd can use directly: links to what the decision answers (MADR drivers, Tyree/Akerman "Related requirements", Y-statement "in the context of"), supersession or replacement, and a section of unresolved questions tied to a gate (Rust, KEP). Decisions and open questions are the most common and best-specified types. Assumptions come next: the research literature shows that invalid ones do the most damage, but no standard item format exists for them. Spikes have no primary-source template. Design elements matter for SysML-style coverage and are the least analyst-centric. Constraints and risks appear in the sources as sections or as attributes of other items, not as separate decision-level types.

### Cited Findings

**MADR 4.x (template, develop branch)**
- Front matter: `status: "{proposed | rejected | accepted | deprecated | … | superseded by ADR-0123}"`, `date: {YYYY-MM-DD when the decision was last updated}`, `decision-makers`, `consulted` ("two-way communication") and `informed` ("one-way communication"). — [MADR adr-template.md](https://raw.githubusercontent.com/adr/madr/develop/template/adr-template.md)
- Sections:
  - Context and Problem Statement
  - Decision Drivers (optional): "qualities, concerns, constraints, or forces"
  - Considered Options
  - Decision Outcome (chosen option with justification), with optional Consequences and an optional **Confirmation**: how implementation and compliance are verified, for example by review, fitness function or test
  - Pros and Cons of the Options (optional; good, neutral, bad)
  - More Information (optional): evidence, agreement, when to implement, **when to revisit**, related decisions

  — [MADR adr-template.md](https://raw.githubusercontent.com/adr/madr/develop/template/adr-template.md)
- Supersession is written into the status value ("superseded by ADR-0123"), not into a separate link field. — [MADR adr-template.md](https://raw.githubusercontent.com/adr/madr/develop/template/adr-template.md)

**ecADR (Zimmermann 2020): definition of done, not a template**
- A decision is done when there is Evidence (a proof of concept, a spike or an expert endorsement), Criteria (at least two real alternatives), Agreement, Documentation, and a Realization and review plan (scheduled implementation, a compliance check, a planned review). "ADs age." — [Ozimmer, A Definition of Done for Architectural Decision Making](https://ozimmer.ch/practices/2020/05/22/ADDefinitionOfDone.html)

**Y-statement (Zimmermann; Zdun et al.)**
- Clauses: *In the context of* <use case or story>, *facing* <concern>, *we decided for* <option> *and neglected* <other options>, *to achieve* <quality>, *accepting* <downside>, optionally *because* <rationale>. — [Zimmermann, Y-statements](https://medium.com/olzzio/y-statements-10eb07b5a177); [DPR Y-form template](https://socadk.github.io/design-practice-repository/artifact-templates/DPR-ArchitecturalDecisionRecordYForm)

**PEP 12 (the PEP template), the PEP 1 process**
- Preamble: `PEP`, `Title`, `Author`, `Sponsor`, `PEP-Delegate`, `Discussions-To`, `Status: Draft | Active | Accepted | Provisional | Deferred | Rejected | Withdrawn | Final | Superseded`, `Type`, `Topic`, `Requires`, `Created`, `Python-Version`, `Post-History`, `Replaces`, `Superseded-By`, `Resolution: <url>`. — [PEP 12](https://peps.python.org/pep-0012/)
- Sections in order: Abstract, Motivation, Specification, Rationale, Backwards Compatibility, Security Implications, How to Teach This, Reference Implementation, Rejected Ideas, Open Issues, Acknowledgements, Footnotes, Change History, Copyright. — [PEP 12](https://peps.python.org/pep-0012/)
- Open Issues guidance: "Any points that are still being decided/discussed." Rejected Ideas: "Why certain ideas that were brought while discussing this PEP were not ultimately pursued." — [PEP 12](https://peps.python.org/pep-0012/)
- Transitions: Accepted moves to Final when the implementation is merged; Provisional means accepted pending feedback. — [PEP 1](https://peps.python.org/pep-0001/) (as profiled in phase 1)

**KEP (kep.yaml and the KEP process)**
- Status values as defined in the process KEP:
  - provisional: "proposed and is actively being defined"
  - implementable: "The approvers have approved this KEP for implementation"
  - implemented: "has been implemented and is no longer actively changed"
  - deferred: "proposed but not actively being worked on"
  - rejected: "The approvers and authors have decided that this KEP is not moving forward"
  - withdrawn: "The authors have withdrawn the KEP"
  - replaced: "replaced by a new KEP", with `superseded-by` pointing to the new KEP

  A `last-updated` field records "the date that the KEP was last changed significantly". — [KEP-0000 KEP process README](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/sig-architecture/0000-kep-process/README.md)
- kep.yaml also carries `stage` (alpha, beta, stable), `latest-milestone`, `milestone` per stage, `see-also` and `replaces`. — [KEP template kep.yaml](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/kep.yaml) (phase 1)
- Goals: "What is it trying to achieve? How will we know that this has succeeded?" Non-goals: "What is out of scope for this KEP? Listing non-goals helps to focus discussion and make progress." — [KEP template README](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/README.md)

**Rust RFCs**
- Template sections: Summary, Motivation, Guide-level and Reference-level explanation, Drawbacks, Rationale and alternatives, Prior art, Unresolved questions (those to resolve before merge, and those to resolve during implementation before stabilization), Future possibilities. — [rust-lang/rfcs 0000-template.md](https://raw.githubusercontent.com/rust-lang/rfcs/master/0000-template.md) (phase 1)
- Lifecycle: a merged RFC is "active" and "may be implemented"; a pull request is merged or closed; "An RFC closed with 'postponed' is marked as such." "Every accepted RFC has an associated issue tracking its implementation in the Rust repository." — [rust-lang/rfcs README](https://raw.githubusercontent.com/rust-lang/rfcs/master/README.md)

**Oxide RFD 1**
- States: prediscussion, ideation (topic and scope only), discussion (open PR), published (merged, can still be updated through a PR), committed (fully implemented), abandoned. Metadata: `authors`, `state`, `discussion`, `labels`. — [Oxide RFD 1](https://rfd.shared.oxide.computer/rfd/0001) (phase 1)

**Google design docs**
- Context and scope, goals and non-goals, the design, alternatives considered, cross-cutting concerns. Changes become amendments linked from the original. — [Design Docs at Google](https://www.industrialempathy.com/posts/design-docs-at-google/) (phase 1)

**Kruchten 2004, "An Ontology of Architectural Design Decisions"**
- Decision categories: existence decisions (structural, behavioural, ban), property decisions, and **executive decisions**. Executive decisions are business, process, organisational or tool choices rather than structure. Positive property decisions act as design rules; negative ones as constraints. — summarised by secondary sources in [search results (scribd excerpt, annals-csis.org, univie eprint)](https://es.scribd.com/document/285377833/Arc-Design-Deci). The original paper was not retrieved.
- Decision states, from an excerpt of the paper:
  - Tentative: "Allows running what if scenarios"; a tentative decision cannot constrain other decisions.
  - Decided, Approved and Rejected.
  - Challenged: "Previously approved or decided decision that is now in jeopardy; it may go back to approved without ceremony, but can also be demoted to tentative or rejected".
  - Obsolesced: "Similar to rejected, but …"; the rest of the definition was cut off.

  Phase 1 search results also name an "Idea" state. — [Scribd copy of Kruchten 2004 (via search excerpt)](https://es.scribd.com/document/285377833/Arc-Design-Deci). Unofficial copy; the wording is unverified.

**Tyree/Akerman (2005)**
- Fields: Issue, Decision, Status, Group, Assumptions, Constraints, Positions, Argument, Implications, Related decisions, Related requirements, Related artifacts, Related principles, Notes. Critics call it not lightweight. — [Redfin template](https://github.com/redfin/react-server/wiki/Architecture-Decision-Record-Template); [Wikimedia Phabricator](https://phabricator.wikimedia.org/transactions/detail/PHID-XACT-TASK-4htqcee72xfczvn/) (phase 1)

**Atlassian decision template (DACI), used by product teams**
- Header fields: decision status, impact, involved parties, due date and outcome. Roles: Driver, Approver, Contributors, Informed. Sections: Background, Options considered ("at least three options", each with pros, cons and a cost rating), Action items, Outcome. The page shows no explicit status values. — [Atlassian Confluence decision template](https://www.atlassian.com/software/confluence/templates/decision)

### Inferences
- **Decisions and open questions come first.** Every source has a decision core. Open questions are in PEP, Rust, KEP and Oxide ideation, and they are the most frequent daily need of an analyst. Both reuse requimd's existing machinery (IDs, typed links, status, `check`) without new mechanics.
- **Assumptions come second.** They give the strongest impact signal (see the re-evaluation section), but they need a status-driven propagation rule that requimd does not have yet (FR-116 ignores `status`).
- **Spikes come third and can be light.** A spike is an inquiry item: a question or hypothesis, a timebox, a success signal and a verdict. It feeds decisions as ecADR "Evidence".
- **Design elements come last.** SysML `satisfy` gives clear semantics, but analysts gain less from them, and they overlap with code tracing (DD-4).
- **No separate types for constraints and risks.**
  - A constraint is a requirement-level item: a BR (business rule) or an NFR. Kruchten treats negative property decisions as constraints, and Tyree/Akerman and MADR list constraints as decision inputs, so `driven-by` covers them.
  - Risks stay a body section ("Risks and mitigations", as in KEP) until a project asks for a RAID-style register.
- **The project's own practice points at business decisions.** Most `docs/status.md` rows are product and scope decisions, and ADR-0010 uses "Amends". The model should handle both kinds of decision, and *amends* as well as *supersedes*.

### Gaps
- The original Kruchten 2004 paper could not be read (the PDF fetch returned binary, and the Scribd copy was truncated). The full state list ("Idea", the definition of "Obsolesced") and his relation list, which I recall as roughly "constrains, forbids, enables, subsumes, conflicts with, overrides, comprises, is an alternative to, is bound to, is related to", plus decision-to-artefact "traces to" and "does not comply with", are **unverified** and must be checked before they are cited.
- No exact Atlassian status values were visible on the page.

---

## Should agreement and realisation be two separate status axes?

### Takeaway
Yes. Every proposal process that tracks realisation separates it from agreement, but they do it in different ways:

- **PEP** puts both in one status field: Accepted, then Final once the implementation is merged.
- **KEP** uses two fields: `status` (implementable, then implemented) and `stage` (alpha, beta, stable), with milestones per stage.
- **Oxide** uses one field: published, then committed.
- **Rust** keeps the realisation out of the RFC altogether: the RFC is "active", and stabilization is tracked in a separate tracking issue and a stabilization report.

ADR templates (Nygard, MADR, arc42) track agreement only, and handle realisation through MADR's "Confirmation" section and ecADR's "R". For requimd, the evidence favours storing agreement in `status` and **computing** realisation from links. requimd already has `ver` and `impl` links, so a second, hand-maintained field would only drift.

### Cited Findings
- KEP status and stage are separate fields in kep.yaml, `status` with its approval states and `stage: alpha|beta|stable`. "implemented" means "no longer actively changed". — [KEP-0000 process](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/sig-architecture/0000-kep-process/README.md); [kep.yaml](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/kep.yaml)
- KEP graduation is gated by criteria per stage. "Beta criteria must include all functional, security, monitoring, and testing requirements along with resolving all issues and gaps identified"; GA criteria "must not include any functional, security, monitoring, or testing requirements". — [KEP template README](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/README.md)
- KEP Release Signoff Checklist: "(R) KEP approvers have approved the KEP status as `implementable`", "(R) Graduation criteria is in place", "(R) Production readiness review completed/approved", with (R) items "required *prior to targeting to a milestone / release*". — [KEP template README](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/README.md)
- PEP: a single field holds both axes (Accepted, then Final; Provisional). — [PEP 12](https://peps.python.org/pep-0012/); [PEP 1](https://peps.python.org/pep-0001/)
- Rust: an accepted RFC gets a tracking issue for its implementation. The stabilization report summarises "The main design decisions and deviations since the RFC was accepted", and notes "the final stabilized language feature has significant design deviations from the original RFC". — [rust-lang/rfcs README](https://raw.githubusercontent.com/rust-lang/rfcs/master/README.md); [rustc-dev-guide stabilization guide](https://rustc-dev-guide.rust-lang.org/stabilization-guide.html)
- Oxide: published (agreed, merged) versus committed (implemented). — [Oxide RFD 1](https://rfd.shared.oxide.computer/rfd/0001)
- MADR "Confirmation" and ecADR "Realization and review plan" put the realisation check inside the decision, as a plan, not as a status. — [MADR](https://raw.githubusercontent.com/adr/madr/develop/template/adr-template.md); [ecADR](https://ozimmer.ch/practices/2020/05/22/ADDefinitionOfDone.html)
- Kruchten distinguishes "Decided" from "Approved". This is a two-step agreement axis (the team decides, the authority approves), which matches BR-004's split between the analyst and the product owner. — [Kruchten excerpt](https://es.scribd.com/document/285377833/Arc-Design-Deci) (unverified copy)

### Inferences
- **The agreement axis** is `status`, with values per type (see the proposal). It stays a human decision, as BR-004 requires.
- **The realisation axis is computed**, not stored:
  - *unconfirmed*: accepted, with no `ver` from a verification record.
  - *confirmed*: at least one TC with `ver: DEC-…` exists (F2: any item with a `ver` link is a verification record).
  - *verified*: requires imported results (B2 and DD-4), later.

  The Rust model (realisation tracked elsewhere) and MADR's Confirmation both support linking rather than restating. A stored `stage:` key, as in KEP, is worth it only once results can be imported. Otherwise two fields describe the same fact and drift apart.
- **Per-type status values make FR-015 obsolete.** It fixes `draft`, `review`, `approved` and `deprecated` for all types. The traceability ADR must replace FR-015 with a mapping: each type's values map onto the phases that FR-109, FR-400, FR-403 and FR-404 read, for example `accepted` to "approved-like" and `superseded` and `rejected` to "deprecated-like".

### Gaps
- No source compares one-field and two-field designs empirically. The recommendation is a design inference from requimd's existing `ver` semantics.

---

## Which link names do the sources and SysML v2 use, and which minimal set covers the cases?

### Takeaway
Across the sources the link meanings cluster into seven:

1. what the decision is *about* (addresses, in the context of, pertains to)
2. what *shapes* it (drivers, constraints, assumptions)
3. *replacement* (supersedes or replaces, amends)
4. *dependency* (requires or depends-on)
5. *evidence* (spike, proof of concept)
6. *answering* an open question
7. design *satisfaction* (SysML `satisfy … by`)

SysML v2 contributes `subject`, `assume` versus `require` constraints, `satisfy` and derivation. requimd's convention is that the downstream item carries the link and points upstream (FR `impl` UC, TC `ver` FR). Following it gives a minimal set of eight new link keys, with inverse names shown in hover and on the site, never written by hand.

### Cited Findings
- MADR: "Decision Drivers" are qualities, concerns, constraints or forces; supersession is written as "superseded by ADR-0123". — [MADR template](https://raw.githubusercontent.com/adr/madr/develop/template/adr-template.md)
- PEP: `Requires`, `Replaces`, `Superseded-By`, `Resolution` (URL), `Discussions-To`. The forward link and its inverse are both *written* by hand. — [PEP 12](https://peps.python.org/pep-0012/)
- KEP: `see-also`, `replaces`, `superseded-by` (written on the replaced KEP). — [KEP-0000](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/sig-architecture/0000-kep-process/README.md)
- Rust: substantial changes "should be new RFCs, with a note added to the original RFC"; "Only very minor changes should be submitted as amendments". — [rust-lang/rfcs README](https://raw.githubusercontent.com/rust-lang/rfcs/master/README.md)
- Google design docs: amendments linked from the original. — [Design Docs at Google](https://www.industrialempathy.com/posts/design-docs-at-google/)
- requimd's own ADRs: "Amends ADR-0007" and, on ADR-0007, "Amended by ADR-0010": both directions written by hand. — `docs/decisions/0010-html-comments-outside-the-hash.md`, `docs/decisions/0007-lock-file.md` (local)
- SysML v2:
  - `subject vehicle : Vehicle;` binds a requirement to the element it is about, and child requirements inherit it.
  - `assume constraint` is "taken for granted", not verified by the system; `require constraint` must be guaranteed "provided the assumptions hold". If an assumption fails, the guarantee is void.
  - `assert satisfy brakingReq by brakes;` is "the primary traceability link between requirements and design".
  - Flow-down uses `specializes`.

  — [Ran Wei SysML v2 tutorial, Module 5](https://wrwei.github.io/tutorials/sysml/SysMLv2_Module5_Requirements_EN.html)
- SysML v2 derivation is a library connection (`#derivation`, one original to many derived requirements), not a keyword. — [No Magic SysML v2 docs](https://docs.nomagic.com/SYSML2P/2026x/derive-requirement-254423094.html) (phase 1)
- ISO/IEC/IEEE 42010: a decision *pertains to* concerns, *affects* elements, and can *raise* concerns. — [quality.arc42.org on ISO 42010](https://quality.arc42.org/standards/iso-42010) (phase 1, secondary)
- Zimmermann et al. 2009 model decisions as directed acyclic graphs with typed edges for containment and refinement, decomposition and triggering, plus logical relations such as incompatibility between alternatives. On that formal basis integrity constraints can be checked. — [IBM Research abstract, JSS 82(8) 2009](https://www.research.ibm.com/publications/managing-architectural-decision-models-with-dependency-relations-integrity-constraints-and-production-rules). Exact edge names were not verified.
- Kruchten's ontology defines "as many as ten different kinds of relations" between decisions. — [search summary of secondary sources](https://annals-csis.org/Volume_1/pliks/70.pdf). The names are unverified.

### Inferences
- **Minimal link set.** Each new key is written on the downstream item, and its inverse is computed.

  | Key (written on) | Target types | Inverse name (shown) | Source of the idea |
  |---|---|---|---|
  | `addresses` (DEC) | UC, UC step, FR, US, BR | addressed by | adr.github.io "addresses", Y-statement context, 42010 pertains to, Tyree/Akerman related requirements |
  | `driven-by` (DEC) | NFR, BR | drives | MADR decision drivers, Tyree/Akerman constraints |
  | `assumes` (DEC, any requirement) | ASM | assumed by | SysML v2 `assume constraint`, Tyree/Akerman assumptions |
  | `answers` (DEC, SPK) | Q | answered by | PEP Open Issues, Rust unresolved questions |
  | `evidence` (DEC) | SPK | evidence for | ecADR Evidence |
  | `supersedes` (DEC) | DEC | superseded by | MADR, PEP Replaces, KEP replaces, Fowler |
  | `amends` (DEC) | DEC | amended by | Rust amendments, Google amendments, requimd ADR-0010 |
  | `about` (Q, SPK) | any item or step | questions / investigated by | KEP UNRESOLVED blocks scoped to a section; Oxide ideation |
  | `satisfies` (DES, later) | FR, NFR, US, DEC | satisfied by | SysML v2 `satisfy … by` |

  The existing `ver` (TC to DEC or DES, as MADR Confirmation), `ref` and `impl` stay as they are.
- **Not recommended:**
  - `blocks`: a gate on `Q` is more precise. It states *which* status a question blocks, as Rust's "before merge" or "before stabilization" do.
  - `depends-on` between decisions: PEP `Requires` exists, but `ref` with the existing cycle check (FR-108) covers it until a project asks.
  - `raises`: Twin Peaks, 42010. It would point downstream against requimd's convention; the new item carries `ref: DEC-…` instead.
- **Computed inverses remove a known pain.** PEP, KEP and requimd's own ADRs write both directions by hand ("Amends" / "Amended by", "Replaces" / "Superseded-By"). requimd can show "superseded by DEC-009" in hover and on the site without the analyst touching the old item. That matters because the old item is immutable once accepted.
- **SysML alignment for a later export.** `satisfies` maps to `satisfy … by`. `assumes` maps to `assume constraint`, though at item level, not as an expression. A requirement's `subject` maps to the DES it is about. `driven-by` and `addresses` have no SysML v2 equivalent and would export as plain dependencies.

### Gaps
- Exact Kruchten and Zimmermann 2009 relation names are unverified (no full text).
- No empirical data was found on which decision link kinds practitioners actually use.

---

## How should re-evaluation propagate through the lock file?

### Takeaway
Research and practice agree that decisions decay. The literature calls the loss of rationale *architectural knowledge vaporization*; ecADR says "ADs age"; Kruchten names a "Challenged" state; and Fowler and product decision logs record re-evaluation triggers. No tool found propagates requirement changes to decisions. requimd gets most of it for free: FR-116 makes any *link* suspect when its target's content changes, so a DEC's `addresses` and `driven-by` links already flag re-evaluation. Two things are missing:

1. **Status-driven propagation.** FR-116 leaves `status` out of the hash, so an assumption turning `invalidated` or a decision turning `superseded` triggers nothing. This needs FR-109-style status rules, not hashes.
2. **Immutability of accepted decisions.** This needs a hash of the item *itself*, which the lock file does not store today: `[links]` stores only target hashes.

### Cited Findings
- **Knowledge vaporization (Jansen and Bosch, WICSA 2005).** High change cost, complexity and erosion are "partially due to knowledge vaporization". Making decisions "an explicit part of a software architecture" reduces it. Decision knowledge is "implicitly embedded in the architecture" and lacks "a first-class representation". — [search summary of Jansen and Bosch 2005 and follow-ups (arXiv 1704.04798, bibsonomy)](https://www.bibsonomy.org/bibtex/21aa2dfe9e83593c3d5b81b0112da625c/leonardo)
- Later work recovers lost decisions by mining issue and code histories (UnArch; Shahbazian et al. ICSA 2018, which traces decisions through project histories). — [Shahbazian et al. 2018](https://www.cs.washington.edu/homes/brun/pubs/pubs/Shahbazian18icsa.pdf); [arXiv 1704.04798](https://arxiv.org/pdf/1704.04798) (abstract-level)
- **Decision evolution (Capilla et al.).** The ADDSS research tool captured decisions and kept trace links between them and other artefacts. Capilla, Nava and Dueñas (SHARK/ADI 2007) modelled "the evolution of architectural design decisions". Related work stresses that there has been "little emphasis on the evolution of design decisions", and that managing ADD evolution "helps to maintain consistency between requirements and the deployed system". — [Capilla and Nava, core.ac.uk](https://core.ac.uk/download/357308011.pdf); [Swinburne figshare record](https://figshare.swinburne.edu.au/articles/conference_contribution/Attributes_for_characterizing_the_evolution_of_architectural_design_decisions/26250155); [ICSE 2013 doctoral abstract](https://2013.icse-conferences.org/content/approach-documenting-and-evolving-architectural-design-decisions.html). A 2023 paper notes that ADDSS was not developed further. — [SciTePress 2023](https://www.scitepress.org/Papers/2023/118480/118480.pdf)
- **Assumptions become invalid.**
  - Yang, Liang and Avgeriou (ECSA 2017, 24 architects): assumptions are "context-dependent and have a dynamic nature", some "turning out to be invalid or vice versa during their lifecycle". They connect to requirements and design decisions. "Neither the term nor the concept of architectural assumption is commonly used in industry." — [Chalmers research record](https://research.chalmers.se/en/publication/252320)
  - Survey of 112 practitioners: few record assumptions, mainly for lack of approaches and tools. — [RUG record, JSS 2016](https://research.rug.nl/en/publications/a-survey-on-software-architectural-assumptions/)
  - Systematic mapping study (IST 2018): most negative consequences come from invalid or implicit assumptions. — [mapping-study listing](https://www.bohrium.com/paper-details/assumptions-and-their-management-in-software-development-a-systematic-mapping-study/813096835148152833-105711) (secondary listing, abstract-level)
  - Their Architectural Assumption Management process has four activities: making, describing, evaluating and maintaining. Practitioners found evaluating the most time-consuming and the most helpful. — search summary of the SCP 2018 paper (same research group). Primary text not fetched.
- **Kruchten "Challenged":** a previously approved decision "now in jeopardy" that may return to approved "without ceremony" or be demoted. — [Kruchten excerpt](https://es.scribd.com/document/285377833/Arc-Design-Deci) (unverified copy)
- **Fowler (March 2026):** record the confidence level and the "product context changes that should prompt the team to reevaluate". Accepted ADRs are not edited; a change is a new, superseding ADR. — [Fowler, ArchitectureDecisionRecord](https://martinfowler.com/bliki/ArchitectureDecisionRecord.html) (phase 1)
- **ecADR:** plan when to revisit or expire a decision; "ADs age". — [ecADR](https://ozimmer.ch/practices/2020/05/22/ADDefinitionOfDone.html)
- **Product decision logs** separate a revisit *date* from a revisit *trigger*, the event or evidence that should reopen the choice. If nothing would reopen it, the entry should say it stands until the project ends. — [Boardmix decision log article](https://boardmix.com/articles/decision-log-template/); [Quire blog, 18 June 2026](https://quire.io/blog/p/the-decision-log.html) (vendor blogs, secondary quality)
- **Immutability policies differ.**
  - Strict (ADR tradition): Fowler, Nygard.
  - "Should not be substantially changed"; minor amendments allowed: Rust. — [rust-lang/rfcs README](https://raw.githubusercontent.com/rust-lang/rfcs/master/README.md)
  - Living until implemented: KEP, where "implemented" means "no longer actively changed". — [KEP-0000](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/sig-architecture/0000-kep-process/README.md)
  - Published but still updatable through a PR: Oxide. — [Oxide RFD 1](https://rfd.shared.oxide.computer/rfd/0001)
- **requimd mechanics (local).**
  - FR-116 hashes the target's title, metadata except `status`, and body, per link.
  - ADR-0007 `[links]` keys are `<source> <key> <target>`; only a review changes an existing hash (FR-218 code action, FR-307 CLI).
  - FR-109 is a status-based check that already exists.

  — `requirements/functional/checks.md`, `docs/decisions/0007-lock-file.md`

### Inferences
Mechanism, in four parts:

1. **Requirement changes, so the decision needs re-evaluation.** Nothing new in the lock file. `DEC-004 addresses FR-104` is a link like any other; when FR-104's hash changes, FR-116 reports it at the DEC. New are:
   - an item-level aggregate: "DEC-004 needs re-evaluation: 2 suspect links (FR-104, NFR-003)", as a diagnostic on the heading, in hover and on the site;
   - a wording in FR-218 that, for a DEC, "mark as reviewed" means "decision still holds" (re-confirmed).

   If it no longer holds, the analyst writes a superseding DEC. This is Kruchten's "Challenged" as a *computed* flag, so no status edit is needed and BR-004 is untouched.
2. **Assumption invalidated, so dependent items are suspect.** A status change does not change the hash, so this is a status rule, check `invalid-assumption`. Any item with `assumes: ASM-x`, where ASM-x has status `invalidated`, gets a warning at the link. It mirrors FR-109. Content changes to an ASM still flow through FR-116 as usual.
3. **Decision superseded, so items pointing at it are suspect.** This is also a status rule, check `superseded-target`. A `satisfies`, `ver`, `assumes` or `addresses`-inverse link that targets a DEC with status `superseded`, `deprecated` or `rejected` gets a warning naming the successor ("DEC-004 is superseded by DEC-009"). `ref` and `supersedes` links are exempt, since history is allowed. This extends FR-109 from "approved links to draft or deprecated" to "anything links to a dead decision".
4. **Accepted decision edited, so it is flagged.** This needs a new lock section, for example:

   ```toml
   [accepted]
   "DEC-004" = "5b1e0c9a7d23f418"
   ```

   - `requimd check` records the item's own hash, by the same FNV-1a rules as ADR-0007, the first time it sees the item as `accepted`.
   - Like `[links]`, it never changes an existing entry by itself.
   - A differing hash raises `decision-changed` (warning).
   - The fix is a superseding or amending DEC. If the edit was editorial, an explicit "record as editorial" action (code action or CLI, the FR-218/FR-307 analogue) updates the hash, and the lock-file diff in the PR makes it visible.

   Keys are sorted one per line, so merge conflicts stay line-local, as in ADR-0007.
- **Propagate one hop only.** FR-116 is per link. Transitive propagation (FR changes, so DEC is suspect, so DES `satisfies` DEC is suspect) would flood diagnostics and has no precedent in the sources. Impact analysis (LSP, `requimd diff`) can show the chain on demand. DD-3 should confirm this against Doorstop and ReqView.
- **Time-based triggers stay opt-in.** A `revisit:` date is deterministic only with an explicit `--as-of <date>`, keeping NFR-006 and deterministic output (phase 1, R-66). A free-text `trigger:` body field (product-log style) costs nothing and is checkable by humans only.

### Gaps
- No tool or paper was found that propagates requirement changes to decisions through stored hashes. This is absence of evidence within this search, consistent with phase 1.
- The full texts of the Capilla evolution model, the Yang et al. AAM process and Jansen and Bosch were not read (abstract and summary level only). Their exact attribute lists for "evolving decisions" are unknown.
- Whether the `[accepted]` section raises the lock-file merge burden noticeably is unknown (DD-3).

---

## How do open questions gate progress, and how could a question block an item?

### Takeaway
Rust and KEP gate on open questions; PEP only lists them:

- **Rust RFCs** split unresolved questions into those that must be resolved *before merge* (acceptance) and those *before stabilization* (realisation). The stabilization report then lists deviations since acceptance.
- **KEPs** mark text "under active debate" with inline `<<[UNRESOLVED …]>>` … `<<[/UNRESOLVED]>>` blocks, and use a signoff checklist whose (R) items are required before targeting a release.
- **PEPs** have an "Open Issues" section ("Any points that are still being decided/discussed"), but PEP 12 states no rule that blocks acceptance while it is non-empty.

So the precedent is a question with a *named gate*, and requimd can express that as a key on the question.

### Cited Findings
- Rust template: unresolved questions to resolve "through the RFC process before this gets merged" versus "through the implementation of this feature before stabilization". — [rust-lang/rfcs 0000-template.md](https://raw.githubusercontent.com/rust-lang/rfcs/master/0000-template.md) (phase 1)
- Rust stabilization report: "The main design decisions and deviations since the RFC was accepted"; deviations are acceptable but must be highlighted; a final comment period (FCP) on the stabilization PR, proposed with "@rfcbot fcp merge". — [rustc-dev-guide stabilization guide](https://rustc-dev-guide.rust-lang.org/stabilization-guide.html)
- KEP template: "You can denote sections that are under active debate as follows:" `<<[UNRESOLVED optional context]>>` … `<<[/UNRESOLVED]>>`. A `provisional` KEP "is a working document and subject to change". The template does not say when UNRESOLVED blocks must be resolved. — [KEP template README](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/README.md)
- KEP beta criteria must include "resolving all issues and gaps identified", a gate on open issues at a stage. — [KEP template README](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/NNNN-kep-template/README.md)
- KEP-0000 itself ends with an "Unresolved Questions" section, for example "How reviewers and approvers are assigned to a KEP". — [KEP-0000](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/sig-architecture/0000-kep-process/README.md)
- PEP Open Issues: "Any points that are still being decided/discussed." No acceptance rule tied to it was found. — [PEP 12](https://peps.python.org/pep-0012/)
- Oxide "ideation": a topic and scope only, which others can pick up. It is a question-like pre-state of a design. — [Oxide RFD 1](https://rfd.shared.oxide.computer/rfd/0001)

### Inferences
- **A question item (`Q`)** has status `open`, `answered` or `obsolete`, and an optional `gate:` key naming the status that its `about` targets may not reach while the question is open.
  - `gate: review` is like Rust "before merge": the item cannot go to review with this open.
  - `gate: approved` is like Rust "before stabilization": it can be reviewed but not approved.
  - For a DEC target, the gate value is `accepted`.
- **Check `open-question-gate`** (warning by default, error configurable through FR-117). It reports at the target's `status:` value when the target's status is at or past the gate while a Q `about` it is `open`. requimd never changes status. The check only reports, and CI (`check --locked`, errors) turns it into a merge gate, which keeps BR-004 intact. This needs an *order* on each type's status values (`draft < review < approved`; `proposed < accepted`), which the per-type status ADR must define.
- **Answered questions need a trace.** `answered` without any `answers` inverse (from a DEC, an SPK or a changed requirement with `ref: Q-…`) gets an info diagnostic. Otherwise "answered" becomes an unsupported claim.
- **Inline unresolved markers** (KEP style) would fit requimd's HTML-comment mechanism (ADR-0010 keeps comments out of the hash; FR-119 has inline markers), for example `<!-- requimd: open Q-003 -->`. An item is the better first step, since it is linkable, listable and shows on the site. Inline markers could follow as a "mark this paragraph" convenience.
- **Language server.** Hover on a requirement lists its open questions with their gates. A code action "New question about this item" creates `## Q-? …` with `about:` pre-filled (FR-010 placeholder).

### Gaps
- No Python source defines whether a PEP may be accepted with open issues; the practice is unknown.
- No source defines a standard open-question register with states. The states above are synthesis, as in phase 1.

---

## Business decisions versus architecture decisions: where should analysts start?

### Takeaway
Analysts should start with business and requirements decisions (scope, policy, rule and option choices) and open questions. Kruchten already counts "executive" (business, process, organisational) decisions as design decisions. BABOK's decision analysis and the DACI and product decision-log templates use the same core fields as ADRs (problem, options, criteria, outcome, owner, revisit), and requimd's own decision log is mostly product decisions. One decision type with an optional `kind` serves both audiences. A separate type for each audience would duplicate the lifecycle.

### Cited Findings
- **BABOK v3 (10.16 Decision Analysis).**
  - Purpose: to formally assess "a problem and possible decisions in order to determine the value of alternate outcomes under conditions of uncertainty". — quoted in [open-exam-prep CCBA guide](https://open-exam-prep.com/study-guides/ccba/babok-techniques/root-cause-decision-techniques) (secondary)
  - Activities: define the problem statement, define alternatives, evaluate them against agreed evaluation criteria, choose, implement. Components include consequences, outcome value, decision tables and decision trees; multi-criteria analysis handles conflicting criteria. — [Fhyzics BA technique: decision analysis](https://blog.fhyzics.net/fibaar/ba-technique-decision-analysis) (secondary; cites BABOK but not the section text)
  - BABOK lists evaluation criteria, decision modelling and decision analysis as overlapping techniques. — [search summary (BA Times, Packt)](https://www.batimes.com/?p=6138) (secondary)
- **DACI (Atlassian):** status, impact, driver, approver, contributors, informed, due date, outcome, background, options with pros, cons and cost, action items. — [Atlassian decision template](https://www.atlassian.com/software/confluence/templates/decision)
- **Product decision logs** converge on ID, date, decision, owner, status, alternatives, rationale, and a revisit date or trigger. A stable ID lets tickets and release notes point to the same decision. — [Remio product decision record](https://www.remio.ai/post/product-decision-record-template); [Docmost guide](https://docmost.com/blog/how-to-document-product-decisions/); [Boardmix](https://boardmix.com/articles/decision-log-template/) (vendor blogs, secondary quality)
- **Kruchten:** executive decisions are one of three top categories, beside existence and property decisions. — [secondary summaries via search](https://annals-csis.org/Volume_1/pliks/70.pdf)
- **adr.github.io:** architectural knowledge management "can extend to design and other decisions". — [adr.github.io](https://adr.github.io/) (phase 1)
- **Fowler:** ADRs are a vehicle of the "Advice Process" (gathering advice before deciding), which suits facilitating analysts. — [Fowler](https://martinfowler.com/bliki/ArchitectureDecisionRecord.html) (phase 1)
- **Local:** `docs/status.md` has about 77 dated decision rows with a "where recorded" column, mostly owner decisions on scope and policy (F1, F2, F3, C2, D2, D7). This is a business-decision log in practice.

### Inferences
- **DEC with `kind: business | requirements | architecture`** (configurable value set). A business decision usually `addresses` a UC, US or BR, and is `driven-by` business rules. An architecture decision is `driven-by` NFRs. Same lifecycle, same checks.
- **The Y-statement suits business decisions.** "In the context of UC-012.s2, facing …, we decided for … and neglected …, to achieve BR-007, accepting …" puts the requirement links in the sentence and is checkable by the language engine. Clauses: EN "In the context of / facing / we decided for / and neglected / to achieve / accepting"; DE terms to be validated with the owner (cards42 uses a German card).
- **Decision accountability** keys (`decided-by`, like the DACI approver; `consulted`) stay optional metadata. For analysts, the who matters more than for developers. BR-004 already names the product owner as approver for requirements; a DEC `accepted` could follow the same rule.

### Gaps
- No primary BABOK text was available (members only). The element list above is from secondary study guides.
- No study was found of analysts or product managers (as opposed to architects) using decision records. Product decision-log sources are vendor blogs.

---

## Spikes: which fields do teams record?

### Takeaway
No canonical primary-source spike template exists. SAFe defines spikes as enabler stories that are "estimated, implemented and demonstrated" to "reduce the risk of a technical approach". Practitioner templates converge on hypothesis or question, approach, expected outcome, success criteria fixed in advance, timebox and share-back date. The outcome is one of three: a decision, a follow-up story, or discarding the spike. For requimd, a spike is a small inquiry item whose verdict feeds a decision (`evidence`) or an assumption's status.

### Cited Findings
- SAFe: "Spikes are a type of SAFe Enabler Story", for "exploration, architecture, infrastructure, research, design, and prototyping", to "gain the knowledge necessary to reduce the risk of a technical approach". "Like other stories, spikes are estimated, implemented and demonstrated." Technical and functional spike types exist; the detail is behind a login. — [SAFe Spikes](https://framework.scaledagile.com/spikes)
- Taiga template: "We believe that…" (hypothesis), "We will…" (approach), "We expect to…" (expected outcome), "We will consider ourselves right if…" (success criteria), "The spike will be timeboxed to [duration]" plus a date for sharing learnings. Outcomes: an informed decision, promotion to a story, or discard. — [Taiga community: What is a spike in Agile?](https://community.taiga.io/t/what-is-a-spike-in-agile/1451)
- Secondary sources: the output is knowledge, not software, and often an estimate for the original story; a timebox of 1 to 3 days is suggested; without one a spike becomes "a bottomless pit". — [search summary (plane.so, agilemania, softwareplanetgroup)](https://plane.so/blog/what-is-a-spike-in-agile-definition-types-and-examples) (secondary)
- Hypothesis cards fix the success signal before the experiment. — [Thoughtworks](https://www.thoughtworks.com/insights/articles/how-implement-hypothesis-driven-development) (phase 1)
- ecADR names a spike or proof of concept as Evidence for a decision. — [ecADR](https://ozimmer.ch/practices/2020/05/22/ADDefinitionOfDone.html)
- The XP original (extremeprogramming.org "spike solution") could not be fetched because of a certificate error.

### Inferences
- **SPK fields:**
  - metadata: `about` (Q or ASM), `timebox`, `status` (`planned`, `done`, `abandoned`) and `verdict` (`supported`, `refuted`, `inconclusive`)
  - body fields: `hypothesis`, `signal` (success criteria, written before the spike runs), `result`
- **Check `spike-verdict`:** a `done` SPK without a `verdict` or `result`. A check `spike-signal` that flags a missing `signal` is optional; it enforces "fix the signal in advance".
- **No automatic status change on ASM or Q** when a spike finishes. The analyst sets `validated` or `answered` (BR-004 spirit). An info diagnostic suggests it: "SPK-001 supported ASM-002; ASM-002 is still assumed".

### Gaps
- No primary-source spike record template (SAFe details are behind a login; XP page unreachable).

---

## Proposal for requimd

### Takeaway
One ADR, "Traceability model for the design level", introduces five types in three steps:

- **Step 1:** decisions (DEC) and open questions (Q). Decisions get supersession, amendment, immutability and re-evaluation through existing suspect links; questions get gates.
- **Step 2:** assumptions (ASM) with status-driven propagation, and spikes (SPK) as evidence.
- **Step 3:** design elements (DES) with `satisfies`.

Agreement is the `status` axis per type. Realisation is computed from `ver`, not stored. Constraints stay BR or NFR; risks stay a body section. The examples below follow requimd's format: heading with ID and title, metadata lines up to the first blank line (FR-003), and body fields (FR-005). Options use labels like use-case steps, which would need a spec change: FR-006 currently applies to use cases only.

### Item types and example Markdown

**Decision (DEC), step 1**

```markdown
## DEC-004 Round VAT per line item, not per order
addresses: FR-104, UC-012.s2  driven-by: BR-007, NFR-003
answers: Q-003  assumes: ASM-002  evidence: SPK-001
status: accepted  kind: business  decided-by: Product owner
revisit: 2027-11-01

In the context of the cart total (UC-012.s2), facing totals that differ
between cart and invoice, we decided for rounding VAT per line item and
neglected rounding per order, to achieve totals that match the ERP
invoice (BR-007), accepting cent differences against a manual total.

options:
- o1: Round per line item. Matches the ERP; cent differences possible.
- o2: Round per order. Exact total; the invoice differs from the ERP.

outcome: o1

consequences:
- Support needs a help text on cent differences.

trigger: The ERP changes its rounding, or BR-007 changes.
```

Status values: `draft` (Oxide ideation, PEP Draft), `proposed` (MADR, Nygard), `accepted`, `rejected`, `superseded`, `deprecated`. Only a human sets `accepted` (the BR-004 analogue). Tools and AI create DECs as `draft` or `proposed` (ADR Guard practice, phase 1). Realisation is computed: *unconfirmed* while no item links with `ver: DEC-004`, *confirmed* once one does:

```markdown
## TC-031 Cart and invoice totals match
ver: DEC-004, FR-104  method: inspection
```

Superseding and amending:

```markdown
## DEC-009 Round VAT per order after the ERP migration
addresses: FR-104  supersedes: DEC-004  assumes: ASM-005
status: proposed  kind: business
...

## DEC-011 Rounding applies to shipping costs too
amends: DEC-004  status: accepted
...
```

DEC-004 stays unchanged. Once DEC-009 is `accepted`, the analyst sets DEC-004 to `superseded`, and hover and the site show "superseded by DEC-009" (computed inverse).

**Open question (Q), step 1**

```markdown
## Q-003 Which rounding does the ERP use?
about: FR-104, UC-012.s2  gate: approved
status: open  ask: Finance

FR-104 cannot be approved until we know whether the ERP rounds per line
or per order; the invoice must match the cart.
```

Status values: `open`, `answered`, `obsolete`. The question is answered by a DEC or SPK with `answers: Q-003`, or by a requirement change with `ref: Q-003`.

**Assumption (ASM), step 2**

```markdown
## ASM-002 The ERP rounds VAT per line item
status: assumed  owner: Finance  check-by: 2026-12-15

If this is wrong, DEC-004 and FR-104 need to be revisited.
```

Status values: `assumed`, `validated`, `invalidated`. The `assumes` link can come from a DEC or from any requirement, which covers SysML `assume constraint` at item level.

**Spike (SPK), step 2**

```markdown
## SPK-001 Compare cart and ERP totals for 200 orders
about: ASM-002, Q-003  timebox: 2d
status: done  verdict: supported

hypothesis: We believe the ERP rounds VAT per line item.

signal: We will know when at least 195 of 200 sample invoices match a
per-line calculation to the cent.

result:
- 200 of 200 matched.
- Credit notes round per order; see Q-007.
```

Status values: `planned`, `done`, `abandoned`. Verdict values: `supported`, `refuted`, `inconclusive`.

**Design element (DES), step 3**

```markdown
## DES-002 Tax calculation module
satisfies: FR-104, DEC-004  status: draft

Calculates VAT per line item for cart, checkout and invoice export.
```

DES uses the generic status set (`draft`, `review`, `approved`, `deprecated`). A requirement could carry `subject: DES-002` (SysML `subject`), which is optional and later.

**Not new types:** constraints are BR (business) or NFR (quality) items, linked with `driven-by`. Risks are a body section ("risks:") of a DEC or DES until a RAID-style register is requested.

### Link vocabulary (written on the downstream item; inverses computed)

| Key | From | To | Inverse shown | Feeds suspect check |
|---|---|---|---|---|
| `addresses` | DEC | UC, step, FR, US, BR | addressed by | yes (FR-116) |
| `driven-by` | DEC | NFR, BR | drives | yes (FR-116) |
| `assumes` | DEC, FR, NFR, US, BR | ASM | assumed by | yes (FR-116), plus `invalid-assumption` |
| `answers` | DEC, SPK | Q | answered by | yes |
| `evidence` | DEC | SPK | evidence for | yes |
| `supersedes` | DEC | DEC | superseded by | no (history) |
| `amends` | DEC | DEC | amended by | no (history) |
| `about` | Q, SPK | any item or step | questions / investigated by | yes |
| `satisfies` (step 3) | DES | FR, NFR, US, DEC | satisfied by | yes |
| `ver` (existing) | TC | DEC, DES (new targets) | verified by | yes |

### Checks

| Check ID | Rule | Default severity | Precedent |
|---|---|---|---|
| `suspect-link` (existing FR-116) | Covers all new links; on a DEC, the diagnostic reads "re-evaluate", and FR-218 means "decision still holds" | warning | Fowler triggers, ecADR "ADs age", Kruchten Challenged |
| `decision-re-evaluate` | Item-level summary on the DEC heading when any `addresses`, `driven-by` or `assumes` link is suspect | info | same |
| `decision-changed` | Accepted DEC's own hash differs from the `[accepted]` entry in the lock file | warning | Fowler and Nygard immutability, Rust "should not be substantially changed" |
| `supersession` | `supersedes` target must be `superseded` once the source is `accepted`; a `superseded` DEC must have an accepted successor; no supersession cycles; no two accepted successors | error (cycle), warning (others) | PEP Replaces/Superseded-By, KEP replaced |
| `superseded-target` | Link other than `ref`, `supersedes` or `amends` to a `superseded`, `deprecated` or `rejected` DEC; the message names the successor | warning | extends FR-109 |
| `invalid-assumption` | Link `assumes` to an ASM with status `invalidated` | warning | Yang et al. (invalid assumptions cause most harm), SysML assume |
| `open-question-gate` | Target status at or past the Q's `gate` while the Q is `open` | warning (error configurable) | Rust before merge / before stabilization; KEP UNRESOLVED and signoff |
| `answer-trace` | Q `answered` without an `answers` inverse or `ref` from a changed item | info | PEP Resolution |
| `decision-driver` | DEC without `addresses` or `driven-by` | info | adr.github.io "addresses", Y-statement context |
| `spike-verdict` | SPK `done` without `verdict` or `result` | warning | Taiga template, Thoughtworks |
| `revisit-due` (opt-in, `--as-of <date>`) | `revisit:` or `check-by:` date passed | info | ecADR, MADR "when to revisit", product logs |

### Lock file

- **Unchanged:** `[ids]` (now also DEC, Q, ASM, SPK, DES), and `[links]`, which covers every new link key so re-evaluation reuses FR-116 and FR-218.
- **New `[accepted]` section:** one sorted line per accepted DEC: `"DEC-004" = "<16 hex>"`, the item's own hash under the ADR-0007 rules (title, metadata except `status`, body; HTML comments excluded per ADR-0010).
  - Written silently the first time a check sees the item as `accepted`.
  - Never changed by a check.
  - Changed only by an explicit "record as editorial change" code action or CLI review, so the change is visible in the PR diff.
  - Removed when the item is no longer `accepted`.
- **Status-driven propagation** (`invalid-assumption`, `superseded-target`, `open-question-gate`) reads the current statuses in the files and needs no lock entries. It is deterministic and works offline.
- **Propagation goes one hop.** Chains appear in impact analysis on demand.

### Open questions for the product owner

1. **Prefixes.** `DEC` or `ADR` for decisions? `Q`, `OQ` or `QST` for questions? `ASM`, `SPK`, `DES`? Should requimd's own `docs/decisions/ADR-*` become items, which needs the metadata-line format or a reader for "Status: **accepted** on …" lines (R-70)?
2. **Step 1 scope.** DEC and Q only, or ASM in the same step? The research shows assumption invalidation is the strongest impact signal, but it needs the new status rule.
3. **Status values per type.** Do you accept `draft`/`proposed`/`accepted`/`rejected`/`superseded`/`deprecated` for DEC, `open`/`answered`/`obsolete` for Q, `assumed`/`validated`/`invalidated` for ASM, and `planned`/`done`/`abandoned` plus a verdict for SPK? This replaces FR-015's fixed status names (F4) with a per-type mapping onto the phases that FR-109, FR-400, FR-403 and FR-404 read.
4. **Who may accept.** Does BR-004 extend to `accepted` on decisions (product owner only)? What about `validated` on assumptions and `answered` on questions (any analyst)?
5. **Realisation.** Computed from `ver` only (unconfirmed or confirmed), or also a stored `stage:` or `realised:` key as in KEP and PEP Final?
6. **Immutability policy.** Strict (any change to an accepted DEC is a warning until superseded or amended), Rust-like (an explicit editorial escape), or Oxide-like (no immutability check)? Severity of `decision-changed`?
7. **Gate semantics.** Is `gate:` a status name of the target type (`review`, `approved`, `accepted`)? Which order of status values do you want per type? Warning or error by default?
8. **`addresses` versus `driven-by`.** Keep both (problem versus forces; clearer matrices), or merge into one key for analysts?
9. **`kind` of decision.** Is `business | requirements | architecture` the right value set? Should the site's decision log filter by it?
10. **Y-statement.** Optional compact form with a clause check? Which German clause terms (cards42 has a German card)?
11. **Option labels.** Extend FR-006 step labels to `o1`, `o2` in decisions (BR-003 stability), so options can be referenced as `DEC-004.o2`?
12. **Lock file.** Is a new `[accepted]` section acceptable, given the known rebase pain with `requimd.lock` (DD-3)?
13. **Revisit dates.** Keep `revisit:` and `check-by:` keys with an opt-in `--as-of` check, or only a free-text `trigger:` field?
14. **Design elements.** Step 3 at all, or leave design elements to code tracing (DD-4) and to SysML tools via export?

### Gaps
- Effort and value were not re-estimated; they follow phase 1 (R-54 to R-72).
- The check IDs and severities are proposals, not validated with users.
- The Kruchten state and relation names, the BABOK element list and the Capilla evolution model are secondary or unverified and must be checked before being cited in an ADR.
