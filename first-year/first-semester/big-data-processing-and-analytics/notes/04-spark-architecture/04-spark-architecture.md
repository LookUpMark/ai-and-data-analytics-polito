---
title: Spark Architecture
aliases: [SparkContext, SparkConf, driver executors, spark-submit, RDD lineage, lazy evaluation, shuffle]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> How a Spark application is structured and run in BDPA: `SparkConf`/`SparkContext` (and `SparkSession` for Spark SQL), notebooks vs `spark-submit`, `--master local` vs `--master yarn`, and the execution model of RDDs — partitions, lazy evaluation, lineage graph, transformations vs actions, and the shuffle. Sources: the RDD slides (`slides/spark/11-sparkrddbasedprogramming-bigdata-newstyle.pdf`) and Labs 5–9.

## The SparkContext: Connecting to the Cluster

The "connection" of the driver to the cluster is based on the **SparkContext** object (`SparkContext` class in Python) — slides, p. 3. It is created from a configuration object:

```python
from pyspark import SparkConf, SparkContext

# Create a configuration object and set the name of the application
conf = SparkConf().setAppName("Application name")
# Create a Spark Context object
sc = SparkContext(conf=conf)
```

There is **always one single SparkContext per application**; `getOrCreate` returns the existing one if it already exists, otherwise creates it (slides, p. 5):

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

## Ways of Running a PySpark Application (Lab 5)

| Approach | How | Where driver/executors run |
|---|---|---|
| **PySpark (Local) notebook** | `jupyter.polito.it` → new "PySpark (local)" notebook | Both on the **gateway**; data still read from/stored on HDFS |
| **PySpark (Yarn) notebook** | New "PySpark (Yarn)" notebook | Driver and executors on the **cluster nodes** |
| **Python script + `spark-submit`** | `.py` file uploaded to the gateway, run in a Terminal | Depends on options (below) |

The `spark-submit` variants given in Lab 5:

```bash
# driver on the gateway, executors on the cluster nodes
spark-submit --master yarn --deploy-mode client <your_script>.py

# driver AND executors on the gateway (still reading/writing HDFS)
spark-submit --master local --deploy-mode client <your_script>.py
```

Notes from the lab text: the script must end with `.py` for Spark to interpret it as a Python application; in `--deploy-mode client` the driver is instantiated on the gateway. Lab 5 also requires the script version to take **prefix, input folder and output folder as command-line arguments** (`sys.argv[]`).

## The RDD Abstraction

From the slides (p. 8): a Spark RDD is an **immutable distributed collection of objects**.

- Each RDD is **split in partitions**; this is what allows code to run in parallel — code is executed on each partition in isolation.
- RDDs can contain any type of Python/Scala/Java objects, including user-defined classes.
- Because RDDs are immutable, an operation never modifies an RDD: it returns a **new** RDD.

### Partitioning rules

- `sc.textFile(name)`: if the file is on HDFS, the number of partitions equals the number of **HDFS blocks** used to store the file — "to support data locality" (p. 11). You can force more parallelism with `textFile(name, minPartitions)`.
- `sc.parallelize(c)`: Spark sets the number of partitions automatically based on the cluster; `parallelize(c, numSlices)` sets it manually.
- `saveAsTextFile(path)`: writes **one output file per partition** of the RDD into the output folder (pp. 24–27). This is why Lab 7's handout tells you to call `coalesce(1)` before saving when you need a single KML file.

## Lazy Evaluation and the Lineage Graph

Key idea repeated all over the slides: **no computation occurs when you create or transform an RDD**.

- `textFile()`, `parallelize()` and every transformation only *record how to create* the RDD; data is read/computed **only when an action needs it** (pp. 13, 21, 26, 35).
- The graph of dependencies between RDDs is the **lineage graph** (a DAG). It is used (p. 36):
  - to compute the content of an RDD the first time an action is invoked on it;
  - to **recompute** lost partitions when failures occur.

