# DD-8: German and English requirement quality rules (INCOSE GtWR v4, SOPHIST/Rupp MASTeR, IREB) and what requimd can check deterministically

Research date: 2026-11-18. Builds on phase 1 (`requirements-and-design-tools-2026.md`, R-09 to R-23; `notes/methods-and-standards.md`), which is not repeated here. Only public sources were used. No rule text is reproduced. Rule titles are given as short paraphrases, because the official INCOSE wording could not be read (INCOSE PDFs return HTTP 403 to automated fetches) and is copyrighted anyway.

Source caveats:
- Several PDFs (QRA "Automating the INCOSE Guide", the SOPHIST MASTeR brochure, the Rupp/Joppich chapter 10 excerpt, an FAU thesis, an NLP4RE 2019 poster) were fetched but their compressed text could not be extracted. Their content is not used, except where an HTML page or search snippet states it; such statements are marked.
- Statements marked **[background, unverified]** come from domain knowledge and were not confirmed in a source during this pass.

Format note: the caller asked for the notes to end with "Proposal for requimd". It is the last key-question section, with the same Takeaway/Cited Findings/Inferences/Gaps structure; the requested tables are under its Inferences.

## 1. What is the authoritative INCOSE rule list (GtWR v4 and any 2025-2026 edition), its characteristics, and which rules are deterministically checkable?

### Takeaway
The current edition is still GtWR v4 (INCOSE-TP-2010-006-04, 2023): 42 rules in 14 categories, 15 characteristics (C1-C9 single item, C10-C15 set). No 2025 or 2026 successor was found; "GfWR" (Guide *for* Writing Requirements) is the guide's older name, not a new edition. Rule numbers changed between editions (older material cites R43/R44 or "41 rules"), so a finding must cite the edition with the rule number.

