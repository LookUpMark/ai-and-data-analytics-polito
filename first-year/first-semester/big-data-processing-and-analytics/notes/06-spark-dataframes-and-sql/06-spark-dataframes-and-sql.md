---
title: Spark DataFrames and SQL
aliases: [SparkSession, Spark SQL, DataFrames API, spark.read, createOrReplaceTempView, UDF, broadcast join, left anti join, explain, relational operators, Spark ML pipelines]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> The Spark SQL layer of BDPA (slides 08 and 15) as used in Labs 7–9: what Spark SQL and DataFrames are (Catalyst optimizer, Row objects), creating DataFrames from CSV/JSON/RDDs/lists, the DataFrame API catalog (select, filter, join with all join types, groupBy/agg, set operations, broadcast joins, explain), SQL queries via `createOrReplaceTempView` + `spark.sql`, UDFs, how the classic relational operators (selection, projection, union/intersection/difference, join, aggregation) map onto distributed execution, and the Lab 9 `spark.ml` pipeline (VectorAssembler, SQLTransformer, Decision Tree vs Logistic Regression) with the template's evaluation code.

## What Spark SQL Is (slides 15)

> [!definition] Spark SQL
> Spark SQL is the Spark component for **structured data processing**. It provides a programming abstraction called **DataFrame** and can act as a **distributed SQL query engine**: the input data can be queried by ad-hoc methods or by an SQL-like language.

- The Spark SQL interfaces provide **more information about the structure** of both the data and the computation being performed, with respect to plain RDDs. Spark SQL uses this extra information to perform extra optimizations based on an "SQL-like" optimizer called **Catalyst** → *programs based on DataFrames are usually faster than standard RDD-based programs*.
- Conceptually: RDD = distributed **list of objects** (unstructured); DataFrame ≈ distributed **relational table** (structured).

> [!definition] DataFrame
> A distributed collection of **structured data** organized into **named columns**, conceptually equivalent to a table in a relational database. DataFrames are lists of `Row` objects (classes `pyspark.sql.DataFrame` and `pyspark.sql.Row`). They can be created from structured files (CSV, JSON), existing RDDs, Hive tables, or external relational databases, and they benefit from Spark SQL's optimized execution engine.

All the Spark SQL functionalities are based on an instance of the `pyspark.sql.SparkSession` class:

```python
from pyspark.sql import SparkSession

spark = SparkSession.builder.getOrCreate()
...
spark.stop()   # good practice: close the session
```

## From RDDs to DataFrames

Lab 8 introduces the rule: the same bike-sharing problem solved with RDDs in Lab 7 must be solved **twice** — (i) with **DataFrames and the associated APIs**, (ii) with **SQL queries** via `SparkSession.sql("SELECT …")`. This is also where the slides' optimization remark applies: Spark's optimizer can reorder/merge transformations only on DataFrames/Datasets (see note 04).

The entry point is the `SparkSession` (built on top of the existing context):

```python
import findspark
import pyspark
from pyspark.sql import SparkSession

findspark.init()
sc = pyspark.SparkContext.getOrCreate()
spark = SparkSession.builder.getOrCreate()
```

## Reading Structured Data

From `labs/lab-8/lab-08.ipynb` (tab-separated CSVs with header):

```python
registerDF = spark.read.load('sampleData/registerSample.csv',
                             format='csv',
                             header=True,
                             inferSchema=True,
                             sep='\t')

stationsDF = spark.read.load('sampleData/stations.csv',
                             format='csv',
                             header=True,
                             inferSchema=True,
                             sep='\t')
```

- `inferSchema=True` guesses column types, so `station`, `used_slots`, `free_slots` become numeric and `timestamp` becomes a timestamp column.
- Lab 8's hint for the delimiter: use `.option("delimiter", "\\t")` when reading input files separated by tabs.
- Lab 9 reads the Amazon reviews the same way: `spark.read.load("ReviewsSample.csv", format="csv", header=True, inferSchema=True)`.
- Without `inferSchema=True` **all columns are considered strings**.

The same `load` API reads **JSON** files (slides 15). The expected input is the "JSON Lines text format" (newline-delimited JSON): **one JSON object (tuple) per line**, not a "standard" multiline JSON file:

```
{"name":"Michael"}
{"name":"Andy", "age":30}
{"name":"Justin", "age":19}
```

