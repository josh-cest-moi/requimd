# DD-4: Coverage policy, tracing into code and tests, and test-result import

Scope: a deep dive that goes past phase 1 (R-24 to R-31, R-49 to R-53 in `requirements-and-design-tools-2026.md`). It covers OpenFastTrace (OFT) and its Rust port, LOBSTER, sphinx-test-reports, Xray, GitLab, other scanners (tracey, duvet, mantra, jamb, StrictDoc markers) and the JUnit XML conventions of common test runners. It ends with a proposal for requimd's policy section, tag scanner and result import.

Starting point in requimd (read from the repository on 2026-11-18):
- The task brief names FR-305 as the coverage requirement. In the current spec, FR-305 is the `show` command. Coverage is **FR-306**: "for each use case the number of steps, the number of covered steps and the uncovered steps" (`requirements/functional/cli.md`). The step check is FR-106 (`uncovered-step`, warning). The matrix (FR-401) lists steps against `impl` and `ver` items. FR-014 defines the `test` type (prefix TC, key `method` with `test`, `inspection`, `analysis`, `demonstration`) and the link `ver` to functional, quality and story items.
- B2 in `docs/feature-report.md`: "Test results from JUnit XML, shown as pass or fail", effort L, open dependency "how automated tests are matched to items", planned for "later". B10: "a check on items that no item links to with `ver`", effort S to M, decision F2 says "any `ver` link counts", planned for "next".
- `acceptance/*.md` holds TC items (`### TC-300 ...` with `ver: FR-300`), in Given/When/Then prose, followed by "Automated:" lists of Rust test names such as `fr_300_exit_code_0_without_errors`. NFR-010 already measures traceability by "a script that matches test names against requirement IDs" (`requirements/quality.md`). The repository's own convention is therefore **ID in the test name** (snake case, `fr_300_`) plus **ID in a code comment** (`// FR-006`, from CLAUDE.md).
- Dependencies today: pulldown-cmark, toml, serde, serde_json, clap, globset, walkdir, lsp-server, lsp-types, crossbeam-channel, toml_edit. There is **no regex crate and no XML parser** (workspace `Cargo.toml`). This matters for the scanner and the JUnit reader.

## 1. OpenFastTrace: tag syntax, artifact types, coverage algorithm, statuses, reports; would OFT-compatible tags give interoperability (cargo-ovft)?

### Takeaway
OFT is the most precise model of the field. Its IDs are `type~name~revision`, items declare `Needs:` (artifact types that must cover them), downstream items declare `Covers:`, and code tags have the shape `[impl->dsn~name~1]`. Coverage means "for every needed type at least one covering item exists", recursively (deep coverage), and revision mismatches make links outdated or predated. But OFT's tags always point at OFT-style IDs, and OFT and cargo-ovft read OFT-style Markdown items, not requimd items. So adopting the bracket-arrow *shape* would give familiarity, not real interoperability.