### Cited Findings
- Edition: v4 is INCOSE-TP-2010-006-04; one secondary source dates it July 2023, another June 2023 (the v4 summary sheet is dated June 2023) — [reqi.io, published 2025-07-31, updated 2026-09-26](https://reqi.io/articles/incose-requirements-quality-42-rule-guide); [Jama, 2026-08-20](https://www.jamasoftware.com/requirements-management-guide/writing-requirements/incose-requirements-writing-guide/); [INCOSE v4 summary sheet (403 to fetch; listed by search)](https://www.incose.org/docs/default-source/working-groups/requirements-wg/guidetowritingrequirements/incose_rwg_gtwr_v4_summary_sheet.pdf)
- v4 has 42 rules and 15 characteristics; R40 (decimal format) and C15 (correct, for sets) are new in v4 — [Jama](https://www.jamasoftware.com/requirements-management-guide/writing-requirements/incose-requirements-writing-guide/)
- A v4 final-draft PDF exists on the INCOSE site (`incose_rwg_gtwr_v4_040423_final_drafts.pdf`), and a 2023 INCOSE webinar 168 presented v4 — [INCOSE draft PDF (listed by search, not readable)](https://www.incose.org/docs/default-source/working-groups/requirements-wg/gtwr/incose_rwg_gtwr_v4_040423_final_drafts.pdf); [INCOSE webinar 168](https://www.incose.org/events/webinars/!previous/2023/10/25/default-calendar/webinar-168-guide-to-writing-requirements-version-4)
- The "GfWR" webinar 144 ("Raising the Ante") is a 2020 webinar; the 2026 date in its URL is an upload date of the slides, not a new edition — [INCOSE webinar 144 event page, 2020-10-21](https://incose.org/events-master-hidden/2020/10/21/default-calendar/webinar-144-the-incose-guide-for-writing-requirements-raising-the-ante); [slides, uploaded 2026-01](https://www.incose.org/wp-content/uploads/2026/01/webinar_144_the_incose_gfwr_raising_the_ante.pdf) (403)
- A vendor document based on "GFWR V2" speaks of 41 rules in 14 categories — [QRA, Automating the INCOSE Guide](https://qracorp.com/guides_checklists/automating-the-incose-guide-for-writing-requirements/) (as summarised by search; PDF text not extractable)
- A course page cites INCOSE rules up to R43 and R44 under a heading that says "40 rules", evidence of older, different numbering in circulation — [kiranm516 sw-eng notes](https://kiranm516.gitlab.io/sw-eng/writing-requirements.html)
- The INCOSE RWG also publishes a Needs and Requirements Manual (NRM) next to the GtWR — [INCOSE RWG page, via search summary](https://incose.org/communities/working-groups-initiatives/requirements) (403 to fetch; version not confirmed)
- Rule list with categories (reqi.io, citing v4; titles below are short paraphrases, not INCOSE wording) — [reqi.io](https://reqi.io/articles/incose-requirements-quality-42-rule-guide):
  - Accuracy: R1 structured statement (agreed pattern), R2 active voice / responsible subject, R3 subject and verb fit the entity level, R4 defined terms, R5 definite articles, R6 units of measure, R7 vague terms, R8 escape clauses, R9 open-ended clauses
  - Concision: R10 superfluous infinitives, R11 separate clauses (one per condition)
  - Non-ambiguity: R12 grammar, R13 spelling, R14 punctuation, R15 logical expressions, R16 use of "not", R17 oblique symbol "/"
  - Singularity: R18 single thought, R19 combinators, R20 purpose phrases, R21 parentheses, R22 enumeration, R23 supporting diagrams/models
  - Completeness: R24 pronouns, R25 independence from headings
  - Realism: R26 absolutes
  - Conditions: R27 explicit conditions, R28 multiple conditions
  - Uniqueness: R29 classification, R30 unique expression
  - Abstraction: R31 solution-free
  - Quantifiers: R32 universal quantification ("each")
  - Tolerance: R33 range of values
  - Quantification: R34 measurable performance, R35 temporal dependencies
  - Uniformity of language: R36 consistent terms and units, R37 acronyms, R38 abbreviations, R39 style guide, R40 decimal format
  - Modularity: R41 related requirements grouped, R42 structured sets
- Characteristics: C1 necessary, C2 appropriate, C3 unambiguous, C4 complete, C5 singular, C6 feasible, C7 verifiable, C8 correct, C9 conforming; sets: C10 complete, C11 consistent, C12 feasible, C13 comprehensible, C14 able to be validated, C15 correct — [reqi.io](https://reqi.io/articles/incose-requirements-quality-42-rule-guide); C1-C7, C15 confirmed by [Jama](https://www.jamasoftware.com/requirements-management-guide/writing-requirements/incose-requirements-writing-guide/)
- Independent confirmation of individual numbers: R8 escape clauses, R10 superfluous infinitives ("be able to", "be capable of", "be designed to"), R20 purpose phrases ("in order to", "so that"), R32 universal qualification ("all", "any", "both" replaced by "each") — [search over INCOSE RWG material, Altium, workbench quick reference card](https://ssaleem74.github.io/incose-requirements-workbench/quick-reference-card.pdf); R3 subject-verb, R5 definite articles, R6 units appear in a 2019-edition index — [dokumen.pub index, via search snippet](https://dokumen.pub/incose-guide-for-writing-requirements-2019.html) (page now "under maintenance"); R1, R2, R4, R7, R8, R16, R18, R19, R31, R34, R36, R39, R40 named on [Jama](https://www.jamasoftware.com/requirements-management-guide/writing-requirements/incose-requirements-writing-guide/); R1, R2, R5-R10, R14, R18, R33, R34 shown as checks in the [INCOSE requirements workbench](https://ssaleem74.github.io/incose-requirements-workbench/)
- One source puts escape clauses at R9 instead of R8; the INCOSE RWG material and Altium use R8 — [search result comparison, kiranm516 vs Altium](https://kiranm516.gitlab.io/sw-eng/writing-requirements.html)

### Inferences
- The 42-rule, 14-category structure from reqi.io is consistent with every number confirmed independently above (no conflict found for v4), so it is a reliable basis for requimd rule IDs. Rule *titles* in requimd should be requimd's own short wording.
- Deterministic classification (refining phase 1 against the verified numbering):
  - **Lexical, word-list based (high value, low to medium false positives):** R7, R8, R9, R10, R16, R17, R20, R26, R32, R35, R38.
  - **Pattern or structure based (deterministic, medium false positives):** R1 (EARS/MASTeR/story pattern), R2 (passive), R6 (number without unit), R18 (more than one binding verb), R19 (combinators between clauses), R21 (parentheses), R24 (pronouns), R27/R28 (condition clause shape), R33/R34 (missing value or tolerance in NFRs), R37 (acronym not in glossary), R40 (decimal separator vs file language).
  - **Set-level, deterministic:** R4 and R36 (glossary and aliases), R29 (type metadata), R30 (exact or n-gram near-duplicates), R41/R42 (file and section structure), R25 (pronoun or "this function" at text start).
  - **Not deterministic or out of scope:** R3 (needs system-level knowledge), R5 (article choice; high noise), R11 (clause analysis), R12-R14 (grammar, spelling, punctuation: leave to editor tools, see phase 1 R-23), R15 (notation convention only), R22, R23, R31 (only a project-specific technology word list), R39 (the style guide is the rule profile itself).

### Gaps
- Official v4 rule titles and the v4 summary sheet could not be read (HTTP 403). The title list is a secondary paraphrase.
- The version and date of the Needs and Requirements Manual were not confirmed.
- No statement from INCOSE on a v5 or successor was found; absence of evidence, not confirmation.
- No vendor mapping that lists all 42 rules with "automatable yes/no" could be read (the QRA PDF was not extractable).

## 2. What are the current SOPHIST/Rupp MASTeR templates, their bindingness words and conditions, and the SOPHIST rule set of linguistic defects?

### Takeaway
The current SOPHIST MASTeR set (brochure "Satzschablonen für alle Fälle", 6th edition, 2024) has a FunktionsMASTeR with three activity types, three NFR templates (EigenschaftsMASTeR, UmgebungsMASTeR, ProzessMASTeR) and condition templates that split the ambiguous "wenn" into FALLS (logical), SOBALD (event) and SOLANGE (period). Bindingness is MUSS / SOLLTE / WIRD (shall / should / will); KANN is not a bindingness word. This differs from the 2009 LMU handout cited in phase 1 (muss / soll / wird, wenn / falls), so requimd should follow the current brochure.

### Cited Findings
- Editions: MASTeR brochure "Satzschablonen für alle Fälle", 6th edition, dated 2024-07-31, 52 pages, German; Rupp, *Requirements-Engineering und -Management*, Hanser, 6th ed. 2014 (ISBN 978-3-446-43893-4) and 7th ed. 2020 (ISBN 978-3-446-45587-0) — [Peterjohann Consulting, Satzschablonen](https://www.peterjohann-consulting.de/satzschablonen/); brochure PDF at [sophist.de](https://sophist.de/wp-content/uploads/2025/03/MASTeR-Broschuere-6-Auflage.pdf) (not text-extractable here)
- Bindingness: MUSS (mandatory; acceptance can be refused if unmet), SOLLTE (a wish, not required), WIRD (intention, future integration); KANN must not be used for bindingness; the keywords should be agreed with stakeholders and defined in the glossary — [Peterjohann](https://www.peterjohann-consulting.de/satzschablonen/); [SOPHIST brochure excerpt "Schablonen für alle Fälle" (older edition, ≤2016)](https://silo.tips/download/die-sophistenschablonen-fr-alle-flle-2)
- English equivalents given by SOPHIST: SHALL / SHOULD / WILL — [brochure excerpt](https://silo.tips/download/die-sophistenschablonen-fr-alle-flle-2)
- FunktionsMASTeR slots: optional condition, system, bindingness, then object with details, and the process verb last as infinitive; a leading condition moves the modal to second position. Activity types: independent system activity (modal + infinitive), user interaction ("<wem> die Möglichkeit bieten, … zu <Verb>"), interface requirement ("fähig sein, … zu <Verb>") — [brochure excerpt](https://silo.tips/download/die-sophistenschablonen-fr-alle-flle-2); [Peterjohann](https://www.peterjohann-consulting.de/satzschablonen/)
- Process verbs come from a project process-word list (Prozesswortliste) — [Peterjohann](https://www.peterjohann-consulting.de/satzschablonen/)
- Conditions: FALLS = logical condition (LogikMASTeR), SOBALD = event (EreignisMASTeR, perfect tense suggested), SOLANGE = period/state (ZeitraumMASTeR); WENN is replaced because it is ambiguous — [brochure excerpt](https://silo.tips/download/die-sophistenschablonen-fr-alle-flle-2); "falls" and "wenn" must be kept apart — [Peterjohann](https://www.peterjohann-consulting.de/satzschablonen/)
- EigenschaftsMASTeR: property of the object, modal verb, optional comparison, mandatory value; example: "leicht" becomes "kleiner gleich 100 Gramm"; "schnell" and "benutzerfreundlich" are named as vague words to replace with a checkable value — [SOPHIST blog, A. Grün, 2026-10-19](https://blog.sophist.de/2026/10/19/nicht-funktionale-anforderungen/)
- Comparison operators used in the EigenschaftsMASTeR: "kleiner gleich", "größer gleich", "mindestens", "maximal", "größer (als)", "von … bis" — [brochure excerpt](https://silo.tips/download/die-sophistenschablonen-fr-alle-flle-2)
- UmgebungsMASTeR: object + modal + "so gestaltet sein, dass" + subordinate clause ending in "betrieben werden kann" (the only place KANN appears); ProzessMASTeR: an actor such as the contractor (Auftragnehmer) is the subject, for activities and contractual duties — [brochure excerpt](https://silo.tips/download/die-sophistenschablonen-fr-alle-flle-2); the three NFR templates are confirmed for 2026 — [SOPHIST blog](https://blog.sophist.de/2026/10/19/nicht-funktionale-anforderungen/)
- Implicit rules: active voice, complete sentences, full verbs (Vollverben) for processes, one requirement per process word — [brochure excerpt](https://silo.tips/download/die-sophistenschablonen-fr-alle-flle-2); [SOPHIST MASTeR brochure, via search summary](https://sophist.de/wp-content/uploads/2025/03/MASTeR-Broschuere-6-Auflage.pdf)
- The older 2009 LMU handout uses muss / soll / wird and wenn (temporal) / falls (logical) — [LMU handout](https://www.pst.ifi.lmu.de/Lehre/fruhere-semester/sose-2009/seprakt/Formulierungsregel_DE_Rupp_Schablone.pdf) (phase 1 source)
- The SOPHIST linguistic-defect rule set is organised by deletion, generalisation and distortion (Tilgung, Generalisierung, Verzerrung) — [Speakerdeck "Requirements 101"](https://speakerdeck.com/donschado/requirements-101-de) (phase 1 source)

### Inferences
- requimd's German example configuration (ADR-0009, decision 4) should use MUSS/SOLLTE/WIRD, not "soll". Femmer et al. (section 5) observed that "soll" in German specifications behaves like a loophole, which supports flagging "soll" and accepting "sollte" only as the explicit "should" level.
- The SOPHIST activity phrases "die Möglichkeit bieten … zu" and "fähig sein … zu" are deliberate. An INCOSE R10 check ("be able to") translated to German must exempt them, while still flagging "in der Lage sein".
- "wird" has two readings: SOPHIST WIRD (intention) and the passive auxiliary. A deterministic rule: "wird/werden" followed, at clause end, by a past participle (ge-…-t / ge-…-en, -iert, or a participle from an inseparable-prefix verb list) is passive (R2); "wird" with a clause-final infinitive is the WIRD level.
- SOPHIST defect classes, as commonly taught **[background, unverified: from the Rupp book, not confirmed in a public source in this pass]**: deletion covers incompletely specified process words, implicit assumptions, incomplete conditions, incomplete comparatives and superlatives, and modal operators of possibility/necessity; generalisation covers universal quantifiers ("alle", "immer", "nie", "jeder"), nouns without reference, and incomplete conditions; distortion covers nominalisations and function-verb constructions (Funktionsverbgefüge). Several map to INCOSE: universal quantifiers to R26/R32, comparatives to R33, nominalisation to R2.

### Gaps
- The 6th-edition brochure could not be read directly; the slot details come from an older brochure excerpt and a consultant summary, which agree with each other and with the 2026 SOPHIST blog where they overlap.
- The exact SOPHIST list of linguistic defects (names, count) was not confirmed from a public source.
- No English edition of the MASTeR brochure was confirmed.

## 3. What does IREB CPRE Foundation Level 3.x say about natural-language requirements and templates, and which terms does it use?

### Takeaway
IREB's glossary defines three template kinds (phrase template, form template, document template), and the glossary definitions are English-only by policy, with translated glossaries available. The syllabus text on templates could not be read in this pass.

### Cited Findings
- Phrase template: a template for the sentence structure of a single requirement or user story; form template: predefined fields (use cases, quality requirements); document template: a skeleton for a requirements document — [IREB CPRE glossary](https://cpre.ireb.org/en/downloads-and-resources/glossary)
- Glossary (in IREB's sense) includes cross-references, synonyms, homonyms and acronyms; unambiguity means a requirement cannot be understood differently by different people; acceptance criteria are conditions a work product or backlog item must meet — [IREB CPRE glossary](https://cpre.ireb.org/en/downloads-and-resources/glossary)
- IREB keeps glossary definitions in English to avoid translation ambiguity; the glossary is available in German and 12 other languages — [IREB CPRE glossary](https://cpre.ireb.org/en/downloads-and-resources/glossary); [search summary of ireb.org glossary page](https://www.ireb.org/en/cpre/cpre-glossary/)
- "Phrase template" is the IREB English term used for Rupp-style sentence templates — [Peterjohann](https://www.peterjohann-consulting.de/satzschablonen/)
- A German CPRE FL handbook v1.3.2 exists — [IREB handbook DE v1.3.2](https://cockpit-v1.ireb.org/media/pages/downloads/cpre-foundation-level-handbook/a46a1b6077-1776936817/cpre_foundationlevel_handbook_de_v1.3.2.pdf) (listed by search, not read)

### Inferences
- requimd documentation can use the IREB terms "phrase template" (Satzschablone) for EARS/MASTeR/story checks and "glossary" with synonyms and acronyms, which matches FR-114's aliases and a future R37 acronym check.
- The glossary's inclusion of homonyms and acronyms suggests two cheap deterministic extensions: an acronym used but not in the glossary (R37), and a homonym marker in the glossary that makes requimd hint on each use.

### Gaps
- Whether syllabus 3.2 names EARS and the Rupp template explicitly, and its German terms (Satzschablone, Formularschablone, Dokumentschablone), were not verified; the syllabus PDF was not readable.

## 4. What is specific to checking German requirement texts (modal verbs, word order, passive, nominalisation, inflection and compounds, EARS keywords, public German word lists)?

### Takeaway
German needs different rules, not just translated lists: "soll" is weak, "muss nicht" means "need not", "wird" is ambiguous, nouns are always capitalised (no proper-noun heuristics), adjectives inflect into up to six forms plus comparatives, and compounds hide weak words and glossary terms. There is no published German EARS translation; German Gherkin keywords are official. Public German weak-word resources exist only as research (Krisch et al. 2016, Daimler context) without a downloadable list.

### Cited Findings
- In German specifications, "soll" caused many loophole findings because it is weaker than English "must"; this heavily affected two of the evaluated artefacts — [Femmer et al., Rapid quality assurance with requirements smells](https://ar5iv.labs.arxiv.org/html/1611.08847)
- German inflection (case, gender, number) made dictionary matching miss plural forms of singular entries (false negatives) — [Femmer et al.](https://ar5iv.labs.arxiv.org/html/1611.08847)
- German grammatical gender and case often leave only one possible referent, so most pronoun findings were false (precision 0.26) — [Femmer et al.](https://ar5iv.labs.arxiv.org/html/1611.08847)
- Krisch, Dick, Jauch, Heid: "A Lexical Resource for the Identification of 'Weak Words' in German Specification Documents", LREC 2016, pp. 2846-2850, car-manufacturing specifications; context queries decide when a use is weak (e.g. dimension adjectives "kurz", "lang" without an exact value); F1 above 0.95 on development and evaluation corpora; no download of the resource is linked — [ACL Anthology L16-1454](https://aclanthology.org/L16-1454/). A search summary quotes F1 0.98 for "good" and 0.89 for "bad" requirements, which conflicts with the abstract; the abstract is preferred — [search result](https://preview.aclanthology.org/fix_video/L16-1454.pdf)
- A rule-based German detector (NLP4RE 2019 poster) reports 108 true positives, 40 false positives, 38 false negatives (precision about 0.73), and its passive rule uses strict German word order — [NLP4RE 2019 poster, via search snippet](https://nlp4re.github.io/2019/uploads/poster-for-detection-of-defective-requirements-using-rule-based-scripts.pdf) (PDF not extractable; authors not confirmed)
- On a mainly German dataset, condition-detection rules written for German transferred easily to English, but not the reverse — [ICED23, Automated condition detection in requirements engineering, via search snippet](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/0144A4EE256C7C7BE0BDFEEC318F0BA8/S2732527X23000718a.pdf/automated-condition-detection-in-requirements-engineering.pdf)
- German Gherkin keywords: Funktionalität/Funktion; Grundlage/Hintergrund/Voraussetzungen/Vorbedingungen; Beispiel/Szenario; Szenariogrundriss/Szenarien; Beispiele; Angenommen/Gegeben sei/Gegeben seien; Wenn; Dann; Und; Aber; Regel — [cucumber gherkin-languages.json](https://raw.githubusercontent.com/cucumber/gherkin/main/gherkin-languages.json)
- No official German EARS translation was found; EARS clause order and keywords are defined only in English — [search over German EARS material](https://en.wikipedia.org/wiki/Easy_Approach_to_Requirements_Syntax); [alistairmavin.com/ears](https://alistairmavin.com/ears/) (phase 1)
- SOPHIST conditions use FALLS / SOBALD / SOLANGE (section 2) — [brochure excerpt](https://silo.tips/download/die-sophistenschablonen-fr-alle-flle-2)

### Inferences
- **Modal semantics.** "muss" binding; "muss nicht" = "need not" (not a prohibition; the prohibition is "darf nicht"); "soll" weak/ambiguous (and in German legal usage a "Soll-Vorschrift" means "as a rule" **[background, unverified]**); "sollte" recommendation; "kann" possibility; "darf" permission; "wird" intention or passive. The German verbal forms of standards (DIN 820-2 / ISO-IEC Directives Part 2 German: muss, darf nicht, sollte, darf, kann) follow the same split **[background, unverified]**. A deterministic "muss nicht" finding (suggest "darf nicht") is German-specific, cheap and nearly free of false positives.
- **Word order.** Main clause: finite modal in second position, infinitive last ("Das System muss die Rechnung archivieren."). After a fronted condition the modal comes first in the main clause ("Sobald …, muss das System …"). Separable verbs stay joined in the infinitive ("anzeigen") and in zu-infinitives ("anzuzeigen"), so the "required verb + clause-final infinitive" test works without a lexicon of separable verbs.
- **Passive (R2).** Detect forms of "werden" (wird, werden, wurde, wurden, worden) plus a clause-final participle, and the modal passive "ist/sind … zu <Verb>" and "lässt sich … <Verb>". The strict word order makes this more deterministic than English, as the NLP4RE poster also used.
- **Nominalisation.** Flagging every -ung/-heit/-keit noun is noisy because domain objects end that way (Bestellung, Rechnung, Sicherheit). A lower-noise rule flags function-verb constructions from a list ("erfolgt", "findet statt", "wird durchgeführt", "zur Verfügung stellen", "zur Anzeige bringen", "vornehmen").
- **Capitalisation.** All German nouns are capitalised, so any heuristic that treats capitalised words as defined terms or proper nouns (for R4) fails in German; "Sie" (formal you) vs "sie" (they/she) differ only in case.
- **Inflection and compounds.** "schnell" has the forms schnell, schnelle, schnellen, schneller, schnelles, schnellem, plus superlatives (schnellste…, am schnellsten) and "schnellstmöglich"; "schneller" is both an inflected positive and the comparative. Compounds such as "Benutzerfreundlichkeit" or "Echtzeitverarbeitung" are missed by whole-word matching. Glossary terms appear inflected (Kunde, Kunden, Kundin) and in compounds (Kundenkonto).
- **Tokenisation.** Gender-inclusive forms (Kund*innen, Kund:innen, Kund_innen, KundInnen) must be one token; abbreviations are often written with a narrow space ("z. B.", "d. h.", "u. a.") per German typography, so phrase entries must allow whitespace between parts (FR-110 already does).
- **Decimal format (R40).** German uses "1,5 s" and "1.000" as a thousands separator; English the reverse. The check needs the language of the file.

### Gaps
- No public German weak-word list from IREB, SOPHIST or a tool vendor was found; the Krisch resource is not downloadable.
- DIN 820-2 German verbal forms and the legal meaning of "soll" were not verified from a source.
- No German translation of EARS by Mavin or a German RE body was found.

## 5. Which requirement smells have published precision figures, and in which language?

### Takeaway
The main precision figures come from German industrial specifications (Femmer et al., JSS 2017, preprint 2016): dictionary-based smells are precise (0.70-0.96), POS- or morphology-based smells are not (0.26-0.49). Later German work exists (LREC 2016 lexicon, NLP4RE 2019 rules, ICED23 conditions), but no 2023-2026 German smell study with per-rule precision was found.

### Cited Findings
- Femmer, Méndez Fernández, Wagner, Eder; preprint arXiv 1611.08847 (2016), accepted in Journal of Systems and Software; average precision 59%, average recall 82%, with high variation — [arXiv 1611.08847](https://arxiv.org/pdf/1611.08847)
- The evaluated requirements were German (cases from Daimler, Wacker, TechDivision, University of Stuttgart); Stanford NLP for lemmatisation and POS, RFTagger for morphology; a German morphological analysis was added for comparatives and superlatives — [ar5iv full text](https://ar5iv.labs.arxiv.org/html/1611.08847)
- Precision per smell: subjective language 0.96, ambiguous adverbs and adjectives 0.81, loopholes 0.72, open-ended non-verifiable terms 0.70 (all dictionary-based); superlatives 0.49, comparatives 0.48 (morphology/POS); negative statements 0.33 (POS + dictionary); vague pronouns 0.26 (POS). Pooled precision over 616 findings 0.48 — [ar5iv](https://ar5iv.labs.arxiv.org/html/1611.08847)
- Reasons for false positives: negations mostly sit in conditions ("if the input is not zero"); comparatives were often comparisons with absolute values ("more than 1 second"); pronouns were usually unambiguous through German gender — [ar5iv](https://ar5iv.labs.arxiv.org/html/1611.08847)
- Recall: comparatives 0.95; the four dictionary smells as a group and negation/pronouns 0.84-0.95; superlatives 0.5 on very few data points — [ar5iv](https://ar5iv.labs.arxiv.org/html/1611.08847)
- Practitioners reported more awareness of quality defects; some smells were not clearly distinguishable — [arXiv abstract](https://arxiv.org/pdf/1611.08847)
- Krisch et al. 2016: F1 > 0.95 for context-sensitive German weak-word detection — [ACL Anthology](https://aclanthology.org/L16-1454/)
- NLP4RE 2019 German rules: precision about 0.73 — [poster, via search snippet](https://nlp4re.github.io/2019/uploads/poster-for-detection-of-defective-requirements-using-rule-based-scripts.pdf)

### Inferences
- For requimd's severity defaults: dictionary checks (R7, R8, R9) can be warnings; negation (R16), pronouns (R24) and comparatives/superlatives (R33) should be `info` or off by default, especially in German.
- Excluding negation inside a condition clause (after "if"/"falls"/"sobald"/"solange", before the main clause) removes the largest false-positive source Femmer found, and is deterministic with EARS/MASTeR clause detection.
- Comparatives followed by a number and unit ("mehr als 1 s", "mindestens 99,9 %") are compliant (EigenschaftsMASTeR); only comparatives without a value should be flagged.
- Femmer's 0.96 for subjective language shows that a curated German dictionary works; the precision problem lies in grammar-based rules, which requimd avoids by design (no POS tagger).

### Gaps
- 2023-2026 follow-ups with per-smell precision on German texts were not found in this pass (one search). LLM-based studies were out of scope (requimd is deterministic).
- Per-smell recall was not published for every smell.

## 6. Does any commercial or open tool support German requirement quality rules today?

### Takeaway
No commercial requirement-quality checker found in this pass checks German texts against INCOSE rule IDs. IBM RQA is explicitly English-only and ASCII-only; QVscribe lists English only; Jama Advisor publishes no language list. German support exists only in research prototypes or demonstrators (Smella, Fraunhofer FKIE RAT, ReqInspector) and in general writing tools (LanguageTool, Congree) that do not cite RE rules.

### Cited Findings
- IBM Engineering Requirements Quality Assistant: content must be English, other languages are not supported, and only ASCII characters are allowed (accented letters unsupported) — [IBM Docs, Checking DOORS Next requirements with RQA SaaS](https://www.ibm.com/support/knowledgecenter/SS3UPN/com.ibm.help.rm.assist.doc/topics/t_running_rqa.html?view=kc)
- QVscribe: language listed as English (third-party directory, updated March 2026) — [GetApp, via search summary](https://www.getapp.com/it-management-software/a/qvscribe)
- Jama Connect Advisor: product page describes NLP scoring against INCOSE and EARS but names no supported languages — [Jama Connect Advisor](https://www.jamasoftware.com/platform/jama-connect/jama-connect-advisor/) (via search summary)
- Visure Quality Analyzer: no language information on the page (updated 2026-08-25) — [Visure](https://visuresolutions.com/?p=46004)
- Altium Requirements Portal (ValiAssistant): quality checks are described as AI agents; no rule list or languages (page updated 2026-11-15) — [Altium docs](https://www.altium.com/documentation/altium-365/requirements-portal/valiassistant/quality-assessment?version=22)
- Fraunhofer FKIE Requirements Analysis Toolkit (RAT): German and English; checks passive, incomplete conditions, weak words, duplicates, inconsistent spelling and abbreviations; NLP and machine learning; a demonstrator in user testing, not a released product; no INCOSE/EARS/SOPHIST reference — [Fraunhofer FKIE RAT](https://www.fkie.fraunhofer.de/en/departments/itf/rat.html)
- ReqInspector inspects German specifications; its authors state that available inspection tools are mainly for English — [IREB RE Magazine, ReqInspector, via search snippet](https://re-magazine.ireb.org/print/reqinspector) (page now 404)
- Qualicen Requirements Scout (Femmer, REFSQ Workshops 2018) is a commercial descendant of the smell research — [dblp](https://dblp.org/rec/conf/refsq/Femmer18); its language support and current availability were not confirmed
- Congree offers "linguistically intelligent authoring assistance in requirements engineering" — [Congree](https://www.congree.com/en/knowledge/linguistically-intelligent-authoring-assistance-in-requirements-engineering) (via search result; content not read)
- The open INCOSE requirements workbench checks 42 v4 rules; no German support stated — [workbench](https://ssaleem74.github.io/incose-requirements-workbench/)
- LanguageTool is open source, supports German, and allows custom rules and terminology on a self-hosted server — [search result on LanguageTool](https://europeanpurpose.com/tool/languagetool)

### Inferences
- The phase 1 hypothesis holds with a caveat: deterministic, offline, INCOSE- and SOPHIST-cited checks of German requirement texts are not offered by the large vendors that were checked. The caveat is that Qualicen Scout and Congree may support German; they were not verified.
- LanguageTool covers R12-R14 (grammar, spelling, punctuation) in German; requimd can point users to it (e.g. ltex-ls in Neovim and VS Code) instead of bundling dictionaries (phase 1 R-23).

### Gaps
- Jama Advisor, Visure, Polarion, Innoslate and Qualicen language support was not confirmed from vendor documentation.
- Congree's RE features were not read.

## 7. Proposal for requimd: which rules to check, how, and how findings cite their source

### Takeaway
requimd already covers R7/R8/R9/R17 partly (FR-110), R1 partly (FR-111, FR-112, FR-113), R4/R36 partly (FR-114) and the bindingness part of SOPHIST (FR-111, FR-118). About 20 further rules are deterministic in both languages with word lists and simple clause patterns. The German-specific value lies in five rules no checked vendor offers: MASTeR modal/prio cross-check, "muss nicht", German passive with "wird" disambiguation, function-verb nominalisations, and German EARS/MASTeR condition keywords. Every finding should cite "INCOSE GtWR v4 Rn" and, where applicable, "SOPHIST MASTeR".

### Cited Findings
- Dictionary-based smells reach 0.70-0.96 precision on German texts; POS-based 0.26-0.49 — [Femmer et al.](https://ar5iv.labs.arxiv.org/html/1611.08847)
- Rule numbers differ across GtWR editions (41 rules in v2; R43/R44 in older material; 42 in v4) — [QRA](https://qracorp.com/guides_checklists/automating-the-incose-guide-for-writing-requirements/); [kiranm516](https://kiranm516.gitlab.io/sw-eng/writing-requirements.html); [Jama](https://www.jamasoftware.com/requirements-management-guide/writing-requirements/incose-requirements-writing-guide/)
- Vendors show rule-by-rule INCOSE reports (Jama Advisor, workbench score lines with rule numbers) — [workbench](https://ssaleem74.github.io/incose-requirements-workbench/); phase 1 R-09
- SOPHIST: MUSS/SOLLTE/WIRD, KANN excluded, FALLS/SOBALD/SOLANGE, activity phrases — [brochure excerpt](https://silo.tips/download/die-sophistenschablonen-fr-alle-flle-2); [Peterjohann](https://www.peterjohann-consulting.de/satzschablonen/)
- requimd today: weak words with explicit word forms and no stemming (FR-110, ADR-0005 decision 3); English built-in lists only and a German example configuration (ADR-0009); no-shall configurable (FR-111); modal-verb built-in list should/may/can/could/might/will (FR-118); EARS detection planned at prio could (FR-112) — `requirements/functional/checks.md`, `docs/decisions/0005-language-checks.md`, `docs/decisions/0009-languages-of-requirement-texts.md` (local files)

### Inferences

#### 7.1 Mapping of deterministic rules

FP = false-positive risk (L/M/H). "Has it" refers to requimd's current spec.

| Rule (INCOSE GtWR v4) | SOPHIST | English detection | German detection | FP | requimd has it |
|---|---|---|---|---|---|
| R1 structured statement | MASTeR templates | EARS pattern on first sentence; story pattern | MASTeR: [condition] + system + MUSS/SOLLTE/WIRD + object + clause-final infinitive; German EARS keywords (7.3) | M | partly: FR-111, FR-112 (EN keywords, could), FR-113 |
| R2 active voice | implicit rule "Aktiv"; deletion | form of "be" + past participle (-ed or irregular list) | wird/werden/wurde/worden + clause-final Partizip II; "ist zu" + infinitive; "lässt sich" | M (EN adjectival participles "is closed"); L-M (DE) | no |
| R2/R31 nominalisation | distortion (Nominalisierung, Funktionsverbgefüge) | "perform/carry out the <noun>", "takes place" list | list: erfolgt/erfolgen, findet statt, wird durchgeführt, vornehmen, zur Verfügung stellen, zur Anzeige bringen | L-M (list only, not all -ung nouns) | no |
| bindingness vs `prio` (no INCOSE rule; supports C9) | MUSS/SOLLTE/WIRD | shall/should/will vs `prio` must/should/could | muss/sollte/wird vs `prio`; KANN flagged | L | partly: FR-111/FR-118 find the verbs, no prio cross-check |
| "need not" vs prohibition (supports R16, C3) | – | "need not" (rare) | "muss nicht"/"müssen nicht" → suggest "darf nicht" | L | no |
| R4 defined terms | glossary keywords | acronym/term from glossary only; no capital-letter heuristic | same; capitalisation unusable (all nouns capitalised) | M | partly: FR-114 aliases |
| R6 units | EigenschaftsMASTeR value | number not followed by a unit token (configurable unit list) | same; units after decimal comma; "%" with space | M (counts, versions, IDs) | no |
| R7 vague terms | weak adjectives ("schnell", "leicht") | dictionary (FR-110 list) | dictionary with explicit inflected forms (7.2) | L-M (0.81-0.96 in German) | yes: FR-110 (EN built-in; DE via config) |
| R8 escape clauses | deletion | "if possible", "as appropriate", "where possible", "as required"… | möglichst, soweit möglich, ggf., bei Bedarf, nach Möglichkeit… | L (0.72) | partly: FR-110 ("if possible", "as needed") |
| R9 open-ended clauses | generalisation | "etc.", "including but not limited to", "and so on" | usw., etc., u. a., z. B., beispielsweise, insbesondere, o. Ä. | L-M (0.70) | partly: FR-110 ("etc.") |
| R10 superfluous infinitives | exemption: "fähig sein … zu", "die Möglichkeit bieten … zu" | "be able to", "be capable of", "be designed to" | "in der Lage sein", "imstande sein"; never flag the two SOPHIST phrases | L | no |
| R16 negation | – | not, no, never, n't, none | nicht, kein/keine/keinen…, nie, niemals, weder … noch; skip inside condition clauses | H (0.33) → info | no |
| R17 oblique "/" | – | "and/or", word/word | "und/oder", Wort/Wort; skip units (km/h), dates, paths | L | partly: FR-110 ("and/or") |
| R18 single thought | one requirement per process word | more than one required verb per item or sentence | more than one muss/sollte/wird (main-clause) per item | M | no |
| R19 combinators | – | and/or/but/unless joining two verb phrases (conjunction followed by a modal or second clause) | und/oder/sowie/bzw./sondern/aber/es sei denn followed by a second modal or verb | H for any "and"; M for clause-joining only | no |
| R20 purpose phrases | – | "in order to", "so that", "so as to" | "um … zu", "damit", "sodass", "zwecks"; not for user stories (FR-113 uses "damit"/"um") | L-M | no |
| R21 parentheses | – | "(" … ")" in requirement text, skipping abbreviation definitions "Term (ABC)" | same | M | no |
| R24 pronouns | noun without reference | it, they, them, this, these, those, its | es, dies, dieser/diese/dieses, jene, derselbe, dessen, deren; off by default in DE | H (0.26 in DE) | no |
| R25 independent of heading | – | text starts with "This function", "It" | "Diese Funktion", "Es" at text start | L | no |
| R26 absolutes | generalisation (universal quantifiers) | always, never, 100%, completely, every time | immer, nie, niemals, stets, jederzeit, ausnahmslos, vollständig, 100 %, fehlerfrei | M | no |
| R27/R28 conditions | FALLS/SOBALD/SOLANGE; incomplete conditions | "If" without "then" (EARS); mixed and/or in one condition | "Wenn" flagged as ambiguous (prefer Falls/Sobald); und and oder mixed in one condition clause | M | no (FR-112 planned) |
| R32 universal quantification | universal quantifiers | all, any, both → each | alle, sämtliche, beide, beliebig, irgendein → jede/jeder/jedes | M | no |
| R33 range / tolerance; comparatives | incomplete comparatives | comparative or "at least/at most" without number | Komparativ/Superlativ forms of listed weak adjectives; mindestens/höchstens/maximal without number | M (0.48) | no |
| R34 measurable performance | EigenschaftsMASTeR requires a value | NFR item body without any number | same | L-M | no |
| R35 temporal dependencies | – | soon, eventually, timely, immediately, in real time | bald, zeitnah, umgehend, unverzüglich, sofort, rechtzeitig, in Echtzeit, zeitgleich; never "sobald" | L-M | no |
| R36 consistent terms and units | glossary agreement | glossary aliases; unit spelling variants list | same, plus inflected/compound forms | L | partly: FR-114 |
| R37 acronyms | glossary | all-caps token (2+ letters) not in glossary | same | L-M | no |
| R38 abbreviations | – | e.g., i.e., approx., w.r.t. | z. B., d. h., ggf., bzw., u. a., i. d. R., evtl., ca., o. Ä. | L | no |
| R40 decimal format | – | "1,5" (comma decimal) in an English file | "1.5" in a German file; ambiguous "1.000" | M | no |
| R30 unique expression | – | exact/n-gram near-duplicates | same, after case folding | M | no (phase 1 R-20) |
| R41/R42 grouping, structured sets | document templates | types per file, required sections | same | L | partly: item types (FR-005) |
| C7 verifiable (proxy) | – | acceptance criteria present | same | L | yes: FR-121 |

Rules left out as not deterministic: R3, R5, R11, R12-R15, R22, R23, R29 (beyond metadata), R31 (only a project tech-word list), R39.

#### 7.2 Draft German weak-word and modal-verb lists (for owner validation)

Sources per entry: **S** = SOPHIST blog/brochure examples ([blog](https://blog.sophist.de/2026/10/19/nicht-funktionale-anforderungen/), [excerpt](https://silo.tips/download/die-sophistenschablonen-fr-alle-flle-2)); **K** = Krisch et al. 2016 example category ([ACL](https://aclanthology.org/L16-1454/)); **F** = Femmer et al. finding ([ar5iv](https://ar5iv.labs.arxiv.org/html/1611.08847)); **I** = German rendering of an INCOSE rule's English example words (my translation, cited rule); **P** = phase 1 compiled list (unsourced). Every entry needs owner validation. Forms follow ADR-0005 (explicit forms, no stemming).

| Entry and forms | Rule | Source | Proposed reason (English output, ADR-0009) |
|---|---|---|---|
| schnell, schnelle, schnellen, schneller, schnelles, schnellem, schnellstmöglich | R7 | S | not measurable; state a time limit |
| leicht, leichte, leichten, leichter, leichtes | R7 | S | not measurable; state a value |
| benutzerfreundlich (+ -e, -en, -er, -es), Benutzerfreundlichkeit, bedienerfreundlich | R7 | S | subjective; state a usability criterion |
| einfach (+ forms), intuitiv (+ forms), komfortabel, übersichtlich | R7 | P | subjective; state a criterion |
| kurz, kurze, kurzen; lang, lange, langen | R7 | K | dimension without a value |
| flexibel, flexible; robust, robuste; effizient, effiziente; performant | R7 | I (EN list in FR-110), P | not measurable |
| angemessen, angemessene; ausreichend, ausreichende; hinreichend; geeignet, geeignete | R7 | I (FR-110 "appropriate", "adequate") | vague; state the criterion |
| optimal, bestmöglich | R7/R26 | P | superlative without criterion |
| mehrere, einige, diverse, zahlreiche, viele, wenige | R7 | I (FR-110 "several") | vague quantity; state a number |
| ca., circa, etwa, ungefähr | R7/R33 | P | state a tolerance |
| regelmäßig, häufig, selten, normalerweise, üblicherweise, in der Regel, i. d. R. | R7/R35 | P | vague frequency |
| möglichst, soweit möglich, wenn möglich, falls möglich, nach Möglichkeit, sofern möglich | R8 | I | escape clause; state the condition |
| gegebenenfalls, ggf., bei Bedarf, nach Bedarf, falls nötig, falls erforderlich, sofern erforderlich | R8 | I (FR-110 "as needed") | escape clause |
| unter Umständen, u. U., eventuell, evtl. | R8 | P | escape clause |
| usw., etc., und so weiter, u. a., unter anderem, o. Ä., u. Ä., und Ähnliches | R9 | I (FR-110 "etc.") | incomplete list; name all items |
| z. B., zum Beispiel, beispielsweise, insbesondere | R9 | I | open list; name all items or move to a note |
| und/oder | R17 | I (FR-110 "and/or") | ambiguous; state "und" or "oder" |
| bzw., beziehungsweise | R17/R19 | P | two requirements or an ambiguous choice |
| in der Lage sein (forms: ist in der Lage, sind in der Lage), imstande | R10 | I | superfluous; use the process verb |
| immer, stets, jederzeit, nie, niemals, ausnahmslos, vollständig, fehlerfrei | R26 | I, S (universal quantifiers, background) | absolute; usually unverifiable |
| alle, sämtliche, beide, beliebig, beliebige, irgendein | R32 | I | use "jede/jeder/jedes" or name the set |
| zeitnah, bald, umgehend, unverzüglich, sofort, rechtzeitig, in Echtzeit | R35 | I | state a time limit |
| unterstützen, verwalten, bearbeiten, verarbeiten, handhaben, ermöglichen, gewährleisten, sicherstellen | SOPHIST incomplete process word | S (background, unverified) | vague process verb; name the activity |
| erfolgt, erfolgen, findet statt, finden statt, wird durchgeführt, vornehmen, zur Verfügung stellen | R2 / SOPHIST nominalisation | S (background, unverified) | nominalised process hides the actor |

Modal-verb list (FR-118) for German, in two variants:
- **SOPHIST variant (default proposal):** required verbs (FR-111) `muss`, `müssen`, `sollte`, `sollten`, `wird`, `werden`; modal-verb findings for `soll`, `sollen` (F: behaves like a loophole), `kann`, `können`, `könnte`, `könnten`, `darf`, `dürfen` (unless "darf nicht"), `möchte`, `möchten`, `will`, `wollen`; extra finding "muss nicht" / "müssen nicht" → "darf nicht".
- **Strict variant:** required verbs `muss`, `müssen` only; `sollte` and `wird` also flagged (matches English default "shall" only).
- In either variant, "werden" as passive auxiliary is handled by the R2 passive rule, not by the modal list.
- Cross-check with `prio`: muss ↔ `must`, sollte ↔ `should`, wird ↔ `could` (or a dedicated value); a mismatch is a finding (phase 1 R-12).

#### 7.3 German EARS keyword proposal

No published German EARS exists (section 4). Proposal aligned with the SOPHIST condition keywords German BAs learn, with verb-second order after the clause:

| EARS pattern | English | German proposal | Also accept | Example |
|---|---|---|---|---|
| Ubiquitous | The <system> shall … | Das <System> muss … <Infinitiv>. | – | Das System muss jede Bestellung protokollieren. |
| State-driven | While <state>, the <system> shall … | Solange <Zustand>, muss das <System> … | Während | Solange der Wartungsmodus aktiv ist, muss das System Anmeldungen ablehnen. |
| Event-driven | When <trigger>, the <system> shall … | Sobald <Auslöser>, muss das <System> … | Wenn (with an info hint "ambiguous; prefer Sobald or Falls") | Sobald der Kunde die Bestellung abgeschickt hat, muss das System eine Bestätigung senden. |
| Unwanted behaviour | If <trigger>, then the <system> shall … | Falls <Bedingung>, (dann) muss das <System> … | Wenn …, dann | Falls die Zahlung abgelehnt wird, dann muss das System die Bestellung sperren. |
| Optional feature | Where <feature>, the <system> shall … | Sofern <Merkmal> vorhanden ist, muss das <System> … | Bei <Merkmal> | Sofern das Modul Gutscheine installiert ist, muss das System Gutscheincodes annehmen. |
| Complex | While …, when …, the <system> shall … | Solange …, sobald …, muss das <System> … | – | – |

Parsing notes: the main clause after the condition starts with the modal ("muss das System"), not the subject; "dann" is optional in German and should not be required; "Falls" in MASTeR covers all logical conditions, so in German "Falls" cannot by itself distinguish unwanted behaviour from an optional feature (open question 5).

#### 7.4 Rule provenance (R-09)

- Each check and each built-in list entry carries one or more rule references, e.g. `INCOSE-GtWR-v4:R8`, `SOPHIST-MASTeR:bindingness`, `EARS:event-driven`. The edition is part of the reference, because numbering changed across editions.
- Message format (English, ADR-0009): `"möglichst" is an escape clause; state the condition explicitly (INCOSE GtWR v4 R8)`. Short rule titles are requimd's own wording; no INCOSE text is quoted.
- LSP: `Diagnostic.code` stays the check ID (`weak-word`); `codeDescription.href` points to the rule catalogue page in the requimd docs; hover shows the rule reference and a one-line explanation. No network access is needed at runtime (NFR-006): the link is only shown.
- Config: list entries accept an optional `rule` key next to `w` and `why`, so project-specific words can cite a rule (`{ w = "möglichst", why = "...", rule = "INCOSE-GtWR-v4:R8" }`).
- JSON export and site: findings carry `rules: [...]`, which enables a per-rule summary (phase 1 R-16) and audit evidence.
- Docs: a rule catalogue mapping each check ID to INCOSE rule numbers, SOPHIST templates and EARS patterns, with the false-positive guidance from section 5.

#### 7.5 Open questions for the product owner

1. Which SOPHIST bindingness set should the German example (and any preset) use: the current MUSS/SOLLTE/WIRD, or the older muss/soll/wird? Should "soll" be flagged?
2. How should bindingness map to `prio` (wird ↔ could?), and should a mismatch be a finding?
3. ADR-0009 rejected built-in language packs. Do the rule mapping and a German list justify a shipped German preset (opt-in, e.g. `preset = "de-sophist"`), or does it stay a documented example configuration?
4. Inflection and compounds: keep explicit forms only (ADR-0005 decision 3), or allow an opt-in suffix/compound match for single entries (e.g. "-freundlich")? The same question applies to glossary matching (phase 1 R-14).
5. German EARS keywords: Sobald or Wenn for event-driven; Falls for unwanted behaviour despite its broader MASTeR meaning; Sofern or Bei for optional feature?
6. May requimd cite INCOSE rule numbers with its own short titles in messages and docs? (Numbers and paraphrased titles seem safe; quoting rule text is not.)
7. Default severities for low-precision rules (negation, pronouns, comparatives, combinators): `info`, or off by default?
8. How does requimd know the language of a file (needed for R40 decimal format and list choice): one project language in `requimd.toml`, per path, or per file?
9. Should the "muss nicht" → "darf nicht" rule and the "Wenn is ambiguous" hint be built in (they are language facts rather than team preferences)?
10. Purpose phrases (R20) conflict with the German story pattern "damit"/"um": confirm that R20 only reads FR/NFR types by default.

### Gaps
- The FP ratings in 7.1 for rules Femmer did not measure are judgement.
- The German lists in 7.2 are drafts: only a few entries come from a German RE source (SOPHIST, Krisch, Femmer); most are translations of INCOSE examples or phase 1 practice and need owner validation and a test against `examples/` texts.
- Whether vendors with possible German support (Qualicen, Congree) already offer some of these rules was not verified.
