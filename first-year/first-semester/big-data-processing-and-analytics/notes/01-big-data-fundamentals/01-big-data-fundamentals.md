---
title: Big Data Fundamentals
aliases: [BDPA fundamentals, big data basics, BigData@Polito lab platform]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> This note introduces the Big Data Processing and Analytics (BDPA) course workflow: developing applications locally, uploading them to the BigData@Polito gateway (`jupyter.polito.it`), managing data on HDFS through HUE, and submitting jobs with YARN. It also catalogs the recurring datasets (Amazon fine-food reviews, Barcelona bike-sharing stations, PM10 sensors, web server logs) used across all labs and exam-style exercises. Everything here is grounded in the lab handouts of the course.

## Why Distributed Processing in This Course

The course is built around one practical fact, repeated in every lab: the input files are too large to process on a single lab PC, so applications are **submitted to the BigData@Polito cluster** and executed in parallel on its nodes (Lab 1, `labs/lab-1/lab1-bigdata-vscode.pdf`).

Consequences that show up throughout the labs:

- You **compile locally** (VSCode) but **run remotely** (cluster). The local project only contains the libraries needed to *compile*; Lab 1 states explicitly that you *cannot* use that project to run MapReduce locally.
- Statistics such as *the number of mappers* are not chosen in code: they depend on how the input is split on the distributed file system, and you read them from the job output on the terminal (Lab 1, task 4).
- Even simple questions ("which words start with `ho`?") are solved by **Hadoop/Spark jobs reading from HDFS**, not by opening a file in an editor. Lab 2: *"inspecting these results manually is not feasible"*.

The recurring cost model, stated in Lab 2 and worth keeping in mind for every exercise:

> - How many pairs and bytes are read from HDFS?
> - How many pairs and bytes are emitted by the mappers and hence how many data are sent on the network?

## The BigData@Polito Platform

Three cooperating pieces, all introduced in Lab 1:

| Component | URL / command | Role |
|---|---|---|
| Gateway (JupyterHub) | `https://jupyter.polito.it` | Login point; hosts notebooks, terminals, and a **local** file system for your scripts/jars |
| HUE | `https://hue.polito.it` | Web browser for HDFS: upload/download/move/delete files, browse job history |
| YARN + HDFS cluster | via `hadoop` / `yarn` / `spark-submit` commands | Executes MapReduce and Spark applications |

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

Meaning of each token (Lab 1, task 4):

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

The same few formats appear in labs and in the exercise collection (`exercises/spark/02-spark-exercises-bigdatanb.pdf`). Learn them once, reuse everywhere.

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

### PM10 sensors (exercises 32–39)

CSV lines `sensorId,date,PM10 value`, e.g. `s1,2016-01-01,20.5`. The most reused toy dataset in the exercise book: max, top-k, per-sensor aggregation, critical dates, group-by patterns.

### Other exercise inputs

- Web server logs: `66.249.69.97 - - [24/Sep/2014:22:25:44 +0000] "GET http://www.google.com/bot.html"` (exercises 30–31).
- Questions/answers (ex. 42): `QuestionId,Timestamp,Text` and `AnswerId,QuestionId,Timestamp,Text`.
- Video-on-demand watched movies / preferences / movie catalog (exercises 44–45).
- Temperature time series `timestamp, temperature`, Unix timestamps, 1-minute sampling (ex. 46).

## From Lab 1 to the Rest of the Course

- Labs 1–4: **Hadoop MapReduce** in Java (see [02 Hadoop and HDFS](../02-hadoop-and-hdfs/02-hadoop-and-hdfs.md) and [03 MapReduce Programming](../03-mapreduce-programming/03-mapreduce-programming.md)).
- Lab 5 onward: **Apache Spark**, first with the RDD API in PySpark (labs 5–7), then Spark SQL DataFrames (lab 8) and `spark.ml` pipelines (lab 9) — see notes 04–06.
- The exercise book (`02-spark-exercises-bigdatanb.pdf`, exercises 30–46) mirrors the exam style; note 08 solves representative ones end-to-end.

## Key Takeaways

- Develop locally, run on the cluster: thin jar from VSCode, upload to the gateway, submit with `hadoop jar` / `spark-submit`.
- Gateway local FS ≠ HDFS home; job input/output folders live in HDFS.
- `yarn application -list` + `yarn logs -applicationId <id> -log_files stdout` is the standard debugging loop.
- Output folders cannot be overwritten: delete before re-running.
- Always estimate pairs/bytes read from HDFS and shuffled over the network — this is the course's complexity model.
