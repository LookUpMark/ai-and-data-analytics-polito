---
title: ETL and Oracle SQL
aliases: [ETL Process, Oracle Analytical SQL, Materialized Views, SQL Window Functions, ROLLUP CUBE GROUPING SETS]
tags: [computer-science/databases, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> Physical design of the data warehouse and the ETL process, plus the SQL machinery for analytical queries. Materialized views as precomputed aggregates: the multidimensional lattice, the view-selection problem (cost function, constraints), and Oracle DDL (REFRESH FAST/COMPLETE, query rewrite, materialized view logs). The ETL pipeline in detail: extraction techniques (static/incremental; log-, trigger-, timestamp-based, with comparison table), data cleaning, transformation and loading order. SQL extensions: computation windows (PARTITION BY / ORDER BY / ROWS vs RANGE), ranking functions (RANK, DENSE_RANK, ROW_NUMBER, CUME_DIST, NTILE, top-N), and GROUP BY extensions (ROLLUP, CUBE, GROUPING SETS).

## Part 1 — Physical Design of the DW

DW workload characteristics: **aggregate queries** accessing a large fraction of each table, **read-only** access, **periodic refresh** possibly rebuilding physical structures (indices, views).

- **Indices** differ from OLTP ones: bitmap index, join index, bitmapped join index; a B+-tree is *not* appropriate for attributes with low-cardinality domains or queries with low selectivity.
- **Materialized views**: the query optimizer must be able to exploit them (**aggregate navigation** — write a query without knowing a view already answers it).
- The optimizer must be **cost-based** (uses statistics).
- Procedure: select structures supporting the most frequent/relevant queries, prefer structures improving more than one query; constraints: disk space and the available time window for updates.
- **Tuning**: a-posteriori change of physical structures, based on workload monitoring; frequently required for OLAP.
- **Parallelism**: data fragmentation, inter-query and intra-query parallelization; join and group-by lend themselves well to parallel execution.

### Index selection in the DW

| Purpose | Guideline |
|---|---|
| Selection predicates on dimensions | High domain cardinality → B-tree; low cardinality → bitmap |
| Joins | Indexing only FKs in the fact table is rarely appropriate; **bitmapped join index** suggested (if available) |
| Group by | Use materialized views |

## Part 2 — Materialized Views

> [!definition] Materialized view
> A **precomputed summary** of the fact table, explicitly stored in the DW; increases performance of aggregate queries. Defined by SQL statements (from base tables or views with higher granularity).

Example: view `v3 = {category, month, city}` with `SUM(Quantity), SUM(Income)` grouped by City, Category, Month, over dimension tables Month, City, Category.

A materialized view can answer several different queries — but not for all operators: in the **multidimensional lattice** of aggregation patterns, views of one "color" cannot answer queries about dimensions of another color.

### View selection problem

The lattice of allowed aggregations is huge (most attribute combinations eligible). Selecting the "best" set of views means **minimizing a cost function**:

- query execution cost;
- view maintenance (update) cost;

under constraints: available space, time window for update, response time, data freshness. Trade-off shown in the slides: with a single view you minimize disk space and update window but not query cost; with many views query cost drops but space/update window grow — the optimum balances all constraints.

## Part 3 — The ETL Process

### Data extraction

Two **extraction methods**:

- **Static**: snapshot of operational data — used at the first DW population.
- **Incremental**: select the updates after the last extraction — used for periodical refresh; can be *immediate* or *deferred*. Computed as **incremental difference** between source states (records tagged with action D=Deleted / U=Updated / I=Inserted).

Which data can be extracted depends on **how operational data is collected**:

| Source type | Description | Complexity |
|---|---|---|
| **Historical** | All modifications stored for a given time (bank transactions, insurance data) | Operationally simple |
| **Partly historical** | Only a limited number of states stored, for a limited time window | Operationally complex |
| **Transient** | OLTP keeps only the current state (stock inventory) | Operationally complex |

**Extraction techniques** (comparison from Devlin 1997):

| | Static | Timestamps | Application assisted | Trigger | Log |
|---|---|---|---|---|---|
| Transient/semi-periodic data | No | Incomplete | Complete | Complete | Complete |
| File-based systems | Yes | Yes | Yes | No | Rare |
| Enterprise-specific development cost | None | Medium | High | None | None |
| Legacy systems | Yes | Difficult | Difficult | Difficult | Yes |
| Changes to applications | None | Likely | Likely | None | None |
| Impact on operational performance | None | None | Medium | Medium | None |
| Complexity of extraction procedures | Low | Low | High | Medium | Low |

Notes: **application-assisted** captures modifications with ad-hoc application functions (requires changing OLTP apps — hardly avoidable in legacy systems); **log-based** reads DBMS log files via APIs (efficient, no interference, proprietary format, supports deferred incremental extraction); **trigger-based** propagates changes at DBMS level (no app change, but extra load); **timestamp-based** marks modified records (schema change, deferred, may lose intermediate states of transient data).

### Data cleaning

Techniques for improving data quality (correctness/consistency). Problems: duplicate data, missing data, unexpected use of a field, impossible/wrong values, inconsistency between logically connected data — caused by data entry errors, different field formats, evolving business practices. Solutions are ad hoc:

- **Data dictionary** — for data-entry/format errors, only for limited-cardinality domains;
- **Approximate fusion** — detect duplicates/similar data: **approximate join** (join on common fields when no common primary key exists, e.g. ORDER and CUSTOMER from two different DBs matched on surname+address) and the **purge/merge problem** (identify duplicate customers across the Milano and Roma marketing DBs; need a record-similarity criterion);
- **Outlier identification**, deviations from business rules.

Prevention (reliable, rigorous OLTP data-entry procedures) is the best strategy. Typical cleaning chain per attribute (example: address "C.so Duca degli Abruzzi 24, ZIP 20129, Torino, I"): **normalization** (split into fields) → **standardization** (Corso, Italia) → **correction** (ZIP 10129).

### Data transformation

Conversion from operational format to DW format; requires **integration** through a uniform representation (the *reconciled schema*). Two steps:

1. From operational sources to reconciled data **in the staging area**: conversion and normalization, matching, (possibly) significant data selection.
2. From reconciled data **to the DW**: **surrogate key generation** and **aggregation computation**.

### Data warehouse loading

Update propagation preserving data integrity, in this **order**:

1. **Dimension tables** (via a look-up table mapping source identifiers ↔ surrogate keys; identify updates);
2. **Fact tables** (map identifiers to surrogate keys of already-loaded dimensions);
3. **Materialized views and indices**.

Constraints: limited time window, transactional properties (reliability, atomicity).

## Part 4 — SQL Extensions for OLAP

Interface tools need: new aggregate functions (moving average, median, rank), report functions (partial/cumulative totals), operators computing several group-bys at once. Answer: **OLAP functions in the ANSI standard** (implemented from DB2 UDB 7.1, Oracle 8i v2) and the **SQL-99 extensions of GROUP BY**.

Running example: `Sales(City, Month, Amount)` with 12 rows (Milano/Torino × months 7–12).

### Computation windows (analytic functions)

An **OVER** clause defines:

1. **Partitioning** — rows grouped *without collapsing them* (unlike GROUP BY); no partitioning → one single group;
2. **Row ordering** inside each partition (similar to ORDER BY);
3. **Aggregation window** — the set of rows on which the function is computed for each row.

```sql
-- Moving average: current month + two preceding, per city
SELECT City, Month, Amount,
       AVG(Amount) OVER (PARTITION BY City
                         ORDER BY Month
                         ROWS 2 PRECEDING) AS MovingAvg
FROM Sales;

-- Cumulative total per city
SELECT City, Month, Amount,
       SUM(Amount) OVER (PARTITION BY City
                         ORDER BY Month
                         ROWS UNBOUNDED PRECEDING) AS CumeTot
FROM Sales;

-- Detailed data together with totals (no sort needed, window = whole partition)
SELECT City, Month, Amount,
       Amount / SUM(Amount) OVER ()                    AS TotalFract,
       Amount / SUM(Amount) OVER (PARTITION BY City)   AS CityFract,
       Amount / SUM(Amount) OVER (PARTITION BY Month)  AS MonthFract
FROM Sales;
```

- **Physical interval** (`ROWS n PRECEDING`, `ROWS BETWEEN a PRECEDING AND b FOLLOWING`, `ROWS UNBOUNDED PRECEDING`): counts rows — appropriate for sequences **without gaps**; more than one sort key allowed.
- **Logical interval** (`RANGE`, e.g. `RANGE 2 MONTH PRECEDING`): an interval **on the sort key** — appropriate for sparse data with gaps; single sort key only (alphanumeric/date).
- The window sort order does **not** enforce output order (use the final `ORDER BY`); incomplete windows compute on available rows; windows can be combined with GROUP BY (the "temporary table" produced by GROUP BY is the operand):

```sql
SELECT City, Month, SUM(Amount) AS TotMonth,
       AVG(SUM(Amount)) OVER (PARTITION BY City
                              ORDER BY Month ROWS 2 PRECEDING) AS MovingAvg
FROM Sales
WHERE <join conditions>
GROUP BY City, Month;
```

### Ranking functions

```sql
-- Rank per city in December (RANK leaves gaps after ties)
SELECT City, Amount,
       RANK()       OVER (ORDER BY Amount DESC) AS Ranking,
       DENSE_RANK() OVER (ORDER BY Amount DESC) AS DenseRanking
FROM Sales
WHERE Month = 12
ORDER BY City;   -- output order independent of window order
```

| Function | Behaviour |
|---|---|
| `RANK()` | Rank with gaps: after two firsts, next rank is 3 |
| `DENSE_RANK()` | Rank without gaps: after two firsts, next rank is 2 |
| `ROW_NUMBER()` | Progressive number of each row within its partition |
| `CUME_DIST()` | Fraction of preceding-or-equal values: `#values ≤ current / N` (0–1) |
| `NTILE(n)` | Splits each partition into *n* subgroups of (nearly) equal size, labeled 1..n |

**Top-N pattern**: nest the ranking query as a table in an outer query filtering on the rank (the temporary table is created at runtime and dropped at the end of the outer query):

```sql
SELECT * FROM
  (SELECT COD_I, SUM(SoldAmount),
          RANK() OVER (ORDER BY SUM(SoldAmount)) AS SalesRank
   FROM Facts GROUP BY COD_I)
WHERE SalesRank <= 2;
```

Two rankings in one query (`RANK() OVER (ORDER BY Weight)` and `RANK() OVER (ORDER BY SUM(SoldAmount))`) require GROUP BY on all non-aggregated attributes.

### GROUP BY extensions (SQL-99)

Schema: `Time(Tkey,Day,Month,Year,…) Shop(Skey,City,Region,…) Product(Pkey,PName,Brand,…) Sales(Skey,Tkey,Pkey,Amount)`.

```sql
-- ROLLUP: aggregations removing columns one by one (order matters)
-- computes (City,Month,Pkey), (City,Month), (City), () — super-aggregates shown as NULL
SELECT City, Month, Pkey, SUM(Amount) AS TotSales
FROM Time T, Shop S, Sales V
WHERE T.Tkey = V.Tkey AND S.Skey = V.Skey AND Year = 2000
GROUP BY ROLLUP (City, Month, Pkey);

-- CUBE: all combinations (2^k groupings); column order irrelevant
SELECT City, Month, Pkey, SUM(Amount) AS TotSales
FROM Time T, Shop S, Sales V
WHERE T.Tkey = V.Tkey AND S.Skey = V.Skey AND Year = 2000
GROUP BY CUBE (City, Month, Pkey);

-- GROUPING SETS: only the listed groupings — avoids unnecessary ones
SELECT City, Month, Pkey, SUM(Amount) AS TotSales
FROM Time T, Shop S, Sales V
WHERE T.Tkey = S.Tkey AND S.Skey = S.Skey AND Year = 2000
GROUP BY GROUPING SETS (Month, (City, Month, Pkey));
```

Efficiency exploits the **distributive/algebraic** properties of aggregates: previously computed group-bys are reused; ROLLUP needs a single sort; CUBE is a combination of ROLLUPs (partial reuse of previous sorts, e.g. sort on (A,B) reused for (A,C)).

## Part 5 — Materialized Views in Oracle

> [!definition] Oracle materialized view
> The query result is **precomputed and stored on disk**; improves response times (aggregations and joins precomputed); usually associated with aggregate queries but usable for any query; can be used as a table in any query. With **query rewrite**, the DBMS can automatically answer other queries through the view, without user intervention (high-end editions only; otherwise the user must rewrite the query manually).

```sql
CREATE MATERIALIZED VIEW Sup_Item_Sum
  BUILD IMMEDIATE              -- or DEFERRED: create now, populate later
  REFRESH COMPLETE ON DEMAND   -- refresh method and timing
  ENABLE QUERY REWRITE
AS
SELECT Cod_S, Cod_I, SUM(Measure)
FROM Facts
GROUP BY Cod_S, Cod_I;
```

- **BUILD**: `IMMEDIATE` creates and loads immediately; `DEFERRED` creates but does not load.
- **REFRESH**: `COMPLETE` recomputes from all data; `FAST` applies only the changes since the last refresh; `FORCE` = FAST when possible else COMPLETE; `NEVER` = no update via standard procedures.
- **Timing**: `ON COMMIT` (automatic refresh when SQL operations affect the view) or `ON DEMAND` (explicit `DBMS_MVIEW.REFRESH('view', 'C'|'F')` — C complete, F fast).

**FAST refresh** requires proper structures logging changes: a **MATERIALIZED VIEW LOG** per table of the view:

```sql
CREATE MATERIALIZED VIEW LOG ON Facts
WITH SEQUENCE, ROWID (Cod_S, Cod_I, Measure)
INCLUDING NEW VALUES;

CREATE MATERIALIZED VIEW Sup_Item_Sum2
BUILD IMMEDIATE REFRESH FAST ON COMMIT ENABLE QUERY REWRITE
AS SELECT Cod_S, Cod_I, SUM(Measure) FROM Facts GROUP BY Cod_S, Cod_I;
```

Constraints for FAST refresh exist (on aggregating attributes, tables, joins — e.g. with GROUP BY, an aggregation function such as COUNT/SUM must appear in the SELECT). Administration: `ALTER MATERIALIZED VIEW name options;` / `DROP MATERIALIZED VIEW name;` / `DBMS_MVIEW.EXPLAIN_MVIEW` (inspects refresh type, fast-refresh capabilities, query-rewrite status, errors). Whether materialized views are actually used by frequent queries is verified by inspecting the **execution plan** (`SET AUTOTRACE ON` in SQL*Plus, or the Explain link in the Oracle web interface).
