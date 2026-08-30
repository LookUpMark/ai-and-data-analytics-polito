---
title: Spark RDD Programming
aliases: [PySpark transformations, PySpark actions, pair RDDs, reduceByKey, groupByKey, aggregate, RDD caching, persist, accumulators, broadcast variables, numeric RDD actions]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> The complete RDD toolbox used in BDPA, with PySpark code: the `SparkContext`/`SparkConf` setup, creating RDDs (`textFile`, `parallelize`) and their partitioning rules, lazy evaluation and the lineage DAG, basic and set transformations with their shuffle costs, all the driver-side actions (including `reduce`/`fold`/`aggregate` and the numeric actions `sum`/`mean`/`stdev`/`variance`/`max`/`min`), pair RDDs (`reduceByKey`, `groupByKey`, `join`, `cogroup`, …), RDD persistence (`cache`/`persist`, storage levels) and shared variables (accumulators, broadcast variables), and how Labs 5–7 combine them. Examples come verbatim from slides 11, 13, 14 and the lab notebooks.

## The Spark Context and What an RDD Is

The driver "connects" to the cluster through the **Spark Context** object (`SparkContext` class in Python). It is built by means of the constructor of the `SparkContext` class, whose only parameter is a configuration object:

```python
from pyspark import SparkConf, SparkContext

# Create a configuration object and set the name of the application
conf = SparkConf().setAppName("Application name")

# Create a Spark Context object
sc = SparkContext(conf=conf)

# Or: retrieve the current SparkContext object or create a new one
sc = SparkContext.getOrCreate(conf=conf)
```

- `SparkContext.getOrCreate(conf)` returns the existing `SparkContext` if one already exists for this application, otherwise it creates a new one.
- There is **one single `SparkContext` object for each application**.

> [!definition] RDD
> A Spark RDD is an **immutable distributed collection of objects**. Each RDD is split in **partitions**; this choice allows parallelizing the code based on RDDs, because the code is executed on each partition in isolation. RDDs can contain any type of Scala, Java, and Python objects, including user-defined classes.

## Creating RDDs

```python
# From a textual file: one RDD element per line; folder = all files inside it
lines = sc.textFile("myfile.txt")

# Set a minimum number of partitions (must be >= HDFS blocks for HDFS files)
lines = sc.textFile("myfile.txt", 4)

# From a local Python collection; partitions chosen by Spark (or numSlices)
distRDDList = sc.parallelize(['First element', 'Second element', 'Third element'])
distRDDList = sc.parallelize(inputList, 3)
```

All of these are **lazy**: nothing is read until an action runs (Spark only records *how to create* the RDD; the data is read when an action is applied on the RDD or on one of its "descendants"). If `name` is a folder, *all* files inside are read — "also those without suffix or with a suffix different from `.txt`" (slides, p. 16).

Partitioning rules stated in the slides:

- By default, for an HDFS file the number of partitions of the created RDD is **equal to the number of HDFS blocks** used to store the file — this supports **data locality**.
- `textFile(name, minPartitions)` manually sets a (minimum) number of partitions to increase the parallelization of the application; for HDFS files `minPartitions` must be **greater than the number of blocks/chunks**, and the slides mark this option as *not suggested* when using HDFS.
- With `parallelize(c)` Spark tries to set the number of partitions **automatically based on your cluster's characteristics**; `parallelize(c, numSlices)` overrides it.
- `saveAsTextFile(path)` is an **action**: the content of the RDD is computed when it is invoked, and the output folder contains **one textual file per partition** of the RDD.
- `collect()` copies the whole RDD into a local Python list of the driver: use it *if and only if you are sure the list is small*, otherwise store the RDD with `saveAsTextFile`.

Storing results:

```python
linesRDD.saveAsTextFile("risFolder/")   # action; one file per partition
```

## Passing Functions: Lambda vs Local `def`

Any transformation taking a function accepts either form (slides, pp. 46–56):

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

Equivalent in efficiency; lambdas are more concise and readable, `def`s support multi-statement functions (and statements that do not return a value) and code reuse across applications.

