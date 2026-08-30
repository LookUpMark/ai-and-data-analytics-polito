---
title: Hadoop and HDFS
aliases: [Hadoop architecture, NameNode, HDFS blocks, HDFS commands, HUE file browser, hadoop jar submission, InputFormat, Writable, Combiner, in-mapper combiner]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> Hadoop from the theory slides (03–04b) combined with the lab practice: what Hadoop is and its history, the two core components (MapReduce infrastructure + HDFS with 64–128 MB replicated blocks and the NameNode), the Hadoop ecosystem, the full HDFS/Hadoop command line, the anatomy of Driver/Mapper/Reducer classes, Writable types and custom data types, InputFormat/OutputFormat, the Configuration object, user-defined counters, map-only jobs, Combiners and in-mapper combiners, plus the lab workflow on BigData@Polito (HUE, output folders, YARN logs).

## What Is Hadoop (slides 03)

> [!definition] Hadoop
> A **scalable, fault-tolerant distributed system for big data**, providing **distributed data storage** and **distributed data processing**. It borrowed concepts from Google's systems (Google File System → HDFS, Google MapReduce → Hadoop MapReduce) and is an **open source project under the Apache license** (commercial implementations exist: Cloudera, Hortonworks, MapR).

Timeline: Dec 2004 Google publishes the GFS paper → Jul 2005 Nutch uses MapReduce → Feb 2006 Hadoop becomes a Lucene subproject → Apr 2007 Yahoo! runs it on a 1000-node cluster → Jan 2008 Apache Top Level Project → Jul 2008 tested on 4000 nodes → Feb 2009 Yahoo! Search Webmap on >10,000 cores → 2010 Facebook claims the largest cluster (21 PB, 30 PB by July 2011). Users include Amazon, Facebook, IBM, LinkedIn-era startups, New York Times, Yahoo!, OpenAI.

**Hadoop vs HPC**: Hadoop is designed for **data-intensive** workloads (usually no CPU-demanding tasks); HPC supercomputers are designed for **CPU-intensive** tasks on usually "small" datasets and are measured in FLOPS.

**Core components**:

1. **Distributed big data processing infrastructure based on the MapReduce paradigm** — provides a high-level abstraction: programmers do not handle task scheduling and synchronization; node and task failures are automatically managed (fault-tolerant).
2. **HDFS (Hadoop Distributed File System)** — highly available, fault-tolerant distributed storage.

Separation of concerns: Hadoop programs are based on MapReduce, which **abstracts away the "distributed" part** (scheduling, synchronization). Programmers focus on **what**; the framework handles **how**. Still, an in-depth knowledge of the framework matters: applications must **exploit data locality and limit network usage/data sharing**.

## HDFS Architecture (slides 03)

Typical usage pattern: **huge files (GB to TB)**, data **rarely updated**, reads and appends are common; random read/write operations are usually not performed.

> [!definition] HDFS blocks/chunks
> Each file is split into **chunks/blocks** spread across the servers. Typically each chunk is **64–128 MB**; a chunk contains a part of the content of **one single file** (never two files in the same chunk).

- Each chunk is **replicated** on different servers — usually **3 replicas** — ensuring persistence and availability; replicas are stored in **different racks** when possible. With `N` servers you tolerate up to `N−1` failures at a manageable replication cost.
- The **master node (NameNode)** stores the **HDFS metadata**, e.g. the mapping between the name of a file and the location of its chunks; it might be replicated.
- **Client applications** access files through HDFS APIs: they talk to the NameNode to find the chunk servers of the file of interest, then connect directly to the selected chunk servers to read/write data.

### The Hadoop ecosystem (slide 03)

Each project addresses one specific class of problems:

| Project | Role |
|---|---|
| **Hive** | distributed relational database on MapReduce; SQL-like query language over HDFS data |
| **HBase** | distributed column-oriented database using HDFS for storage |
| **Pig** | data-flow language + execution environment (on MapReduce) for exploring large datasets |
| **Sqoop** | moves data between relational databases and HDFS |
| **ZooKeeper** | distributed coordination service (e.g., distributed locks) |

## Why MapReduce (slides 03)

Running example — **word count**: input a large textual file; output `<word, number of occurrences>` for every word.

