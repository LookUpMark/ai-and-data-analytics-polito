---
title: Spark RDD Programming
aliases: [PySpark transformations, PySpark actions, pair RDDs, reduceByKey, groupByKey, aggregate]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> The complete RDD toolbox used in BDPA, with PySpark code: creating RDDs (`textFile`, `parallelize`), basic and set transformations, all the driver-side actions (including `reduce`/`fold`/`aggregate`), pair RDDs (`reduceByKey`, `groupByKey`, `join`, `cogroup`, …), and how Labs 5–7 combine them. Examples come verbatim from the RDD slides and the lab notebooks.

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

All of these are **lazy**: nothing is read until an action runs. If `name` is a folder, *all* files inside are read — "also those without suffix or with a suffix different from `.txt`" (slides, p. 16).

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

Equivalent in efficiency; lambdas are more readable, `def`s support multiple statements and reuse.

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

From `labs/lab-5/main.ipynb` (input: `word\tfreq` lines):

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

From `labs/lab-6/main.ipynb` (one job: transpose → pair frequencies → sorted output):

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

The stored output folder contains files like `part-00000` with lines `('B1,B3', 3)` — verified in `labs/lab-6/output/`.

## Lab 7 (RDD part): Criticality per (station, timeslot)

From `labs/lab-7/main.ipynb` (RDD solution block):

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
- Driver-side actions: `collect`/`count`/`countByValue`/`take`/`first`/`top`/`takeOrdered`/`takeSample`; aggregating actions `reduce` (assoc+comm), `fold` (assoc), `aggregate` (assoc, different result type).
- `top(n, key)` / `takeOrdered(n, key)` with a key function solve most "max/top-k" exam questions.
- Pair-RDD aggregations: prefer `reduceByKey` (partial aggregation on map side) over `groupByKey` when a combine step exists; `join`/`cogroup` for multi-input problems.
- The labs are the templates: Lab 5 = filter + stats, Lab 6 = transpose + pairs + top-k, Lab 7 = ratio per composite key + per-key argmax with tie-breaks.