### Transformations vs Actions (p. 33)

| | Transformations | Actions |
|---|---|---|
| Return | a **new RDD** | local Python values (to the driver) or output files |
| When executed | lazily, only when an action triggers them | immediately |
| Examples | `filter`, `map`, `flatMap`, `distinct`, `union`, … | `collect`, `count`, `reduce`, `saveAsTextFile`, … |

Canonical example from the slides (pp. 39–43):

```python
inputRDD  = sc.textFile("log.txt")
errorsRDD = inputRDD.filter(lambda line: line.find('error') >= 0)
warningRDD = inputRDD.filter(lambda line: line.find('warning') >= 0)
badLinesRDD = errorsRDD.union(warningRDD)
uniqueBadLinesRDD = badLinesRDD.distinct()

numBadLines = uniqueBadLinesRDD.count()   # FIRST action: only now the file is read
print("Lines with problems:", numBadLines)
```

The input file is read **only when `count()` runs** — the first action of the program. `filter`, `union`, `distinct` are transformations; `textFile` is lazy too (though it is not a transformation, since it is not applied on an RDD, p. 43).

### Optimization remark (p. 44)

Spark, like an SQL optimizer, can rewrite chains of transformations — e.g. the two filters + union + distinct above into a single filter "contains error OR warning". The slides are explicit about the limit: **Spark can perform this kind of optimization only on particular types of RDDs: Datasets and DataFrames** — not on plain RDDs. (This is the bridge to Lab 8 and note 06.)

## The Shuffle

Some transformations need to compare data that lives in different partitions, on different nodes. The framework then executes a **shuffle**: data is repartitioned so that all equal keys/elements end in the same partition (p. 90).

| Transformation | Shuffle? | Notes from the slides |
|---|---|---|
| `union(other)` | **No** | duplicates kept — removing them would need a global view of both RDDs; "use `distinct()` only if duplicate removal is indispensable" (pp. 113–114) |
| `distinct()` | Yes | hash-based repartition, one copy per element per partition (p. 90) |
| `intersection(other)` | Yes | compares partitions to find common elements (p. 115) |
| `subtract(other)` | Yes | result = elements only in the first RDD, duplicates kept (p. 116) |
| `cartesian(other)` | Yes (heavy) | "a large amount of data is sent on the network" (p. 118) |
| pair-RDD aggregations (`reduceByKey`, `groupByKey`, …) | Yes | same mechanism, key-based (labs 5–7) |

The shuffle is the Spark counterpart of MapReduce's map→reduce data movement: it is the expensive step, and Lab 2's questions ("how many bytes are sent on the network?") apply to it unchanged.

## Actions and the Driver

Actions that return data to the driver run **locally on each node** first, then partial results are sent over the network to the driver, which computes the final value (p. 137). Hence the repeated warning: `collect()`/`countByValue()` results must fit in the driver's main memory — otherwise store the RDD with `saveAsTextFile` (pp. 143, 152).

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

- One `SparkContext` per application: `SparkConf().setAppName(...)` → `SparkContext(conf=conf)` or `getOrCreate`; close with `sc.stop()` in scripts.
- Notebooks: PySpark local (gateway) vs yarn (cluster). Scripts: `spark-submit --master local|yarn --deploy-mode client script.py`, parameters via `sys.argv`.
- RDD = immutable, partitioned, lazily computed collection; lineage DAG enables recomputation after failures.
- Transformations build the DAG; actions (`count`, `collect`, `saveAsTextFile`, …) trigger execution.
- Shuffle-heavy operations (`distinct`, `intersection`, `subtract`, `cartesian`, key-based aggregations) are the costly ones; `union` is cheap but keeps duplicates.
- Catalyst-style reordering of transformations works on DataFrames/Datasets, not plain RDDs.
- One output file per partition; use `coalesce(1)` when a single file is required.