## Lazy Evaluation and the Lineage Graph

- RDDs support two types of operations: **transformations** (return a *new* RDD — RDDs are immutable, you never modify one in place) and **actions** (return results to the driver as local Python variables, or write the result to storage, where the size can be large).
- Transformations are **computed lazily**: when one is invoked, Spark keeps only track of the dependency between the input RDD and the new RDD; the content is computed only when an action needs it.
- The graph of dependencies between RDDs is the **lineage graph**, a DAG (Directed Acyclic Graph). It is needed to compute the content of an RDD the first time an action is invoked on it, and to **compute again the content of an RDD (or some of its partitions) when failures occur**.
- Also `textFile()` is lazy, but it is *not* a transformation because it is not applied on an RDD.
- Actions that return a value to the driver run in two steps: (i) local results are computed on each node containing partitions of the RDD; (ii) local results are **sent on the network to the driver**, which computes the final result — mind the size and the network traffic.

The complete skeleton of a standalone application from the slides: the input file is read only when the first action (`count()`) is invoked.

```python
from pyspark import SparkConf, SparkContext
import sys

if __name__ == "__main__":
    conf = SparkConf().setAppName("SparkApplication")
    sc = SparkContext(conf=conf)

    # Read the content of a log file
    inputRDD = sc.textFile("log.txt")

    # Select the rows containing the word "error" / "warning"
    errorsRDD  = inputRDD.filter(lambda line: line.find('error') >= 0)
    warningRDD = inputRDD.filter(lambda line: line.find('warning') >= 0)

    # Union + duplicate removal (lines containing both words)
    badLinesRDD       = errorsRDD.union(warningRDD)
    uniqueBadLinesRDD = badLinesRDD.distinct()

    # First action of the program: the input file is read now
    numBadLines = uniqueBadLinesRDD.count()
    print("Lines with problems:", numBadLines)
```

On the execution plan of this example the slides remark that Spark, similarly to an SQL optimizer, *could* optimize the chain of transformations (e.g., merge the two filters + union + distinct into a single filter) — but this kind of optimization is performed **only on Datasets and DataFrames**, not on plain RDDs (see note 06).

## Basic Transformations

Examples use `inputRDD = [1, 2, 3, 3]` (slides, p. 128).

### `filter(f)` — keep elements satisfying a Boolean function

```python
greaterRDD = inputRDD.filter(lambda num: num > 2)        # -> [2 is dropped] [3, 3]
errorsRDD = inputRDD.filter(lambda e: e.find('error') >= 0)
```

### `map(f)` — exactly one output element per input element (type may change)

```python
lenghtsRDD = inputRDD.map(lambda line: len(line))        # strings -> integers
squaresRDD = inputRDD.map(lambda element: element * element)
```

### `flatMap(f)` — f returns a **list** (possibly empty) per element; results are concatenated

```python
listOfWordsRDD = inputRDD.flatMap(lambda l: l.split(' '))
```

The returned RDD is an RDD of the *elements* of the lists, not of lists. Duplicates are not removed.

### `distinct()` — remove duplicates (shuffle)

```python
distinctIntRDD = inputRDD.distinct()                     # -> [1, 2, 3]
```

The result requires a **shuffle**: data from different input partitions must be compared to remove duplicates. The shuffle repartitions the input data so that **all the repetitions of the same element** are associated with the **same output partition** (a hash function assigns each input element to one of the new partitions), where a single copy is kept.

### `sortBy(keyfunc, ascending=True)` — sort by the values returned by `keyfunc`

```python
sortedNamesRDD    = inputRDD.sortBy(lambda name: name)        # alphabetical
sortedNamesLenRDD = inputRDD.sortBy(lambda name: len(name))   # by length
```

### `sample(withReplacement, fraction)` — random sample (transformation)

```python
randomSentencesRDD = inputRDD.sample(False, 0.2)   # 20%, without replacement
```

## Set Transformations on Two RDDs

With `inputRDD1 = [1, 2, 2, 3, 3]` and `inputRDD2 = [3, 4, 5]` (slides, pp. 132–134):

