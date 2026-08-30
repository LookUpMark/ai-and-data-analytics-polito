---
title: Big Data Fundamentals
aliases: [BDPA fundamentals, big data basics, 3Vs, Lambda Architecture, BigData@Polito lab platform]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> Foundations of the Big Data Processing and Analytics (BDPA) course: what big data is (sources, the 3+2 Vs), why single-node analysis fails (disk bandwidth, failure rates, network bottleneck), scale-up vs scale-out, the challenges of distributed computing, and the Lambda Architecture (batch layer, speed layer, serving layer). Plus the course logistics (exam format, prerequisites, textbooks) and the practical lab workflow: developing locally, uploading to the BigData@Polito gateway (`jupyter.polito.it`), managing data on HDFS through HUE, and the recurring lab datasets. Sources: slides 00–02 and the lab handouts.

## Course Organization (slides 00)

- Teachers: **Paolo Garza** (`paolo.garza@polito.it`) and **Luca Colomba** (questions at class time or by e-mail).
- **Lectures**: 45 hours. **Practices**: 15 hours, Friday (two teams A–K / L–Z); no lab activity during the first week.
- Program: introduction to big data → Hadoop (architecture, MapReduce) → Spark (architecture, RDDs, Spark SQL) → MLlib (scalable machine learning) → Spark Streaming. Laboratory activities on Hadoop and Spark.
- Prerequisites: basic **Java** (MapReduce is Java-based) and **Python** (Spark) programming; relational database/SQL basics recommended.
- Reference books: *Spark: The Definitive Guide* (Zaharia, Chambers), *Learning Spark* (Zaharia, Karau, Konwinski, Wendell), *Hadoop: The Definitive Guide* (White), *MapReduce Design Patterns* (Miner, Shook).
- Course web page: `https://dbdmg.polito.it/dbdmg_web/2024/bigdata-processing-and-analytics-2024-25`.

### Written exam (slides 00)

| Part | Points | Content |
|---|---|---|
| 2 programming exercises | max 27 | Java programs with Hadoop MapReduce and/or Spark RDDs |
| 2 questions/theoretical exercises | max 4 | Hadoop & Spark architecture, HDFS, MapReduce paradigm, Spark RDDs/transformations/actions, Spark SQL, Spark Streaming, Spark MLlib |

On-site, on the exam platform with Lockdown browser, **your own PC**, **1.5 hours**, **open book**: paper material allowed, no electronic devices except the exam PC. Exam examples are published on the course web page.

## What Is Big Data (slides 01)

- Motivating example: **Google detected a flu outbreak two weeks ahead of the CDC** (U.S. Centers for Disease Control) by analyzing search queries — *nowcasting*: going big to get quick predictions.
- Data derives from various sources, all producing large amounts of data to analyze:
  - **User Generated Content** (web & mobile): Facebook, Instagram, Yelp, TripAdvisor, Twitter, YouTube;
  - **Health and scientific computing**;
  - **Log files**: web server logs, machine/system logs;
  - **Internet of Things**: sensor networks, RFIDs, smart meters.

> [!definition] The (3+2) Vs of big data
> - **Volume**: scale of data.
> - **Variety**: different forms of data.
> - **Velocity**: speed at which data is generated and streamed (analysis of streaming data).
> - Plus **Veracity**: uncertainty of data; **Value**: exploiting the information provided by data.

The amount of data increases every day. Order of magnitude (∼2012): **Google processes 100+ PB/day, Facebook 10+ PB/day** — analyzing them requires systems that scale with the data volume.

### Why a single node is not adequate

Back-of-the-envelope example from the slides (analyze 10 billion web pages, average 20 KB each ⇒ **200 TB**):

| Storage | Read bandwidth | Time to read the collection (no analysis) |
|---|---|---|
| HDD | 150 MB/s | more than **15 days** |
| SSD | 550 MB/s | more than **4 days** |

A single-node architecture is not adequate: data must be spread over multiple disks and processed in parallel.

### Failures are everyday life

A single server stays up ~3 years (~1000 days), but with more nodes failures become frequent:

- 10 servers → 1 failure every ~100 days;
- 100 servers → 1 failure every 10 days;
- 1000 servers → **1 failure per day**.

Sources: hardware/software faults, electrical/cooling problems, resource overload. The system must be designed assuming failures.

### The network is the bottleneck

Data-center network bandwidth ≈ 10 Gbps: moving 10 TB between two servers takes more than 2 hours. Consequence (repeated all over the course):

