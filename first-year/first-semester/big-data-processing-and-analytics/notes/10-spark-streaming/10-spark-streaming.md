---
title: Spark Streaming
aliases: [Spark Streaming, DStream, StreamingContext, micro-batch, windowed computations, reduceByKeyAndWindow, updateStateByKey, checkpointing, exactly-once]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> Stream processing with Spark Streaming (slide 21): what stream processing is and why it matters, continuous vs micro-batch approaches, the three delivery guarantees (at-most-once, at-least-once, exactly-once), the DStream abstraction (a sequence of RDDs, one per batch), the `StreamingContext` lifecycle and its rules, input sources (TCP sockets, HDFS folders, Kafka/Flume/Kinesis), the DStream transformations (including the ones that are transformations only in name: `reduce`, `count`, `countByValue`), output operations, windowed computations (window length and sliding interval, `reduceByKeyAndWindow` with inverse function), checkpointing, stateful processing with `updateStateByKey`, and the `transform` escape hatch for arbitrary RDD operations — each with the word-count example from the slides.

## Stream Processing (slides 21)

> [!definition] Stream processing
> The act of **continuously incorporating new data to compute a result**. Input data is unbounded (no beginning and no end): a series of events that arrive at the stream processing system. The application outputs multiple versions of the results as it runs, or stores them.

Many important applications must process large streams of live data and provide results **in near-real-time**: social network trends, website statistics, intrusion detection systems, …

Benefits claimed for stream processing systems:

- Vastly higher throughput in data processing;
- Low latency (e.g., in seconds) — state can be kept in memory;
- More efficient than repeated batch jobs at updating a result, because the computation is automatically **incrementalized**;
- Scalable to large clusters;
- Simple programming model;
- Processing each event **exactly once** despite machine failures;
- Efficient fault-tolerance in stateful computations.

Open challenges (harder than batch): processing **out-of-order data** based on application timestamps (**event time**), maintaining large amounts of state, handling load imbalance and stragglers, updating the application's business logic at runtime.

Several frameworks follow the same recipe — a cluster of servers scaling horizontally with the data volume: Apache Spark (Streaming component), Apache Storm, Apache Flink, Apache Samza, Apache Apex, Apache Flume, Amazon Kinesis Streams, …

### Two approaches, three guarantees

| Approach | How it works | Latency |
|---|---|---|
| **Continuous** computation | Data are processed **as soon as they arrive**: every record is processed immediately and a result is emitted as soon as possible (one record at a time) | Real-time |
| **Micro-batch** processing | Input data are collected in **micro-batches** (typically less than a few seconds of data); every time a micro-batch is ready, its entire content is processed and a result is emitted (one micro-batch at a time) | Near real-time |

Delivery guarantees for input elements:

- **At-most-once** ("no guarantee"): every element is processed once or less; the result can be wrong/approximated;
- **At-least-once**: elements are replayed on failures, so processed once or more; the result can be wrong/approximated;
- **Exactly-once**: elements are replayed on failures but already-processed elements are not reprocessed; the result is always correct — **slower** than the others.

## Spark Streaming and DStreams

Spark Streaming is a framework for large-scale stream processing:

- Scales to 100s of nodes; second-scale latencies;
- Simple **batch-like API** for implementing complex algorithms;
- **Micro-batch** streaming processing with **exactly-once** guarantees;
- Absorbs live data from Kafka, Flume, ZeroMQ, Twitter, …

> [!definition] DStream
> Spark Streaming runs a streaming computation as a series of very small, deterministic **batch jobs**: the live input stream is split into **batches of X seconds**, each batch is treated as an RDD and processed with RDD operations, and the processed results are returned in batches. A **DStream** (Discretized Stream) is the sequence of RDDs representing this discretized version of the input stream — one RDD for each batch.

