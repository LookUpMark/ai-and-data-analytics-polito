---
title: Data Warehouse Design
aliases: [DW Conceptual Design, Dimensional Fact Model, Star and Snowflake Schema, OLAP Operators]
tags: [computer-science/databases, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> The design pipeline of a data mart: risk factors, top-down vs bottom-up approaches, Kimball's Business Dimensional Lifecycle, and requirement analysis. Conceptual design with the Dimensional Fact Model (facts, dimensions, measures, hierarchies), advanced modeling constructs (optional attributes, shared hierarchies, multiple edges, configuration attributes, factless facts, slowly changing dimensions) and the classification of measures/aggregations. Logical design mapping DFM to the relational model: star schema, snowflake schema, bridge tables vs push-down for many-to-many, degenerate and junk dimensions. Closes with the OLAP operators (roll-up, drill-down, slice, dice, pivot).

## Risk Factors in DW Projects

- **High user expectations** — the DW is seen as the solution to all company problems.
- **Data and OLTP process quality** — incomplete or unreliable data, non-integrated or non-optimized business processes.
- **"Political" management of the project** — cooperation with information owners, end-user acceptance, deployment, appropriate training.

## Design Approaches

| | Top-down | Bottom-up |
|---|---|---|
| Idea | DW = global, complete representation of business data | Incremental growth by adding data marts on specific business areas |
| Cost/time | Significant cost and time | Limited cost and delivery time |
| Design | Complex analysis and design tasks | Focused on single areas; easy intermediate checks |

**Kimball's Business Dimensional Lifecycle** organizes the whole process around three tracks — DATA (dimensional modeling, physical design, feeding design), TECHNOLOGY (product selection/installation, architecture), APPLICATIONS (user application analysis/development) — under a shared umbrella of Planning → Requirement definition → … → Deployment → Maintenance.

### Data mart design framework

```
operational source schemas ──RECONCILIATION──> reconciled schema
user requirements ──CONCEPTUAL DESIGN──> fact schema
   (conceptual schema, workload, data volume) ──LOGICAL DESIGN──> logical schema
   (logical schema, workload, data volume, DBMS) ──PHYSICAL DESIGN──> physical schema
reconciled schema ──FEEDING DESIGN──> feeding schema
```

## Requirement Analysis

Collects **data analysis requirements** to be supported by the data mart and **implementation constraints** of existing information systems. Sources: business users and operational system administrators. The first data mart selected should be crucial for the company and fed by (few) reliable sources.

- **Application requirements**: description of relevant events (**facts** — e.g. in the CRM domain: complaints, services) with descriptive dimensions (setting the granularity), history span, relevant measures (gathered in a glossary); workload description (periodical business reports, queries in natural language).
- **Structural requirements**: feeding periodicity, available space (data + derived data such as indices and materialized views), system architecture (number of levels, dependent/independent data marts), deployment planning (startup, training).

## Conceptual Design: Dimensional Fact Model (DFM)

No modeling formalism is currently adopted; the ER model is not adequate. The course uses the **Dimensional Fact Model** (Golfarelli, Rizzi), a graphical model that for a given fact defines dimensions, hierarchies and measures; it serves as design documentation both for requirement review with users and after deployment.

| Concept | Meaning |
|---|---|
| **Fact** | Models a set of relevant events (sales, shippings, complaints); evolves with time |
| **Dimension** | Analysis coordinates of a fact (e.g. each sale described by sale date, shop, sold product); many, typically categorical, attributes |
| **Measure** | Numerical property of a fact (e.g. sold quantity); aggregates are frequently computed on measures |
| **Hierarchy** | Attributes of a dimension structured at different abstraction levels; represents a generalization relationship, i.e. a functional dependency (1:n) |

Canonical example: `SALE(date, shop, product; sold quantity, sale amount, number of customers, unit price)` where *product* has hierarchy product → brand → brand city → category type → department, *shop* has shop → shop city → region → country (with sales manager, sale district), and *date* has day → week → month → quarter → year (with holiday).

**Comparison with ER**: the same content as an ER diagram (PRODUCT, BRAND, CITY, MONTH, …) but the DFM is a *quick visualization*, more friendly; ER cardinalities (1,n)/(1,1) are implicit in the hierarchies.

### Advanced DFM constructs

- **Optional dimensions / descriptive attributes**: dimensions (or attributes) that do not apply to every fact (e.g. *promotion* with start/end date, discount, advertisement, cost) — modeled with an **optional edge**.
- **Convergence**: two hierarchies sharing the same attribute (e.g. caller district / called district converging on district).
- **Shared hierarchy**: manage complex contexts with more dimensions and relations between them, using separated dimensions for each role (PHONE CALL with caller/called as two roles of the subscriber dimension).
- **Multiple edge**: many-to-many between fact and dimension (book ↔ authors) — see logical design below.
- **Configuration attribute**: a **multi-valued categorical attribute** with few distinct values (≤10), represented by enumerating the possible values as Y/N attributes (e.g. in VIDEOGAME_SALES, "types of store" with collectable toys, videogame company, mangas, accessories, plus `holiday (Y/N)`).
- **Factless fact schema**: events with no measures — records only the occurrence of an event; used for counting occurred events (e.g. ATTENDANCE of students to courses) and for representing events *not* occurred (coverage set).

### Representing time

- Fact evolution over time is represented by event occurrences (time dimension + facts).
- **Dimensions may change too** — *slowly changing dimension* [Kimball], e.g. client demographic data:

| Type | Strategy | Example (Mario Rossi marries) |
|---|---|---|
| **1** | Overwrite with current value; past overridden; used when explicit representation of change is not needed | All his purchases map to "married" Mario Rossi |
| **2** | After each state change a **new dimension instance** is created; events are partitioned after the changes | Purchases split between "unmarried" Mario and "married" Mario (new instance) |
| **3** | Explicit management of changes: two timestamps (**validity start/end**) plus an attribute identifying the sequence of modifications (e.g. a "master" pointer to the root instance) | Validity-end of first instance = marriage date; validity-start of new instance = same day; track all changes of the instance |

## Aggregation and Measure Classification

**Aggregation** computes measures at a coarser granularity (usually by climbing a hierarchy) with operators SUM, MIN, MAX, AVG, COUNT. Some measures cannot be aggregated.

Classification of measures by additivity:

- **Stream measures**: evaluated cumulatively at the end of a time period; aggregable with all standard operators (sold quantity, sale amount).
- **Level measures**: snapshot at a given time; **not additive along time** (inventory level, account balance).
- **Unit measures**: at a given time, in relative terms; **not additive along any dimension** (unit price).

Classification of aggregate operators:

- **Distributive**: higher-level aggregations can always be computed from more detailed data (sum, min, max).
- **Algebraic**: computable from detailed data only with supplementary support measures (avg requires count).
- **Holistic**: cannot be computed from more detailed data (mode, median).

Example (slides): average unit price per quarter computed from product-level data differs from the average of quarterly averages — AVG is algebraic, not distributive; the count must travel with it.

## Workload and Data Volume

- **Workload**: defined by standard reports and approximate estimates discussed with users; the actual workload is hard to evaluate at design time (if the DW succeeds, users and queries grow). Tuning happens after deployment by monitoring the real workload.
- **Data volume**: estimate space for data and derived data (indices, materialized views) from event cardinality of each fact, domain cardinality of hierarchy attributes, attribute length, temporal span of storage, and **sparsity**.
- **Sparsity**: occurred events are not all combinations of dimension elements (e.g. only ~10% of products are actually sold in each shop/day); it decreases with increasing aggregation level and may significantly affect the accuracy of cardinality estimates.

## Logical Design (ROLAP)

Inputs: conceptual fact schema, workload, data volume, system constraints → output: relational logical schema. Contrary to traditional logical design, it is *based on* **data redundancy** and **table denormalization**.

### Star schema

- **One table per dimension**: surrogate (generated) primary key; contains **all** dimension attributes; hierarchies are not explicitly represented (all attributes at the same level) — a totally denormalized representation causing redundancy.
- **One fact table per fact schema**: primary key composed of the foreign keys of all dimensions; measures are attributes.

```
Week(Week_ID*, Week, Month)          -- dimension (time)
Shop(Shop_ID*, Shop, City, Country, Salesman)   -- dimension
Product(Product_ID*, Product, Type, Category, Supplier) -- dimension
Sales(Shop_ID*, Week_ID*, Product_ID*, Quantity, Amount) -- fact
```

### Snowflake schema

Some functional dependencies are separated by partitioning dimension data into several tables: a new table separates two branches of a dimensional hierarchy (the hierarchy is "cut" on a given attribute); a new foreign key links dimension and new table.

- Decreases the space for the dimension — but the decrease is frequently **not significant** (most space is the fact table, orders of magnitude larger).
- Increases the cost of reading an entire dimension (extra joins).

> [!tip] Star or snowflake?
> The snowflake schema is usually **not recommended** and rarely used in data mart design: storage savings are rarely beneficial and join costs may be significant.

### Many-to-many (multiple edges): bridge table vs push-down

For book sales with multiple authors:

- **Bridge table**: new table modelling the many-to-many relationship with a **weight** attribute apportioning each tuple's contribution (`Books_Book_Authors(Book_ID, Author_ID, Weight)`).
- **Push-down**: the multiple edge is integrated in the fact table by adding a dimension (author) to the fact — the fact table size grows (significant redundancy).

```sql
-- Weighted query (author income) with bridge table
SELECT Author_ID, SUM(Income * Weight) ... GROUP BY Author_ID;
-- Impact query (book copies sold per author) with bridge table
SELECT Author_ID, SUM(Quantity) ... GROUP BY Author_ID;
```

Comparison: with push-down, weight is "wired" in the fact table (computed at feeding time; hard to modify; impact queries are hard; more redundancy) but query execution is cheaper (fewer joins).

### Degenerate and junk dimensions

- **Degenerate dimensions**: dimensions with a single attribute (Order, SRL/ShippingMode, ReturnCode, LineOrderStatus) pushed down in the fact table as primary-key attributes providing additional information. Implementations: integration into the fact table (for very small attributes) or a **junk dimension** — a single dimension containing several degenerate dimensions with **no functional dependencies** among attributes (all value combinations allowed); feasible only for small domain cardinalities.

## OLAP Operators

Available query operations on the multidimensional cube (usable together and in sequence — successive refinements build the **OLAP session**):

| Operator | Effect |
|---|---|
| **Roll-up** | Decrease detail: climb up a hierarchy (`group by store, month` → `group by city, month`) or **drop a whole dimension** (`group by product, city` → `group by product`) |
| **Drill-down** | Increase detail: walk down a hierarchy (`group by city, month` → `group by store, month`) or **add a dimension**; usually operates on a subset of data produced by the initial query |
| **Slice** | Selection with an **equality predicate** on one dimension (Year = 2005) — selects a "slice" of the cube |
| **Dice** | Selection with a **predicate expression** on several dimensions (Category = 'Food' AND City = 'Torino') |
| **Pivot** | Reorganization of the multidimensional structure *without* changing the detail level: two dimensions are the main grid axes; changing their position (or using attribute values as new axes) increases readability |
| **Sorting** | Order the result |

Together with simple SQL aggregation, OLAP analysis supports complex aggregate functions (moving average, top ten) and comparison operations between business trends (hard in plain SQL) — these SQL extensions are covered in [[04-etl-and-oracle-sql]].