```python
outputUnionRDD        = inputRDD1.union(inputRDD2)         # [1,2,2,3,3,3,4,5]  no shuffle, duplicates kept
outputIntersectionRDD = inputRDD1.intersection(inputRDD2)  # [3]                shuffle
outputSubtractRDD     = inputRDD1.subtract(inputRDD2)      # [1, 2, 2]          shuffle
outputCartesianRDD    = inputRDD1.cartesian(inputRDD2)     # [(1,3),(1,4),...]  heavy network traffic
```

`cartesian` allows mixed types (e.g. integers × strings) and returns an RDD of tuples.

Costs and caveats spelled out in the slides:

- `union` keeps duplicates **on purpose**: removing them would require a global view of the whole content of the two RDDs (all partitions would have to be "shared"), which is computationally costly. No shuffle is needed for `union`. If you really need a set union, apply `distinct()` on the output — but `distinct()` triggers a shuffle: *use it if and only if duplicate removal is indispensable*.
- `intersection` executes a **shuffle** (elements from different input partitions must be compared to find common elements) and returns elements *without* duplicates.
- `subtract` executes a **shuffle** too; here the two input RDDs play different roles (the result contains the elements appearing only in the RDD the method is invoked on) and duplicates are **not** removed.
- `cartesian` sends a **large amount of data on the network**: elements from different input partitions (possibly on different servers) must be combined to compute the returned pairs.

## Actions

### Retrieving content

```python
contentOfLines  = linesRDD.collect()          # whole RDD as a local list — mind the size!
numLines        = linesRDD.count()            # number of elements
namesOccurrences = namesRDD.countByValue()    # dict element -> occurrences (size = #distinct)
retrievedValues = inputRDD.take(2)            # first 2 elements
firstElement    = inputRDD.first()            # first element (take(1) would return a 1-element list)
randomValues    = inputRDD.takeSample(True, 2)  # random sample to the driver
```

### Top / smallest elements

```python
top2          = inputRDD.top(2)                        # 2 largest, default order, descending
longestNames  = inputRDD.top(2, lambda s: len(s))       # 2 largest by key function
smallest2     = inputRDD.takeOrdered(2)                 # 2 smallest, ascending
shortestNames = inputRDD.takeOrdered(2, lambda s: len(s))
```

The `key` version applies the function before comparing — this is how the exercise solutions select maxima: `pm10ValuesRDD.takeOrdered(1, lambda n: -n)[0]` (invert the order to get the maximum, see note 08).

### `reduce(f)` — combine everything into one value

`f` **must be associative and commutative**, otherwise the result depends on partitioning (slides, pp. 180–190):

```python
sumValues = inputRDD.reduce(lambda e1, e2: e1 + e2)          # [1,2,3,3] -> 9
maxValue  = inputRDD.reduce(lambda e1, e2: max(e1, e2))
```

### `fold(zeroValue, op)` — reduce with a neutral zero value

`op` must be associative; **not** required to be commutative (pp. 192–196):

```python
finalString = inputRDD.fold('', lambda s1, s2: s1 + s2)      # string concatenation
```

### `aggregate(zeroValue, seqOp, combOp)` — result type ≠ RDD type

The workhorse for computing a value of type `U` from an RDD of type `T` (pp. 198–208):

- `seqOp(acc, element)` merges an element into the accumulator **of one partition**;
- `combOp(p1, p2)` merges the partial results of **two partitions**;
- both must be associative.

```python
zeroValue = (0, 0)   # (sum, count)

sumCount = inRDD.aggregate(zeroValue,
    lambda acc, e: (acc[0] + e, acc[1] + 1),          # seqOp: element -> accumulator
    lambda p1, p2: (p1[0] + p2[0], p1[1] + p2[1]))    # combOp: partition result + partition result

myAvg = sumCount[0] / sumCount[1]   # average of [1,2,3,3] -> 9/4
```

Walkthrough from the slides for `[1,2] | [3,3]` (two partitions): accumulators evolve `(0,0)→(1,1)→(3,2)` and `(0,0)→(3,1)→(6,2)`, then `combOp` gives `sumCount=(9,4)`.