- Any operation applied on a DStream translates to operations on the **underlying RDDs**, computed by the Spark engine.
- Fault tolerance: DStreams remember the sequence of operations that created them from the original fault-tolerant input data; batches of input data are **replicated in memory of multiple worker nodes**; data lost due to a worker failure can be **recomputed** from the input data.

## The StreamingContext

A streaming application follows this skeleton:

1. Define a **Spark Streaming Context** (`pyspark.streaming.StreamingContext`), specifying the **batch duration** in seconds;
2. Specify the input stream and define a DStream based on it;
3. Specify the operations to execute for each batch (transformations and actions similar to the RDD ones);
4. `start()` the computation;
5. Wait until the application is killed (`awaitTermination()`) or a timeout expires (`awaitTerminationOrTimeout(milliseconds)`) — if no timeout is set and the application is not killed, it runs forever.

```python
from pyspark.streaming import StreamingContext

ssc = StreamingContext(sc, 10)   # batches of 10 seconds
```

Rules to remember (slides):

- Once a context has been **started**, no new streaming computations can be set up or added to it;
- Once a context has been **stopped**, it cannot be restarted;
- Only **one StreamingContext per application** can be active at the same time;
- `ssc.stop()` also stops the SparkContext — pass `stopSparkContext=False` to stop only the streaming context.

## Input Sources

- **TCP socket**: `ssc.socketTextStream(hostname, port)` — e.g. `lines = ssc.socketTextStream("localhost", 9999)`.
- **HDFS folder**: `ssc.textFileStream(inputFolder)` — every time a **new file** is inserted in the folder its content is stored in the current batch. Updating the content of an existing file does **not** trigger the DStream. Usually used to test applications during development.
- **Dedicated sources**: DStreams are usually defined on top of streams emitted by specific applications — Apache Kafka, Flume, Kinesis, Twitter. You can write your own stream-generator applications, but Kafka/Flume-like tools are usually the more reliable and effective solution.

## DStream Transformations

Transformations return a new DStream; each batch (RDD) of the input DStream is associated with **exactly one** batch (RDD) of the returned DStream. Many transformations are the standard RDD ones:

| Transformation | Behavior on DStreams |
|---|---|
| `map(func)` / `flatMap(func)` / `filter(func)` | As on RDDs, batch by batch |
| `reduce(func)` | New DStream of **single-element RDDs** aggregating each batch — note: on DStreams it is a **transformation**, not an action; `func` must be associative and commutative |
| `reduceByKey(func)` | Aggregates the values of each key within each batch |
| `combineByKey(createCombiner, mergeValue, mergeCombiners)` | As on RDDs, per batch |
| `groupByKey()` | `(K, V)` → `(K, Iterable<V>)` per batch |
| `count()` | Number of elements of each batch — again a **transformation** (returns a DStream of single-element RDDs) |
| `countByValue()` | `(K, Long)` pairs with the frequency of each key **in each batch** — a **transformation** |
| `union(otherStream)` / `join(otherStream)` / `cogroup(otherStream)` | Pairwise per batch: `join` gives `(K, (V, W))`, `cogroup` gives `(K, (Seq[V], Seq[W]))` |
| `pprint()` | Prints the **first 10 elements of every batch** on the driver's standard output — useful for development/debugging |

Output operation: `saveAsTextFiles(prefix, [suffix])` saves each batch in its own folder, named `prefix-TIME_IN_MS[.suffix]` from the batch time — e.g. `counts.saveAsTextFiles(outputPathPrefix, "")`.

## Worked Example — Per-Batch Word Count

Input: a stream of sentences from `localhost:9999`; batches of 5 seconds; print and store the occurrences of each word **of each batch**:

