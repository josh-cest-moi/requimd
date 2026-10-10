# Quality requirements

## NFR-007 Cart response time
ref: UC-012.s2  prio: must
scale: seconds until the total is shown, 95th percentile
meter: load test with 500 concurrent users
must: 2
plan: 1

The cart total shall be displayed within the target under peak load.