> **Move code (programs) and computation to the data** instead of moving data to the code. Programs are small (KBs/MBs); exploit **data locality**.

Split the dataset across many distributed disks and process the different portions in parallel.

### From small data to big data processing

| Data size | Approach |
|---|---|
| Small data (fits in main memory) | load everything in RAM; shallow ML / statistics |
| Large data (does not fit in RAM) | "classical" data mining: load one chunk at a time, process it, combine the statistics |
| Huge data (does not fit on one machine) | **cluster of servers**: computation and data distributed across nodes |

A **typical big data problem**: iterate over a big amount of records, extract something of interest from each record, aggregate intermediate results, generate the final output.

## Scale Up vs Scale Out (slides 01)

Current systems must scale both in data volume and number of users. Two approaches:

- **Vertical scalability (scale up)**: add resources (CPU, RAM) to a single high-performing node. Cost of supercomputers is *not* linear in resources → expensive.
- **Horizontal scalability (scale out)**: add more commodity nodes; cost scales approximately linearly, but data-center efficiency is a hard problem.

Standard big data architecture (∼2012): **clusters of commodity Linux nodes** (~32 GB RAM each, 1 Gbps Ethernet inside a rack, 2–10 Gbps backbone between racks, 16–64 nodes per rack) — cheap servers in great numbers; if one breaks, replace it.

**Scale-out is preferred for big data**: at the same cost you get a system that processes data more efficiently and is more fault-tolerant. But distributed computing is hard:

- **Problem decomposition and parallelization**; **task synchronization**;
- **Task scheduling**: assign tasks to nodes to speed up execution, exploit (almost) all resources, reduce the impact of failures;
- **Distributed data storage**: keep data persistent and available when nodes fail → redundancy, which adds complexity;
- **Network bottleneck**: move computation and code to data;
- Previous solutions (HPC ∼1960, grid computing ∼1990, distributed databases ∼1990) are not adequate for today's data volumes; the challenges are parallelization, distributed storage (TB–PB), node-failure management, network bottleneck, and data heterogeneity.

## Big Data Architectures and the Lambda Architecture (slides 02)

> [!definition] Big data architecture
> "A big data architecture is designed to handle the ingestion, processing, and analysis of data that is too large or complex for traditional database systems." Typical workloads: batch processing of data at rest, real-time processing of data in motion, interactive exploration, predictive analytics / machine learning.

The most frequently used big data architecture is the **Lambda Architecture**, proposed by **Nathan Marz in 2011**. Motivation (Marz): Hadoop parallelizes large batch computations but has high latency; NoSQL stores scale with limited data models and, being mutable, are not human-fault tolerant. Combining tools intelligently gives scalable systems for arbitrary data problems with human-fault tolerance. Per Databricks/Wikipedia: lambda architecture processes massive quantities of data with a **hybrid batch + stream approach**, balancing latency, throughput and fault tolerance; it relies on an **append-only, immutable** data source as system of record.

**Core idea**: `query = function(all data)`, with two data paths:

- **Cold path (batch layer)**: stores *all* incoming data in raw form and performs batch processing; results are stored as **batch views**.
- **Hot path (speed layer)**: analyzes data in real time, designed for **low latency at the expense of accuracy**.

Execution flow (as enumerated in the slides):

1. All data entering the system is dispatched to **both** the batch layer and the speed layer.
2. The batch layer has two functions: (i) managing the **master dataset** (immutable, append-only set of raw data) and (ii) pre-computing the batch views.
3. The **serving layer** indexes the batch views so they can be queried in a low-latency, ad-hoc way.
4. The speed layer compensates for the high latency of updates to the serving layer and deals with **recent data only**.
5. Any incoming query is answered by **merging results from batch views and real-time views**.

Desired properties: fault-tolerant against both hardware failures and human errors; support low-latency queries as well as updates; linear scale-out; extensible. Query properties to reason about: **latency** (time to run a query), **timeliness** (freshness/consistency of results), **accuracy** (trade-off with performance/scalability, possible approximations).

## Why Distributed Processing in This Course (labs)

The course is built around one practical fact, repeated in every lab: the input files are too large to process on a single lab PC, so applications are **submitted to the BigData@Polito cluster** and executed in parallel on its nodes (Lab 1, `labs/lab-1/lab1-bigdata-vscode.pdf`).

Consequences that show up throughout the labs:

- You **compile locally** (VSCode) but **run remotely** (cluster). The local project only contains the libraries needed to *compile*; Lab 1 states explicitly that you *cannot* use that project to run MapReduce locally.
- Statistics such as *the number of mappers* are not chosen in code: they depend on how the input is split on the distributed file system, and you read them from the job output on the terminal (Lab 1, task 4).
- Even simple questions ("which words start with `ho`?") are solved by **Hadoop/Spark jobs reading from HDFS**, not by opening a file in an editor. Lab 2: *"inspecting these results manually is not feasible"*.

The recurring cost model, stated in Lab 2 and worth keeping in mind for every exercise:

> - How many pairs and bytes are read from HDFS?
> - How many pairs and bytes are emitted by the mappers and hence how many data are sent on the network?

## The BigData@Polito Platform

Three cooperating pieces, all introduced in Lab 1 (and in slide deck 04b):

| Component | URL / command | Role |
|---|---|---|
| Gateway (JupyterHub) | `https://jupyter.polito.it` | Login point; hosts notebooks, terminals, and a **local** file system for your scripts/jars |
| HUE | `https://hue.polito.it` | Web browser for HDFS: upload/download/move/delete files, browse job history |
| YARN + HDFS cluster | via `hadoop` / `yarn` / `spark-submit` commands | Executes MapReduce and Spark applications |

The cluster topology (slide 04b): a set of servers running Hadoop plus one **access gateway server** used to submit jobs, run `hdfs` commands, and analyze log files.

### Two different "homes" — do not confuse them

Lab 1 warns (footnote on page 3): if the difference between the two homes is not clear, *stop and clarify before continuing*.

1. **Gateway local file system** — where you upload the `.jar` or `.py` from your PC (drag & drop in JupyterHub). Input data is *not* here.
2. **HDFS home** — your folder *inside the Hadoop cluster*, browsed via HUE (`Browsers/Files`). This is where input/output folders of jobs live.

A job reads input from HDFS and writes output to HDFS; the jar/script lives on the gateway local file system.

### Cluster etiquette

At the end of every JupyterHub session, shut down the container so other students can connect: `File -> Hub Control Panel -> Stop My Server` (all labs, final section).

## Life Cycle of a Lab Application (Lab 1 Workflow)

1. **Import the project in VSCode** — check the "Extension Pack for Java" is installed, then `File -> Open folder…`; VSCode auto-detects pure-Java vs Maven projects.
2. **Export a thin jar** — `Java Projects -> Export Jar`, choose `<without main class>`, keep **only the `bin` folder**. Lab 1: including the libraries would produce a "fat jar heavy to transfer"; they are already in the cluster classpath.
3. **Upload the jar** to the gateway local file system on `jupyter.polito.it`.
4. **Prepare data on HDFS** via HUE: create the `example_data` folder, upload the sample files.
5. **Open a Terminal** on the gateway (Launcher -> Terminal) and **submit** (single line!):

```bash
hadoop jar Exercise1-1.0.0.jar it.polito.bigdata.hadoop.DriverBigData 2 example_data ex1_out
```

Meaning of each token (Lab 1, task 4; same decomposition as slide 04b):

- `Exercise1-1.0.0.jar` — the jar containing the application;
- `it.polito.bigdata.hadoop.DriverBigData` — the driver class (fully qualified name);
- `2` — number of reducers (first parameter of this specific driver);
- `example_data` — input folder in HDFS (relative paths start in your HDFS home);
- `ex1_out` — output folder in HDFS (not on the gateway).

6. **Inspect results** in HUE and the job statistics printed on the terminal (number of mappers is in the last part of the output).
7. **Re-running fails** unless the output folder is deleted first (Lab 1 asks you to discover this): `hdfs dfs -rm -r <path>` or delete via HUE.

## YARN: Finding Your Job and Its Logs

Every submission gets an **application id** with format `application_number_number` (e.g. `application_1584304411500_0009`, Lab 1).

```bash
# list running applications and find yours by username
yarn application -list

# full logs of a finished/running application
yarn logs -applicationId application_1584304411500_0009 -log_files stdout
yarn logs -applicationId application_1584304411500_0009 -log_files stderr
```

The `stdout` log contains **one section per task**: one for the driver, one per mapper, one per reducer (Lab 1 & 2). The application id can also be found in the HUE job browser. Later labs (6–9) add the variant used for Spark applications:

```bash
yarn application -list -appStates ALL | grep 'sXXXXXX'
yarn logs -applicationId <application_id>
```

## The Course Datasets