```python
from pyspark.streaming import StreamingContext

outputPathPrefix = "resSparkStreamingExamples"

conf = SparkConf().setAppName("Streaming word count")
sc = SparkContext(conf=conf)
ssc = StreamingContext(sc, 5)

# Receiver DStream connected to localhost:9999
lines = ssc.socketTextStream("localhost", 9999)

# Chain of transformations (the returned RDDs are DStream RDDs)
words      = lines.flatMap(lambda line: line.split(" "))
wordsOnes  = words.map(lambda word: (word, 1))
wordsCounts = wordsOnes.reduceByKey(lambda v1, v2: v1 + v2)

wordsCounts.pprint()                                # driver stdout
wordsCounts.saveAsTextFiles(outputPathPrefix, "")   # one folder per batch

ssc.start()
ssc.awaitTerminationOrTimeout(90)   # run for 90 seconds
ssc.stop(stopSparkContext=False)
```

Slide trace (batch size 10 s in that example): batch 1 (`Test Spark streaming`, `Second sentence Spark streaming`) → `(test,1), (spark,2), (streaming,2), (second,1), (sentence,1)`; the next batch is counted **independently** — counts never mix across batches.

## Windowed Computations

Spark Streaming can apply transformations over a **sliding window** of data:

- Each window contains a set of batches of the input stream; windows can be **overlapped** (the same batch can belong to many consecutive windows);
- Every time the window slides over the source DStream, the RDDs that fall within the window are combined and operated upon to produce the RDDs of the windowed DStream.

Any window operation needs two parameters, and **both must be multiples of the batch interval** of the source DStream:

- **Window length** — the duration of the window (e.g. 3 batches);
- **Sliding interval** — the interval at which the window operation is performed (e.g. every 2 batches).

Window transformations:

| Transformation | Behavior |
|---|---|
| `window(windowLength, slideInterval)` | Returns the windowed DStream of the source |
| `countByWindow(windowLength, slideInterval)` | Single-element DStream with the number of elements of each window |
| `reduceByWindow(reduceFunc, invReduceFunc, windowDuration, slideDuration)` | Aggregates each window with `reduceFunc` (associative and commutative); if `invReduceFunc` is not `None`, the reduction is done **incrementally** using the old window's reduced value |
| `countByValueAndWindow(windowDuration, slideDuration)` | `(K, Long)` frequencies per **window** |
| `reduceByKeyAndWindow(func, invFunc, windowDuration, slideDuration=None, numPartitions=None)` | Reduces each key's values over the batches in a sliding window; if `slideDuration` is `None`, the context's batch duration is used (slide = 1 batch); if `invFunc` is provided, computed **incrementally** by subtracting the values that left the window |

Slide trace: batch interval 10 s, window length 20 s (2 batches), slide 10 s (1 batch) — the counts of the current and previous batch are merged, e.g. `(spark,3)` = 2 occurrences from batch 1 + 1 from batch 2.

### Windowed word count, 15-second windows

```python
ssc = StreamingContext(sc, 5)                    # 5-second batches
ssc.checkpoint("checkpointfolder")               # needed by some window transformations

lines = ssc.socketTextStream("localhost", 9999)
words     = lines.flatMap(lambda line: line.split(" "))
wordsOnes = words.map(lambda word: (word, 1))

# window of 15 s (3 batches); no inverse function here
wordsCounts = wordsOnes.reduceByKeyAndWindow(lambda v1, v2: v1 + v2, None, 15)

wordsCounts.pprint()
wordsCounts.saveAsTextFiles(outputPathPrefix, "")
ssc.start()
ssc.awaitTermination()
```

With the **inverse function** the result is computed incrementally — new values are added and the values that left the window are subtracted:

```python
wordsCounts = wordsOnes \
    .reduceByKeyAndWindow(lambda v1, v2: v1 + v2,
                          lambda vnow, vold: vnow - vold, 15)
```

## Checkpointing

A streaming application must operate **24/7** and be resilient to failures unrelated to the application logic (system failures, JVM crashes, …). Spark Streaming must therefore **checkpoint** enough information to a fault-tolerant storage system to be able to recover from failures: operations that store the data and metadata needed to restart the computation.