- **Case 1, file fits in main memory**: a traditional single-node solution is probably the most efficient; the overheads of a distributed system hurt when files are "small" (and "small" depends on your resources).
- **Case 2, file too large**: split the problem into (almost) independent sub-tasks executed in parallel. In the slides' example: 100 GB file, ≤1000 distinct words, 101 servers, file spread over 100 of them. Each server reads 1 GB locally (seconds), produces a local list of ≤1000 pairs (MBs), and the maximum network transfer is 100 × local list (some MBs).

Scalability is defined along two dimensions: with **2× data** the job takes ~2× as long; with **2× servers** ~half as long (in this example, where the aggregation cost is negligible — frequently it is not, and it depends on the problem and on the developer's ability to limit network traffic).

Design principles for Hadoop applications:

- **Scale "out", not "up"**;
- **Move processing to data** (limited network bandwidth);
- **Process data sequentially, avoid random access** (seeks are expensive; big data applications read all input records).

**Hadoop/MapReduce is designed for** batch processing involving (mostly) full scans of input data and data-intensive applications (whole-web processing/PageRank, social graphs/link prediction, log analysis). **It does not fit well** iterative problems, recursive problems, stream data processing, real-time processing — this is one reason Spark exists (see [04 Spark Architecture](../04-spark-architecture/04-spark-architecture.md)).

## The MapReduce Programming Model (slides 03)

MapReduce is based on functional programming; everything revolves around two functions with predefined signatures — the model is deliberately **limited and strict** (solving complex problems is difficult, but log analysis, PageRank, social graph analysis, sensor/smart-city/network data all fit).

> [!definition] Map and Reduce (formal)
> - `map: (k1, v1) → [(k2, v2)]` — applied over each input key-value pair; returns a **list** of pairs (possibly empty).
> - `reduce: (k2, [v2]) → [(k3, v3)]` — invoked **once per distinct key** `k2`, receives the complete list of values, returns a list of pairs (possibly empty).
>
> Both input and output of a MapReduce program are lists of key-value pairs; the **shuffle and sort** phase (grouping the map output by key) is always the same and is provided by the framework — the developer never writes it.

Phases over the word-count list `L = [toy, example, toy, example, hadoop]`:

1. **Map**: `m(w) = (w, 1)` ⇒ `Lm = [(toy,1), (example,1), (toy,1), (example,1), (hadoop,1)]`;
2. **Shuffle and sort**: group by key ⇒ `(toy,[1,1]) (example,[1,1]) (hadoop,[1])`;
3. **Reduce**: `r(Gw) = (w, sum(Gw.values))` ⇒ `[(toy,2), (example,2), (hadoop,1)]`.

Each `m` invocation happens **in isolation** (trivially parallel); each reduce group is processed in isolation (parallel per key). Keys and values can be integers, floats, strings, or (almost) arbitrary developer-defined data structures. In many applications the input key is ignored (e.g., word count); some applications exploit it (keys can identify records).

## Executing on the Cluster: Jobs, Tasks, Splits (slides 04)

> [!definition] Job, Task, Input split
> - **(Hadoop) Job**: execution/run of a MapReduce program over a dataset.
> - **Task**: execution of a Mapper (map task) or Reducer (reduce task) on a slice of data; many tasks per job.
> - **Input split**: fixed-size piece of the input data; usually each split has approximately the size of an HDFS block/chunk.

Roles:

- The **Driver** contains `main()` (entry point, command-line arguments), configures the job, submits it, coordinates the workflow; **it runs on the client machine, not on the cluster**.
- The **Mapper** "implements" the map phase; its `map(…)` method is invoked once per input `(key, value)` pair; runs on the cluster.
- The **Reducer** "implements" the reduce phase; `reduce(…)` is invoked once per distinct key; runs on the cluster.

Data flow and storage (important for the complexity questions):

- Input key-value pairs are read from **HDFS** (per the InputFormat);
- intermediate pairs emitted by `map` are stored in the **local file system** of the computing server (**not** in HDFS) — intermediate results are **transient**;
- the shuffle-and-sort groups them into `(key, [list of values])` — each key is assigned to one reducer via a **hash function on the key**; potentially all mappers send data to all reducers;
- the `reduce` output pairs are stored in **HDFS** (final result).

To parallelize, Hadoop instantiates **one Mapper (task) per input split** plus a **user-specified number of Reducers**; each reducer receives and processes all pairs of its set of keys. Usually you use fewer reducers than mappers; there is no precise optimal number (trial-and-error; a trade-off).

## Writing the Code: Driver, Mapper, Reducer (slides 04)

- The **Driver class** extends `org.apache.hadoop.conf.Configured` and implements `org.apache.hadoop.util.Tool` (you *can* skip them, but then you manage low-level command-line details yourself). The developer implements `main(…)` (via `ToolRunner.run`) and `run(…)` which configures: job name, input/output formats, mapper class + its output key/value types, reducer class + output key/value types, number of reducers, then `job.waitForCompletion(true)`.
- The **Mapper class** extends `org.apache.hadoop.mapreduce.Mapper<InputKeyType, InputValueType, OutputKeyType, OutputValueType>` (generic, 4 type parameters; input types must be consistent with the Driver's InputFormat). Implement `map(…)`, emit with `context.write(key, value)`.
- The **Reducer class** extends `org.apache.hadoop.mapreduce.Reducer<…>`; implement `reduce(key, Iterable<values>, context)`. The reducer input types must match the mapper output types.

The full word-count reference implementation (driver/mapper/reducer code) is in [03 MapReduce Programming](../03-mapreduce-programming/03-mapreduce-programming.md), together with the lab skeletons.

### Writable types and custom data types

Hadoop has its own basic types, **optimized for network serialization**: `org.apache.hadoop.io.Text` (≈ String), `IntWritable`, `LongWritable`, `FloatWritable`, …

- Basic types implement `Writable` and `WritableComparable`.
- **Keys must be `WritableComparable`** (keys are compared during sort and shuffle); values only need `Writable` (though usually they are also comparable).
- Custom types: implement `public void readFields(DataInput in)` and `public void write(DataOutput out)`; override `toString()` for textual output. If used as a **key**, also implement `compareTo()` and `hashCode()` (the latter to split keys into groups). Slide example: `SumAndCountWritable` with `float sum` + `int count`, used for complex values (sum-and-count pairs for averages).

### InputFormat and OutputFormat

- The input of the program is an HDFS file/folder; the **InputFormat** logically transforms it into `(key, value)` pairs. It (i) validates the input format, (ii) splits the input into **logical input splits** (one per mapper), (iii) provides the **RecordReader** that divides each split into records.
- **TextInputFormat**: one pair per line; key = byte **offset** of the line, value = line content. Example from the slides: file `Toy example file for Hadoop.\nHadoop running example.\nTextInputFormat is used to split data.\n` ⇒ `(0, "Toy example file for Hadoop.")`, `(31, "Hadoop running example.")`, `(56, "TextInputFormat is used to split data.")`.
- **KeyValueTextInputFormat**: each line has the format `key<separator>value`, default separator tab. Example: `10125\tMister John` ⇒ `(10125, "Mister John")`.
- **SequenceFileInputFormat**: for sequential/binary files.
- **OutputFormat**: `TextOutputFormat` writes one line `key\tvalue\n` per output pair; `SequenceFileOutputFormat` for binary output.

### Configuration object

Used to share the (basic) configuration across driver, mappers, reducers: a list of `(property-name, property-value)` pairs. In the driver: `conf.set("property-name", "value")`; in mapper/reducer: `context.getConfiguration().get("property-name")` (returns a String). Use it for small constant parameters available only during execution; mappers/reducers cannot modify them.

### User-defined counters

- Built-in counters record job statistics (input/output records, transmitted bytes).
- **User-defined counters**: declared as a Java **enum** in the driver (e.g. `public static enum COUNTERS { ERROR_COUNT, MISSING_FIELDS_RECORD_COUNT }`), incremented in mappers/reducers with `context.getCounter(COUNTERS.ERROR_COUNT).increment(1)`, retrieved in the driver at the end with `job.getCounters().findCounter(COUNTERS.ERROR_COUNT)`.
- Dynamic counters can be created on the fly with `incrCounter("group name", "counter name", value)` when the set of counters is unknown at design time.

### Map-only jobs

Some applications (e.g., record filtering) need only mappers: set `job.setNumReduceTasks(0)`. The reduce phase **and the shuffle-and-sort phase are skipped**; the map output is directly stored in HDFS.

### Combiners ("mini-reducers")

Standard applications send all mapper output through the network; a **Combiner** pre-aggregates the pairs emitted by the mappers of a single node to limit network traffic.

- The Combiner class **extends `org.apache.hadoop.mapreduce.Reducer`** (there is no dedicated combiner class); it is set in the driver with `job.setCombinerClass(WordCountCombiner.class)`; no need to specify its input/output types again.
- It works on the mapper's local output (main memory or local disk); the compression factor is not significant for small files but pays off on large data.
- It is **safe only if the reduce function is commutative and associative** (e.g., sums, max).
- **Hadoop may or may not execute the combiner** — the decision is at runtime and you cannot check it from your code; your job must be correct without it.
- In ~99% of applications the combiner and reducer are the same class: `job.setCombinerClass(WordCountReducer.class);`.

### `setup` and `cleanup` — in-mapper combiners

Mapper and Reducer classes have `setup(Context)` and `cleanup(Context)` methods (empty if not overridden):

- `setup` is called **once per mapper instance** before all `map` calls: initialize **in-mapper variables** (state preserved across `map` calls; each mapper instance has its own copy);
- `map` updates the in-mapper statistics, usually emitting nothing;
- `cleanup` is called once after all `map` calls: emit the `(key, value)` pairs derived from the in-mapper state.

The **in-mapper combiner** pattern (word-count pseudocode from the slides):

```
method setup:      A ← new AssociativeArray        # once per mapper
method map(k, l):  for all word w ∈ line l: A{w} ← A{w} + 1
method cleanup:    for all word w ∈ A: EMIT(term w, count A{w})
```

It can outperform standard combiners, but **mind memory**: `A` lives in the mapper's local main memory, so it must be smaller than the memory assigned to each mapper (out-of-memory risk). The same `setup`/`cleanup` structure applies to reducers (Lab 3's `TopKVector` is exactly this pattern — see note 03).

## HDFS and Hadoop Command Line (slides 03b)

Ways to access HDFS content: (1) command line, (2) the basic Apache Hadoop web interface (browse + download only, **no upload**), (3) vendor-specific web UIs with full functionality — e.g. **HUE** of Cloudera, used in the labs.

Each cluster user has a personal HDFS folder; **the default is `/user/username`** (`.` = user home). User mapping: via a Kerberos ticket if Kerberos is active, otherwise the local Linux user is considered.

```bash
hdfs dfs -ls folder            # list a folder; `hdfs dfs -ls .` lists your HDFS home
hdfs dfs -cat file             # print a file's content
hdfs dfs -put local_file HDFS_path   # copy local → HDFS  (hdfs dfs -put /data/document.txt /user/garza/)
hdfs dfs -get HDFS_path local_file   # copy HDFS → local  (hdfs dfs -get /user/garza/document.txt /data/)
hdfs dfs -rm HDFS_path         # delete a file
# other Linux-like commands: rmdir, du, tail, ...
```

Reference: `https://hadoop.apache.org/docs/r2.7.1/hadoopproject-dist/hadoop-hdfs/HDFSCommands.html`.

Hadoop programs are submitted with the `hadoop` command (jar file, driver class, application arguments):

```bash
hadoop jar MyApplication.jar it.polito.bigdata.hadoop.DriverMyApplication 1 inputdatafolder/ outputdatafolder/
```

Here args[0] = number of reducers, args[1] = input folder, args[2] = output folder.

## Managing HDFS with HUE (Lab 1, task 3)

1. Log in at `https://hue.polito.it/` with the BigData@Polito credentials.
2. Open `Browsers/Files`: you see your HDFS home.
3. Create the folder `example_data` on HDFS.
4. Upload the sample files from the local `example_data` folder of your PC into the HDFS folder.
5. Find out on your own how to **delete/move/download** files — Lab 1 explicitly asks for this because it is needed in later labs.

HUE is also used to inspect the content of output folders and to browse the **job browser** (`hue.polito.it/hue/jobbrowser/#!jobs`), where you can retrieve application ids of your Spark jobs (Labs 7–8).

## Submitting a Job on BigData@Polito (slides 04b + Lab 1)

Cluster = servers running Hadoop + an access gateway (`jupyter.polito.it`) used to submit jobs, run `hdfs` commands, and analyze logs. Workflow:

1. Log into `https://jupyter.polito.it`.
2. Copy the jar from your PC to the gateway **local** file system (drag & drop).
3. Copy input data to HDFS — via HUE, or drag & drop on Jupyter + `hdfs` commands.
4. Open a Terminal in Jupyter and submit:

```bash
hadoop jar Exercise1-1.0.0.jar it.polito.bigdata.hadoop.exercise1.DriverBigData 2 ex1_data ex1_out
```

### Jar preparation rules (Lab 1, task 1)

- Export with VSCode `Java Projects -> Export Jar`, selecting **`<without main class>`**.
- Keep **only the `bin` folder** in the jar: the libraries are needed locally to compile but are *already present in the classpath of the cluster*; shipping them produces a heavy fat jar for no benefit.

## Output Folders and `part-*` Files

- The output of a job is a **folder** (not a single file), created by the framework. It contains one `part-XXXXX` file **per reducer** (map-only jobs: one per mapper; with MultipleOutputs, one file per prefix per reducer — see note 03).
- Lab 1 task 4.7: re-running the same job without cleanup **fails** because the output folder already exists. Delete it first (`hdfs dfs -rm -r` or HUE).
- Lab 1 task 4.8: re-run with a different number of reducers and look inside the output folder — the number of `part-*` files follows the number of reducers.
- The same structure appears with Spark: `saveAsTextFile` writes one output file **per partition** of the RDD, and Lab 6's stored result is literally `output/part-00000`, `output/part-00001`.

## Number of Mappers and Reducers

- **Reducers**: set by the first parameter of the Lab 1 driver (`job.setNumReduceTasks(numberOfReducers)` in the driver source).
- **Mappers (map tasks)**: not passed as a parameter — the count depends on how the input files are split into input splits; Lab 1 asks you to read the number of map tasks *"in the information showed on the terminal during the execution"*. The Spark slides state the analogous rule for `sc.textFile`: the number of partitions equals the number of HDFS blocks used to store the file — this is what *supports data locality*.

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

## Absolute HDFS Paths Used in the Labs

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

## Practical Checklist (from the labs)

1. Jar exported thin (only `bin`), no main class.
2. Input data uploaded to HDFS (HUE), header lines handled in code when present.
3. Output folder name chosen, folder deleted if it already exists.
4. Submit with `hadoop jar …` (MapReduce) or `spark-submit …` (Spark).
5. Read job statistics from the terminal; get the application id from the submission output, `yarn application -list`, or HUE job browser.
6. Inspect logs with `yarn logs -applicationId … -log_files stdout/stderr`.
7. Shut down the JupyterHub container when done.

## Key Takeaways

- Hadoop = MapReduce (distributed processing) + HDFS (distributed storage); inspired by Google GFS/MapReduce; open source under Apache.
- HDFS: files split into 64–128 MB chunks, ~3 replicas across (possibly different) racks; NameNode stores the metadata; access = ask NameNode, then read chunks directly; huge files, read/append, no random access.
- `hdfs dfs -ls/-cat/-put/-get/-rm`; every user's home is `/user/username`; `hadoop jar <jar> <DriverClass> <args…>` submits jobs (args[0] = #reducers, args[1] = input, args[2] = output in the course template).
- MapReduce formal model: `map: (k1,v1)→[(k2,v2)]`, `reduce: (k2,[v2])→[(k3,v3)]`; shuffle-and-sort is framework-provided; intermediate pairs are transient and local.
- One map task per input split (≈ one per HDFS block); reducers are user-specified; each key goes to exactly one reducer via hashing.
- Keys must be `WritableComparable`; custom types implement `write`/`readFields` (+`compareTo`/`hashCode` as keys).
- Combiners cut network traffic but their execution is not guaranteed — only use commutative+associative logic; in-mapper combiners (setup/map/cleanup) are the in-memory alternative, watch memory.
- Map-only jobs: `setNumReduceTasks(0)`; user counters via enums; parameters via the `Configuration` object.
- Output folders are never overwritten and contain one `part-` file per reducer; `yarn logs -applicationId … -log_files stdout` is how you debug everything on the cluster.