Other actions listed in the slides' catalog: `foreach()` (applies a function to each element; listed among the basic actions returning/using driver-side results). All the driver-side actions share the same two-step execution: local results per node, then network transfer to the driver.

### Quick reference (slides, pp. 217–220, on `[1,2,3,3]`)

| Action | Example | Result |
|---|---|---|
| `collect()` | `inputRDD.collect()` | `[1,2,3,3]` |
| `count()` | `inputRDD.count()` | `4` |
| `countByValue()` | `inputRDD.countByValue()` | `[(1,1),(2,1),(3,2)]` |
| `take(2)` / `first()` | `inputRDD.take(2)` | `[1,2]` / `1` |
| `top(2)` | `inputRDD.top(2)` | `[3,3]` |
| `reduce(f)` | `inputRDD.reduce(lambda e1,e2: e1+e2)` | `9` |
| `fold(z,f)` | `inputRDD.fold(0, lambda v1,v2: v1+v2)` | `9` |
| `aggregate(z,seq,comb)` | the tuple above | `(9,4)` |

## Actions on RDDs of Numbers (slides 13)

Spark provides specific actions for RDDs containing numerical values (integers or floats). RDDs of numbers are created with `parallelize` or with transformations that return an RDD of numbers, and support the actions `sum()`, `mean()`, `stdev()`, `variance()`, `max()`, `min()`.

Example from the slides on `[1.5, 3.5, 2.0]`:

```python
inputRDD = sc.parallelize([1.5, 3.5, 2.0])

print("sum:",      inputRDD.sum())       # 7.0
print("mean:",     inputRDD.mean())      # 2.3333
print("stdev:",    inputRDD.stdev())     # 0.8498
print("variance:", inputRDD.variance())  # 0.7223
print("max:",      inputRDD.max())       # 3.5
print("min:",      inputRDD.min())       # 1.5
```

## RDD Persistence: `cache()` and `persist()` (slides 14)

Spark computes the content of an RDD each time an action is invoked on it — so an RDD used multiple times is **recomputed** for every action (on it or on its "descendants"). This is expensive, especially for iterative applications: you can ask Spark to **persist/cache** RDDs.

- When you persist an RDD, each node stores the content of its partitions **in memory** and reuses it in the next actions on that RDD (or RDDs derived from it). Future actions can be much faster ("often by more than 10x", slides).
- `rdd.cache()` persists with storage level **`MEMORY_ONLY`** (the default level).
- `rdd.persist(storageLevel)` selects the level explicitly (values in `pyspark.StorageLevel`: `MEMORY_ONLY`, `MEMORY_AND_DISK`, `DISK_ONLY`, `NONE`, `OFF_HEAP`, `MEMORY_ONLY_2`, `MEMORY_AND_DISK_2`, …).
- Both `persist` and `cache` **return a new RDD** (RDDs are immutable).
- Spark automatically monitors cache usage and drops old partitions in a **least-recently-used (LRU)** fashion; `rdd.unpersist()` manually removes an RDD from the cache.

| Storage Level | Meaning |
|---|---|
| `MEMORY_ONLY` | Deserialized Java objects in the JVM; partitions that do not fit in memory are **not cached** and recomputed on the fly. Default. |
| `MEMORY_AND_DISK` | Partitions that do not fit in memory are stored on local disk and read from there. |
| `DISK_ONLY` | Partitions stored only on disk. |
| `MEMORY_ONLY_2`, `MEMORY_AND_DISK_2`, … | Same as the levels above, but each partition is **replicated on two cluster nodes** (if one node fails, the other copy is used without recomputation). |
| `OFF_HEAP` (experimental) | Similar to `MEMORY_ONLY`, but data is stored in off-heap memory (must be enabled). |

When is it worth it?

- Persistence gives an advantage **if and only if the same RDD is used multiple times** (multiple actions on it or on its descendants).
- Disk-based storage levels are useful **only if** the RDD is significantly smaller than the input dataset, or the functions computing it are expensive — otherwise recomputing a partition may be as fast as reading it from disk.

