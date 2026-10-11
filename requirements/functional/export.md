# Export and reporting

## FR-400 Static site
impl: UC-006.s1, UC-006.s2  prio: should

The system shall provide `requimd export site <dir>`, which generates a static
website with one page per source file, anchors per item and step, clickable
references, a backlinks section per item, and an index of all items. The
site shall work without a web server.

## FR-401 Traceability matrix
impl: UC-006.s2  prio: should

The system shall generate a traceability matrix of use case steps against
implementing and verifying items, as part of the static site and as CSV via
`requimd export matrix`.

## FR-402 JSON export
impl: UC-006.s1  prio: should

The system shall provide `requimd export json`, which writes the complete item
graph with items, steps, links, mentions and source locations as JSON.

## FR-403 Requirement changelog
impl: UC-006.s2  prio: could

The system shall provide `requimd diff <ref1> [ref2]`, which lists items
added, removed or changed between two git references.

## FR-404 Diagram export
impl: UC-006.s2  prio: could

The system shall export the links of a given item and its neighbours as a
Mermaid diagram.

## FR-405 Publishing target
impl: UC-006.s3  prio: could

The system shall publish the generated site to a Confluence space as
read-only pages. Details are to be specified.
