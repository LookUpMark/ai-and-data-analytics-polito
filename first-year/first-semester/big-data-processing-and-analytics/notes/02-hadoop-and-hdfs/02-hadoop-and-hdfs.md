---
title: Hadoop and HDFS
aliases: [HDFS commands, HUE file browser, hadoop jar submission, YARN applications]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> How the BDPA labs actually use HDFS and YARN: managing files with HUE and `hdfs dfs`, the anatomy of the `hadoop jar` submission command, relative vs absolute paths, the rule that output folders must not exist, and where job results and logs end up. All commands are the ones given in Lab 1 and reused in Labs 2–9.

## HDFS in the Lab Workflow

HDFS (Hadoop Distributed File System) is the storage layer of the BigData@Polito cluster. In the labs you never process local files on the cluster: **jobs read their input from HDFS folders and write their output to HDFS folders** (Lab 1, task 4). Sample files that you download from the course web page must be uploaded to HDFS before a job can see them.

Two file systems coexist and Lab 1 insists on the distinction:

- **Gateway local file system** (`jupyter.polito.it`): holds your `.jar`/`.py` files, uploaded by drag & drop.
- **HDFS home**: browsed at `https://hue.polito.it` under `Browsers/Files`; holds input/output data. Initially empty — *"this is not the same file system as in task 2"*.

## Managing HDFS with HUE (Lab 1, task 3)

1. Log in at `https://hue.polito.it/` with the BigData@Polito credentials.
2. Open `Browsers/Files`: you see your HDFS home.
3. Create the folder `example_data` on HDFS.
4. Upload the sample files from the local `example_data` folder of your PC into the HDFS folder.
5. Find out on your own how to **delete/move/download** files — Lab 1 explicitly asks for this because it is needed in later labs.

HUE is also used to inspect the content of output folders and to browse the **job browser** (`hue.polito.it/hue/jobbrowser/#!jobs`), where you can retrieve application ids of your Spark jobs (Labs 7–8).

## HDFS Command Line

From a Terminal opened on the gateway (Lab 1, task 4):

```bash
# delete a folder recursively (needed before re-running a job on the same output)
hdfs dfs -rm -r <path of the HDFS folder you want to delete>
```

Paths can be:

- **Relative** — they start in *your HDFS home* (e.g. `example_data`, `ex1_out`).
- **Absolute** — full paths inside HDFS, e.g. the shared read-only datasets used by the labs:

| Absolute HDFS path | Content | Used in |
|---|---|---|
| `/data/students/bigdata-01QYD/Lab1/finefoods_text.txt` | Amazon fine-food reviews, one per line | Lab 1 |
| `/data/students/bigdata-01QYD/Lab2/` | Copy of the Lab 1 word-count output (`word\tfreq`) | Labs 2, 5 |
| `/data/students/bigdata-01QYD/Lab3/AmazonTransposedDataset_Sample.txt` | Reviewer → reviewed products | Lab 3 |
| `/data/students/bigdata-01QYD/Lab4/Reviews.csv` | Amazon reviews with ratings | Lab 4 |
| `/data/students/bigdata-01QYD/Lab6_DBD/Reviews.csv` | Amazon reviews with ratings | Lab 6 |
| `/data/students/bigdata-01QYD/Lab7/register.csv`, `.../stations.csv` | Barcelona bike-sharing data | Labs 7, 8 |
| `/data/students/bigdata-01QYD/Lab9/Reviews.csv` | Amazon reviews with helpfulness votes | Lab 9 |
| `/data/students/bigdata-01QYD/ex_data/ExNN/data/` | Exercise inputs (PM10, logs, …) | Exercises 30–46 |

## Submitting a MapReduce Job

Lab 1's submission command (must be typed on one line):

```bash
hadoop jar Exercise1-1.0.0.jar it.polito.bigdata.hadoop.DriverBigData 2 example_data ex1_out
```

