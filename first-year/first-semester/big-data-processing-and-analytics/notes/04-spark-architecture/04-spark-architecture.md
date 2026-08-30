---
title: Spark Architecture
aliases: [Spark architecture, SparkContext, SparkConf, driver executors, spark-submit, deploy mode, Spark stack, RDD lineage, lazy evaluation, shuffle, stages]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> How a Spark application is structured and run in BDPA: why Spark exists (in-memory processing vs MapReduce's disk I/O), the Spark stack (Core, SQL, Streaming, MLlib, GraphX), cluster terminology (driver, executors, tasks, jobs, stages), `SparkConf`/`SparkContext` (and `SparkSession` for Spark SQL), notebooks vs `spark-submit` with all its options (`--master`, `--deploy-mode`, resources), and the execution model of RDDs — partitions, lazy evaluation, lineage DAG, transformations vs actions, and the shuffle. Sources: slides 10, 10b, 10c and Labs 5–9.

## Why Spark (slide 10)

- Spark is a **fast and general-purpose engine for large-scale data processing**. Goals in the big data context: **generality** (diverse workloads, operators, job sizes), **low latency** (sub-second), **fault tolerance** (faults are the norm), **simplicity** (often coming from generality). Originally developed at UC Berkeley's AMPLab.
- Motivation: using MapReduce for **complex iterative jobs** or **multiple jobs on the same data** involves lots of disk I/O — every iteration/stage writes to and re-reads HDFS, and disk I/O is very slow even when local. MapReduce also cannot analyze streams of data efficiently.
- Opportunity: the cost of main memory decreased, so large memories are available in each server. **Solution: keep more data in main memory** — the basic idea of Spark.
  - *Iterative job*: MapReduce reads/writes HDFS at every iteration; Spark shares data between iterations in main memory (or at least part of them) — "10 to 100 times faster than disk".
  - *Multiple analyses of the same data*: MapReduce reads the input once per query; Spark reads once from HDFS and keeps the data in the distributed main memory for all queries.

### Hadoop MapReduce vs Spark (slide 10)

| | Hadoop MapReduce | Spark |
|---|---|---|
| Storage | Disk only | In-memory or on disk |
| Operations | Map and Reduce | Map, Reduce, Join, Sample, etc. |
| Execution model | Batch | Batch, interactive, streaming (near real-time) |
| Programming environments | Java | Scala, Java, Python, R |

Spark also has lower overhead for starting jobs and less expensive shuffles. Benchmark from the slides on two iterative ML algorithms (K-means clustering, logistic regression): Hadoop MR ≈ **121 s** vs Spark ≈ **4.1 s**, and ≈ 80 s vs ≈ 0.96 s. Spark also won (tied 1st place) the **Daytona Gray 100 TB sort benchmark**.

## The Spark Stack (slide 10)

> [!definition] Spark Core
> The basic component exploited by all high-level analytics components: **task scheduling, memory management, fault recovery**, and the **APIs used to create RDDs and apply transformations and actions on them**. One uniform core is more efficient than Hadoop's many non-integrated tools: when the core gets faster, every component benefits.

Components on top of the core:

- **Spark SQL** — structured data via SQL or querying APIs; based on Datasets/DataFrames; supports Hive Query Language; interacts with Hive tables, Parquet, JSON, …; exploits a query optimizer engine (Catalyst — see note 06).
- **Spark Streaming** — real-time processing of live streams; its APIs operate on RDDs and are similar to the standard ones (see note 10).
- **MLlib** — machine learning/data mining library: preprocessing and dimensionality reduction, classification, clustering, itemset mining (see note 09).
- **GraphX** — graph processing library (subgraph searching, PageRank, …); **no Python API**; for Python use **GraphFrames**, a graph library based on DataFrames.

Schedulers that can execute Spark applications: **Hadoop YARN** (the standard Hadoop scheduler), **Mesos**, and the **Standalone Spark Scheduler** included in Spark.

## Cluster Overview Terminology (slide 10, from spark.apache.org)

| Term | Meaning |
|---|---|
| **Application** | user program built on Spark: a driver program + executors on the cluster |
| **Driver program** | the process running the `main()` of the application and creating the `SparkContext` |
| **Cluster manager** | external service acquiring resources (standalone, Mesos, YARN) |
| **Deploy mode** | where the driver runs: *cluster* mode → driver inside the cluster; *client* mode → driver outside (on the submitting machine) |
| **Worker node** | any cluster node that can run application code |
| **Executor** | process launched for an application on a worker node; runs **tasks** and keeps data in memory/disk across them; each application has its own executors |
| **Task** | unit of work sent to one executor |
| **Job** | parallel computation of multiple tasks, spawned in response to an **action** (e.g. `save`, `collect`) |
| **Stage** | each job is divided into stages; the output of one stage is the input of the next, except final stages whose output is stored in HDFS/a database; **a shuffle is always executed between two stages** |

The shuffle: data must be regrouped/repartitioned under a grouping criterion different from the previous stage's — similar to the map→reduce data movement of MapReduce, and a heavy operation.

**RDDs are distributed across executors**: each RDD is split in partitions spread over the available executors; each executor runs the specified code on its partitions. **More partitions = more parallelism** (with a trade-off). Spark programs can also run **locally**: local threads act as "pseudo-worker" nodes, useful to develop and test before deploying on the cluster.

## The SparkContext: Connecting to the Cluster

The "connection" of the driver to the cluster is based on the **SparkContext** object — it represents a connection to the cluster, allows creating RDDs, "submitting" executors, and invoking transformations/actions. It is created from a configuration object (slides 10b/11):

```python
from pyspark import SparkConf, SparkContext

# Create a configuration object and set the name of the application
conf = SparkConf().setAppName("Application name")
# Create a Spark Context object
sc = SparkContext(conf=conf)
```

There is **always one single SparkContext per application**; `getOrCreate` returns the existing one if it already exists, otherwise creates it (slides 10b/11):

```python
sc = SparkContext.getOrCreate(conf=conf)
```

The notebooks used in Labs 5–8 all follow this pattern before anything else:

```python
import findspark
findspark.init()
import pyspark
sc = pyspark.SparkContext.getOrCreate()
```

(`findspark.init()` locates the local Spark installation in the Jupyter environment.) For scripts run with `spark-submit`, Lab 5 states that the script **must create the SparkContext explicitly** and **must stop it at the end**:

```python
from pyspark import SparkConf, SparkContext
conf = SparkConf().setAppName("Name of my application")
sc = SparkContext(conf=conf)
# ... application ...
sc.stop()
```

**Local variables vs RDDs** (slide 10): local Python variables live in the main memory of the driver process and can store only "small" objects; RDDs store "big" collections in the nodes of the cluster (main memory of executors when possible, local disks when necessary).

### First examples (slide 10)

Line count:

```python
from pyspark import SparkConf, SparkContext

if __name__ == "__main__":
    conf = SparkConf().setAppName("Spark Line Count")
    sc = SparkContext(conf=conf)

    inputFile = "myfile.txt"
    linesRDD = sc.textFile(inputFile)        # one RDD element per line
    numLines = linesRDD.count()              # action
    print("NumLines:", numLines)
```

Word count (input file from `sys.argv[1]`, output folder `sys.argv[2]`):

```python
lines = sc.textFile(inputFile)
words = lines.flatMap(lambda line: line.split(sep=' '))
words_one = words.map(lambda word: (word, 1))
counts = words_one.reduceByKey(lambda c1, c2: c1 + c2)
counts.saveAsTextFile(outputPath)
sc.stop()
```

## Ways of Running a PySpark Application (Lab 5 + slides 10c)

| Approach | How | Where driver/executors run |
|---|---|---|
| **PySpark (Local) notebook** | `jupyter.polito.it` → new "PySpark (Local)" notebook | Both on the **gateway** (application runs in a container, in a local instance of Spark); data still read from/stored on HDFS |
| **PySpark (Yarn) notebook** | New "PySpark (Yarn)" notebook | Driver and executors on the **BigData@Polito cluster** |
| **Python script + `spark-submit`** | `.py` file uploaded to the gateway, run in a Terminal | Depends on options (below) |

Both notebook kinds **read/write data from/in HDFS**. The 10c slides show the intended workflow: copy input data to HDFS, open an interactive PySpark notebook, execute code step by step, store the result in the output HDFS folder. Notebooks run against the gateway `jupyter.polito.it`; the user's "local" home is on a shared area.

The interactive workflow: (1) copy input data to HDFS, (2) open a PySpark notebook, (3) write and execute Python/Spark code step-by-step, (4) results are stored in the HDFS output folder.

## `spark-submit` (slides 10b)

Spark programs are executed (submitted) with the **`spark-submit`** command line program: it takes the `.py` (or jar) file, the driver/entry-point class, the application parameters, plus two key options:

### `--master` — which scheduler/environment executes the application

| Value | Meaning |
|---|---|
| `spark://host:port` | Spark standalone scheduler |
| `mesos://host:port` | Mesos scheduler |
| `yarn` | YARN scheduler (the Hadoop one) |
| `local` | application executed exclusively on the local PC |

### `--deploy-mode` — where the driver is launched

- `client`: the driver is launched locally, on the machine executing `spark-submit`; cluster nodes store RDDs and run transformations/actions; in YARN client mode the ApplicationMaster only requests executor containers.
- `cluster`: the driver is launched on one node of the cluster — in YARN cluster mode the driver runs in the ApplicationMaster, a single YARN-container process both driving the application and requesting resources; the client's resources are not used.

### Resource options (defaults from slide 10b)

```text
--num-executors NUM      default 2
--executor-cores NUM     default 1 per executor
--executor-memory MEM    default 1GB per executor
--driver-cores NUM       default 1
--driver-memory MEM      default 1GB
```

Maxima are limited by the cluster configuration (and, in cluster mode, also the driver options).

### The two course commands

```bash
# on the cluster (YARN), driver inside the cluster
spark-submit --deploy-mode cluster --master yarn MyApplication.py arguments

# everything on the local PC (no Hadoop needed, only Spark)
spark-submit --deploy-mode client --master local MyApplication.py arguments
```

Lab 5's variants:

```bash
# driver on the gateway, executors on the cluster nodes
spark-submit --master yarn --deploy-mode client <your_script>.py

# driver AND executors on the gateway (still reading/writing HDFS)
spark-submit --master local --deploy-mode client <your_script>.py
```

Notes from the lab text: the script must end with `.py` for Spark to interpret it as a Python application; in `--deploy-mode client` the driver is instantiated on the gateway. Lab 5 also requires the script version to take **prefix, input folder and output folder as command-line arguments** (`sys.argv[]`).

## The RDD Abstraction

From the slides: a Spark RDD is an **immutable distributed collection of objects**.

- Each RDD is **split in partitions**; this is what allows code to run in parallel — code is executed on each partition in isolation.
- RDDs can contain any type of Python/Scala/Java objects, including user-defined classes.
- Because RDDs are immutable, an operation never modifies an RDD: it returns a **new** RDD.
- RDDs are stored in the **main memory of the executors** (when possible) or their **local disk**; they are **automatically rebuilt on machine failure**.

Creation rules (slide 10):

- **parallelizing** an existing local collection: the number of partitions is set by Spark (or by the user via `numSlices`);
- **from files in HDFS**: **one partition per HDFS block**;
- from files in traditional file systems/databases;
- by **transforming an existing RDD**: the number of partitions depends on the transformation.

### Partitioning rules in the API

- `sc.textFile(name)`: if the file is on HDFS, the number of partitions equals the number of **HDFS blocks** used to store the file — "to support data locality" (10b/11). You can force more parallelism with `textFile(name, minPartitions)`; for HDFS files `minPartitions` must be greater than the number of blocks (the slides mark manual setting on HDFS as *not suggested*).
- `textFile(folder)`: all files in the folder are read — *"also those without suffix or with a suffix different from `.txt`"*.
- `sc.parallelize(c)`: partitions chosen automatically from the cluster's characteristics; `parallelize(c, numSlices)` sets them manually.
- `saveAsTextFile(path)`: writes **one output file per partition** of the RDD into the output folder. This is why Lab 7's handout tells you to call `coalesce(1)` before saving when you need a single file.

## Lazy Evaluation and the Lineage Graph

Key idea repeated all over the slides: **no computation occurs when you create or transform an RDD**.

- `textFile()`, `parallelize()` and every transformation only *record how to create* the RDD; data is read/computed **only when an action needs it** (also on any of its "descendant" RDDs).
- The graph of dependencies between RDDs is the **lineage graph**, a **DAG** connecting input data and RDDs. It is used:
  - to compute the content of an RDD the first time an action is invoked on it;
  - to **recompute** lost partitions when failures occur (Spark tracks lineage for each RDD);
  - for optimization: Spark can consider the chain of transformations and potentially change the order or merge some of them.

### Transformations vs Actions

| | Transformations | Actions |
|---|---|---|
| Return | a **new RDD** | local Python values (to the driver) or output files |
| When executed | lazily, only when an action triggers them | immediately |
| Examples | `filter`, `map`, `flatMap`, `distinct`, `union`, … | `collect`, `count`, `reduce`, `saveAsTextFile`, … |

Canonical example from the slides:

```python
inputRDD  = sc.textFile("log.txt")
errorsRDD = inputRDD.filter(lambda line: line.find('error') >= 0)
warningRDD = inputRDD.filter(lambda line: line.find('warning') >= 0)
badLinesRDD = errorsRDD.union(warningRDD)
uniqueBadLinesRDD = badLinesRDD.distinct()

numBadLines = uniqueBadLinesRDD.count()   # FIRST action: only now the file is read
print("Lines with problems:", numBadLines)
```

The input file is read **only when `count()` runs** — the first action of the program. `filter`, `union`, `distinct` are transformations; `textFile` is lazy too (though it is not a transformation, since it is not applied on an RDD).

### Optimization remark

Spark, like an SQL optimizer, can rewrite chains of transformations — e.g. the two filters + union + distinct above into a single filter "contains error OR warning". The slides are explicit about the limit: **Spark can perform this kind of optimization only on particular types of RDDs: Datasets and DataFrames** — not on plain RDDs. (This is the bridge to Lab 8 and note 06.)

## Passing Functions: Lambda vs Local `def`

```python
# lambda: concise, one single expression
errorsRDD = inputRDD.filter(lambda l: l.find('error') >= 0)

# local def: multi-statement bodies, reusable code
def myFunction(l):
    if l.find('error') >= 0:
        return True
    else:
        return False

errorsRDD = inputRDD.filter(myFunction)
```

Equivalent in efficiency; lambdas are more concise/readable but do not support multi-statement bodies; `def`s support them and allow code reuse across applications.

## The Shuffle

Some transformations need to compare data that lives in different partitions, on different nodes. The framework then executes a **shuffle**: data is repartitioned so that all equal keys/elements end in the same partition; a hash function assigns each element to its new partition.

| Transformation | Shuffle? | Notes from the slides |
|---|---|---|
| `union(other)` | **No** | duplicates kept — removing them would need a global view of both RDDs; "use `distinct()` only if duplicate removal is indispensable" |
| `distinct()` | Yes | hash-based repartition, one copy per element per output partition |
| `intersection(other)` | Yes | compares partitions to find common elements |
| `subtract(other)` | Yes | result = elements only in the first RDD, duplicates kept |
| `cartesian(other)` | Yes (heavy) | "a large amount of data is sent on the network" |
| pair-RDD aggregations (`reduceByKey`, `groupByKey`, …) | Yes | same mechanism, key-based (labs 5–7) |

The shuffle is the Spark counterpart of MapReduce's map→reduce data movement; in the stage model, a shuffle always separates two stages. Lab 2's questions ("how many bytes are sent on the network?") apply to it unchanged.

## Actions and the Driver

Actions that return data to the driver run **locally on each node** first, then partial results are sent over the network to the driver, which computes the final value. Hence the repeated warning: `collect()`/`countByValue()` results must fit in the driver's main memory — otherwise store the RDD with `saveAsTextFile` (the size can be large in that case, since it goes directly to the distributed file system).

## From RDDs to SparkSession (Labs 7–9)

The Spark SQL labs add one object on top of the context:

```python
from pyspark.sql import SparkSession
sc = pyspark.SparkContext.getOrCreate()
spark = SparkSession.builder.getOrCreate()
```

Lab 7's notebook uses both: RDD transformations for one solution, DataFrames + `spark.sql(...)` for another. Lab 9 uses only the `SparkSession`. Details in notes [05](../05-spark-rdd-programming/05-spark-rdd-programming.md) and [06](../06-spark-dataframes-and-sql/06-spark-dataframes-and-sql.md).

## Controlling Partitions in Practice (from the labs)

```python
# number of partitions of an RDD (Lab 6 inspection cell)
orderedByValueRDD.getNumPartitions()

# reduce the number of partitions before saving (Lab 6: 2 files; Lab 7 hint: 1 file)
orderedByValueRDD = orderedByValueRDD.coalesce(2)
orderedByValueRDD.saveAsTextFile("output")
```

`coalesce(1)` is Lab 7's official hint to obtain **one single output file** from `saveAsTextFile()`.

## Key Takeaways

- Spark = in-memory distributed engine: iterative jobs and repeated queries avoid HDFS round-trips; 10–100× faster than MapReduce on those workloads.
- Stack: Spark Core (RDDs) + Spark SQL + Spark Streaming + MLlib + GraphX/GraphFrames; schedulers: YARN, Mesos, standalone.
- Vocabulary: application = driver + executors; job (per action) → stages (separated by shuffles) → tasks (per partition, on executors).
- One `SparkContext` per application: `SparkConf().setAppName(...)` → `SparkContext(conf=conf)` or `getOrCreate`; close with `sc.stop()` in scripts.
- Notebooks: PySpark local (gateway) vs yarn (cluster). Scripts: `spark-submit --master local|yarn --deploy-mode client|cluster script.py`, resources via `--num-executors/--executor-cores/--executor-memory/--driver-*`, parameters via `sys.argv`.
- RDD = immutable, partitioned, lazily computed collection; lineage DAG enables recomputation after failures; HDFS files → one partition per block.
- Transformations build the DAG; actions (`count`, `collect`, `saveAsTextFile`, …) trigger execution.
- Shuffle-heavy operations (`distinct`, `intersection`, `subtract`, `cartesian`, key-based aggregations) are the costly ones; `union` is cheap but keeps duplicates; a shuffle always separates two stages.
- Catalyst-style reordering of transformations works on DataFrames/Datasets, not plain RDDs.
- One output file per partition; use `coalesce(1)` when a single file is required.
