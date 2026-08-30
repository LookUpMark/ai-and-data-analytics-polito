---
title: Data Warehouse Fundamentals
aliases: [DW Introduction, Data Warehouse Architecture, Data Mart, ROLAP MOLAP HOLAP]
tags: [computer-science/databases, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> Motivation and basic concepts of data warehousing: decision support systems and Business Intelligence, the Inmon definition of a data warehouse, and why analytical data is kept separate from operational databases. Introduces the multidimensional (hypercube) representation with dimensions and measures, the relational star representation, the reference architecture (sources → ETL/staging area → data warehouse → data marts → analysis tools), the distinction between data warehouse and data marts (dependent vs independent), ROLAP/MOLAP/HOLAP servers, and the role of metadata.

## From Operational Data to Decision Support

Huge operational databases exist in most companies and contain a wealth of useful information. **Decision Support Systems (DSS)** provide the means for in-depth analysis of a company's business → faster and better decisions. Strategic decision support includes:

- demand evolution analysis and forecast;
- identification of critical business areas;
- budgeting and management transparency (reporting, fraud and money-laundering detection);
- identification and implementation of winning strategies → cost reduction, profit increase.

**Business Intelligence (BI)** supports strategic decisions by transforming company data into actionable information, at different detail levels, for analysis applications. Users have heterogeneous needs; BI requires an appropriate hardware/software infrastructure.

Typical application domains: manufacturing (order management, client support), distribution (user profile, stock management), financial services (buyer behavior with credit cards), insurance (claim analysis, fraud detection), telecommunications (call analysis, churning, fraud detection), public service (usage analysis), health (service analysis and evaluation).

## What Is a Data Warehouse?

> [!definition] Data Warehouse (Inmon, 1992)
> A database devoted to decision support, kept separate from company operational databases. Its data is:
> - **devoted to a specific subject** (e.g. sales, shipments, complaints),
> - **integrated and consistent**,
> - **time dependent** (historical),
> - **non-volatile**.

### Why keep analytical data separate?

- **Performance**: complex analytical queries would reduce the performance of operational transaction management; the two workloads need different physical access methods.
- **Data management**: operational systems miss information (e.g. history), data consolidation and quality (inconsistencies between sources).

## Multidimensional Representation

Data is represented as an **(hyper)cube** with three or more dimensions; the **measures** on which analysis is performed sit in the cells at dimension intersections.

Example — data warehouse for tracking sales in a supermarket chain:

- **dimensions**: product, shop, time;
- **measures**: sold quantity, sold amount, …

### Data warehouse size

Size example from the slides (one supermarket chain):

| Parameter | Value |
|---|---|
| Time dimension | 2 years × 365 days = 730 |
| Shop dimension | 300 shops |
| Product dimension | 30,000 products, of which ~3,000 sold every day in every shop |
| Fact table rows | 730 × 300 × 3000 = **657 million** |
| Fact table size | ≈ **21 GB** |

The kinds of analysis available on this data: OLAP analysis (complex aggregate computations, e.g. moving average, top ten), data mining (machine learning techniques), presentation tools (separate activity: query results can be rendered by different tools), and **data exploration** by progressive "incremental" refinements (e.g. drill-down).

### Relational representation: star model

- **Numerical measures** are stored in the **fact table** (attribute domain is numeric).
- **Dimensions** describe the context of each measure; each dimension is characterized by many descriptive attributes.

`SALE(shop, date, product, sold_quantity, sale_amount, number_of_customers, unit_price)` with dimensions Shop, Date, Product.

## Data Warehouse Architectures

Guiding principle: **separation between transactional computing and data analysis** — one-level architectures are to be avoided. Architectures have two or more levels: data entering the DW is separated from analyzed data; more levels are more scalable.

Reference (conceptual) architecture:

```
(External) data sources            ← Source level
        │
   ETL tools → Staging area         ← ETL level
        │
   Data warehouse (+ metadata, DW management)   ← DW level
        │
   OLAP servers / data marts / analysis tools    ← Analysis level
```

### Data warehouse vs data mart

| | Data warehouse | Data mart |
|---|---|---|
| Scope | Company-wide: all information on the business | Departmental subset focused on a specific subject |
| Design | Extensive functional modelling; long design/implementation time | Faster implementation |
| Caution | — | Requires careful design to avoid later data mart integration problems |

Two data mart architectures: **dependent** (fed by the company DW) and **independent** (fed directly by the sources).

### Servers for data warehouses

| Server type | Description |
|---|---|
| **ROLAP** (Relational OLAP) | Extended relational DBMS: compact representation for sparse data, SQL extensions for aggregate computation, specialized access methods for OLAP access |
| **MOLAP** (Multidimensional OLAP) | Data in proprietary multidimensional matrix format; sparse data require compression; special OLAP primitives |
| **HOLAP** (Hybrid OLAP) | Combines the two |

## ETL: Extraction, Transformation, Loading

The **ETL phase** prepares data to be loaded into the DW; it is performed at first load and during periodical refresh. Four sub-phases:

1. **Data extraction** from (OLTP and external) sources;
2. **Data cleaning** — improving data quality (correctness, consistency);
3. **Data transformation** — conversion from operational format to DW format;
4. **Data loading** — update propagation to the data warehouse.

### Staging area

A **buffer area** separating ET management from DW loading:

- better reliability, at the price of greater overhead;
- eases complex transformation and cleaning operations;
- provides an integrated model of business data still close to the OLTP representation — sometimes denoted **Operational Data Store (ODS)**;
- introduces further redundancy (more disk space).

### Two-level architecture features

- Decoupling between source and DW data;
- management of external (non-OLTP) sources (e.g. text files);
- data modelling suited for OLAP analysis; physical design tailored to the OLAP load;
- easy management of different temporal granularity of operational vs analytical data;
- partitioning between transactional and analytical load;
- "on the fly" data transformation and cleaning.

## Metadata

> [!definition] Metadata
> Information which describes data: *metadata = data about data*.

Types of metadata in the DW environment:

- **For data transformation and loading**: describe data sources and the needed transformation operations; a common notation is useful (CWMI — Common Warehouse Metadata Initiative, an OMG standard to exchange data between DW tools and metadata repositories in heterogeneous/distributed environments).
- **For data management**: describe the structure of the data in the DW (including materialized views).
- **For query management**: data on query structure and execution monitoring — SQL code, execution plan, memory and CPU usage.