Breakdown:

| Token | Meaning |
|---|---|
| `hadoop jar Exercise1-1.0.0.jar` | Run the application contained in this jar |
| `it.polito.bigdata.hadoop.DriverBigData` | Main class = the driver |
| `2` | Application parameter 0: **number of reducers** |
| `example_data` | Application parameter 1: **input folder** in HDFS |
| `ex1_out` | Application parameter 2: **output folder** in HDFS |

The driver parses these `args` positions itself (see the driver in `labs/lab-1/mapreducebasicprojectwindows/.../driverbigdata.java`).

### Jar preparation rules (Lab 1, task 1)

- Export with VSCode `Java Projects -> Export Jar`, selecting **`<without main class>`**.
- Keep **only the `bin` folder** in the jar: the libraries are needed locally to compile but are *already present in the classpath of the cluster*; shipping them produces a heavy fat jar for no benefit.

## Output Folders and `part-*` Files

- The output of a job is a **folder** (not a single file), created by the framework. It contains one `part-XXXXX` file **per reducer**.
- Lab 1 task 4.7: re-running the same job without cleanup **fails** because the output folder already exists. Delete it first (`hdfs dfs -rm -r` or HUE).
- Lab 1 task 4.8: re-run with a different number of reducers and look inside the output folder — the number of `part-*` files follows the number of reducers.
- The same structure appears with Spark: `saveAsTextFile` writes one output file **per partition** of the RDD (Spark slides, pp. 24–27), and Lab 6's stored result is literally `output/part-00000`, `output/part-00001` containing lines such as `('B1,B3', 3)`.

## Number of Mappers and Reducers

- **Reducers**: set by the first parameter of the Lab 1 driver (`job.setNumReduceTasks(numberOfReducers)` in the driver source).
- **Mappers (map tasks)**: not passed as a parameter — the count depends on how the input files are split into input splits; Lab 1 asks you to read the number of map tasks *"in the information showed on the terminal during the execution"* (last part of the printed statistics). The Spark slides (p. 11) state the analogous rule for `sc.textFile`: the number of partitions equals the number of HDFS blocks used to store the file — this is what *supports data locality*.

## YARN Application Management

Every submitted job is a YARN **application** with id `application_number_number` (example in Lab 1: `application_1584304411500_0009`).

```bash
# find the applicationId of your jobs (search your username)
yarn application -list

# standard output log: one section per task (driver, each mapper, each reducer)
yarn logs -applicationId application_1584304411500_0009 -log_files stdout

# standard error log
yarn logs -applicationId application_1584304411500_0009 -log_files stderr
```

For Spark labs (6–9) the same loop works, with the addition used to find finished applications:

```bash
yarn application -list -appStates ALL | grep 'sXXXXXX'
yarn logs -applicationId <application_id>
```

## Practical Checklist (from the labs)

1. Jar exported thin (only `bin`), no main class.
2. Input data uploaded to HDFS (HUE), header lines handled in code when present.
3. Output folder name chosen, folder deleted if it already exists.
4. Submit with `hadoop jar …` (MapReduce) or `spark-submit …` (Spark).
5. Read job statistics from the terminal; get the application id from the submission output, `yarn application -list`, or HUE job browser.
6. Inspect logs with `yarn logs -applicationId … -log_files stdout/stderr`.
7. Shut down the JupyterHub container when done.

## Key Takeaways

- HUE for browsing/uploading/deleting HDFS files; `hdfs dfs -rm -r` for cleanup from the terminal.
- Relative HDFS paths start at your HDFS home; shared course data lives under `/data/students/bigdata-01QYD/`.
- `hadoop jar <jar> <DriverClass> <numReducers> <input> <output>` — input and output are HDFS folders.
- Output folders are never overwritten and contain one `part-` file per reducer/partition.
- `yarn application -list` + `yarn logs -applicationId` is how you debug everything on the cluster.
