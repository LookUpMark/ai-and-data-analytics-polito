---
title: Data Lakes
aliases: [Data Lake vs Data Warehouse, Data Swamp, Schema-on-Read]
tags: [computer-science/databases, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> The data lake as an extension of the staging area: a repository storing all data in raw form (structured, semi-structured, unstructured, binary) on massive cheap storage, exploited when not all questions the data can answer are known a-priori. Contrasts schema-on-write (data warehouse) with schema-on-read (data lake), lists pros (store everything now, self-service analytics, minimal IT involvement) and cons (no governance → data swamp, uncertain quality, rogue queries), and closes with the data swamp risk and the mitigation "collect less data, at least in the beginning".

## What Is a Data Lake?

> [!definition] Data lake
> A **data repository** — a sort of extension of the staging area — holding:
> - **original data in raw format**;
> - **transformed data** used for various types of reporting.
> Querying is "more similar to a Google search" (+ data wrangling).

**Data formats** managed:

- **Structured** data (e.g. relational data);
- **Semi-structured** data (e.g. CSV, JSON, XML);
- **Unstructured** data (e.g. text documents, emails);
- **Binary** data (e.g. images, audio files).

## Why Data Lakes?

- **Often not all questions data can answer are known a-priori** → it is hard to store data in some "optimal" form. The DW approach forces deciding *in advance* what data to include; a data lake stores data that *might* be used "someday" and defers the decision.
- An attempt to **break down information silos** (information not adequately shared among data systems).
- Based on exploiting **massive, cheap data storage**.

## Characteristics

- **Data lakes store all data** — DW design requires deciding what to include (and what not to include) in the warehouse.
- **Manage all data types** (see formats above).
- **Provide service to all users** — users process a variety of data types and answer new questions.
- **Adapt easily to changes** — all data stored in raw form, always accessible; users are empowered to explore data in novel ways.
- **Provide faster insight** — … but early access to the data comes at a price (quality, governance).

## Data Warehouse vs Data Lake

| | Data warehouse | Data lake |
|---|---|---|
| Data | Relational data from transactional systems, operational databases, line-of-business applications | Both non-relational and relational, from IoT devices, websites, mobile apps, social media, corporate applications |
| Schema | Designed **prior** to DW implementation (**schema-on-write**) | Written **at the time of analysis** (**schema-on-read**) |
| Storage cost | High | Low-cost |
| Data quality | Highly curated; central version of the truth | Any data, may or may not be curated (raw) |
| Users | Business analysts | Data scientists, data developers; business analysts *if using curated data* |
| Analytics | BI and visualization, batch reporting | Full-text search, machine learning, predictive analytics, data discovery and profiling |

## Pros

- Ability to harness **more data, from more sources, in less time**.
- Data structures and business requirements are defined **only when needed**.
- **Empowering users** to collaborate and analyze data in different ways (self-service analytics); integration happens *outside* the storage environment.
- **Minimal involvement of IT** — wrangling with data is a self-service function; sandboxes for self-service analytics (need well-defined problems).

## Cons

- Raw data is stored **with no oversight of the contents**.
- **Storing data does not, by itself, provide business value.**
- Need **data governance**, semantic consistency, a mechanism to **catalog** data.
- Consistency and data quality are **uncertain**.
- Data brought into a lake is **co-located, not integrated**.
- Business users don't have time/willingness to learn — how can they wrangle with raw data?
- **Rogue queries can bring down big clusters.**

The central question posed by the slides: *whether collecting and storing data without a pre-defined business purpose is a good idea.*

## From Data Lakes … to Data Swamps

> [!warning] Data swamp
> Massive repositories of data that are **completely inaccessible to end users**: data collected without any clear way to get value from it — with the risk of the project being abandoned (budget cut).

To avoid drowning in your data lake: **collect less data, at least in the beginning.**
