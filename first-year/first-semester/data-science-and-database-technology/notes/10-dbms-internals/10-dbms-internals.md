---
title: DBMS Internals
aliases: [Buffer Manager, B+-Tree, Hash Index, Bitmap Index, Query Optimization, Physical Design, Concurrency Control, Two Phase Locking, Recovery, WAL, Checkpoint, Distributed Databases, Two Phase Commit, NoSQL, Elasticsearch, MongoDB, CAP Theorem]
tags: [computer-science/databases, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> Internal mechanisms of a relational DBMS: the server architecture (optimizer, access method manager, buffer manager, concurrency control, reliability manager) and the ACID transaction model; buffer management primitives and steal/force policies; physical access structures (heap, sequential, B+-tree, hash, bitmap) with their trade-offs; query optimization (parsing → algebraic transformations → cost-based plan selection, data profiles, join algorithms, plus the Oracle optimizer as a real case); physical design driven by the workload. Then concurrency control (anomalies, serializability, 2PL, hierarchical locking, deadlocks, SQL2 isolation levels), reliability management (log file, WAL and commit precedence, checkpoints, warm/cold restart), distributed databases (fragmentation, transparency levels, transaction classes, Two Phase Commit, X-Open-DTP, parallel DBMS, TPC benchmarks) and NoSQL systems beyond the relational model (CouchDB/MapReduce, CAP and ACID vs BASE, Elasticsearch, MongoDB, Hadoop/Spark).

## DBMS Architecture and Services

A **DBMS** is a software package designed to store and manage databases. The course looks at the internal mechanisms providing services to applications — useful for making the right **system configuration** and **physical design** choices (some services are becoming available also in operating systems). The DBMS server is organized in cooperating blocks over the database (data files, index files, system catalog):

| Component | Role |
|---|---|
| **Optimizer** | selects the execution strategy for queries; receives a SQL (DML) instruction, performs lexical/syntactic/semantic parsing, transforms the query into an internal representation based on **relational algebra**, selects the "right" access strategy — guarantees the **data independence** property |
| **Access Method Manager** | performs physical access to data; implements the strategy selected by the optimizer |
| **Buffer Manager** | manages page transfer disk ↔ main memory; manages the pre-allocated memory portion shared among applications (e.g. Oracle SGA) |
| **Concurrency Control** | manages concurrent access to data (critical for writes); guarantees that applications do not interfere with each other |
| **Reliability Manager** | guarantees correctness of the database content when the system crashes; atomic execution of transactions; exploits **log files** to recover the correct state after a failure |

## Transactions and ACID Properties

> [!definition] Transaction
> A **logical unit of work** performed by an application: a sequence of one or more SQL instructions performing read and write operations on the database, characterized by the **ACID** properties (Atomicity, Consistency, Isolation, Durability).

Classic example — bank transfer of 100 € from account xxx to yyy:

```sql
UPDATE ACCOUNTS SET Balance = Balance - 100 WHERE Account_Number = xxx;
UPDATE ACCOUNTS SET Balance = Balance + 100 WHERE Account_Number = yyy;
```

**Delimiters**: start is typically implicit (first SQL instruction, program start, or after the previous transaction's end); end is **COMMIT** (correct end) or **ROLLBACK** (end with error — the database returns to the state at the beginning of the transaction; the DBMS must restore it). 99.9% of transactions commit; the rest roll back, either required by the transaction itself ("suicide") or by the system ("murder").

- **Atomicity** — a transaction cannot be divided; no intermediate state can be left in the database. Guaranteed by **Undo** (roll back all work of the transaction; used for rollback) and **Redo** (re-execute work of committed transactions; guarantees commit in presence of failure).
- **Consistency** — execution must not violate integrity constraints (primary key, referential integrity, domain constraints declared in the schema); on violation the system may roll back the transaction or auto-correct.
- **Isolation** — execution independent of concurrent transactions; enforced by the Concurrency Control block.
- **Durability** — the effect of a committed transaction is not lost in presence of failures; enforced by the Reliability Manager thanks to log files.

## Buffer Manager

Data are stored on disk in files organized into physical **blocks**; the **buffer** is a large main-memory block pre-allocated to the DBMS and **shared among executing transactions**, organized in **pages** whose size depends on the OS I/O block. Efficient buffer management is a key performance issue.

**Data locality**: recently referenced data is likely to be referenced again — empirical **20–80 law** (20% of data is read/written by 80% of transactions), so only high-usage blocks are kept in memory. For each buffer page the manager keeps a snapshot: **physical location on disk** (file identifier + block number) and **state variables**: a **Count** of transactions using the page and a **Dirty bit** (set if the page has been modified).

### Primitives

| Primitive | Behaviour |
|---|---|
| **Fix** | transaction requests access to a disk page; page loaded into the buffer (I/O only if not already there — data locality), pointer returned, Count incremented. If a free page is needed: first among free pages, then among **victim pages** with Count = 0 (no transaction needs them, but they may still be locked); if the victim has Dirty = 1 it is **synchronously written** to disk first. Requires shared access permission from the concurrency control manager |
| **Unfix** | transaction no longer uses the page; Count decremented |
| **Set dirty** | page modified by the running transaction; dirty bit set to 1 |
| **Force** | **synchronous** transfer of the page to disk; requesting transaction suspended; always entails a disk write |
| **Flush** | internal to the buffer manager, independent of transaction requests; runs in CPU idle time; downloads pages with Count = 0 not accessed for a long time |

### Steal / force policies

- **Steal**: the buffer manager **may select a locked page (Count = 0) of an active transaction as victim** — writes on disk dirty pages of *uncommitted* transactions → on failure these changes must be **undone** (as in rollback). **No-steal** forbids this.
- **Force**: at **commit** all active pages of the transaction are synchronously written to disk → guarantees durability. **No-force**: pages of committed transactions are written asynchronously later by the Flush → on failure these changes must be **redone**.

Typical usage is **steal / no-force** for its efficiency: no-force gives better I/O performance; steal may be mandatory for queries accessing a very large number of pages.

The buffer manager exploits file-system services (create/delete/open/close of files; **direct read** of one block — file id, block number, buffer page; **sequential read** of n blocks from a starting block; analogous writes; directory management).

## Physical Access to Data

**Physical access structures** describe how data is stored on disk to support efficient query execution; different formats serve different query needs. The **Access Method Manager** transforms an optimizer access plan into a sequence of physical access requests to disk pages by means of **access methods**: software modules specialized for a single physical data structure, providing read/write primitives; each selects the blocks to load, requests them to the buffer manager, and knows the organization of data inside a page (space for data, for access-method control information, for file-system control information). Tuples may have varying size (varchar, NULLs) and a single tuple may span several pages (BLOB/CLOB).

In relational systems: physical data storage is **sequential** (heap files, ordered sequential structures) or **hash**; indexing to increase access efficiency uses **tree structures (B-tree, B+-tree)**, **unclustered hash indices**, and **bitmap indices**.

### Sequential structures

- **Heap file (entry sequenced)**: tuples in insertion order (insert = append at the end); block space fully exploited before starting a new block; delete/update may cause wasted space (deleted tuples leave holes; updated tuples may no longer fit). Sequential reading/writing very efficient; does not support any other index-free access. Frequently used in relational DBMS together with **unclustered (secondary) indices** for search and sort.
- **Ordered sequential structure**: tuples written in the order of a **sort key** (one or more attributes, possibly the primary key). Appropriate for **sort, group by, search and join on the sort key** (avoids sorting the whole table when sorting is used for join). Problem: **preserving the order on insertion/update** — solutions: leave a percentage of free space per block (lower compactness; dynamic in-memory re-sorting within the block), or an **overflow file** for tuples that do not fit. Typically used with **B+-Tree clustered (primary) indices** (index key = sort key); also used by the DBMS to store intermediate operation results.

### Tree structures (B-Tree, B+-Tree)

Provide **"direct" access** based on the value of a key field (one or more attributes → **composed index**) without constraining the physical position of tuples. The most widespread structures in relational DBMS: **one root node**, many intermediate nodes, **large fan-out** (many children per node); **leaf nodes provide access to data**.

- **B-Tree**: data pages are reached **only through key values** by visiting the tree.
- **B+-Tree**: provides a **link structure allowing sequential access in the sort order of key values** (leaves are chained).
- **B stands for balanced**: leaves are all at the same distance from the root → **access time is constant** regardless of the searched value.

**Clustered** (key sequenced): the **tuple is contained in the leaf node**; constrains the physical position of tuples in a given leaf (position may change on node split when full); typically used for **primary key** indexing. **Unclustered** (indirect): the leaf contains **physical pointers (to actual data)** stored in a separate structure; the position of tuples is totally unconstrained; used for **secondary indices**. Example on `STUDENT(StudentId, Name, Grade)` indexed on Grade: unclustered leaves hold (grade → TID) pairs, clustered leaves hold the tuples themselves.

Advantages: **very efficient for range queries**; appropriate for sequential scan in key order (always for clustered, not guaranteed otherwise). Disadvantages: insertions may require **splits** of a leaf (possibly of intermediate nodes — computationally intensive); deletions may require **merging uncrowded nodes and re-balancing**.

### Hash structures

Guarantee direct and efficient access based on the value of a key field (one or more attributes). With B blocks, the **hash function** applied to the key returns a value in 0…B−1 defining the record's position; blocks should never be completely filled (room for insertions). Example: `H(StudentId=50)=1` → tuple T1 stored in BLOCK 1 (together with T4, `H(75)=1`). The **unclustered hash index** variant stores **pointers to data** in the blocks (e.g. `30 → T1`, `40 → T2`); actual data in a separate structure.

- Advantages: **very efficient for equality predicates on the key**; no sorting of disk blocks required.
- Disadvantages: **inefficient for range queries**; **collisions** may occur.

### Bitmap index

Based on a **bit matrix** referencing data rows by **RIDs (Row IDentifiers)**; actual data in a separate structure. One **column per distinct value** of the indexed attribute, one **row per tuple**: position (i, j) = 1 if tuple i takes value j, 0 otherwise. Example: `EMPLOYEE(EmployeeId, Name, Job)` with Job ∈ {Engineer, Consultant, Manager, Programmer, …}: bitmap rows mark each employee's job; the Programmer bitmap (0,1,0,1,0) selects T2, T4.

- Advantages: **very efficient for boolean expressions of predicates** (reduced to bit operations); appropriate for attributes with **limited domain cardinality**.
- Disadvantages: not used for **continuous attributes**; required **space grows significantly with domain cardinality** (Bitmap = NR·NK·1 bit vs B-tree = NR·Len(Pointer), with 4×8-bit pointers).

## Query Optimization

The optimizer is a fundamental building block of a relational DBMS: it selects an efficient strategy for query execution and guarantees **data independence** — the form in which a SQL query is written does not affect its implementation, and physical reorganization of data does not require rewriting queries. It automatically generates a **query execution plan** (formerly hard-coded by programmers), evaluating many alternatives and exploiting **statistics** (data profiles) stored in the system catalog, periodically recomputed so that decisions adapt to changes in data distribution.

Pipeline:

```
SQL QUERY → LEXICAL, SYNTACTIC AND SEMANTIC ANALYSIS ─┐ (with DATA DICTIONARY)
   → INTERNAL REPRESENTATION (relational algebra)
   → ALGEBRAIC OPTIMIZATION → "CANONICAL" QUERY TREE
   → COST BASED OPTIMIZATION (with DATA PROFILES) → ACCESS PROGRAM + SET OF DEPENDENCIES
```

1. **Analysis**: detect lexical errors (misspelled keywords), syntactic errors (SQL grammar), semantic errors (references to non-existent objects — needs the data dictionary). Output: internal representation in (extended) **relational algebra** — procedural, explicitly represents operator order, and has a corpus of theorems/properties to transform the query tree.
2. **Algebraic optimization**: equivalence transformations considered **always beneficial** (e.g. anticipation of selection with respect to join); eliminates differences among formulations; usually independent of data distribution.
3. **Cost-based optimization**: selects the best access method per table and the best algorithm per operator using a cost model; generates the access program plus a **set of dependencies** (conditions for plan validity, e.g. existence of an index).

**Compile & go**: compile and immediately execute, no plan storage, no dependencies — effective for one-shot dynamic SQL. **Compile & store**: the access plan is stored with its dependencies and executed on demand; recompiled when the data structure changes — efficient for repeated (parametric) executions.

### Algebraic transformations

Two expressions are **equivalent** if they produce the same result for any database instance; interesting transformations reduce the size of intermediate results or prepare for one that does. Main properties (σ = selection, π = projection, ⋈ = join, × = Cartesian product):

1. **Atomization of selection**: σF1∧F2(E) ≡ σF2(σF1(E)) ≡ σF1(σF2(E))
2. **Cascading projections**: πX(E) ≡ πX(πX,Y(E))
3. **Anticipation of selection with respect to join** (pushing selection down): σF(E1 ⋈ E2) ≡ E1 ⋈ σF(E2), F on attributes of E2 only
4. **Anticipation of projection with respect to join**: πL(E1 ⋈ E2) ≡ πL(πL1∪J(E1) ⋈ πL2∪J(E2)), with L1 = L − Schema(E2), L2 = L − Schema(E1), J = join attributes
5. **Join derivation from Cartesian product**: σF(E1 × E2) ≡ E1 ⋈F E2 (F relates attributes of E1 and E2)
6. **Distribution of selection over union**: σF(E1 ∪ E2) ≡ σF(E1) ∪ σF(E2)
7. **Distribution of selection over difference**: σF(E1 − E2) ≡ σF(E1) − σF(E2) ≡ σF(E1) − E2
8. **Distribution of projection over union**: πX(E1 ∪ E2) ≡ πX(E1) ∪ πX(E2); over difference πX(E1 − E2) ≡ πX(E1) − πX(E2) holds **only if X includes the primary key** (unique and not null)
9. Other: σF1∨F2(E) ≡ σF1(E) ∪ σF2(E); σF1∧F2(E) ≡ σF1(E) ∩ σF2(E)
10. **Distribution of join over union**: E ⋈ (E1 ∪ E2) ≡ (E ⋈ E1) ∪ (E ⋈ E2). All binary operators are commutative and associative **except difference**.

Worked example — `SELECT DISTINCT DName FROM EMP, DEPT WHERE EMP.Dept#=DEPT.Dept# AND Salary > 1000` (Card(EMP) ≈ 10,000, Card(DEPT) ≈ 100, selection ≈ 50):

```
πDName(σDept#=Dept# ∧ Salary>1000(EMP × DEPT))     -- initial tree
→ Prop 1 (atomize) → Prop 5 (join derivation) → Prop 3 (push selection on EMP)
→ Prop 2+4 (push projections)
πDName( πDept#(σSalary>1000(EMP)) ⋈ πDept#,DName(DEPT) )
```

The final tree reads EMP once through a small intermediate result (≈50 tuples) instead of filtering the Cartesian product.

### Data profiles

Quantitative information on tables and columns stored in the **data dictionary**: cardinality (also estimated for intermediate expressions), tuple size, attribute sizes, number of **distinct values** (active domain cardinality), min/max per attribute. Refreshed periodically via **update statistics** on demand (during transaction processing it would overload the system). Used to estimate intermediate sizes, e.g. under the **uniform distribution hypothesis**:

```
Card(σAi=v(T)) ≈ Card(T) / Val(Ai in T)
```

### Access operators

The query tree has physical structures (tables, indices) as leaves and operations as intermediate nodes (scan, join, group by):

- **Sequential scan (full table scan)**: accesses all tuples; during the scan it can perform projection, selection on simple predicates, sorting, insert/update/delete.
- **Sorting**: classical algorithms; size matters — memory sort (e.g. quicksort) for small datasets, disk sort for big ones; may exploit index access.
- **Index access**: simple equality `Ai = v` — hash, B+-tree or bitmap; **range** `v1 ≤ Ai ≤ v2` — **only B+-tree**; predicates with limited selectivity → full table scan (consider bitmap if available). Space trade-off B-tree vs bitmap (see above).
- **Conjunction** `Ai=v1 ∧ Aj=v2`: evaluate the **most selective predicate first** through the index, then the others on the intermediate result; optimization: intersect bitmaps/RIDs from available indices before reading the table (example: female ∧ exempt ∧ Piemonte → RID 5).
- **Disjunction**: index access only if **all** predicates are supported by an index, otherwise full table scan.

### Join operation

Critical for relational DBMSs: connection is based on **values instead of pointers**, and the intermediate result is typically larger than the smaller table. Algorithms:

| Algorithm | How it works | Notes |
|---|---|---|
| **Nested loop** | one full scan of the outer table; for each outer tuple a full scan (or indexed scan) of the inner table looking for matching values ("brute force") | not symmetric — cost depends on which table is inner; efficient when the inner table is small and fits in memory or the join attribute of the inner table is indexed |
| **Merge scan join** | both tables sorted on the join attributes, scanned in parallel generating pairs on corresponding values | symmetric; requires sorting both tables (may come from a previous operation or from a clustered index on the join attribute); efficient for large tables (only a portion of the sorted table in memory); more used in the past; if the outer is ordered the output is ordered too |
| **Hash join** | apply the same hash function to the join attributes of both tables → tuples to be joined end up in the same buckets (collisions possible); local sort and join within each bucket | very fast join technique |
| **Bitmapped join index** | bit matrix that **precomputes the join** between two tables: 1 if tuple RID j of A joins with tuple RID i of B | updates may be slow; typically used in **OLAP** queries joining several tables to a large central table (e.g. Exam joined to Student and Course); access to the central table is the last step; complex queries combine bitmapped join indices with bitmap indices for single-table predicates |

Bitmapped join example: average grade of male students for first-year master courses — AND/OR of bitmaps for `CourseYear='1M'`, `Gender='M'` and the join bitmaps yields the RIDs of Exam tuples to read.

**Group by**: sort-based (sort on the grouping attributes, then aggregate) or hash-based (hash on grouping attributes, sort each bucket, aggregate); **materialized views** may improve aggregation performance.

### Execution plan selection

Cost-based optimization evaluates alternatives along: how data is read (full scan vs index), execution order among operators (e.g. join order), implementation technique per operator (join method), when to sort. The optimizer builds a **tree of alternatives** (each internal node a decision, each leaf a complete plan). Example: join R ⋈ S ⋈ T → 4 join techniques × 4 join techniques × 3 join orders = **at most 48 alternatives**; the leaf with the lowest cost is selected:

```
CTotal = CI/O · nI/O + Ccpu · ncpu
```

Selection uses operation-research techniques (e.g. **branch and bound**); the final plan is an **approximation** of the best solution (same order of magnitude); with compile & go the search stops when the time spent searching is comparable to executing the current best plan.

### The Oracle optimizer (case study)

Components: **Query Transformer** (parsed query as nested/interrelated query blocks; innermost block optimized first, bottom-up; may rewrite the query — e.g. correlated sub-queries or views into equivalent joins), **Estimator** (selectivity, cardinality, cost = disk I/O, CPU, memory; uses dictionary statistics incl. histograms), **Plan Generator** (tries plans combining access paths, join methods, join orders; internal **cutoff** based on the current best plan's cost).

**EXPLAIN PLAN** displays the chosen execution plan (table order, access method per table, join method, filter/sort/aggregation, cost and cardinality estimates, partitioning, parallel execution). Access paths:

- **Full table scan** — reads all rows, filters non-matching; sequential multiblock reads (parameter `DB_FILE_MULTIBLOCK_READ_COUNT`). Used for: lack of index, retrieval of a large portion of data, small tables. Decision influenced by the **index clustering factor**: low = rows with similar indexed values concentrated in few blocks; high = values scattered across blocks → range scan by rowid costs more.
- **Index unique scan** — at most one rowid per value (UNIQUE / PRIMARY KEY equality).
- **Index range scan** — selective access returning data in ascending index order (`col1 = :b1`, `col1 <= :b1`, `col1 >= :b1` on leading columns); unique or non-unique indices; **avoids sorting** when the index covers ORDER BY/GROUP BY.
- **Index full scan** — reads the whole index **singly**, ordered by key; can eliminate a sort (GROUP BY, ORDER BY, MERGE JOIN); usable with no predicate when all queried columns are in the index and at least one is NOT NULL.
- **Fast full index scan** — alternative to full table scan when the index contains all needed columns and at least one key column is NOT NULL; reads the index with **multiblock reads**; faster than a normal full index scan but **cannot eliminate sorting** (not ordered by key).
- **Rowid scan** — rowid = physical address (file, block, position); fastest way to a single row; rowids come from a previous index scan.
- **Bitmap indexes** — most effective for **multiple conditions** in the WHERE clause (Boolean merge of bitmaps); easier to destroy and recreate than to maintain.

Join methods: **nested loop** (small subsets, good driving condition; outer = driving row source, inner ideally accessed by index scan), **hash join** (large data sets or large fraction of a small table, equijoin; smaller table builds an in-memory hash table, larger table probes it — best when the smaller table fits in memory), **sort merge** (inputs already sorted or sort reusable for the next operation; also for inequality conditions like <, ≤, >, ≥; better than nested loop for large data sets; steps: sort both inputs on the join key, merge).

**Statistics**: table stats (rows, blocks, average row length), column stats (NDV, nulls, histograms — height-balanced and frequency for skewed distributions), index stats (leaf blocks, levels, clustering factor), system stats (I/O, CPU). Gathered automatically by `GATHER_STATS_JOB` (nightly + weekends) or manually via the **DBMS_STATS** package (weekly/monthly for incrementally modified tables; with bulk loads as part of the batch); updated statistics invalidate parsed statements, which are re-parsed with new plans. Optimizer goals: **best throughput** (`ALL_ROWS`, default) vs **best response time** (`FIRST_ROWS_n`, `ALTER SESSION SET OPTIMIZER_MODE = FIRST_ROWS_1;`). An EXPLAIN PLAN alone does not prove efficiency — check index selectivity and test.

## Physical Design

Phases of database design: **conceptual** (ER/UML, from application requirements — workload not considered) → **logical** (relational tables) → **physical** (physical schema; **the workload must be considered**). Goal of physical design: **good performance** for database applications without affecting application software (**data independence**). It requires selection of the DBMS product (different products provide different storage structures and access techniques).

**Inputs**: logical schema; features of the DBMS (index types, page clustering); **workload** — important queries with estimated frequency, update operations with frequency, required performance. **Outputs**: physical schema (table organization, indices); set-up parameters (initial file size, extensions, free space, buffer size, page size — defaults provided).

**Workload characterization**: for each query — accessed tables, projected attributes, attributes in selections/joins, selectivity; for each update — attributes/tables in selections, selectivity, update type and updated attributes. Decisions: physical storage of tables (file structure heap/clustered, hashing on a hash key, clustering of several relations with interleaved tuples), indices per table (which attributes, hash or B+-tree, clustered or unclustered — **only one clustered/primary index per table**, many unclustered/secondary), possible changes to the logical schema (BCNF-preserving alternatives or not, e.g. data warehouses), partitioning on different disks.

> [!warning] No general methodology
> Physical design is **trial and error**, driven by general criteria, common sense and heuristics; it can be improved after deployment (**database tuning**). **Hints** (Oracle) force the optimizer decision but lose data independence — should almost never be used.

Guidelines:

- Index the **primary key** (usually used for selections and joins); clustered or unclustered depending on other constraints.
- Add indices for the most common query predicates: pick a frequent query, compare its current plan with the plan using a new index; add the index if cost improves, then verify the effect on the **modification workload** and disk space.
- **Never index small tables** (a full read costs few disk reads) and **never index low-cardinality attributes** (low selectivity, e.g. gender) — not true in **data warehouses** (different workloads, bitmap indices).
- Equality predicate → hash preferred (or B+-tree); range predicate → B+-tree. Consider a clustered index for slow queries.
- Many simple predicates → **multi-attribute (composite) index**; **key order matters** (affects usability of the index) and update overhead grows.
- Joins: nested loop → index on the **inner** table join attribute; merge scan → B+-tree on the join attribute (clustered if possible). Group by → hash or B+-tree on grouping attributes; consider **group by push down** (anticipation of group by w.r.t. joins — not available in all systems; check the execution plan).

Worked examples (EMP/DEPT): `Salary/12 = 1500` defeats an index (arithmetic expression → index disregarded); `Salary = 18000` uses the index but with no benefit if the value is very frequent (distribution!); secondary index on `EMP.Dept#` is useless if DEPT has 50 values (each page contains almost all departments → sequential scan) but appropriate with 2000 departments; `DName='Toys'` join → hash index on DName (selection) + hash on EMP.Dept# (nested loop with EMP inner); equality predicates are usually more selective than ranges (hash on Hobby vs B+-tree on Salary; smart optimizers intersect RIDs); for `GROUP BY Dept#` a **clustered** index on Dept# avoids sorting, a **covering index** on Dept# (`SELECT Dept#, COUNT(*)`) answers the query without reading the table; composite index **⟨Age, Salary⟩** is fastest for `Age = 25 AND Salary BETWEEN 3000 AND 5000` if Age is the most selective.

## Concurrency Control

Operational DBMS workload is measured in **tps (transactions per second)** — ≈ 10–10³ for banking and flight reservations. Concurrency control provides concurrent access while **maximizing throughput and minimizing response time**. Elementary operations: **r(x)** read of a data object (SELECT), **w(x)** write (INSERT, UPDATE, DELETE); they may require reading/writing an entire page.

The **scheduler** (block of the concurrency control manager) decides if and when read/write requests can be satisfied; its absence causes **anomalies**:

- **Lost update**: T1 and T2 both read x=2, both increment; T2 commits, then T1's write overwrites it → x=3 instead of 4 (T2's effect lost because both read the same initial value).
- **Dirty read**: T2 reads a value written by T1 (intermediate state); T1 later **aborts** → cascade rollback; T2 read a state that never becomes stable.
- **Inconsistent read**: T1 reads x twice and gets different values; or T1 sums x+y+z while T2 transfers 100 between y and z → total 1100 instead of 1000 (T1 only partially observes T2's effect); aggregate example: AVG salary before/after a **ghost update** (insert of a new employee not yet in the database).

### Theory: schedules and serializability

A **transaction** is a sequence of read/write operations with the same **TID**; a **schedule** is the sequence of read/write operations of concurrent transactions in arrival order. The scheduler must accept/reject operations **without knowing the transaction outcome** (abort/commit). Simplifying hypothesis of **commit projection**: only committing transactions in the schedule (dirty read not addressed — removed later).

- **Serial schedule**: actions of each transaction appear in sequence, no interleaving.
- A schedule (commit projection) is **serializable** when it yields the same result as an arbitrary serial schedule of the same transactions.

**Equivalence classes** (each detects a set of acceptable schedules with different detection complexity): **view equivalence**, **conflict equivalence** ("used in all DBMSs"), 2-phase locking, timestamp equivalence.

**View equivalence**: `ri(x)` **reads-from** `wj(x)` when wj(x) precedes ri(x) (i ≠ j) and no other wk(x) lies between; `wi(x)` is a **final write** if it is the last write of x in the schedule. Two schedules are **view equivalent** if they have the same reads-from set and the same final-write set. A schedule is **view serializable (VSR)** if view equivalent to a serial schedule of the same transactions. Detecting view equivalence to a *given* schedule is linear; to an *arbitrary serial* schedule is **NP-complete** → not feasible in real systems.

**Conflict equivalence**: actions Ai(x), Aj(x) (i ≠ j) are **in conflict** when both operate on the same object, at least one is a write, and no other write on x lies between — read-write (RAW/WAR) and write-write (WAW) conflicts. Two schedules are **conflict equivalent** if they have the same conflict set with each conflicting pair in the same order; a schedule is **conflict serializable (CSR)** if conflict equivalent to a serial schedule. Detection via **conflict graph** (node per transaction, edge Ti → Tj if Ai precedes Aj in a conflict): **acyclic ⇔ CSR**; cyclicity check is linear. Still expensive at real rates (100 tps, ≈10 pages/transaction, ≈5 s duration → graph of 500 nodes, 5000 accesses to update and check per access). **CSR ⊂ VSR** (schedules exist that are VSR but not CSR).

### Two Phase Locking (2PL)

A **lock** is a block on a resource which may prevent access to others: **read lock (R-Lock)** — shared, a counter tracks holders, free when count = 0 — and **write lock (W-Lock)** — exclusive, incompatible with any other lock on the same data. Each read is preceded by an R-Lock request and followed by unlock; similarly for writes. **Lock escalation**: R-Lock followed by W-Lock on the same data. The scheduler becomes a **lock manager**: granted request → resource acquired until unlock; denied → requesting transaction **suspended in a waiting queue**, resumed when the resource is freed (first in queue).

Lock conflict table (request × resource state):

| Request | Free | R-Locked | W-Locked |
|---|---|---|---|
| R-Lock | Ok / R-Locked | Ok / R-Locked | **No** |
| W-Lock | Ok / W-Locked | **No** | **No** |
| Unlock | Error | Ok (free if no other R-lock) | Ok / Free |

**2PL** (used by most commercial DBMSs): **growing phase** (locks acquired) followed by **shrinking phase** (locks released) — a transaction cannot acquire a new lock after releasing any lock. 2PL guarantees serializability: **2PL ⊂ CSR ⊂ VSR** (some serializable schedules are not accepted). Worked example: T1 and T2 on x, y, z with r_lock/w_lock, waits and unlocks.

**Strict 2PL**: locks released **only at the end of the transaction** (after COMMIT/ROLLBACK) → data stable after the end → drops the commit-projection hypothesis and **avoids dirty reads**. Lock manager interface: `R-Lock(T, x, ErrorCode, TimeOut)`, `W-Lock(...)`, `UnLock(T, x)`; a request either is satisfied (small delay) or the transaction waits up to **TimeOut** — on expiry the transaction is resumed with a not-ok error code and may roll back or retry (probability of conflict ≈ (K·M)/N with K active transactions, M objects accessed on average, N objects in the database).

### Hierarchical locking

Locks can be acquired at different **granularity levels**: table → group of tuples (fragment, physical partitioning such as data page, or logical partitioning such as tuples satisfying a property) → single tuple → single field. Primitives: **SL** (shared), **XL** (exclusive), **ISL** (intention of shared on a descendant), **IXL** (intention of exclusive on a descendant), **SIXL** (shared on the current object + intention of exclusive on descendants). Compatibility matrix:

| Request \ State | ISL | IXL | SL | SIXL | XL |
|---|---|---|---|---|---|
| **ISL** | Ok | Ok | Ok | Ok | No |
| **IXL** | Ok | Ok | No | No | No |
| **SL** | Ok | No | Ok | No | No |
| **SIXL** | Ok | No | No | No | No |
| **XL** | No | No | No | No | No |

Request protocol: (1) locks are requested **from the root going down**; (2) released from the smaller granularity **going up**; (3) SL or ISL on a node requires ISL (or IXL) on its parent; (4) XL, IXL or SIXL requires IXL or SIXL on the parent. **Granularity choice**: localized reads/updates of few objects → low levels (fine); massive reads/updates → high levels (rough). Too coarse reduces concurrency (high conflict likelihood); too fine overloads the lock manager. **Predicate locking** addresses the ghost-update (insert) anomaly: 2PL does not consider a read in conflict with the insert of a new tuple (it cannot be locked in advance); predicate locking locks **all data satisfying a predicate**; implemented in real systems by **locking indices**.

**SQL2 standard**: transaction types read-write (default) and read-only (shared locks suffice). Isolation levels set via `SET TRANSACTION [ISOLATION LEVEL <level>] [READ ONLY | READ WRITE]`:

| Level | Behaviour |
|---|---|
| **SERIALIZABLE** | highest; strict 2PL **with predicate locking** |
| **REPEATABLE READ** | strict 2PL without predicate locking: reads of existing objects repeatable, but **no protection against ghost update (b)** — aggregate computations cannot be repeated |
| **READ COMMITTED** | not 2PL: the read lock is released as soon as the object is read; no reading of intermediate states (dirty reads avoided) |
| **READ UNCOMMITTED** | not 2PL: data read **without acquiring the lock** — dirty reads allowed; only for read-only transactions |

The isolation level may be lowered only for reads: **write operations always run under strict 2PL with exclusive lock**.

### Deadlock

T1 holds a lock on x and waits for y; T2 holds y and waits for x → both wait forever (typical of locking-based concurrency). Solutions:

- **Timeout** (typical of commercial DBMSs): after expiry the transaction gets a negative answer and rolls back; interval length is a trade-off — long → long waits; short → **overkill** (rolling back transactions that are not deadlocked), overloading the system.
- **Prevention**: pessimistic 2PL (acquire **all** locks before the transaction starts — not always feasible); **timestamps** (only "younger" or only "older" transactions may wait — may cause overkill).
- **Detection**: **wait graph** (nodes = transactions, edge = waiting state); a cycle is a deadlock; expensive to build and maintain — used in distributed DBMSs.

## Reliability Management and Recovery

The Reliability Manager is responsible for **atomicity and durability**; it implements `begin transaction` (implicit), `commit work`, `rollback work` and provides the recovery primitives **warm restart** (main-memory failures) and **cold restart**; it interacts with the buffer manager for read/write requests and may generate new ones for reliability purposes, exploiting the **log file** and preparing data via **checkpoint** and **dump**.

**Stable memory**: memory resistant to failure — an abstraction approximated via **redundancy** and **robust write protocols**; failures in stable memory are considered catastrophic.

### Log file

Sequential file in stable memory recording transaction activities in **chronological order**; records of different transactions are interleaved. Record types:

- **Transaction records** — delimiters `B(T)`, `C(T)`, `A(T)` (begin/commit/abort, T = TID) and data modifications: `I(T, O, AS)` insert, `D(T, O, BS)` delete, `U(T, O, BS, AS)` update, where O is the written object (RID), **AS/BS** the after/before states of O.
- **System records** — `Dump` and `Checkpoint CK(L)` where L = TIDs of active transactions.

**Undo and Redo semantics** (inverse actions):

| Action | Undo | Redo |
|---|---|---|
| insert O | delete O | write AS of O |
| update O | write BS of O | write AS of O |
| delete O | write BS of O | delete O |

**Idempotency**: undo/redo can be repeated an arbitrary number of times without changing the outcome — `UNDO(UNDO(action)) = UNDO(action)` — useful for crashes *during* recovery.

### Rules for writing the log

- **WAL (Write Ahead Log)**: the **before state (BS)** of a log record is written in stable memory **before the database data is written on disk** → during recovery, undo can be executed on data already written to disk.
- **Commit precedence**: the **after state (AS)** is written in stable memory **before commit** → redo is possible for transactions that committed but were not yet written to disk.

The log is written **synchronously (force)** for data modifications written to disk and on commit; **asynchronously** for abort/rollback. The **commit record is a border line**: if it is not in the log, the transaction must be **undone** upon failure; if it is, it must be **redone**. Combining with buffer policies: **undo/redo** (disk writes before *and after* commit — mixed approach adopted in real systems, requires both operations), **no-undo/redo** (all writes after commit), **undo/no-redo** (all writes before commit — force). Log writing is optimized (compact format, parallelism, commit of groups of transactions) because robust protocols cost as much as a database update.

### Checkpoint and dump

**Checkpoint** (periodically requested by the Reliability Manager to the Buffer Manager; enables a **faster recovery**): (1) record the TIDs of all active transactions — during the checkpoint **no transaction can commit** until it ends; (2) pages of **concluded** transactions (committed or aborted) are **synchronously written** on disk (force primitive); (3) a checkpoint record `CK(L)` with the active-transaction set is synchronously written to the log. After a checkpoint: effects of all committed transactions are permanently on disk; the state of pages written by active transactions is **unknown**.

**Dump**: complete copy of the database, typically offline, stored in stable/tertiary/off-line storage, possibly **incremental**; a dump record (date/time, device) closes it in the log.

### Failures and restarts

- **System failure** (software problems, power supply): loses **main memory** (buffer) but not the disk (database and log) → **warm restart**.
- **Media failure** (secondary-memory device failure): loses the database on disk but not the log (stable storage) → **cold restart**.

Fail-stop model: normal operation → STOP at failure → RECOVERY → availability.

**Warm restart algorithm** (example: T1 completed before the checkpoint; T2, T4 committed after it with writes possibly pending; T3, T5 active at failure tF):

1. Read the log **backwards** until the last checkpoint record.
2. Build the lists: at the checkpoint `UNDO = active transactions`, `REDO = {}`; reading **forward**, add to UNDO every transaction whose `B` is found, and move from UNDO to REDO every transaction whose `C` is found (transactions ending with rollback stay in UNDO). In the example: `UNDO = {T3, T5}`, `REDO = {T2, T4}`.
3. **Data recovery**: (a) read the log **backwards** from the failure until the **begin record of the oldest UNDO transaction** (even before the last checkpoint) and undo all UNDO actions; (b) read **forward** from the begin record of the oldest REDO transaction and redo all REDO actions.

Worked log example — `B(T1) B(T2) U(T2,O1,B1,A1) I(T1,O2,A2) B(T3) C(T1) B(T4) U(T3,O2,B3,A3) U(T4,O3,B4,A4) CK(T2,T3,T4) C(T4) B(T5) U(T3,O3,B5,A5) U(T5,O4,B6,A6) D(T3,O3,B7) A(T3) C(T5) I(T2,O6,A8)` + failure: forward scan gives UNDO = {T2, T3}, REDO = {T4, T5}; undo sequence: DELETE O6, INSERT O3 = B7, O3 = B5, O2 = B3, O1 = B1; redo: O3 = A4, O4 = A6. Without checkpoint record the entire log down to the last dump would have to be read.

**Cold restart**: (1) restore the damaged portion from the last **dump**; (2) from the dump record, read the log forward and **redo all actions** (and commit/rollback) — alternatively, only actions of committed transactions, requiring two log reads (build REDO list, then redo); (3) perform a warm restart.

## Distributed Databases

Data and computation distributed over different machines, with complexity depending on node independence. Typical advantages: performance, availability, reliability.

- **Client/server** (simplest and most widespread): server manages the database, client the user interface. **2-tier** with thick clients (some application logic); **3-tier** with thin clients (browser) + application server (business logic, typically also web server) + DBMS server.
- **Distributed database system**: different DBMS servers on different network nodes, autonomous and cooperating — guaranteeing ACID requires more complex techniques.
- **Data replication**: a **replica** is a copy of data on a different node; the replication server autonomously manages copy updates — simpler architecture than a distributed database.
- **Parallel architectures**: performance is the only objective (multiprocessors, clusters, dedicated networks).
- **Data warehouses**: servers specialized in decision support performing **OLAP** (analytical) vs **OLTP** (transactional) processing.

Relevant properties: **portability** (moving programs between systems — guaranteed by the SQL standard) and **interoperability** (DBMS servers cooperating — interaction protocols: ODBC, X-Open-DTP).

### Distributed database design

> [!definition] Fragmentation
> Given relation R, a **fragment** is a subset of R in terms of tuples, schema, or both. **Horizontal**: subset of tuples (same schema) obtained with σp (partitioning predicate p), non-overlapping fragments; reconstruction R = E1 ∪ E2 ∪ … ∪ EN. **Vertical**: subset of the schema obtained with πX (all tuples); the **primary key must be included** to allow rebuilding; fragments overlap on the primary key; reconstruction R = E1 ⋈ E2. **Mixed**: both. Properties: **completeness** (each information of R in at least one fragment) and **correctness** (R rebuildable from its fragments).

Example — `Employee(Emp#, Ename, DeptName, Tax)` split horizontally by DeptName, or vertically into `π Emp#,Ename,DeptName` and `π Emp#,Ename,Tax`. Each fragment is usually stored in a different file, possibly on a different server; **relation R does not exist** (rebuilt from fragments). The **allocation schema** describes fragment placement: **non-redundant** (each fragment on one node) or **redundant** (fragments replicated — higher availability, complex maintenance, copy synchronization needed).

**Transparency levels** (what applications know about distribution), on `Supplier S(S#, SName, City, Status)` split into S1 (Torino) and S2 (Roma, replicated on two nodes):

| Level | Application view | Example |
|---|---|---|
| **Fragmentation transparency** | tables, not fragments | `SELECT SName FROM S WHERE S#=:CODE` |
| **Allocation transparency** | fragments, not their allocation/replication | query S1, if not found query S2 |
| **Language transparency** | fragment **and** node (the format produced internally by a distributed DBMS) | `SELECT SName FROM S1@xxx.torino.it WHERE ...` |

### Transaction classes (increasing complexity)

| Class | Allowed | Server scope |
|---|---|---|
| **Remote request** | read-only (single SELECT) | single remote server |
| **Remote transaction** | any SQL command | single remote server |
| **Distributed transaction** | any SQL command, **each statement** to one server; global atomicity via **2 phase commit** | multiple |
| **Distributed request** | each command may span **data on different servers**; needs distributed optimization; **fragmentation transparency only in this class** | multiple |

Example: `Account(Acc#, Name, Balance)` fragmented A1 (Acc# < 10000) @Node1, A2 @Node2; a transfer updating Acc# 3000 and 13000 on table `Account` is a **distributed request**; explicit updates on A1/A2 are a **distributed transaction**; both updates on A1 only → remote transaction.

Distributed ACID: atomicity via 2PC; consistency — constraints currently enforced **only locally**; isolation — strict 2PL + 2PC; durability — extension of local procedures. **Distributed query optimization** is performed by the DBMS receiving the request: partition into subqueries (one per server), choose order and techniques of operations across nodes, optionally select replicas — **transmission cost may become relevant**.

### Two Phase Commit (2PC)

Objective: all nodes participating in a distributed transaction implement **the same decision** (commit or rollback). Roles: one **coordinator — Transaction Manager (TM)** and several participants — **Resource Managers (RM)**; any participant (also the client) may act as TM. Failures: node failures, network failures (lost messages — handled with acks and timeouts), network partitioning.

Phase I: the TM writes the **prepare** record in its log (identity of all participating RMs), sends **prepare** to all RMs, sets a timeout. Each RM waits for prepare: if in a **reliable state** it writes the **ready** record (decision irrevocable afterwards; WAL and commit-precedence enforced; resources locked — the RM **loses its autonomy** for the transaction) and answers **ready**; otherwise answers **not ready** and performs local rollback; a crashed RM sends no answer. The TM collects answers: ready from all → **global commit** record; any not-ready or timeout expiry → **global abort** record.

Phase II: the TM sends the global decision and sets a timeout; each RM writes the **commit/abort** record, updates the database, and sends an **ACK**; when all ACKs arrive the TM writes the **complete** record; missing ACKs (timeout) → new timeout, decision resent, until all answers are received.

> [!definition] Uncertainty window
> For each RM it starts after the **ready** message is sent and ends upon receipt of the **global decision**: local resources stay **locked**, so it should be small.

Failure handling: **participant failure** — warm restart is extended with a **READY list**: for transactions whose last log record is *ready*, the global decision is asked to the TM (**remote recovery request**). **Coordinator failure** — if the last TM log record is *prepare*: global **abort** written and sent (alternative: redo phase I, not implemented); if the last record is the global decision: repeat phase II. **Network failures**: any problem in phase I → global abort; in phase II → repeat phase II.

**X-Open-DTP**: protocol for coordinating distributed transactions on **heterogeneous** DBMSs; based on one client, one TM, several RMs; interfaces: **TM interface** (client↔TM) and **XA interface** (TM↔RM, provided by DBMS servers; specialized products implement the TM, e.g. BEA Tuxedo). RMs are passive (answer TM invocations); optimizations: **presumed abort** (with no information in the log the TM answers *abort* to a recovery request — prepare, global abort and complete need not be synchronous; synchronous writes remain for global commit in the TM log and ready/commit in RM logs) and **read only** (an RM that did not modify its database answers *read only* to prepare, writes no log, terminates the protocol, and is ignored in phase II). **Heuristic decisions** allow controlled evolution during TM failure: a blocked RM evolves locally under operator control (transaction end forced, typically rollback, rarely commit); if heuristic and actual TM decisions differ, **atomicity is lost** but the inconsistency is notified to the client — resolution is up to user applications.

### Parallel DBMS and benchmarks

**Inter-query parallelism**: different queries on different processors — OLTP workloads (simple short transactions, high load 100–1000 tps), load balancing on the processing pool. **Intra-query parallelism**: subparts of the same query on different processors — OLAP workloads (complex queries, reduced load); complex queries split into subqueries operating on data subsets (e.g. parallel large table scan on fragments on different disks; group by partitioned on processors then merged); group by and join are easily parallelizable; pipelining possible.

**TPC (Transaction Processing Council) benchmarks** standardize performance measurement: each defines transaction load, arrival-time distribution, database size/content (randomized generation), transaction code, measurement/certification techniques. **TPC-C** order entry (OLTP; evolution TPC-E), **TPC-H** decision support (OLAP, mix of complex queries; also TPC-DI, TPC-DS), **TPCx-HS** big data management (Hadoop clusters).

## Beyond Relational Databases (NoSQL)

«NoSQL» birth: 1998 Carlo Strozzi's lightweight relational database without the SQL interface; 2009 Johan Oskarsson's (Last.fm) event on non-relational databases, promoted with the hashtag **#NoSQL**.

Main features: **schema-less** (no tables, implicit schema), **no joins** (joins are expensive; a different design approach is needed, e.g. embedding), **horizontal scalability** on smaller, cheaper devices. Contrast with relational databases:

| Relational | Non-relational |
|---|---|
| table-based, structured rows | specialized storage: document-based, key-value, graph, columnar |
| predefined schema; changes blocking (expensive in distributed live environments) | schema-less/schema-free; dynamic change per document; semi/un-structured data |
| vertical scalability (more powerful hardware) | horizontal scalability (more servers in the pool) |
| SQL, very powerful | custom query languages per database type |
| suitable for complex queries with joins | no standard interfaces for complex queries, no joins |
| flat structured data | complex (e.g. hierarchical) data, similar to JSON/XML |
| MySQL, Oracle, SQLite, Postgres, SQL Server | MongoDB, BigTable, Redis, Cassandra, HBase, CouchDB |

Types of NoSQL databases: **key-value** (simplest; keys ↔ values, no structure, great performance, easily scaled, very fast — Redis, Riak, Memcached); **column-oriented** (columnar format; a column is a possibly-complex attribute; key-value pairs retrieved on key in parallel systems; rows rebuilt from column values, transparent to the application — Cassandra, HBase, Hypertable, Amazon DynamoDB); **graph** (vertex and edges; networks — Neo4J, Infinite Graph, OrientDB); **document** (keys mapped to **self-describing documents**; hierarchical tree nested structures: maps, lists, datetime; heterogeneous documents — MongoDB, CouchDB, RavenDB).

### CouchDB and MapReduce

**CouchDB** (Cluster Of Unreliable Commodity Hardware): document-oriented, written in Erlang, incremental replication with bidirectional conflict detection and resolution, queried and indexed in a **MapReduce** fashion, **HTTP RESTful API** (a "web" database: `GET /somedatabase/some_doc_id`, `PUT` with revision to avoid conflicts → 201 Created / 409 Conflict).

**MapReduce** (published by Google 2004, Dean & Ghemawat): distributed programming model to process large data sets with parallel algorithms on clusters of common machines — moves the **computation to the data**. Two functions (Reduce optional → map-only jobs): **Map** processes each record/document and returns key-value pairs (independent of any information outside the document → incremental and parallel view generation); **Reduce** reduces, for each key, the list of its values to a single (possibly complex) value. Worked examples on exam-record documents:

```js
// Map: list of exams and marks               // Reduce: average mark per exam
function(doc) { emit(doc.exam, doc.mark); }    function(key, values) {
                                                  S = sum(values); N = len(values);
                                                  AVG = S/N; return AVG; }
```

Composite keys (`[doc.exam, doc.AYear]`) group per exam *and* academic year; values can be lists (`[doc.mark, doc.CFU]` → CFU-weighted average per student: `S = sum([X*Y for X,Y in values]); N = sum([Y for X,Y in values]); AVG = S/N`); **re-reduce** applies reduce to its own results (group level=1 aggregation). **Views (indexes)**: the only way to query CouchDB; a view is produced by a MapReduce and materialized as values **sorted by key** (multiple primary indexes per DB); the predefined view has document ID as key, whole document as value, no reduce.

### Replication, CAP, ACID vs BASE

Replication places the same data (whole or chunks) in different places (local/remote servers, clusters, data centers) for redundancy (availability) and performance. **Master-slave**: the master takes all writes, slaves serve reads (read scalability only; master = single point of failure; CouchDB supports master-master). **Synchronous replication**: master waits for (all) slaves to commit before committing — conceptually like 2PC, a performance killer; trade-off waiting for a subset/majority. **Asynchronous replication**: master commits locally; slaves independently fetch updates — faster but unreliable (lost data if no slave replicated; reconciliation needed otherwise).

**CAP theorem** (Brewer's conjecture, 1999–2000; proof by Gilbert & Lynch 2002): a distributed system cannot simultaneously guarantee **Consistency** (all nodes provide the same data), **Availability** (failures do not prevent survivors from operating) and **Partition tolerance** (operation despite message loss/network partitions). With two nodes on opposite sides of a partition: allowing updates forfeits C; preserving consistency forfeits A; without partitions both C and A are possible. Combinations: **CA without P** (local consistency — multiple independent systems with internal CA), **CP without A** (transaction locking — block access to non-synchronized replicas; global consistency), **AP without C** (best effort — no assurance of global consistency). Beyond CAP ("2 of 3" is misleading): partitions are rare (little reason to forfeit C or A when not partitioned); the C-vs-A choice occurs at fine granularity (per subsystem, operation, data, user); all three properties are continuous, not binary.

**ACID vs BASE**: opposite design philosophies on the consistency–availability spectrum. ACID (traditional, focused on consistency): atomic operations; valid-state transitions preserving all database rules; isolation (at most one side of a partition); permanent results. **BASE** — **B**asically **A**vailable, **S**oft state, **E**ventually consistent (focus on high availability; works well in presence of partitions; example: DNS). Conflicts under eventual consistency (two customers booking the last hotel room on non-synchronized nodes): applications resolve them with custom business logic; the database detects the conflict and may provide a local solution — CouchDB picks a **deterministic winner** (longest revision history; ties broken by ASCII order of `_rev` values).

**Polyglot persistence**: building one app with multiple databases — e.g. Elasticsearch for text search, MongoDB for non-relational storage, Cassandra for IoT sensor data.

### Elasticsearch

Real-time **distributed search and analytics engine**: scalable data exploration, full-text search (highlighted snippets, search-as-you-type, did-you-mean, more-like-this), structured search, analytics on mixed data types. **Document-oriented (JSON)**, built on the **Lucene** library; highly available and horizontally scalable; RESTful API in any language; modifications recorded in transaction logs replicated on multiple nodes. Users: GitHub (130+ B lines of code), Wikipedia, StackOverflow (full-text + geolocation).

Terminology mapping: **field** ≈ SQL column (may hold **multiple values**), **document** ≈ row (flexible, no strict schema), **index** ≈ table, **cluster** ≈ database. "Index" is overloaded: (noun) a collection of documents; (verb) *to index* = insert/replace a document; (**inverted index**) the additional structure accelerating retrieval — list of all unique words in the collection, each with the list of documents containing it (like the analytic index of a book); **every field is indexed** and non-indexed fields are not searchable. A document is a JSON root object uniquely identified by (index, id) — the id can be provided or generated by ES.

Search options: structured query on specific fields (SQL-like, possibly sorted), full-text query (all matching documents **sorted by relevance**), or a combination. Key concepts: **Mapping** (how each field is interpreted; ES guesses data types — inference, e.g. dates), **Analysis** (how full text is processed to become searchable), **Query DSL** (the query language, JSON in an HTTP body). **Exact values** (integer, float, date, strings like username) answer "does this document match?"; **full text** answers "**how well** does it match?" — relevance; intent matters (USA vs United States, singulars/plurals, synonyms, word order). **Analysis**: (1) tokenization into terms for the inverted index; (2) normalization (lowercase, stemming — cats vs cat, synonym management); indexed text and query string must be analyzed the same way. **Analyzers**: character filters (clean before tokenization, e.g. `&` → `and`) + tokenizers (split into words) + token filters (lowercase, remove stopwords, add synonyms); built-in analyzers provided.

**Filter vs query**: a **filter** (exact values) gives a boolean matches/does-not-match answer and is more efficient — used to reduce the documents examined; a **query** (full-text) computes a **relevance _score** used to sort matches. Hint: query clauses for full-text or anything affecting the score; filter clauses for everything else. Query DSL examples:

```json
POST departments/_search { "query": { "match": { "name": "John" } } }

POST departments/_search
{ "query": { "bool": {
    "should": [ {"match": {"name": "John"}}, {"match": {"name": "Mark"}} ],
    "minimum_should_match": 1,
    "must":     { "match": { "title": "developer" } },
    "must_not": { "match": { "lastname": "Smith" } } } } }
```

`bool` is the compound query; `should` = OR, `must` = AND, `must_not` = NOT. The **match query** works on full-text fields (analyzes the query string, returns _score) and exact fields (exact value, _score 1); a bool query **combines the _scores** of matching must/should clauses. Multiple indices can be searched in the URI (`POST rooms,students/_search`); by default the **top 10** relevant results are returned; index types deprecated since 7.0.

Data modification: insert via `POST /index_name/<id>` with the JSON document (id optional, auto-generated); documents are **immutable** — update = retrieve old document, modify the copy, delete old, index new (old version cleaned in background); update via `PUT index_name/123/_update {"color": "red"}`; delete via `DELETE index_name/id` (removal not immediate).

**Scoring**: relevance is a floating-point `_score` per matching document; default sort descending by _score. Pipeline: (1) compute matching results; (2) select top hits (default 10); (3) optionally rescore with a more expensive algorithm. Similarity: **Boolean model** selects matches (fast); term weights via **TF/IDF** in the **Vector Space Model**; cosine similarity between query and document vectors. TF/IDF factors (computed and stored **at index time**): **term frequency** `Tf(t in d) = frequency` (more often → more relevant); **inverse document frequency** `Idf(t) = 1 + log(numDocs / (docFreq + 1))` (more common → less relevant); **field-length norm** `norm(d) = 1 / sqrt(numTerms)` (longer field → less relevant). Vector example: query "happy hippopotamus" → [2, 5]; documents → [2,0], [0,5], [2,5]; the angle between vectors gives the relevance.

**Horizontal scalability — sharding**: an index is divided into **shards**, each an instance of a Lucene index; each document belongs to one shard; writes become available for querying every ~1 second in immutable Lucene segments. Shards are the elementary units distributed on cluster nodes: data split into smaller chunks (1 TB storable without any single 1-TB node), operations parallelized across nodes, shards **replicated** for availability. **Document versioning**: ES uses **optimistic concurrency control** (conflicts assumed unlikely; updates fail if data changed between read and write — unlike ACID locking); replication sends primary copy first, replicas may arrive out of sequence; every document has a **_version** number incremented on change so older versions never overwrite newer ones; update/delete APIs accept a version parameter.

### MongoDB

Leading document-based NoSQL database: high performance, high availability, native scalability, high flexibility, open source. Terminology: **collection** ≈ table, **document** ≈ record, **field** ≈ column. Records are stored as **BSON** documents (binary JSON): field-value pairs, possibly nested; mapping into developer-language objects (date, timestamp, arrays, sub-documents). `_id` is reserved as primary key: unique in the collection, immutable, possibly auto-generated, any type except array. Max BSON document size **16 MB** (GridFS for larger); fields in a BSON document are **ordered** (unlike JavaScript objects).

Each instance manages multiple databases, each with collections; schema-less (documents in one collection may differ), but since 3.2 **document validation rules** can be enforced on insert/update. Shell basics: `show databases`, `use <db>`, `db.dropDatabase()`, `db.createCollection("authors", {capped: true})` (also created implicitly on first reference), `show collections`, `db.<coll>.drop()`.

**CRUD** — Create: `db.people.insertOne({user_id: "abc123", age: 55, status: "A"})`; `insertMany([{...},{...}])` (arrays and nested documents allowed: `favorite_colors: ["blue","green"]`, `address: {street:…, city:…}`). Read: `db.<coll>.find({conditions}, {fields of interest})` — conditions `{field: value}` or regular expressions; projection with `1/true` include, `0/false` exclude (`_id` always returned unless `_id: 0`); `findOne` returns the first document in natural (disk) order; nested fields via dot notation (`"address.city": "Rome"`, `"size.h": {$lt: 15}`). **No join operator exists** (but `$lookup` in aggregation): relations via manual references (`_id` + second query) or DBRef `{"$ref":…, "$id":…, "$db":…}`. Comparison operators: `$eq`, `$gt`, `$gte`, `$lt`, `$lte`, `$ne`, `$in`, `$nin`; logical: AND = comma-separated conditions, `$or`, `$not`, `$and`; element: `{item: null}` (null **or** missing), `{$exists: false}`, `{$type: 10}`. `find()` returns a **cursor**: `sort({field: 1/-1})` (ORDER BY), `count()`, `limit()`, `forEach(print…)`, `pretty()`. Update: `updateOne/updateMany(<filter>, <update>, <options>)` with `$set`, `$inc`, `$currentDate` (e.g. `db.people.updateMany({age: {$gt: 25}}, {$set: {status: "C"}})`). Delete: `deleteMany({status: "D"})`, `deleteMany({})` deletes all.

**Aggregation pipeline**: documents enter a multi-stage pipeline transforming a collection into an aggregated result; stages may repeat (except `$out`, `$merge`, `$geoNear`); expressions are stateless except accumulators in `$group`; max **100 MB RAM per stage**; an alternative to map-reduce (preferred since v4.4's `$accumulator`/`$function`). SQL mapping: WHERE→`$match`, GROUP BY→`$group`, HAVING→`$match`, SELECT→`$project`, ORDER BY→`$sort`, LIMIT→`$limit`, SUM→`$sum`, COUNT→`$sum: 1`. Examples:

```js
db.people.aggregate([ { $match: {status: "A"} },
                      { $group: {_id: null, count: {$sum: 1}} } ])

db.orders.aggregate([ { $group: {_id: "$status", total: {$sum: "$age"}} },
                      { $match: {total: {$gt: 1000}} } ])   // GROUP BY + HAVING
```

Notable stages: `$group` (one output document per distinct value of the identifier expression), `$unwind` (deconstructs an array — one output document per element), `$match`, `$project`/`$set`/`$addFields`/`$unset` (reshape; expressions like `{$floor: …}`), `$sort`, `$limit`, `$skip`, `$sample`, `$count`, `$sortByCount`, `$bucket`/`$bucketAuto`, `$facet` (multiple pipelines on the same input), `$lookup` (join to another collection), `$graphLookup` (recursive search — e.g. reporting hierarchy), `$geoNear`, `$out`/`$merge` (write results, must be last), `$indexStats`, `$collStats`. Worked examples: per-country average price with `$unwind` on the price array → `$group` by `$price.country` → `$match` on bookcount → `$project` with `{$floor: '$review'}` → `$limit: 20`; 95th percentile of pages via `$match` (tag "guide") → `$sort` → `$group`+`$push` → `$project` + `$arrayElemAt` at index `{$floor: {$multiply: [0.95, {$size: "$value"}]}}`; median of review_score similarly.

**Indexing**: without indexes MongoDB performs a **collection scan**; indexes store ordered values of a field/set of fields to support equality matches, range queries and sorting. A unique index on `_id` is created with the collection and cannot be dropped. `db.collection.createIndex(<keys>, <options>)` (pre-3.0: `ensureIndex`); options: name, unique, background… Types: **single field** (`{orderDate: 1}`), **compound** (`{orderDate: 1, zipcode: -1}`), **multikey** (array contents — separate entries per element), **geospatial** (**2d** planar, **2dsphere** spherical geometry on GeoJSON or coordinate pairs; operators `$geoIntersects`, `$geoWithin`, `$near`, `$nearSphere` — e.g. all places within 5000 m sorted nearest-first), **text** (root words only, no stop words; wildcard `{"$**": "text"}` indexes all string fields), **hashed** (hash of the value; random distribution but **equality matches only**, no range).

**Views**: queryable objects defined by an **aggregation pipeline** on other collections/views; contents not persisted (computed on demand); read-only; since 4.2 **on-demand materialized views** via `$merge`. **MongoDB Compass**: GUI to explore data, analyze documents/fields, visually build queries and aggregation pipelines, analyze query performance, validate data.

### Hadoop and Spark

**Hadoop** — the de facto standard Big Data platform. Timeline: 2003 Google File System; 2004 MapReduce (Jeff Dean); 2005 Hadoop funded by Yahoo; 2006 migration to Apache and Google BigTable; 2008 Terabyte Sort Benchmark (1 TB sorted in 209 s vs previous 297 s); 2009 additional components. Core components: **Hadoop Common** (utilities), **HDFS** (distributed file system, high-throughput access), **YARN** (job scheduling and cluster resource management), **MapReduce** (YARN-based parallel processing). Related projects: Ambari (cluster management), Avro (serialization), Cassandra (multi-master DB), HBase (distributed large tables), Hive (data warehouse with SQL-like querying), Mahout (ML library), Pig (data-flow language), **Spark** (fast general compute engine), Tez (DAG execution on YARN), ZooKeeper (coordination). Why Hadoop: distributed, fault-tolerant, heterogeneous huge-data storage; flexible parallel scalable processing (Java, Python, Scala, R; batch and real-time); high availability (node failures handled by design); scalability by adding low-cost nodes; low cost (commodity hardware). **Apache Spark**: up to 100× faster than MapReduce in memory (10× on disk) via an advanced DAG engine and in-memory computing; 80+ high-level operators in Java, Scala, Python, R; generality (SQL/DataFrames, MLlib, GraphX, Streaming); runs on Hadoop, Mesos, standalone or cloud, reading HDFS, Cassandra, HBase, S3.

> [!warning] NoSQL is not a silver bullet — design matters
> Bank-account example: two separate document updates (decrease one balance, increase another) cannot be guaranteed atomic by CouchDB — a failure between them loses the balance. Design recipe: store **transactions** (`{from, to, qty, when}`) and derive balances with MapReduce (`emit(from, amount*-1)`, `emit(to, amount)`, `reduce = sum`) — the same data model reshaped to be failure-safe.