```python
# Print the number of lines and the number of distinct words of words.txt.
# cache() avoids reading/computing the RDD twice
inputRDD = sc.textFile("words.txt").cache()

print("Number of words: ", inputRDD.count())
# first action: content is computed from words.txt AND stored in the
# main memory of the nodes
print("Number of distinct words: ", inputRDD.distinct().count())
# second action: distinct()+count() reads the data from memory
```

## Shared Variables: Accumulators and Broadcast Variables (slides 14)

When a function passed to a Spark operation is executed on a remote cluster node, it works on **separate copies** of all the variables it uses: the variables are copied to each node and **no updates are propagated back** to the driver. Spark provides two kinds of shared variables for the two recurring patterns.

### Accumulators — aggregate on the write side

> [!definition] Accumulator
> A shared variable that is only "added to" through an associative operation, and can therefore be efficiently supported in parallel. Used to implement counters or sums — usually to compute simple statistics *while* performing some other action, instead of a separate `reduce()` action.

- The **driver** defines and initializes it: `acc = sc.accumulator(value)` (native support for integers and floats; custom types need a class subclassing `AccumulatorParam`, with methods `zero` — the zero value of the type — and `addInPlace` — how to add two values together).
- The code executed on the worker nodes (the functions of the transformations) **increases** it with `acc.add(value)`.
- Only the **driver can read** the final value (`acc.value`); workers can only add to it.
- Because transformations are lazily evaluated, the value of the accumulator is computed **only when an action** is executed on the RDD whose transformations increase it.

```python
# Select valid emails (containing '@'), save them, and count the invalid ones
invalidEmails = sc.accumulator(0)
emailsRDD = sc.textFile("emails.txt")

def validEmailFunc(line):
    if line.find('@') < 0:
        invalidEmails.add(1)     # increment on the worker
        return False
    return True

validEmailsRDD = emailsRDD.filter(validEmailFunc)
validEmailsRDD.saveAsTextFile(outputPath)        # action: the function runs
print("Invalid email addresses: ", invalidEmails.value)
# correct only because saveAsTextFile executed validEmailFunc on every element
```

### Broadcast variables — share a read-only lookup table

> [!definition] Broadcast variable
> A **read-only** (small/medium) shared variable instantiated in the driver and sent **once per executor** that uses it in at least one Spark operation (not once per task). This limits the amount of data sent on the network with respect to "standard" variables. Typically used to share lookup tables.

- Created in the driver with `sc.broadcast(value)`. The parameter must be a **local variable, not an RDD** — e.g., materialize a dictionary first with `collectAsMap()`.
- Its value is retrieved (usually inside transformations) with `.value`.
- It must be small enough to fit in the main memory of the driver **and** of the executors.

```python
# dictionary.txt: lines "word integerValue"; document.txt: sentences to transform
# Output: each word replaced by its integer (output "1 2\n2 3 1" for the slide example)
dictionaryRDD = sc.textFile("dictionary.txt").map(
    lambda line: (line.split(" ")[0], line.split(" ")[1]))

dictionary = dictionaryRDD.collectAsMap()        # local dict on the driver
dictionaryBroadcast = sc.broadcast(dictionary)   # broadcast it (once per executor)

textRDD = sc.textFile("document.txt")

def myMapFunc(line):
    transformedLine = ''
    for word in line.split(' '):
        intValue = dictionaryBroadcast.value[word]   # read on the executor
        transformedLine = transformedLine + intValue + ' '
    return transformedLine.strip()

mappedTextRDD = textRDD.map(myMapFunc)
mappedTextRDD.saveAsTextFile(outputPath)
```

## Pair RDDs (key-value RDDs)

Elements are Python tuples `(key, value)`. Created with `map`:

```python
# Lab 5: word\tfreq  ->  (word, freq)
mappedRDD = filteredRDD.map(lambda line: (line.split('\t')[0], int(line.split('\t')[1])))

# Lab 6: CSV review line -> (UserId, ProductId)
mappedRDD = filteredRDD.map(lambda line: (line.split(",")[2], line.split(",")[1]))
```

Operations used in the labs:

```python
rdd.reduceByKey(lambda x, y: x + y)              # Lab 5: sum counts per word (Group totals)
rdd.reduceByKey(lambda x, y: x + "," + y)        # Lab 6: concatenate product ids per user
rdd.groupByKey()                                 # Ex. 39: all dates of a sensor in one iterable
rdd.mapValues(lambda dates: list(dates))         # turn the iterable into a readable list
rdd.flatMapValues(count_pairs)                   # Lab 6: expand each value into many pairs
rdd.countByKey()                                 # Lab 6: dict pair -> #users (driver-side)
rdd.keys()                                       # Ex. 44: just the user ids
rdd.sortBy(lambda x: x[1], ascending=False)      # Lab 6: sort pairs by value, descending
rdd.join(otherPairRDD)                           # Ex. 44: join values sharing the same key
rdd.cogroup(otherPairRDD)                        # Ex. 42/44: both value-iterables per key
rdd.distinct()                                   # Lab 6: distinct (user, product) pairs
```

`join` output: `(key, (value1, value2))`. `cogroup` output: `(key, (iterable1, iterable2))` — reformatted with `mapValues(lambda v: (list(v[0]), list(v[1])))` before storing.

## Lab 5 in Full: Filter + Statistics on Word Frequencies

From `labs/lab-05/main.ipynb` (input: `word\tfreq` lines):

```python
inputRDD = sc.textFile("SampleLocalFile.csv")

# Task 1: keep only words starting with 'ho'
filteredRDD = inputRDD.filter(lambda line: line[:2] == 'ho')
print(filteredRDD.count())                       # number of selected lines

mappedRDD = filteredRDD.map(lambda line: (line.split('\t')[0], int(line.split('\t')[1])))
max_freq_line = mappedRDD.top(1, lambda x: x[1]) # line with maximum frequency
```

Task 2 (notebook variant: frequencies normalized over the total count):

```python
valuesRDD    = mappedRDD.map(lambda x: x[1])
total_count  = valuesRDD.reduce(lambda x, y: x + y)
max_freq     = max_freq_line[0][1] / total_count
threshold    = max_freq * 0.8

freqRDD = mappedRDD.map(lambda x: (x[0], x[1] / total_count))
most_common_linesRDD = freqRDD.filter(lambda x: x[1] >= threshold)
print(most_common_linesRDD.count())              # lines above 80% of max frequency
# words only (one per line) would then be saved with:
# most_common_linesRDD.keys().saveAsTextFile(outputPath)
```

Task 3: frequency distribution into groups 0–5 ([0,100), …, [500,+inf)):

```python
def assign_to_group(x):
    if x < 100:   return 0
    elif x < 200: return 1
    elif x < 300: return 2
    elif x < 400: return 3
    elif x < 500: return 4
    else:         return 5

groupedRDD = inputRDD.map(lambda line: (line.split('\t')[0], int(line.split('\t')[1])))
groupedRDD = groupedRDD.map(lambda x: ("Group " + str(assign_to_group(x[1])), 1))
finalRDD = groupedRDD.reduceByKey(lambda x, y: x + y)
```

## Lab 6 in Full: Product Pairs Reviewed Together

From `labs/lab-06/main.ipynb` (one job: transpose → pair frequencies → sorted output):

```python
# 0. read and remove the header
inputRDD   = sc.textFile("ReviewsSample.csv")
filteredRDD = inputRDD.filter(lambda line: line.find('Id') == -1)

# 1. transpose: (user, product) -> distinct -> (user, comma-separated products)
mappedRDD = filteredRDD.map(lambda line: (line.split(",")[2], line.split(",")[1])).distinct()
reducedRDD = mappedRDD.reduceByKey(lambda x, y: x + "," + y)
mappedRDD = reducedRDD.map(lambda x: (x[0], x[1].split(",")))

# 2. all ordered pairs per user
def count_pairs(line):
    line.sort()
    for i in range(len(line) - 1):
        for j in range(i + 1, len(line)):
            yield line[i] + "," + line[j]

pairsRDD = mappedRDD.flatMapValues(count_pairs)
pairsRDD = pairsRDD.map(lambda x: (x[1], x[0]))          # (pair, user)

# 3. count how many users reviewed each pair, sort by decreasing frequency
countRDD = pairsRDD.countByKey()
countedRDD = sc.parallelize(countRDD.items())
orderedByValueRDD = countedRDD.sortBy(lambda x: x[1], ascending=False)

# bonus: top-10 most frequent pairs
orderedByValueRDD.take(10)

# store (only pairs appearing more than once in the full solution)
orderedByValueRDD = orderedByValueRDD.coalesce(2)
orderedByValueRDD.saveAsTextFile("output")
```