- Enabled with `ssc.checkpoint(folder)` on the StreamingContext (folder for the temporary data);
- Checkpointing is **necessary even for some window transformations and stateful transformations** (as in the examples above).

## Stateful Operations: `updateStateByKey`

The `updateStateByKey` transformation maintains a **state** for each key, continuously updated every time a new batch is analyzed. Two steps:

1. **Define the state** — the data type can be arbitrary;
2. **Define the state update function** — how to update the state of a key from its previous state and the new values of the current batch for that key.

In every batch, Spark applies the update function for **all existing keys**: it is invoked once per key on the list of new values and the former state, and returns the new aggregated value.

Example — cumulative word count over the **union of all batches** from the beginning of the stream (for efficiency, each new value combines the last value of the key with the values of the current batch):

```python
ssc.checkpoint("checkpointfolder")               # stateful ops need checkpointing

lines = ssc.socketTextStream("localhost", 9999)
words     = lines.flatMap(lambda line: line.split(" "))
wordsOnes = words.map(lambda word: (word, 1))

def updateFunction(newValues, currentCount):
    if currentCount is None:                     # first occurrence of the key
        currentCount = 0
    return sum(newValues, currentCount)          # old state + new batch values

# DStream of cumulative counts, updated every batch
totalWordsCounts = wordsOnes.updateStateByKey(updateFunction)

totalWordsCounts.pprint()
totalWordsCounts.saveAsTextFiles(outputPathPrefix, "")
ssc.start()
ssc.awaitTerminationOrTimeout(90)
ssc.stop(stopSparkContext=False)
```

Every 5 seconds the printed/stored counts cover everything received **from time 0 to now**, not just the last batch.

## Combining DStreams and RDDs: `transform`

Some RDD transformations are **not available for DStreams** — e.g. `sortBy`, `sortByKey`, `distinct()`. Moreover, joining every batch with another dataset (a "standard" RDD) is not directly exposed in the DStream API. The `transform(func)` transformation covers both cases:

- It returns a new DStream by applying an **RDD-to-RDD function to every RDD** of the source DStream — arbitrary RDD operations become usable.

Example — per-batch word counts **sorted by decreasing occurrences**:

```python
wordsCounts = wordsOnes.reduceByKey(lambda v1, v2: v1 + v2)

# sortBy is not available on DStreams: apply it on each batch RDD
wordsCountsSorted = wordsCounts \
    .transform(lambda batchRDD: batchRDD.sortBy(lambda pair: -1 * pair[1]))

wordsCountsSorted.pprint()
wordsCountsSorted.saveAsTextFiles(outputPathPrefix, "")
```

## Key Takeaways

- Spark Streaming = **micro-batch** stream processing: the stream is discretized into batches of X seconds, each batch is an RDD inside a **DStream**; exactly-once, second-scale latency.
- Set the batch duration when creating `StreamingContext(sc, X)`; then input DStream → transformations → output operations → `start()` → `awaitTermination[OrTimeout]()`; one active context per application, no restart after `stop()` (`stopSparkContext=False` to keep the SparkContext).
- On DStreams, `reduce`, `count`, `countByValue` are **transformations** (per-batch); `pprint()` prints the first 10 elements per batch; `saveAsTextFiles(prefix, suffix)` writes one folder per batch named `prefix-TIME_IN_MS[.suffix]`.
- Window operations take **window length** and **sliding interval**, both multiples of the batch interval; `reduceByKeyAndWindow` with an **inverse function** computes windows incrementally (add new values, subtract expired ones).
- `checkpoint(folder)` is required for resilience (24/7 operation) and by window/stateful transformations.
- `updateStateByKey` keeps a per-key state updated from the previous state + the new batch values (cumulative counts from time 0).
- `sortBy`/`sortByKey`/`distinct` do not exist on DStreams: use `transform(lambda batchRDD: ...)` to apply arbitrary RDD operations batch by batch.
