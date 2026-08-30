---
title: Spark DataFrames and SQL
aliases: [SparkSession, Spark SQL, DataFrames API, spark.read, createOrReplaceTempView, UDF, Spark ML pipelines]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> The Spark SQL layer of BDPA as used in Labs 7–9: reading CSV into DataFrames with `spark.read.load`, two equivalent styles (DataFrame API vs `spark.sql` queries), UDFs, date functions `date_format`/`hour`, joins, sorting, writing CSV output, and the Lab 9 `spark.ml` pipeline (VectorAssembler, SQLTransformer, Decision Tree vs Logistic Regression) with the template's evaluation code.

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

Inspect with `df.show()`.

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

- `hour(timestamp)` and `date_format(timestamp,'EE')` are **predefined SQL functions** of Spark SQL, usable in SQL queries and in `selectExpr` — the handout shows exactly `myDataframe.selectExpr("date_format(timestamp,'EE') as weekday", "hour(timestamp) as hour")`.
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
- Reading options matter: `header=True` (and filter it when reading as plain text), `inferSchema=True`, `sep='\t'`/`delimiter`.
- Lab 9 pattern: SQL preprocessing → `label` + `features` (VectorAssembler) → `Pipeline` → fit/transform → `MulticlassClassificationEvaluator` metrics + confusion matrix.