The stored output folder contains files like `part-00000` with lines `('B1,B3', 3)` — verified in `labs/lab-06/output/`.

## Lab 7 (RDD part): Criticality per (station, timeslot)

From `labs/lab-07/main.ipynb` (RDD solution block):

```python
registerRDD = sc.textFile("sampleData/registerSample.csv").filter(lambda x: "station" not in x)

# remove dirty readings used_slots=0 and free_slots=0
registerRDD = registerRDD.filter(lambda x: "\t0\t0" not in x)

# key = (station, weekday, hour); value = (1 if free_slots==0 else 0, 1)
mappedRDD = registerRDD.map(lambda x: (
    (int(x.split("\t")[0]),
     datetime.strptime(x.split("\t")[1], "%Y-%m-%d %H:%M:%S").strftime("%a"),
     datetime.strptime(x.split("\t")[1], "%Y-%m-%d %H:%M:%S").hour),
    (1 if int(x.split("\t")[3]) == 0 else 0, 1)))

# criticality = critical readings / total readings per (station, weekday, hour)
criticalityRDD = mappedRDD.reduceByKey(lambda x, y: (x[0] + y[0], x[1] + y[1]))
criticalityRDD = criticalityRDD.map(lambda x: (x[0], x[1][0] / x[1][1]))
criticalityRDD = criticalityRDD.filter(lambda x: x[1] > criticality_threshold)

# one most-critical timeslot per station (tie-break: hour, then weekday name)
criticalityRDD = criticalityRDD.map(lambda x: (x[0][0], (x[0][1], x[0][2], x[1])))
mostCriticalStations = criticalityRDD.reduceByKey(select_most_critical_stations)
```

`select_most_critical_stations(x, y)` compares criticality (`x[2]`), then hour (`x[1]`), then weekday order via `day_list = ["Mon","Tue","Wed","Thu","Fri","Sat","Sun"]` — a reduce function used with `reduceByKey`, so it must behave correctly when combined in any grouping order.

## Key Takeaways

- Learn the shapes: `filter` (Boolean), `map` (1→1), `flatMap` (1→N), `distinct`/`sortBy`/`sample`, set ops with their shuffle costs.
- Driver-side actions: `collect`/`count`/`countByValue`/`take`/`first`/`top`/`takeOrdered`/`takeSample`; aggregating actions `reduce` (assoc+comm), `fold` (assoc), `aggregate` (assoc, different result type); numeric RDDs add `sum`/`mean`/`stdev`/`variance`/`max`/`min`.
- `top(n, key)` / `takeOrdered(n, key)` with a key function solve most "max/top-k" exam questions.
- Shuffle economics: `union` is shuffle-free (duplicates kept by design), while `distinct`/`intersection`/`subtract` shuffle — chain `distinct()` only when indispensable.
- An RDD reused by multiple actions should be cached: `cache()` (= `MEMORY_ONLY`) or `persist(level)`; LRU eviction, `unpersist()` to release.
- Accumulators (driver-initialized, worker-incremented counters read only by the driver, valid after an action) and broadcast variables (read-only, one copy per executor) replace unsafe in-place variable sharing.
- Pair-RDD aggregations: prefer `reduceByKey` (partial aggregation on map side) over `groupByKey` when a combine step exists; `join`/`cogroup` for multi-input problems.
- The labs are the templates: Lab 5 = filter + stats, Lab 6 = transpose + pairs + top-k, Lab 7 = ratio per composite key + per-key argmax with tie-breaks.
