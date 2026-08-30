---
title: Introduction to Data Science
aliases: [DSDT Course Introduction, Big Data and Data Science, DSDT Lecture 01]
tags: [computer-science/databases, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> Opening lectures of Data Science and Database Technology (DSDT): the two workloads a database system must serve (OLTP vs OLAP), the structure of the course in two parts (data warehouse/data mining and DBMS technology), and the Big Data challenge — the five Vs (Volume, Velocity, Variety, Veracity, Value). Introduces data science as "extracting meaning from very large quantities of data", the four-phase data science process (Generation, Acquisition, Storage, Analysis), the main data mining tasks (association rules, classification, clustering), and the open issues of interpretability, algorithmic bias and privacy.

## OLTP vs OLAP

The course opens by distinguishing the two classic database workloads.

| Aspect | OLTP (On-Line Transaction Processing) | OLAP (On-Line Analytical Processing) |
|---|---|---|
| Usage | Traditional DBMS, operational applications | Decision support applications |
| Data | Snapshot of current values, detailed, relational | "Historical", consolidated, integrated data |
| Operations | Structured, repetitive; read/write of few records; short transactions | Ad hoc, complex queries; read access to millions of records |
| Correctness | Isolation, reliability, integrity critical (ACID) | Consistency before/after periodical loads |
| Size | 100 MB – GB | 100 GB – TB |

## Course Structure

- **Part 1 (weeks 1–7)**: data warehouse design, OLAP analysis, data science and data mining.
- **Part 2 (weeks 8–14)**: DBMS server technology, distributed databases, NoSQL databases (MongoDB, Elasticsearch).
- Activities: lessons, classroom exercises, laboratories (commercial and open-source products, starting week 4).

Course books: Golfarelli & Rizzi, *Data Warehouse Design* (McGraw Hill 2021); Tan, Steinbach, Kumar, *Introduction to Data Mining* (Pearson 2006); Atzeni et al., *Basi di dati*; Sullivan, *NoSQL for Mere Mortals*; Chodorow & Bradshaw, *MongoDB: The Definitive Guide*; Gormley & Tong, *Elasticsearch: The Definitive Guide*. Exam: mandatory written test (design exercises + theory questions, no notes/devices allowed), optional individual project and homework.

## The Big Data Challenge

> [!definition] Big Data
> "Data whose scale, diversity and complexity require new architectures, techniques, algorithms and analytics to manage it and extract value and hidden knowledge from it."

Data sources mentioned in the slides: user-generated content (social media), health and scientific computing, log files (web server logs, syslogs), Internet of Things (sensor networks, RFID, smart meters), earth observations, social media data streams.

### The Vs of Big Data

| V | Meaning |
|---|---|
| **Volume** | Data volume grows exponentially: 44× increase from 2009 to 2020; ~35 ZB of digital data in 2020 |
| **Velocity** | Fast generation rate; streaming data must be analyzed in (near) real time to ensure timeliness |
| **Variety** | Heterogeneous collection: numerical data, images, audio, video, text, time series; one application may generate many formats |
| **Veracity** | Data quality: we must be able to process data containing low-quality values |
| **Value** | The most important V: translate data into business advantage — the process only makes sense if it ends in value |

### Challenges

- **Technology & infrastructure**: new architectures, programming paradigms, techniques; transfer the processing power *to the data* (Apache Hadoop/Spark ecosystem).
- **Data management & analysis**: new emphasis on "data".

## Data Science

> [!definition] Data Science
> "Extracting meaning from very large quantities of data." (D.J. Patil coined the term *data scientist*.) It sits at the intersection of machine learning, data mining and pattern recognition.

### The Data Science Process (four phases)

1. **Generation** — how data is produced:
   - *Passive recording*: typically structured (bank transactions, shopping records, archives);
   - *Active generation*: semi-structured/unstructured user-generated content (social networks);
   - *Automatic production*: location-aware, context-dependent, highly mobile data (IoT sensors).
2. **Acquisition**:
   - *Collection*: pull-based (web crawler) or push-based (video surveillance, click stream);
   - *Transmission*: transfer to data center over high-capacity links;
   - *Preprocessing*: integration, cleaning, redundancy elimination.
3. **Storage**:
   - Storage technology (HDD, SSD) and networking (DAS, NAS, SAN);
   - Data management: file systems (HDFS), key-value stores (Memcached), column-oriented databases (Cassandra), document databases (MongoDB);
   - Programming models: MapReduce, stream processing, graph processing.
4. **Analysis**:
   - Objectives: descriptive, predictive, prescriptive analytics;
   - Methods: statistics, machine learning and data mining (association analysis, classification and regression, clustering), text mining, network/graph mining.

### Knowledge Discovery Process (KDD)

```
data → [selection] → selected data → [preprocessing] → preprocessed data
     → [transformation] → transformed data → [data mining] → patterns
     → [interpretation/evaluation] → knowledge
```

- **Selection** generates high-quality data; **preprocessing** (cleaning, integration) adapts data to algorithm requirements — real-world data is "dirty", and without good quality data there are no good quality patterns;
- Practitioners report that **80–90% of the work** is not machine learning: understanding the domain, cataloging sources, wrangling, extracting, integrating and cleaning the data.

## Data Mining Tasks

> [!definition] Data Mining
> Non-trivial extraction of **implicit**, **previously unknown**, **potentially useful** information from available data. Extraction is **automatic** (performed by algorithms) and the result is represented as **abstract models (patterns)**.

Three core tasks:

- **Association rules** — extract frequent correlations from transactional data. Classic example: *diapers → beer* (2% of transactions contain both; 30% of transactions containing diapers also contain beer).
- **Classification** — predict a class label and/or build an interpretable model of a phenomenon from labeled training data (churn detection, fraud detection, pathology classification).
- **Clustering** — detect groups of similar data objects and identify exceptions/outliers.

Other techniques: **sequence mining** (ordering matters, e.g. motif detection in proteins), **time series and geospatial** mining (sensor networks), **regression** (prediction of continuous values, e.g. stock quotes), **outlier detection** (intrusion detection in network traffic).

## Applications and the Data Science Recipe

Application examples: consumer behavior in e-commerce, search engines (query keywords), social network profiles and posts, georeferenced data, user/service profiling and recommendations, market basket analysis (cross-selling), context-aware analysis (location, time, user interest), text mining (brand reputation, sentiment analysis, topic trends), biological data (microarray gene expression levels, patient clinical records, PubMed literature, Gene Ontology).

A data science project needs a **recipe** with different ingredients:

| Role | Expertise |
|---|---|
| Data expert | Data processing, data structures |
| Data analyst | Data mining, statistics, machine learning |
| Visualization expert | Visual art design, storytelling |
| Domain expert | Understanding of the application domain |
| Business expert | Data-driven decisions, new business models |

Before starting, answer: What question are you answering? What is the scope? What data? Which techniques? How will you evaluate results? What maintenance is required?

## Open Issues

- **Interpretability and transparency**: "the ability to explain or to present in understandable terms to a human". There is an **accuracy–interpretability trade-off**. Approaches: *model explanation* (global), *prediction explanation* (local), *interpretable feature selection*.
  - Example (asthma/pneumonia): a learned rule said asthma → lower chance of dying from pneumonia. This contradicts medical knowledge; the explanation is that asthmatics get earlier diagnosis and faster high-quality treatment. A neural network would never have surfaced this issue.
- **Algorithmic and data bias**: a recidivism-risk score used in the US criminal justice system (race not asked, but poverty/joblessness correlated) flagged black defendants as future criminals more often; the training data was biased by a larger black defendant population.
- **Privacy**: Strava's global heatmap (13 trillion GPS points) inadvertently revealed sensitive locations.
- Technical open issues: scalability to huge volumes, data dimensionality, complex/heterogeneous data structures, data quality, streaming data.