The same few formats appear in labs and in the exercise collection (`exercises/spark/02-spark-exercises-bigdatanb.pdf`). Learn them once, reuse everywhere. The same formats are also the toy inputs of the professor's exercise slides (`sensorId,date,PM10 value`, daily income `date\tincome`, questions/answers, user profiles).

### Amazon fine-food reviews

- `finefoods_text.txt` (Lab 1, `/data/students/bigdata-01QYD/Lab1/`): one plain-text review per line; used for word count and n-grams.
- Word-count output format (Lab 2 and 5 input): `word\tfreq`, one word and its integer frequency per line.
- `Reviews.csv` (Labs 4, 6, 9), comma-separated with **header** that must be filtered:

```
Id,ProductId,UserId,ProfileName,HelpfulnessNumerator,HelpfulnessDenominator,Score,Time,Summary,Text
```

Labs 4 and 6 use columns `ProductId` (2nd), `UserId` (3rd), `Score` (7th); Lab 9 uses `HelpfulnessNumerator`/`HelpfulnessDenominator` and `Text`.
- `AmazonTransposedDataset_Sample.txt` (Lab 3): one line per reviewer, `reviewerId,product1,product2,…` — products reviewed by that user.

### Barcelona bike sharing (Labs 7, 8)

- `register.csv` (tab-separated, with header; shared path `/data/students/bigdata-01QYD/Lab7/register.csv`): `stationId\ttimestamp\tusedslots\tfreeslots`, e.g. `23 2008-05-15 19:01:00 5 13`. Some lines are **dirty**: `used slots = 0 AND free slots = 0` and must be filtered out.
- `stations.csv`: `stationId\tlongitude\tlatitude\tname`.

### PM10 sensors (exercises 32–39; professor's exercises 3–9)

CSV lines `sensorId,date,PM10 value`, e.g. `s1,2016-01-01,20.5`. The most reused toy dataset in the exercise book: max, top-k, per-sensor aggregation, critical dates, group-by patterns; the professor's slide exercises use it for count-above-threshold, average, max/min, total count, outlier filtering.

### Other exercise inputs

- Web server logs: `66.249.69.97 - - [24/Sep/2014:22:25:44 +0000] "GET http://www.google.com/bot.html"` (exercises 30–31).
- Questions/answers (ex. 42; professor's exercise 28): `QuestionId,Timestamp,Text` and `AnswerId,QuestionId,Timestamp,Text`.
- Video-on-demand watched movies / preferences / movie catalog (exercises 44–45).
- Temperature time series `timestamp, temperature`, Unix timestamps, 1-minute sampling (ex. 46).

## From Lab 1 to the Rest of the Course

- Labs 1–4: **Hadoop MapReduce** in Java (see [02 Hadoop and HDFS](../02-hadoop-and-hdfs/02-hadoop-and-hdfs.md) and [03 MapReduce Programming](../03-mapreduce-programming/03-mapreduce-programming.md)).
- Lab 5 onward: **Apache Spark**, first with the RDD API in PySpark (labs 5–7), then Spark SQL DataFrames (lab 8) and `spark.ml` pipelines (lab 9) — see notes 04–06.
- The exercise book (`02-spark-exercises-bigdatanb.pdf`, exercises 30–46) mirrors the exam style; note 08 solves representative ones end-to-end.
- MLlib and Streaming are covered by the theory slides; see [09 Spark MLlib](../09-spark-mllib/09-spark-mllib.md) and [10 Spark Streaming](../10-spark-streaming/10-spark-streaming.md).

## Key Takeaways

- Big data = Volume, Variety, Velocity (+ Veracity, Value); sources: UGC, logs, IoT, scientific computing.
- One node cannot do it: 200 TB take days just to read, ~1 failure/day on 1000 nodes, 10 TB take 2 h to move → **move code to data**, exploit data locality.
- Prefer **scale out** (commodity clusters) over scale up; the price is distributed-computing complexity (scheduling, synchronization, storage, network).
- Lambda Architecture = immutable master dataset + batch layer (accurate, slow) + speed layer (fast, approximate) + serving layer; queries merge both views.
- Exam: 27 points of programming (MapReduce/RDDs) + 4 points of theory, open book, 1.5 h.
- Develop locally, run on the cluster: thin jar from VSCode, upload to the gateway, submit with `hadoop jar` / `spark-submit`.
- Gateway local FS ≠ HDFS home; job input/output folders live in HDFS.
- `yarn application -list` + `yarn logs -applicationId <id> -log_files stdout` is the standard debugging loop.
- Always estimate pairs/bytes read from HDFS and shuffled over the network — this is the course's complexity model.