```python
df = spark.read.load("persons.json", format="json")
# or
spark.read.json("persons.json")

# "standard" multiline JSON files (since Spark 2.2.0):
df = spark.read.load("folder_JSONFiles/", format="json", multiLine=True)
```

The slides warn: **reading a set of small JSON files from HDFS is very slow**. The `DataFrameReader` also provides methods for Parquet files, external relational databases via JDBC, Hive tables, etc.

A DataFrame can also be created **from a Python list or an RDD of tuples** with `spark.createDataFrame(data, schema)` (schema = list of column names; if omitted, columns are named `_1, _2, …, _n`):

```python
profilesList = [(19, "Justin"), (30, "Andy"), (None, "Michael")]
df = spark.createDataFrame(profilesList, ["age", "name"])
```

Going back: the `rdd` member of the DataFrame class returns an **RDD of `Row` objects**. Each `Row` is like a dictionary (column names as keys):

- fields accessed like attributes (`row.name`) or dictionary values (`row["name"]`);
- `for key in row` iterates over the keys; `row.asDict()` returns the content as a Python dictionary.

```python
rddRows  = df.rdd
rddNames = rddRows.map(lambda row: row.name)   # extract one column as an RDD
rddNames.saveAsTextFile(outputPath)
```

Inspect with `df.show()`.

## DataFrame API Catalog (slides 15)

The slide deck walks through the DataFrame methods on toy `persons.csv` files (`name,age` / `name,age,gender`). Key facts per method:

- `show(n)` prints the first `n` rows (default `20`); `printSchema()` prints the schema; `count()` returns the number of rows; `distinct()` returns unique **rows** — *always a heavy operation* (shuffle).
- `select(col1, .., coln)` projects columns (`"*"` selects all); runtime errors possible on wrong column names.
- `selectExpr(expr1, .., exprN)` is the SQL-expression variant: `df.selectExpr("name", "age + 1 AS newAge")` — without the `AS`, the new column would be named `"age+1"`.
- `filter(conditionExpr)` keeps rows satisfying a **Boolean SQL expression** (`df.filter("age>=20 and age<=31")`); the string form cannot be checked at compile time. `where` is an **alias** of `filter`.
- `join(right, on, how)`: `on` can be a string (join column), a list of strings, or a condition/expression (e.g. `df.join(df2, df.name == df2.name)`); `how` ∈ `inner, cross, outer, full, full_outer, left, left_outer, right, right_outer, left_semi, left_anti` (default `inner`). The **left anti join** keeps the rows of the left DataFrame with no match on the right (slides' example: profiles of *non-banned* users = `dfPersons.join(dfBannedUsers, dfPersons.uid == dfBannedUsers.uid, "left_anti")`).
- Aggregations: `df.agg({"age": "avg"})` returns a one-row DataFrame whose column is named `"function_name(column)"` (`avg(age)`); `df.groupBy("name").avg("age")` groups and aggregates; multiple aggregates per group with `df.groupBy("name").agg({"age": "avg", "name": "count"})`. Aggregate functions include `avg`, `count`, `sum`, `abs`, …
- `sort(col1, .., coln, ascending=True)` sorts; the Lab 8 solutions use the column-method form `sort(selectedDF.criticality.desc(), …)`.
- Set transformations exist on DataFrames too: `df1.union(df2)`, `df1.intersect(df2)`, `df1.subtract(df2)`.
- `explain()` prints on the standard output the **execution plan** of the code computing the DataFrame.

**Broadcast joins**: Spark SQL automatically implements a broadcast version of the join if one of the two input DataFrames is small enough to fit in the main memory of each executor; you can suggest/force it by wrapping the small DataFrame with `broadcast(...)`:

```python
dfPersonLikesBroadcast = dfUidSports.join(broadcast(dfPersons), dfPersons.uid == dfUidSports.uid)
```

Storing DataFrames (two approaches, slides 15): convert to RDDs with `.rdd` and `saveAsTextFile(outputFolder)`, or use `write` (a `DataFrameWriter`): `df.write.csv(outputPath, header=True)`.

## Lab 8, Version (i): DataFrame API Solution

Goal (identical to Lab 7): criticality of each (station, weekday, hour) pair = fraction of readings with `free_slots = 0`, keep rows above a threshold, join station coordinates, sort, store.

```python
# 1. remove dirty readings (SQL string predicate)
filteredDF = registerDF.filter("not(used_slots = 0 and free_slots = 0)")

# 2. register a UDF usable in expressions and SQL
spark.udf.register("get_criticality",
                   lambda free_slots: 1 if free_slots == 0 else 0,
                   pyspark.sql.types.IntegerType())

# 3. derive weekday and hour with SQL date functions, mark critical readings
selectedDF = filteredDF.selectExpr("station",
                                   "date_format(timestamp, 'EE') as weekday",
                                   "hour(timestamp) as hour",
                                   "get_criticality(free_slots) as critical_slot")

# 4. criticality = average of the 0/1 indicator per (station, weekday, hour)
resultDF = selectedDF.groupBy("station", "weekday", "hour").agg({"critical_slot": "avg"})

# 5. apply the minimum criticality threshold
thresholdedDF = resultDF.filter("avg(critical_slot) > 0.3")

# 6. join with stations to retrieve the coordinates
joinedDF = thresholdedDF.join(stationsDF, thresholdedDF.station == stationsDF.id) \
                        .withColumnRenamed("avg(critical_slot)", "criticality")

# 7. project only the requested attributes
selectedDF = joinedDF.select("station", "weekday", "hour",
                             "criticality", "latitude", "longitude")

# 8. sort: criticality desc, then station, weekday, hour ascending
sortedDF = selectedDF.sort(selectedDF.criticality.desc(),
                           selectedDF.station.asc(),
                           selectedDF.weekday.asc(),
                           selectedDF.hour.asc())

# 9. store as CSV with header
sortedDF.write.csv('result', header=True, sep='\t')
```

Notes on the pieces (all from the Lab 8 handout/notebook):

- `hour(timestamp)` and `date_format(timestamp,'EE')` are **predefined SQL functions** of Spark SQL (like `hour(Timestamp)`, `abs(Integer)`, …), usable in SQL queries and in `selectExpr` — the handout shows exactly `myDataframe.selectExpr("date_format(timestamp,'EE') as weekday", "hour(timestamp) as hour")`.
- The 0/1 indicator trick: the *average* of an indicator column **is** the ratio "critical readings / total readings" — no separate counting needed.
- `filter` accepts SQL predicate strings (`"not(used_slots = 0 and free_slots = 0)"`, `"avg(critical_slot) > 0.3"`).
- `join(df2, condition)` with an explicit equality condition (`thresholdedDF.station == stationsDF.id`) resolves duplicate column names from the two sources.

## Lab 8, Version (ii): SQL Query Solution

Register temporary views and query them:

```python
registerDF.createOrReplaceTempView("register")
stationsDF.createOrReplaceTempView("stations")

selectedDF = spark.sql("SELECT station, "
                       "date_format(timestamp, 'EE') as weekday, "
                       "hour(timestamp) as hour, "
                       "AVG(get_criticality(free_slots)) as criticality "
                       "FROM register "
                       "WHERE NOT(used_slots = 0 AND free_slots = 0) "
                       "GROUP BY station, weekday, hour "
                       "HAVING criticality > 0.3")
selectedDF.show()
selectedDF.createOrReplaceTempView("selected")

joinedDF = spark.sql("SELECT station, weekday, hour, criticality, latitude, longitude "
                     "FROM selected, stations "
                     "WHERE station = id "
                     "ORDER BY criticality DESC, station ASC, weekday ASC, hour ASC")
joinedDF.show()
```

Mapping concepts: `WHERE` = filter, `GROUP BY` = groupBy, `HAVING` = filter **after** aggregation, old-style join `FROM t1, t2 WHERE key = key2`, `ORDER BY … DESC/ASC` = the multi-key sort required by the lab. Note that the UDF `get_criticality` was registered once with `spark.udf.register` and is usable inside the SQL text. Lab 7's notebook contains the same pattern in compact form, including a UDF that formats a timestamp into `"Wed 15"`-style strings:

```python
spark.udf.register("getTimestamp", lambda x: f"{x.strftime('%a')} {x.hour}", "string")
```

## Lab 9: Spark SQL as Input to Machine Learning

Task: binary classification of Amazon reviews as **useful/useless**. Label rule (handout): the helpfulness index is `HelpfulnessNumerator/HelpfulnessDenominator`, computable only for reviews with `HelpfulnessDenominator > 0`; a review is "useful" if the index is above 0.9. Label encoding: `1.0` = useful, `0.0` = useless. First feature set: a single double = **length of `Text`**; later steps add 3–4 more features and finally word-based features from `Text`.

The provided template (`labs/lab-9/lab9-bigdata-template/lab9-template/lab9-template.ipynb`) fixes the **evaluation** part; the student fills preprocessing and pipeline. Imports in the template show the intended toolbox:

```python
from pyspark.ml import Pipeline
from pyspark.sql.types import IntegerType, FloatType
from pyspark.ml.feature import VectorAssembler, StringIndexer, IndexToString, SQLTransformer
from pyspark.ml.classification import *
from pyspark.ml.evaluation import MulticlassClassificationEvaluator
```

### Preprocessing shape (per the handout)

1. Filter out never-rated reviews (`HelpfulnessDenominator = 0`).
2. Build a DataFrame with two fields: `label` (target, 1.0/0.0) and `features` (vector of doubles), "by means of a set of Transformers and/or Estimators (e.g., SQLTransformer, VectorAssembler)".
3. Split into training and test sets; train the pipeline on the first, test on the second.

A minimal pipeline consistent with the template's imports and the handout's first feature (single value: length of `Text`):

```python
# select rated reviews and compute the label and the numeric features
featuresDF = spark.sql("SELECT Id, Text, "
                       "CASE WHEN HelpfulnessDenominator = 0 THEN NULL "
                       "     WHEN HelpfulnessNumerator/HelpfulnessDenominator > 0.9 THEN 1.0 "
                       "     ELSE 0.0 END AS label, "
                       "CAST(length(Text) AS DOUBLE) AS textLength "
                       "FROM reviews WHERE HelpfulnessDenominator > 0")

# assemble the features column (here: one single predictive attribute)
assembler = VectorAssembler(inputCols=["textLength"], outputCol="features")

# classifier (create two versions: Decision Tree and Logistic Regression)
classifier = DecisionTreeClassifier(labelCol="label", featuresCol="features")
# classifier = LogisticRegression(labelCol="label", featuresCol="features")

pipeline = Pipeline(stages=[assembler, classifier])

# train and apply
(reviews_train, reviews_test) = featuresDF.randomSplit([0.8, 0.2], seed=42)
model = pipeline.fit(reviews_train)
predictions = model.transform(reviews_test).cache()
```

(The exact split fractions/seed and the additional features of steps 3–4 are left to the student; the lab asks to compare the quality of the Decision Tree and Logistic Regression versions and to improve the feature set.)

### Evaluation code from the template (verbatim)

```python
evaluatorAcc = MulticlassClassificationEvaluator(labelCol="label", predictionCol="prediction", metricName="accuracy")
evaluatorF1 = MulticlassClassificationEvaluator(labelCol="label", predictionCol="prediction", metricName="f1")
evaluatorRecall = MulticlassClassificationEvaluator(labelCol="label", predictionCol="prediction", metricName="weightedRecall")
evaluatorPrecision = MulticlassClassificationEvaluator(labelCol="label", predictionCol="prediction", metricName="weightedPrecision")

print("Accuracy:", evaluatorAcc.evaluate(predictions))
print("F1:", evaluatorF1.evaluate(predictions))
print("Weighted Recall:", evaluatorRecall.evaluate(predictions))
print("Weighted Precision:", evaluatorPrecision.evaluate(predictions))
```

Confusion matrix and per-class precision/recall, also from the template:

```python
A = predictions.filter("prediction=1 and label=1").count()
B = predictions.filter("prediction=0 and label=1").count()
C = predictions.filter("prediction=1 and label=0").count()
D = predictions.filter("prediction=0 and label=0").count()

print("Precision(Useful):", A/(A+C))
print("Recall(Useful):", A/(A+B))
print("Precision(Useless):", D/(B+D))
print("Recall(Useless):", D/(C+D))
```

## Relational Operators under the Hood (slides 08)

The "SQL operators" slide deck connects the SQL/relational-algebra background to distributed execution — the same operators that DataFrames/SQL expose are implementable as MapReduce (or Spark) jobs:

- The useful operators: **selection σ**, **projection π**, **union/intersection/difference**, **join** (Join design patterns), **aggregation/Group by** (Summarization pattern).
- Relations/tables (also the big ones) can be stored in **HDFS**: broken into blocks and spread across the servers.
- The MapReduce implementation is efficient **only when a full scan of the input table(s) is needed** (non-selective queries processing all data). *Selective* queries, returning few tuples, are usually **not** efficient with a MapReduce approach.
- Remember: in relational algebra, relations **do not contain duplicate records by definition** — the constraint must hold for both input and output tables.

Per-operator implementation sketches from the slides:

| Operator | MapReduce implementation |
|---|---|
| Selection σC(R) | **Map-only** (filtering pattern): each mapper reads one record; if C holds, emits `(record, null)`, else discards it. Example: `σSemester=2(Courses)`. |
| Projection πS(R) | Mappers build the new record r′ (attributes in S) and emit `(r′, null)`; **reducers** de-duplicate by emitting one pair per distinct key. Example: `πPSurname(Professors)` returns each surname once. |
| Union R ∪ S | Mappers (one class per relation) emit `(t, null)` for every record of R and S; reducers emit one copy per distinct key (duplicates removed). |
| Intersection R ∩ S | Mappers emit `(t, "R")` / `(t, "S")`; reducers emit `(t, null)` **only for keys whose value list contains two values** (present in both). |
| Difference R − S | Same tagging; reducers emit `(t, null)` **only for keys whose list contains only "R"** (in R, not in S). |
| Join | The Join pattern — **reduce-side or map-side** depending on the size of the input relations. |
| Aggregation / Group by | The Summarization pattern. |

This is why the Lab 8 queries (`WHERE`, `GROUP BY`, joins, `DISTINCT`-like semantics) translate so directly to `filter`/`groupBy`/`join`/`agg` calls: each SQL clause has a corresponding distributed pattern.

## RDD vs DataFrame Styles, Same Problem

| Aspect | RDD API (Lab 7) | DataFrame/SQL API (Lab 8) |
|---|---|---|
| Reading | `sc.textFile(...)` + `split("\t")` by hand, header filtered with `"station" not in x` | `spark.read.load(..., header=True, inferSchema=True, sep='\t')` |
| Dirty-line removal | `filter(lambda x: "\t0\t0" not in x)` | `filter("not(used_slots = 0 and free_slots = 0)")` |
| Timeslot | `datetime.strptime(...).strftime("%a")` + `.hour` | `date_format(timestamp,'EE')`, `hour(timestamp)` |
| Ratio per key | `reduceByKey` sums then divide, or average of indicator | `groupBy(...).agg({"critical_slot": "avg"})` / `AVG(...)` with `GROUP BY` |
| Enrichment | broadcast dict of coordinates / second RDD join | `join(stationsDF, cond)` or SQL join |
| Ordering | `sortBy(..., ascending=False)` | `sort(col.desc(), ...)` / `ORDER BY` |
| Output | `saveAsTextFile` (one file per partition, `coalesce(1)` for one file) | `write.csv('result', header=True, sep='\t')` |

## Key Takeaways

- One problem, two idioms — practice both: `filter/selectExpr/groupBy/agg/join/sort` and `createOrReplaceTempView` + `spark.sql("SELECT … WHERE … GROUP BY … HAVING … ORDER BY …")`.
- `selectExpr` mixes DataFrame API and SQL expressions (e.g. `hour(timestamp)`, `date_format`).
- UDFs bridge to Python: `spark.udf.register(name, lambda, returnType)`, then use `name(col)` in expressions and SQL text.
- Aggregation trick: `AVG(0/1 indicator)` = fraction of qualifying rows per group.
- The catalog method: `select`/`selectExpr` (with `AS` aliases), `filter`/`where` with SQL strings, `join(right, on, how)` with 11 join types (use `left_anti` for "rows of A not in B"), `groupBy().agg({...})` (result columns named `function(column)`), set ops `union`/`intersect`/`subtract`, `explain()` for the execution plan, `broadcast(df)` to force broadcast joins.
- SQL operators map to known distributed patterns: selection = map-only filter, projection = map + dedup reducer, union/intersection/difference = tag-and-reduce, join = reduce-/map-side per relation size; MapReduce/Spark is efficient only for full scans, not for selective queries.
- Reading options matter: `header=True` (and filter it when reading as plain text), `inferSchema=True` (else everything is a string), `sep='\t'`/`delimiter`; JSON input must be JSON-Lines (one object per line) unless `multiLine=True`.
- Lab 9 pattern: SQL preprocessing → `label` + `features` (VectorAssembler) → `Pipeline` → fit/transform → `MulticlassClassificationEvaluator` metrics + confusion matrix.