### Cited Findings
- **ID grammar** (`req~specification-item-id-syntax~1`): `specification-item-id = artifact-type "~" name ("~" / "#") revision`. Here `artifact-type = 1*(ALPHA)`, `name = unicode-alpha *(unicode-word / "-") *("." 1*(unicode-word / "-"))` and `revision = 1*(DIGIT)`. The `#` separator exists so that two tildes in Markdown are not read as strikethrough. — [OFT system requirements (raw)](https://raw.githubusercontent.com/itsallcode/openfasttrace/main/doc/spec/system_requirements.md)
- **Item sections:**
  - "The "Needs" section list all artifact item types in which coverage for this item must be provided."
  - "The "Covers" section contains a list of all specification item IDs that are covered by this item."
  - "Depends" lists IDs that must be implemented for the item to be complete. — [OFT system requirements](https://raw.githubusercontent.com/itsallcode/openfasttrace/main/doc/spec/system_requirements.md)
- **Markdown item layout:** a heading (the title), then the ID in backticks on its own line (e.g. `` `feat~markdown-import~1` ``), then the description. Optional `Rationale:`, then `Covers:` as a bullet list of links (e.g. `* [feat~plain-text-report~1](#plain-text-report)`), then `Needs: dsn`. The spec also names a compact "forwarding" notation in Markdown (`req~artifact-type-forwarding-in-markdown~1`). The fetched text did not show its exact form. — [OFT system requirements](https://raw.githubusercontent.com/itsallcode/openfasttrace/main/doc/spec/system_requirements.md); [OFT user guide via search summary](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide.md)
- **Code tag forms** (Tag Importer): — [OFT input format support](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide/reference/input_format_support.md)
  - basic: `[ <covered-artifact-type> -> <list-of-specification-object-ids> ]`, e.g. `// [impl->dsn~validate-authentication-request~1]`;
  - with own revision: `// [impl~~2->dsn~validate-authentication-request~1]`;
  - with own name and revision: `// [impl~validate-password~2->dsn~validate-authentication-request~1]`;
  - with needed coverage (the tag creates an item that itself needs coverage): `' [dsn->req~1password-login~1>>impl,test]`.
  - Spaces inside the brackets are optional.
- **"Short" tag format:** OFT also has a "short format that requires additional configuration during import" (`req~import.short-coverage-tag-format~2`). The fetched pages did not give its syntax. — [OFT system requirements](https://raw.githubusercontent.com/itsallcode/openfasttrace/main/doc/spec/system_requirements.md)
- **File selection and comments:** the Tag Importer picks the format by file extension (programming languages, markup, configuration, modelling, Gherkin). The page says tags are "usually" in comments but does not say whether comment detection is language-aware. — [OFT input format support](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide/reference/input_format_support.md)
- **Markdown and RST as tag carriers:** in `.md` and `.rst` files, a tag counts only as "a complete tag inside a standalone, single-line native comment" (an HTML comment in Markdown, `..` in RST). Inline comments, multi-line comments and tag-like text outside a comment are ignored. — [OFT input format support](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide/reference/input_format_support.md)
- **Gherkin:** — [OFT input format support](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide/reference/input_format_support.md)
  - A Scenario or Scenario Outline becomes an item when the tag region before it holds exactly one OFT ID tag.
  - Optional `Covers` and `Needs` comments go after the tags. They are case-sensitive; several `Covers` lines accumulate, and `Needs` may appear only once.
  - The scenario header becomes the title and the steps become the description.
  - Invalid IDs skip only that scenario, with a warning.
  - Coverage-tag-shaped text in executable lines is ignored.
- **Link statuses** (`req~tracing.outgoing-coverage-link-status~1`): — [OFT system requirements](https://raw.githubusercontent.com/itsallcode/openfasttrace/main/doc/spec/system_requirements.md)
  - outdated: "link points to a specification item which has a higher revision number";
  - predated: "...lower revision number";
  - ambiguous: "link points to a specification item that has duplicates";
  - unwanted: "coverage provider has an artifact type the requester does not want";
  - orphaned: "link is broken - there is no matching coverage requester".
  - Incoming statuses: "Covered shallow", "Covered unwanted", "Covered predated", "Covered outdated".
  - Duplicates: an item is "duplicate" if other items with the same ID exist.
- **Deep coverage and defects:**
  - An item is "covered deeply" when it and every item it needs coverage from are covered, recursively.
  - An item is a **defect** if it has duplicates. Otherwise it is a defect if its status is not "rejected" and either (a) one outgoing link has a status other than "Covers", or (b) it is not covered deeply.
  - Items can be filtered (types, statuses, tags) before evaluation. — [OFT system requirements](https://raw.githubusercontent.com/itsallcode/openfasttrace/main/doc/spec/system_requirements.md)
- **Revisions as staleness mechanism:** revisions are "intended to obsolete existing coverage links in case the content of a specification item semantically changed". — [OFT user guide (search summary)](https://github.com/itsallcode/openfasttrace/blob/main/doc/user_guide.md)
- **Command line and reports:** — [OFT command line reference (raw)](https://raw.githubusercontent.com/itsallcode/openfasttrace/main/doc/user_guide/reference/oft_command_line.md)
  - `oft trace` and `oft convert`. Output formats `-o plain` (default), `html`, `aspec`; `convert` writes `specobject`.
  - Verbosity `-v`: `quiet`, `minimal`, `summary`, `failures`, `direct_failures`, `failure_summaries`, `direct_failure_summaries`, `failure_details` (default), `direct_failure_details`, `overview`, `all`.
  - Import filters: `-a/--wanted-artifact-types`, `-t/--wanted-tags` (a leading `_` also keeps untagged items), `-w/--wanted-statuses`, `-i/--ignore-artifact-types`.
  - The reference index also lists an XML tracing report "for machine-based processing". — [OFT reference index](https://raw.githubusercontent.com/itsallcode/openfasttrace/main/doc/user_guide/reference/reference.md)
- **Artifact-type chain:** phase 1 recorded `feat → req → dsn → impl/utest` as the conventional chain. — [OFT system requirements](https://github.com/itsallcode/openfasttrace/blob/main/doc/spec/system_requirements.md)
- **cargo-ovft:**
  - Version 0.5.0, released 2026-10-30; first release 0.1.0 on 2025-10-10. Part of "Open Very Fast Trace", which "brings OpenFastTrace-compatible requirements traceability to Rust projects".
  - Reads OFT Markdown items (`feat~user-auth~1` with `Needs:`/`Covers:`) and Rust comments like `// [impl->feat~login-process~1]`.
  - Output is HTML by default (`requirements_report.html`) or `--format json`. `--check` exits non-zero on issues, for CI.
  - No Rust attributes or doc-comment forms are mentioned. — [docs.rs cargo-ovft](https://docs.rs/cargo-ovft); [ovft-core on lib.rs](https://lib.rs/crates/ovft-core)

### Inferences
- `[impl->FR-012]` is **not a valid OFT tag**, because the target must be a full `type~name~rev` ID. A requimd ID could be written OFT-style as `req~FR-012~1` (`FR-012` fits the `name` grammar). Code tagged for OFT would then name a requimd item, but only if the team keeps OFT names equal to requimd IDs.
- OFT and cargo-ovft read **OFT Markdown items**, not `## FR-012 Title` headings with metadata lines. Pointing OFT at requimd's specification therefore yields nothing. Real interoperability would need a requimd export to OFT Markdown or SpecObject XML (one more converter, not a tag choice).
- So accepting OFT-shaped tags buys: (a) a familiar syntax for teams who know OFT, and (b) the option to read an existing OFT-tagged code base if names line up. It does not buy cargo-ovft interop for free. It also brings OFT's semantics (artifact type on the left, revision suffix), which overlap with requimd's link keys and lock file.
- OFT's `Needs:` on each item is the per-item form of a policy, and LOBSTER's `requires` is the per-level form. requimd already has types, so a per-type policy plus a per-item override (R-25) matches both.
- OFT's "unwanted" status is a good model for a requimd check: a link whose source type is not allowed by the policy, e.g. `ver` from an item of type `functional`.
- OFT uses revision numbers for staleness. requimd's lock-file hashes (ADR-0007) already cover the same need more automatically. Code tags should reuse the lock file rather than add revisions to IDs.

### Gaps
- The exact syntax of OFT's "short" coverage tag and of the Markdown forwarding notation (`arch-->dsn : req~x~1`) was not in the fetched text. I did not verify them.
- I did not verify OFT exit codes (non-zero on defects) or the HTML report's exact contents beyond phase 1.
- I did not find how much of OFT cargo-ovft implements (deep coverage, revisions, Gherkin). Its docs claim compatibility without listing limits.

## 2. LOBSTER: `.lobster` schema, tracing policy, tools and report

### Takeaway
LOBSTER separates extraction from evaluation. Small per-tool converters emit `.lobster` JSON files with items (`tag`, `location`, `refs`, justifications). A `lobster.yaml` policy groups the files into levels (requirements, implementation, activity), says which level each traces to (`trace-to`), and says what each needs (`requires`: a list of AND-groups, each a list of OR-alternatives). The report marks items OK, Missing, Partial, Justified or Warning.

### Cited Findings
- **Top level of a `.lobster` file:** `data`, `generator`, `schema`, `version` ("these four are always present"), plus optional `custom_data`. Schema names: `lobster-req-trace` (requirements), `lobster-imp-trace` (implementation), `lobster-act-trace` (activities, i.e. tests and similar) and `lobster-report` (internal). — [LOBSTER schemas.md](https://raw.githubusercontent.com/bmw-software-engineering/lobster/main/documentation/schemas.md)
- **Item fields** (version 3): — [LOBSTER schemas.md](https://raw.githubusercontent.com/bmw-software-engineering/lobster/main/documentation/schemas.md)
  - common: `tag`, `location`, `name`, `refs`, `just_up`, `just_down`, `just_global`;
  - requirements add `framework`, `kind`, `text`, and version 4 also `status`;
  - implementation adds `language`, `kind`;
  - activities add `framework`, `kind`, `status`;
  - location kinds: `file`, `github`, `codebeamer`, `void`.
  - Tags follow the pattern `[a-z]+ .+`: a namespace, then the name, then an optional version. Examples: `req example.requirement`, `req 12345@42`, `cpp kitten::foo`.
- **Policy file `lobster.yaml`:** — [LOBSTER config_files.md](https://raw.githubusercontent.com/bmw-software-engineering/lobster/main/documentation/config_files.md)
  - Each level has `name`, `kind` (`requirements`, `implementation`, `activity`), `source` (the `.lobster` files) and `trace-to`, which "is always mandatory".
  - `requires`: "a list of AND-groups; each group is itself a list of OR-alternatives". Phase-1 example: Requirements requires `["Code"]` and `["Unit Test", "Formal Proof"]`.
  - The older `lobster.conf` format is deprecated and gives a warning. The fetched page did not show its grammar.
- **Code tags:** — [LOBSTER user manual](https://raw.githubusercontent.com/bmw-software-engineering/lobster/main/documentation/user-manual.md)
  - C++ code uses `// lobster-trace: my.requirement` inside a function body.
  - For C++ tests, references sit in the documentation comment, e.g. `@requirement CB-#0815 CB-#0816` above `TEST(...)`.
  - "Each one of the supported languages has their own way to annotate the tracing tags"; the Python section is marked "DOCUMENTATION TODO".
- **lobster-gtest:** tests call `LOBSTER_TRACE("math.gt");` or `LOBSTER_TRACE("math.gt, math.truth");`. This uses googletest's RecordProperty ("only the last `LOBSTER_TRACE` will be used"). The tool reads the generated gtest XML files or a directory of them. — [manual-lobster_gtest.md](https://raw.githubusercontent.com/bmw-software-engineering/lobster/main/documentation/manual-lobster_gtest.md)
- **Report tools and statuses:** — [LOBSTER user manual](https://raw.githubusercontent.com/bmw-software-engineering/lobster/main/documentation/user-manual.md)
  - Tools: `lobster-report` builds the report; `lobster-html-report`, `lobster-rst-report` (for Sphinx) and `lobster-ci-report` render it.
  - OK: "The item fulfils the configured traceability expectations."
  - Missing: does not fulfil them; the report shows the missing reference.
  - Partial: "fulfils only a subset".
  - Justified: an expectation is unmet "but a justification has been provided".
  - Warning: "requires attention from the user".
- **Converters** (phase 1): lobster-trlc, -python, -cpp, -cpptest, -gtest, -codebeamer and -json. Third parties provide lobster-doxygen, MISS_HIT and lobster-rust. — [LOBSTER GitHub](https://github.com/bmw-software-engineering/lobster)
- **Report vocabulary** (TRLC's published report): "missing reference to Code", "missing up reference", unknown references, and coverage per level (e.g. Specification 72.7%). — [TRLC tracing report](https://bmw-software-engineering.github.io/trlc/tracing.html)

### Inferences
- LOBSTER's three level kinds map well onto requimd. Requirements are items of the configured types. Implementation is code tags. Activity is TC items plus imported test runs, with `status` carried on activities: the result.
- `requires` as a list of AND-groups of OR-alternatives is the clearest policy grammar in the field. TOML expresses it directly as arrays of arrays.
- The justification fields (`just_up`, `just_down`, `just_global`) show that a policy tool needs a recorded "why not" (R-25). In requimd this belongs in the item's own metadata, so that reviewers see it in the diff.
- Reading `.lobster` files as an input would be cheap: plain JSON, and serde_json is already a dependency. It would let requimd use LOBSTER's converters (gtest, C++, Python) as optional evidence sources without code of its own. Writing a `lobster-req-trace` export from `export json` data is also cheap, and would let BMW-style pipelines treat requimd as a requirements source. Both are niche, so they rank as "could".

### Gaps
- The fetched pages did not document the semantics of `just_up`, `just_down` and `just_global`, the lobster-python comment syntax, or lobster-json's options (tag, name and justification attributes).
- I did not confirm how lobster-gtest stores pass or fail in the activity `status`. The gtest manual is silent on it.
- I did not verify whether the lobster-rust converter exists or how it works.

## 3. Test-result mapping: sphinx-test-reports, Xray, GitLab and others; statuses, versions and environments

### Takeaway
Tools match JUnit results to requirements in three ways:
1. **an explicit testcase `<property>`** naming the requirement: Xray `requirement`/`requirements`, sphinx-test-reports `tr_property_link_types` (e.g. `verifies`), gtest `RecordProperty`, pytest `record_property`;
2. **the test's identity** (classname + name): Xray's "Generic Test Definition", StrictDoc's proposal of test name equal to UID, and sphinx-test-reports `tr_link` on matching fields;
3. **a separate results file keyed by requirement**: GitLab `requirements.json`.

Requirement status is the worst status of its tests: OK only if all pass, NOK if any fails, NOTRUN if some are not run, UNCOVERED without tests. Xray computes it per version and per environment from the latest run.

### Cited Findings
- **Xray JUnit matching:** — [Xray Cloud: Taking advantage of JUnit XML reports](https://docs.getxray.app/display/XRAYCLOUD/Taking+advantage+of+JUnit+XML+reports)
  - "the `classname` and `name` attributes are concatenated and mapped to the Generic Test Definition field". An existing test with the same definition is reused, not duplicated.
  - A `test_key` or `test_id` property maps to an existing test (`test_id` wins).
  - Requirements are linked in three ways: a `requirement` attribute on `<testcase>`, `<property name="requirement" value="CALC-123"/>`, or `<property name="requirements" value="CALC-123,CALC-456"/>`.
  - Status: failures or errors → FAIL; skipped → TODO; otherwise PASS.
  - Results go into a new or a given Test Execution.
- **Xray JUnit 5 extensions:** `@Requirement({"CALC-1234","CALC-1235"})` creates "tests" links; `@XrayTest(key="CALC-1000")` forces the mapping; JUnit's `@Tag("performance")` becomes a label. — [xray-junit-extensions](https://github.com/Xray-App/xray-junit-extensions)
- **Xray requirement statuses:** — [Xray: Understanding coverage... (search summary of XRAY35/XRAY32 pages)](https://confluence.getxray.app/display/XRAY35/Understanding+coverage+and+the+calculation+of+Test+and+requirement+statuses)
  - OK: "all the Tests associated with the Requirement are PASSED".
  - NOK: "At least one Test associated with the Requirement is FAILED".
  - NOTRUN: some test is TODO or ABORTED and none failed.
  - UNKNOWN: some test has an unknown status and none failed.
  - UNCOVERED: "The Requirement has no Tests associated".
  - Per Fix Version V, the status is computed "based on the latest Test Run with Fix Version V of each associated Test". Environments are considered separately, and results combine across them per environment.
  - Caveat: these quotes come from older Server doc versions via a search summary; labels may differ in Cloud.
- **Xray coverage reports** (phase 1): stacked bars per version or test plan, per environment and per grouping field. Executions without an environment count as an "empty" environment. — [Xray: Test Coverage](https://confluence.getxray.app/display/XRAY/Test+Coverage); [Overall Requirement Coverage Report](https://confluence.getxray.app/display/XRAY/Overall+Requirement+Coverage+Report)
- **sphinx-test-reports:** — [sphinx-test-reports](https://sphinx-test-reports.readthedocs.io/en/latest/); [configuration](https://sphinx-test-reports.readthedocs.io/en/latest/configuration.html); [dynamic functions (search summary)](https://sphinx-test-reports.readthedocs.io/en/latest/functions.html)
  - Directives `test-file`, `test-suite`, `test-case`, `test-results`, `test-report` and `test-env` (based on tox-envreport) turn JUnit XML or JSON into needs with result `passed`, `failed` or `skipped`.
  - Linking 1, `tr_property_link_types` (since 1.3.0): "Maps a JUnit XML `<property>` name to a sphinx-needs link field". With `{"verifies": "links"}`, a `<property name="verifies" value="REQ_001,REQ_002"/>` "results in `links: REQ_001;REQ_002`".
  - Linking 2, the `tr_link` dynamic function: links a test case to the need whose field (e.g. title) matches a test-case field (e.g. classname). Comma-separated source values can produce several links.
  - Linking 3, a derived link type joins needs on `case_name`.
  - No page I read defines a requirement-level pass/fail roll-up. Results stay on the test-case needs and are shown with needtable and needflow filters.
- **Sphinx-Needs gtest pattern:** `RecordProperty("need_id","T_FACT_001"); RecordProperty("requirement","REQ_MATH_001");`, run with `--gtest_output=xml:test-results.xml`. The page's own diagrams do not show the requirement edge, which is an inconsistency in the source. — [x-as-code: Test Management and Traceability](https://x-as-code.useblocks.com/how-to-guides/testing/index.html)
- **GitLab** (phase 1): a CI job writes `requirements.json` as a `requirements` report artifact. `{"*":"passed"}` satisfies all open requirements, or individual IIDs get `passed` or `failed`; the key becomes `requirements_v2` with work items. Ultimate tier only. — [GitLab docs: Requirements](https://docs.gitlab.com/user/project/requirements/)
- **StrictDoc** has no built-in JUnit import that I could find. Discussion #2162 proposes an XSLT where "the test name in the JUnit file matches the strictdoc UID", needs file and line in the report, uses CTRF meanwhile and points to LOBSTER. — [StrictDoc discussion #2162](https://gb.xjqchip.workers.dev/strictdoc-project/strictdoc/discussions/2162) (mirror URL from search; original at github.com/strictdoc-project/strictdoc/discussions/2162)
- **jamb** (pytest plugin): marker `@pytest.mark.requirement("SRS001", "SRS002")`, optional `@pytest.mark.tc_id("TC-AUTH-001")`. — [jamb pytest integration](https://jamb.readthedocs.io/en/latest/user-guide/pytest-integration.html)
  - Requirement statuses: Passed ("All linked tests passed"), Failed ("One or more linked tests failed"), Partial (a mix), Skipped (all skipped), Not Covered ("No tests linked"), N/A (`testable: false`).
  - `jamb matrix` regenerates matrices "from saved coverage data without re-running tests".
- **mantra** (Rust, MIT, 0.9.0, 2026-09-01): — [mantra on lib.rs](https://lib.rs/crates/mantra)
  - Traces via `#[mantra_macros::req_satisfied("id")]`, `#[mantra_macros::req_test("id")]` and `assert_req!`, of kinds clarifies, satisfies, verifies and links.
  - Reads JUnit XML and Cobertura coverage (cargo-nextest, grcov). "A `verifies` trace alone does not verify a requirement": it uses line coverage.
  - Statuses Failed, Verified, Skipped, Unverified, Deprecated, Excluded. HTML or JSON reports; data in SQLite.
- **JUnit XML de-facto format:** — [testmoapp/junitxml](https://github.com/testmoapp/junitxml)
  - Hierarchy `testsuites` > `testsuite` (may nest, has `file`) > `testcase` (`name`, `classname`, `time`, `file`, `line`, `assertions`).
  - Passed unless the testcase has a `<skipped>`, `<failure>` or `<error>` child.
  - `<properties>` at suite and testcase level; duplicate names allowed; some tools put the value in the element text. Fallback `[[PROPERTY|name=value]]` lines in `system-out` for tools without testcase properties.
  - pytest and Playwright are named as supporting testcase properties.
- **pytest:** `record_property("example_key", 1)` adds `<property name="example_key" value="1"/>` to the testcase. A conftest hook can turn a custom marker into such a property. Warning: this "will break schema verifications for the latest JUnitXML schema". `record_testsuite_property` is schema-compatible but suite-level. — [pytest: Managing pytest's output](https://docs.pytest.org/en/stable/how-to/output.html)
- **cargo-nextest:** one testsuite per test binary and one testcase per test; `classname` is the binary ID and `name` the test. Retries appear as `<rerunFailure>` and `<flakyFailure>`; a flaky test that finally passed may have no `<failure>`. Configured with `[profile.ci.junit] path = "junit.xml"` and written to `target/nextest/ci/junit.xml`. — [nextest: JUnit support](https://nexte.st/docs/machine-readable/junit/)
- **Playwright** has a JUnit reporter (`reporter: [['junit', { outputFile: 'results.xml' }]]`) with `omitTags`. The page I read did not document how annotations become properties. — [Playwright reporters](https://playwright.dev/docs/test-reporters)

### Inferences
- **Explicit property beats name matching for correctness, and name matching beats both for effort.**
  - cargo test and nextest cannot emit testcase properties at all, so Rust projects such as requimd itself can only use **IDs in test names** (`fr_300_...`) or **tags in the test source**.
  - pytest, gtest and Playwright can emit properties, so Python and C++ teams can use explicit properties.
  - A requimd importer should therefore accept both, in a fixed priority order.
- The property name should accept the Xray names (`requirement`, `requirements`), the sphinx-test-reports convention (`verifies`) and a requimd name (`ver`, `tc`). The cost is small, and tests already annotated for Xray or Sphinx-Needs work unchanged.
- Status aggregation is consistent across Xray and jamb: a failure dominates, then not run, then passed; no test means uncovered. requimd should use the same order and avoid new names where it can.
- Xray's "latest run per version and environment" is the only serious treatment of time. For a single offline binary, the result *file* is the run. Version is the git commit the results were produced on, and environment is a label given at import. Per-version history would mean keeping old result files, which is a baseline concern (R-37, R-43) and belongs outside this feature.

### Gaps
- I could not read the sphinx-test-reports pages "Linking test cases to requirements" (CLI) and "Linking a test to requirements" (pytest plugin) in full.
- GitLab's link between test cases and requirements was not documented (also noted in phase 1).
- I did not confirm whether Maven Surefire or Gradle write JUnit 5 `@Tag` values into the XML, or how Cucumber's JUnit formatter represents scenario tags. If neither does, tags in those ecosystems are visible only in the source, not in the result file.
- Xray Cloud's current requirement status labels could not be fetched (connection refused); the values come from older Server docs via a search summary.

## 4. Other scanners and mapping conventions: duvet, mantra, tracey, Rust attributes, pytest markers, JUnit 5 tags, Cucumber tags; which conventions are most common?

### Takeaway
There is no single standard, but four families recur:
1. **comment tags with a verb and an ID**: OFT `[impl->id]`, tracey `r[impl id]` / `r[verify id]`, StrictDoc `@relation(ID, role=Verifies)`, LOBSTER `lobster-trace: id`, duvet `//=`;
2. **language-native annotations**: pytest markers, JUnit 5 `@Tag`, `@Requirement`, Rust attribute macros, gtest RecordProperty;
3. **Gherkin tags** (`@FR-012`);
4. **IDs in test names**: Xray's classname+name, StrictDoc's proposal, NFR-010's own meter.

The verb split "implements vs verifies" appears in every family that has verbs.

### Cited Findings
- **tracey:** — [tracey GitHub](https://github.com/bearcove/tracey)
  - Annotations `// r[impl channel.id.allocation]` and `// r[verify channel.id.parity]`. Verbs `impl` (the default if omitted), `verify`, `depends` and `related`. "The prefix (`r` in this case) can be any lowercase alphanumeric marker. Tracey infers it from the spec files."
  - Scans comments in Rust, Swift, TypeScript/TSX, JavaScript/JSX, Go, C, C++, Objective-C(++), Java, Kotlin, Scala, Groovy, C#, Zig, PHP, YAML and JSON5.
  - Config `.config/tracey/config.styx` with `include`, `exclude`, `test_include` (only `verify` allowed there) and `source_url`.
  - Statuses: Uncovered (no impl), Untested (impl but no verify), Stale (older rule version), Unmapped. No JUnit reading.
  - Accepts StrictDoc markers `// @relation(CH-001, scope=function, role=Verifies)`: no role or `Implements` maps to impl; `Verifies`, `Verification` or `Test` map to verify.
- **duvet** (AWS Labs, Apache-2.0, Rust): — [duvet GitHub](https://github.com/awslabs/duvet); [duvet guide](https://awslabs.github.io/duvet/); [duvet config](https://awslabs.github.io/duvet/config.html)
  - Annotations quote the specification itself: `//= https://www.rfc-editor.org/rfc/rfc2324#section-2.1.1` followed by `//# A coffee pot server MUST accept both the BREW and POST method`.
  - Config is TOML with a `$schema` URL (`.../config/v0.4.json`). `[[source]]` has `pattern = "src/**/*.rs"`, a default `type = "test"` or `implementation`, and a per-group `comment-style = { meta = "##=", content = "##%" }`. `[[requirement]]` loads extra files: requirements, todos and exceptions under `.duvet/`.
  - Reports: `[report.html]`, `[report.json]` and `[report.snapshot]` (`.duvet/snapshot.txt`, for CI diffs).
  - `duvet query --check implementation,test` can also use a coverage report (e.g. JaCoCo XML).
- **mantra:** Rust attribute macros (`#[mantra_macros::req_test("id")]`). Requirements in JSON5, TOML or Markdown headings `` # `id`: Title ``, with dot-notation hierarchy. Reads JUnit and Cobertura. — [mantra on lib.rs](https://lib.rs/crates/mantra). An older mantra-rust-trace crate also collected `[req(<ids>)]` from doc comments. — [mantra-rust-trace (search summary)](https://lib.rs/crates/mantra-rust-trace)
- **pytest markers:** `@pytest.mark.requirement("SRS001")` (jamb). — [jamb](https://jamb.readthedocs.io/en/latest/user-guide/pytest-integration.html). The same pattern is used by other plugins (pytest-requirements, pytest-intent). — [search result listing](https://pypi.org/project/pytest-requirements)
- **JUnit 5 tags:** "A tag must not be `null` or _blank_"; a stripped tag must not contain whitespace, ISO control characters, or the reserved characters `,` `(` `)` `&` `|` `!`. Tag expressions use `!`, `&`, `|` and parentheses. So `@Tag("FR-012")` is valid. — [JUnit user guide: Tags](https://docs.junit.org/current/running-tests/tags.html)
- **Cucumber/Gherkin tags:** — [Cucumber: Tags](https://cucumber.io/docs/cucumber/api/#tags)
  - Tags go above `Feature`, `Rule`, `Scenario`, `Scenario Outline` and `Examples`; "It is _not_ possible to place tags above `Background` or steps".
  - "Tags are inherited by child elements."
  - Tag expressions: `@wip and not @slow`, `(@smoke or @ui) and (not @slow)`.
- **gtest:** `RecordProperty` underlies both LOBSTER (`LOBSTER_TRACE`) and Sphinx-Needs (`RecordProperty("requirement", ...)`). — [lobster-gtest](https://raw.githubusercontent.com/bmw-software-engineering/lobster/main/documentation/manual-lobster_gtest.md); [x-as-code testing](https://x-as-code.useblocks.com/how-to-guides/testing/index.html)

### Inferences
- **Most common shape:** a short marker, then a verb, then an ID, inside an ordinary comment (OFT, tracey, StrictDoc, LOBSTER). tracey accepting StrictDoc markers shows that tools converge by *accepting each other's syntax*, not by one standard.
- **Verb vocabulary:** "impl" and "verify" are near-universal (tracey, StrictDoc roles, mantra `satisfies` and `verifies`, OFT `impl` and `utest`). requimd already has `impl` and `ver` link keys, so tags should reuse them.
- **Collisions:** requimd's `impl` means "implements a use case step" (FR-015). Code that "implements FR-012" is a different relation: it realises a requirement. The scanner should keep its own relation names, or the policy must say which target types each tag kind may hit.
- **Bare IDs** (`// FR-006`, the convention in this repository's CLAUDE.md) are the cheapest tag of all, and analysts can read them. They carry no verb. Treat them as a mention, or as a configurable default kind per glob: `impl` in `src/`, `ver` in `tests/`. tracey's `test_include` does exactly that.
- **Gherkin tags** like `@FR-012` are valid in Cucumber and valid as JUnit 5 tags, so one ID spelling works across BDD and Java. With inheritance, a `@FR-012` tag on a Feature verifies through every scenario. requimd should apply the same inheritance when it scans `.feature` files.
- **Test-name matching** must handle case and separators: requimd's own tests are `fr_300_...`, Java uses `FR012` or `fr012`, and pytest uses `test_fr_012_...`. A normalised match on prefix, optional `-` or `_`, and digits catches all three. It needs a word boundary and the known prefixes from the configuration, to avoid matching `fr_3000` or unrelated words.

### Gaps
- I found no survey that measures which convention is most used. "Most common" above is an inference from the tools read, not a statistic.
- I did not read duvet's annotation `type=` and `reason=` meta keys or its exception mechanism in detail. The config page lists only the `test` and `implementation` types.
- I did not verify that mantra's current version still reads doc-comment `[req(...)]` traces.

## 5. How should a coverage policy be expressed?

### Takeaway
Three designs exist: per item (OFT `Needs:`), per level with AND/OR groups (LOBSTER `requires`), and fixed statuses (tracey: uncovered and untested; Xray: UNCOVERED). The best fit for requimd is per item type, in `requimd.toml`. Each rule names the link kind and, optionally, the source type and a filter on the source's keys (e.g. `method`). Rules combine as AND-groups of OR-alternatives, with a per-item justified exception and a configurable severity.

### Cited Findings
- **Per item:** OFT's `Needs: dsn, uman` on each item; an item that needs nothing ends the chain; "covered deeply" is recursive. — [OFT system requirements](https://raw.githubusercontent.com/itsallcode/openfasttrace/main/doc/spec/system_requirements.md)
- **Per level:** LOBSTER's `requires` is "a list of AND-groups; each group is itself a list of OR-alternatives", with `trace-to` as the mandatory upward expectation. — [LOBSTER config_files.md](https://raw.githubusercontent.com/bmw-software-engineering/lobster/main/documentation/config_files.md)
- **Exceptions:** LOBSTER's justification fields (`just_up`, `just_down`, `just_global`) and its "Justified" report status. — [LOBSTER schemas.md](https://raw.githubusercontent.com/bmw-software-engineering/lobster/main/documentation/schemas.md); [LOBSTER user manual](https://raw.githubusercontent.com/bmw-software-engineering/lobster/main/documentation/user-manual.md). Also jamb's `testable: false` → N/A. — [jamb](https://jamb.readthedocs.io/en/latest/user-guide/pytest-integration.html)
- **Path-scoped rules:** tracey only allows `verify` in `test_include` paths. — [tracey](https://github.com/bearcove/tracey). duvet sets a default annotation type per `[[source]]` pattern. — [duvet config](https://awslabs.github.io/duvet/config.html)
- **Unwanted links:** OFT reports a covering item whose type the requester does not want. — [OFT system requirements](https://raw.githubusercontent.com/itsallcode/openfasttrace/main/doc/spec/system_requirements.md)
- **requimd today:** FR-106 is a hard-coded policy ("every step needs an incoming `impl`"); B10 is a second one ("every FR/NFR/US needs an incoming `ver`"); F2 decided that "any `ver` link counts" for now. — `requirements/functional/checks.md`, `docs/feature-report.md` (local)

### Inferences
- A per-type rule set generalises FR-106 (steps need `impl`) and B10 (requirements need `ver`) without new concepts. The rule for steps could become the first rule in the default configuration, keeping today's behaviour.
- Rules need four parts:
  - the **link kind**, incoming: `ver`, `impl`, `ref`, or a code-tag or result relation;
  - optionally, the **source type** (`test`);
  - optionally, a **filter on the source's keys** (`method = "test" | "inspection"`, `status != "deprecated"`);
  - for result-based rules, a **required result** (`passed`).
  The filter answers the brief's example ("every FR needs `ver` from a TC with method test or inspection"). It also gives R-52 (verification by method) for free.
- **Status scoping:** items with `status: deprecated`, and optionally `draft`, should be exempt, as OFT exempts "rejected". Otherwise new drafts flood the output.
- **Exceptions** should live in the item, not in the configuration, so they appear in the diff and are reviewed like any other change, e.g. `waive: ver` plus a reason in the body (see the proposal). Configuration-level exclusions (by ID) hide decisions from the specification reader.
- Deep or transitive coverage (OFT) should be a later option. For an analyst, "FR-012 has a passing test" is the first question. "UC-003 is fully verified through its FRs" is the second; the matrix (FR-401) partly answers it already.

### Gaps
- I found no evidence of how teams size severities (error vs warning) for policy gaps in practice. This is a product-owner choice.

## Proposal for requimd

### Takeaway
- Add a `[coverage]` policy section to `requimd.toml` with per-type rules (AND-groups of OR-alternatives over link kind, source type and key filters).
- Add a text-only scanner of configured globs that recognises **requimd-native tags** (`requimd: ver FR-012`) and **bare IDs**, and accepts the OFT bracket shape `[impl->FR-012]` as an alias only.
- Read JUnit XML files named in the configuration (no network), matched by property first, then TC ID, then requirement ID in the test name. Mark results stale through the lock-file hashes already used for suspect links.
- Report all of it through new check IDs, an extended `coverage` command, two new matrix columns and the site.

### Cited Findings
- Policy grammar after LOBSTER's `requires` ([config_files.md](https://raw.githubusercontent.com/bmw-software-engineering/lobster/main/documentation/config_files.md)); per-item needs and the "unwanted" status after OFT ([system requirements](https://raw.githubusercontent.com/itsallcode/openfasttrace/main/doc/spec/system_requirements.md)); per-path default kind after tracey `test_include` ([tracey](https://github.com/bearcove/tracey)) and duvet `[[source]] type` ([duvet config](https://awslabs.github.io/duvet/config.html)).
- Property names after Xray `requirement`/`requirements` ([Xray JUnit](https://docs.getxray.app/display/XRAYCLOUD/Taking+advantage+of+JUnit+XML+reports)) and sphinx-test-reports `verifies` ([configuration](https://sphinx-test-reports.readthedocs.io/en/latest/configuration.html)). Status order after Xray OK/NOK/NOTRUN/UNCOVERED ([Xray coverage](https://confluence.getxray.app/display/XRAY35/Understanding+coverage+and+the+calculation+of+Test+and+requirement+statuses)) and jamb Passed/Failed/Partial/Skipped/Not Covered ([jamb](https://jamb.readthedocs.io/en/latest/user-guide/pytest-integration.html)).
- JUnit structure and properties after [testmoapp/junitxml](https://github.com/testmoapp/junitxml); nextest file location and flaky handling after [nextest JUnit](https://nexte.st/docs/machine-readable/junit/).

### Inferences

#### A. Policy section in `requimd.toml` (example)

```toml
# Coverage policy (R-24). Each rule: items of type `for` need, for every
# AND-group in `needs`, at least one incoming link matching one of the
# group's OR-alternatives. Defaults keep today's behaviour (FR-106).

[coverage]
exempt-status = ["deprecated"]          # items in these states need nothing
results = "off"                          # "off" | "info" | "required" (see D)

[[coverage.rule]]
id = "step-impl"                         # becomes the check id in output
for = "step"
needs = [ [ { link = "impl" } ] ]        # = FR-106, sev warning
sev = "warning"

[[coverage.rule]]
id = "unverified"                        # = B10
for = ["functional", "quality", "story"]
needs = [
  [ { link = "ver", from = "test", where = { method = ["test", "inspection"] } },
    { link = "ver", from = "test", where = { method = "demonstration" } } ],
]
sev = "warning"

[[coverage.rule]]
id = "unrealised"                        # code tags (C), off by default
for = "functional"
where = { prio = "must" }                # only must items
needs = [ [ { link = "code" } ] ]
sev = "info"

[[coverage.rule]]
id = "test-not-passing"                  # results (D)
for = "test"
where = { method = "test" }
needs = [ [ { result = "passed" } ] ]
sev = "warning"

# Where tags and results come from (C, D).
[trace]
include = ["src/**", "tests/**", "features/**/*.feature"]
exclude = ["target/**", "node_modules/**"]
default = [ { glob = "tests/**", kind = "ver" },      # bare IDs in tests verify
            { glob = "src/**",   kind = "code" } ]    # bare IDs in src realise
oft = true                                             # accept [impl->FR-012]

[[results]]
path = "target/nextest/ci/junit.xml"
env = "linux"                                          # optional label
[[results]]
path = "reports/pytest-*.xml"
```

Notes on the design:
- `for` accepts the type names of FR-014 plus `step`. `from` and `where` filter the *source* item of the link (its type and metadata keys). `where` at rule level filters the *target* items.
- An item opts out of one rule with a metadata key, e.g. `waive: unverified`. The reason sits in the body, and check reports the waiver as info, not as a gap. This mirrors LOBSTER "Justified" and jamb "N/A", and stays visible in `requimd diff`. It needs a new reserved key in FR-015 (owner decision).
- An "unwanted" check (OFT) falls out of the same data: a `ver` link from a type not allowed by `[links.ver]` is already an error today. A policy-level "wrong method" diagnostic can come later.

#### B. Tag syntax requimd should recognise in code and test files

Accept three forms, in order of strength. All carry requimd IDs as written in the specification (`FR-012`, `UC-003.s2`).

1. **Native tag:** `requimd: <kind> <ID>[, <ID>...]`, e.g. `// requimd: ver FR-012, FR-013` or `# requimd: code FR-020`.
   - The kinds are the configured link keys plus `code` (realises a requirement).
   - The fixed marker word makes it unambiguous and greppable, like LOBSTER's `lobster-trace:` and tracey's `r[...]`.
2. **OFT bracket alias** (if `oft = true`): `[impl->FR-012]`, `[utest->FR-012]` and `[impl->req~FR-012~1]`.
   - `impl` maps to `code`, while `utest`, `itest` and `test` map to `ver`.
   - In the full OFT form the name part is used as the requimd ID and the revision is ignored, with an info diagnostic: requimd uses hashes.
   - The documentation should say plainly that this is syntax compatibility only. OFT and cargo-ovft cannot read requimd's specification without an export.
3. **Bare ID in a comment:** `// FR-006` (this repository's own convention).
   - Its kind comes from the path default (`[trace] default`). Without a default it counts as a mention, shown on hover and in `show`, but not as coverage.
4. **Gherkin tags:** `@FR-012` or `@TC-300` above Feature, Rule, Scenario, Scenario Outline or Examples, with Cucumber's inheritance. These count as `ver`. `@TC-300` links a scenario to an acceptance TC item and carries its `ver` links.
5. **Test names** (only for result matching in D, not as source tags): `fr_300_...`, `test_fr_012_...`, `FR012Test`.

Not recommended for v1: language-specific attribute macros or decorators (mantra, jamb markers) as separate syntaxes. A decorator's string argument such as `@pytest.mark.requirement("FR-012")` is already found by the bare-ID rule when the path default is `ver`. So pytest, JUnit 5 `@Tag("FR-012")` and `#[doc = "FR-012"]` work without parsing the language.

#### C. Keeping scanning fast and offline

- **Files:** scan only `[trace] include` minus `exclude`, with globset and walkdir (both already dependencies). Skip files over a size limit (e.g. 1 MiB) and files with a NUL byte in the first 8 KiB (binary). Respecting `.gitignore` would need the `ignore` crate; explicit excludes avoid the dependency.
- **Comment detection:** use a small built-in table from extension to line-comment markers:
  - `//` for rs, c, h, cpp, java, kt, ts, js, go, cs, swift, scala;
  - `#` for py, rb, sh, toml, yaml, yml;
  - `--` for sql, lua;
  - `;` for ini and lisp-like files; `%` for tex and m files; `'` for vb and PlantUML.
  - Block comments are handled by line: lines starting with `*`, `/*` or `<!--`.
  - Unknown extensions are scanned on every line, but only for native and OFT tags, never for bare IDs, to avoid false positives.
  - `.feature` files: tag lines starting with `@`, plus `#` comments.
  - Markdown outside the specification: only standalone HTML-comment lines, as OFT does.
  - This matches OFT and tracey (format by extension) without a parser per language.
- **Matching without a regex crate:** a hand-written scanner over bytes:
  - find the marker (`requimd:`, `[`, or a configured prefix followed by `-` and digits);
  - check word boundaries;
  - parse the ID with the same function the specification parser uses, so `UC-003.s2` and the digit count follow `[ids] digits`.
  - This keeps NFR-005 (few dependencies) and stays linear in file size. The positions are byte offsets, as the Rust conventions require.
- **Speed:** one read per file and no git calls. Optionally, cache per-file results in memory in the language server, keyed by modification time. For CI, the scan runs only in `check` and `coverage` when `[trace]` is configured, so projects without code see no cost.
- **Offline:** everything is local file reading, so NFR-006 and ADR-0011 hold. No URLs in tags (duvet's URL-based annotations are out of scope).
- **Diagnostics from the scan:**
  - an `unknown-id` in a tag, reported at the code location: CLI output only, because the language server serves Markdown;
  - a tag kind that is not allowed for the target type;
  - a tag in a `src` path with kind `ver` (tracey-style rule, optional).
- **Lock file:** code tags become links like any other. Their target hash goes into `requimd.lock` (ADR-0007), so a changed FR-012 makes `// requimd: ver FR-012` suspect until `requimd review`. This replaces OFT's revision numbers. The cost: the lock file grows with every tag, which DD-3 should weigh against the known merge-conflict pain. An alternative is an option `suspect = false` for tag links.

#### D. JUnit and test-result import

- **Input:** `[[results]] path` globs in `requimd.toml`, or `requimd check --results <file>` and `requimd coverage --results <file>` for CI runs. Read-only, local files only; a missing file is an info diagnostic, not an error, because local runs often have no results. Formats:
  - JUnit XML (the de-facto structure of testmoapp/junitxml, including nested suites, nextest's `flakyFailure` and `rerunFailure`, and properties in element text);
  - optionally, a trivial `results.json` keyed by ID (`{"TC-300": "passed"}`, as in GitLab) for manual or other tools;
  - later, `.lobster` activity files.
- **Dependency:** JUnit needs an XML reader. Options are `quick-xml` (one small, widely used crate) or a minimal hand-written reader for the subset (elements, attributes, entities, CDATA). The first is safer under "survive any input" (FR-012, NFR-009). This is an ADR question.
- **Mapping rules** (first match wins per testcase; one testcase may map to several IDs):
  1. testcase properties named `ver`, `verifies`, `requirement`, `requirements` (comma-separated IDs) → result on those requirements, *through* any TC that links them, else directly;
  2. testcase property `tc` (or `test_key`, as in Xray) → result on that TC item;
  3. an ID in `name` or `classname`, after normalisation (prefix case-insensitive; `-`, `_` or nothing before the digits; word boundary; digits must parse). TC IDs map to the TC, other IDs to the requirement. `fr_300_exit_code_0_without_errors` maps to FR-300, which is exactly NFR-010's meter;
  4. optionally, the testcase `file` and `line` attributes (pytest, gtest) joined with a source tag at or above that line. This is accurate but works only for runners that emit them, so it can come later.
- **Statuses per testcase:** `passed`; `failed` (`<failure>` or `<error>`); `skipped`; `flaky` (passed after retries: counts as passed, shown with a marker). The same ID in several files or environments gives one result per environment label.
- **Roll-up per TC and requirement** (Xray and jamb order): any failed → **failed**; else any skipped, or a TC with method `test` without a result → **not run**; else all passed → **passed**. No `ver` at all → **unverified** (B10). Results for `inspection`, `analysis` and `demonstration` TCs cannot come from JUnit. They come from `results.json` or a manual record (open question), or count as **verified by record** only.
- **Staleness (requirement changed after the test run):**
  - In CI, results come from the same commit as the specification, so they are fresh by construction.
  - For committed result files (manual tests, imported reports), use the lock-file mechanism: `requimd results record <file>` stores, per result, the content hashes of the TC and of each requirement it verifies, plus the file's timestamp and `git rev-parse HEAD`. The record goes into `requimd.lock` or a sibling `requimd.results` file.
  - A result is **stale** when a stored hash differs from the current one. It then counts as *not run*, with a diagnostic "result predates a change of FR-012". This mirrors OFT's "outdated" link status, with hashes instead of revisions.
  - A simpler variant compares the JUnit `timestamp` with the git date of the last commit that touched the item (same git calls as B6). It is weaker for uncommitted edits.
- **Unknown IDs in results** (a test names FR-999): a warning, like LOBSTER's "unknown reference". Testcases with no match: counted as "unmapped" in `coverage` (tracey term), not as diagnostics.

#### E. How results appear in `check`, `coverage`, the matrix and the site

- **`check`:**
  - each policy rule is a check ID with its own severity and `[checks]` override (FR-300 format), e.g. `requirements/functional/cli.md:57:1: warning[unverified] FR-306 has no ver link from a test with method test or inspection`;
  - result checks: `test-failed` (warning or error), `result-stale` (warning) and `unknown-id-in-results` (warning);
  - `--strict` (FR-302) turns them into a CI gate (R-53), so no new flag is needed;
  - waived items: one info line.
- **`coverage`** (FR-306 extended):
  - the current per-use-case step table stays first;
  - then one row per policy rule with counts and percentages (covered, waived, missing), e.g. "unverified: 41/48 covered (85 %), 2 waived, 5 missing: FR-210, ...";
  - with results, one row per status (passed, failed, not run, stale, unverified) and per environment;
  - `--format json` for tools, `--missing` to list only gaps (R-27 lite).
- **Matrix** (FR-401): keep one row per step and add two columns, "result" (worst status over the verifying items) and "realised in" (code tag locations, `path:line`). Optionally a second matrix "requirement × verification" (R-28), one row per FR/NFR/US, with columns `ver` items, method, result, environment and stale.
- **Site:**
  - each item page shows a status badge (passed, failed, not run, stale, unverified, waived), the verifying TCs with their results, and "realised in" with file and line (no links unless `blob-link` is configured, as in duvet);
  - an overview page with the policy table and percentages per rule (R-29).
  - The site states the commit and the time of the result files, so readers know how fresh the results are.
- **JSON export** (FR-402): add `results` and `code_refs` arrays and a policy summary, and bump the format version.
- **Language server:** hover on an item shows "2 ver (1 passed, 1 not run), 1 code ref". A code lens or inlay hint shows the same (R-31). Diagnostics for policy gaps appear at the item heading, with the same check IDs as the CLI.

### Gaps
Open questions for the product owner:
1. **B10 first, alone?** Ship B10 as the first policy rule (`unverified`, any `ver` counts, per F2), and the `method` filter only when the policy section arrives? Or design the `[coverage]` section now?
2. **Which types need verification by default?** All FR, NFR and US, or only `prio: must`? Are `draft` items exempt, or only `deprecated`?
3. **Severity** of a missing `ver`: warning (like FR-106) or info until the team opts in?
4. **Exceptions:** is a per-item `waive:` key with a reason in the body acceptable? It is a new reserved key in FR-015. Should waivers need `status: review`?
5. **Code tags at all?** Analysts see "realised in" only if developers tag code. Is the requimd repository's own convention (`// FR-006`, test names `fr_300_...`) the target user's convention, or should requimd stay requirement-and-test only (F3: requirements level first)?
6. **Relation name for code:** a new kind `code` (realises), or reuse `impl`? `impl` today means "implements a use case step" (FR-015), so reuse would overload it.
7. **OFT alias:** accept `[impl->FR-012]` and `[impl->req~FR-012~1]` as aliases? Recommendation: yes as aliases, documented as syntax compatibility only, without promising interop with OFT or cargo-ovft.
8. **Name matching:** may a test count as verifying FR-300 just because its name starts with `fr_300_` (NFR-010's meter), or must it go through a TC item?
9. **Test results through TC items or directly?** Does a JUnit result attach to a TC, which carries the `ver` link, or may it attach to a requirement directly? Direct attachment bypasses the TC layer, which today records Given/When/Then for analysts.
10. **Manual verification records:** how are results of `inspection` and `demonstration` TCs recorded? Options: a results file in the repository, a metadata key on the TC (`result: passed 2026-11-10`, which would make every verdict a spec change), or not at all in v1.
11. **Staleness:** hash-based (stored at record time, robust, more lock-file entries) or git-date-based (simpler, weaker)? Should tag links be suspect-tracked in `requimd.lock` at all, given the merge-conflict pain (DD-3)?
12. **XML dependency:** accept `quick-xml` (ADR), or require a hand-written reader?
13. **Environments and versions:** is one environment label per results file enough, or are per-release statuses (R-43, Xray-style "per Fix Version") needed? Those would belong with baselines (R-37).
14. **CI gate:** does a failed test make `check` exit 1 by default, or only with `--strict`?
15. **LOBSTER interchange:** worth a `could` item to read `.lobster` activity files, or to export `lobster-req-trace` JSON, for teams in automotive pipelines?

Research gaps carried into the proposal: OFT's short-tag and forwarding syntax, LOBSTER justification semantics, and whether Surefire and Cucumber write tags into JUnit XML were not verified (see sections 1 to 4).
