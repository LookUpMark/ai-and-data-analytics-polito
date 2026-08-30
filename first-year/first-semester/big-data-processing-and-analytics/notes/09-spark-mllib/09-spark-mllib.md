---
title: Spark MLlib
aliases: [Spark MLlib, pyspark.ml, ML pipeline, Transformer, Estimator, VectorAssembler, StringIndexer, TF-IDF, logistic regression Spark, decision tree Spark, CrossValidator, MulticlassClassificationEvaluator]
tags: [computer-science/big-data, note/course-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---

> [!summary] **Document Summary**
> The machine learning layer of BDPA (slides 16a–16b), on the DataFrame-based `pyspark.ml` API: the two MLlib packages, the basic data types (dense/sparse vectors of doubles, `label`/`features` tables), the pipeline model (Transformer, Estimator, Pipeline, parameters), feature engineering (VectorAssembler, scalers, StringIndexer/IndexToString, SQLTransformer, text features with Tokenizer/StopWordsRemover/HashingTF/IDF), classification with logistic regression and decision trees (numeric labels, categorical labels, and text documents, each as a full worked pipeline), model evaluation (`randomSplit`, `MulticlassClassificationEvaluator` metrics, overfitting check) and hyperparameter tuning with `ParamGridBuilder` + `CrossValidator`. The Lab 9 variant of the same workflow is in note 06.

## Overview and Packages (slides 16a)

Spark MLlib is the Spark component providing the machine learning / data mining algorithms:

- Pre-processing techniques;
- Classification (supervised learning);
- Clustering (unsupervised learning);
- Itemset mining.

The MLlib APIs are divided into **two packages**:

| Package | Built on | Status |
|---|---|---|
| `pyspark.mllib` | RDDs (the original APIs) | In **maintenance mode**, probably deprecated in future Spark releases — no longer used in the course |
| `pyspark.ml` | **DataFrames** (higher-level API) | The DataFrame-based version used in this course |

## Basic Data Types: Vectors and Tables (slides 16a)

MLlib is based on a set of basic local and distributed data types (local vector, local matrix, distributed matrix, …), and it uses **DataFrames as input data**:

- The input of the MLlib algorithms is structured data (tables): **all input data must be represented as tables** before applying the algorithms — including document collections, which must be transformed into a tabular format.
- The DataFrames used and created by MLlib algorithms have columns with standard **roles**:
  - `label` — target of a classification/regression analysis;
  - `features` — a vector containing the values of the predictive attributes of the input record;
  - `text` — the original text of a document before being transformed in tabular format;
  - `prediction` — predicted value of a classification/regression analysis.

> [!definition] Local vector (MLlib)
> A `pyspark.ml.linalg.Vector` stores a vector of **double** values; dense and sparse representations are supported. The MLlib algorithms work **only on vectors of doubles**: one vector per input record, and non-double attributes/values must be mapped to doubles first.

The vector `[1.0, 0.0, 3.0]` is dense as `[1.0, 0.0, 3.0]` and sparse as `(3, [0, 2], [1.0, 3.0])` (size, indexes of non-zero cells, their values). Creation:

```python
from pyspark.ml.linalg import Vectors

dv = Vectors.dense([1.0, 0.0, 3.0])

# Sparse version: size + dictionary of index:value pairs of non-zero entries
sv = Vectors.sparse(3, {0: 1.0, 2: 3.0})
```

## The Pipeline Model: Transformers, Estimators, Pipelines (slides 16a)

> [!definition] Transformer
> An ML algorithm/procedure that transforms one DataFrame into another DataFrame by means of the method `transform(inputDataFrame)`. Examples: a **feature transformer** that reads a column (e.g. `text`), maps it into a new column (e.g. feature vectors) and outputs a new DataFrame with the mapped column appended; a trained **classification model** that adds the `prediction` column (pre-trained models are Transformers).

> [!definition] Estimator
> An ML algorithm/procedure that is **fit** on an input (training) DataFrame to produce a Transformer. Each Estimator implements the method `fit(DataFrame)`, which produces a **Model** (a Transformer). Example: the Logistic Regression *algorithm* is an Estimator; calling `fit(trainingDF)` builds a Logistic Regression *Model*, which is a Transformer.

> [!definition] Pipeline
> A chain of multiple Transformers and Estimators specifying a machine learning / data mining **workflow**: the output of each stage is the input of the next one. Example workflow for text classification: split each document into words → convert the words into a numerical feature vector → learn a prediction model from the feature vectors and the class labels.

Transformers and Estimators also share common APIs for specifying the values of their **parameters**. In the new Spark MLlib APIs, the pipeline approach is the recommended one, based on four steps:

1. Instantiate the needed Transformers and Estimators;
2. Create a `Pipeline` object and specify the sequence of stages;
3. Execute the pipeline (`fit`) and train a model;
4. (optional) Apply the model (`transform`) on new data.

## Feature Engineering (slides 16a)

Input data must be preprocessed before applying ML algorithms: to organize data in the format expected by the algorithms, to define good (predictive) features, to remove bias (e.g. normalization), to remove noise and missing values.

MLlib provides transformers organized in three groups (up-to-date list: `https://spark.apache.org/docs/latest/ml-features.html`):

- **Feature extractors**: TF-IDF, Word2Vec, …
- **Feature transformers**: Tokenizer, StopWordsRemover, StringIndexer, IndexToString, OneHotEncoderEstimator, Normalizer, …
- **Feature selectors**: VectorSlicer, …

### VectorAssembler

`pyspark.ml.feature.VectorAssembler` combines a list of columns into a **single vector column** — the standard way to build the `features` column.

- `VectorAssembler(inputCols, outputCol)`; accepted input column types: **all numeric types, boolean type, and vector type** (booleans are mapped to 1/0).
- `transform(df)` returns a DataFrame with the new column **plus all the columns of the input**.

```python
from pyspark.ml.feature import VectorAssembler

inputDF = spark.read.load(inputPath, format="csv", header=True, inferSchema=True)

myVectorAssembler = VectorAssembler(inputCols=['colB', 'colC'], outputCol='features')
transformedDF = myVectorAssembler.transform(inputDF)
```

### Scalers (normalization)

MLlib provides normalization algorithms called scalers: `StandardScaler`, `MinMaxScaler`, `Normalizer`, `MaxAbsScaler`.

`StandardScaler(inputCol, outputCol)` is an **Estimator**: `fit(df)` infers a `StandardScalerModel` (Transformer) that normalizes each "feature" of the input vector column to have **unit standard deviation and/or zero mean**:

```python
from pyspark.ml.feature import StandardScaler

myScaler = StandardScaler(inputCol="features", outputCol="scaledFeatures")
# fit computes mean and standard deviation of the analyzed data first
scalerModel = myScaler.fit(transformedDF)
scaledDF = scalerModel.transform(transformedDF)
```

Slide example: `features = [4.5, 1.0]` becomes `scaledFeatures = [0.903, 2.236]` — each cell is divided by the standard deviation of its column.

### StringIndexer and IndexToString (categorical ↔ numerical)

The classification/regression algorithms work only with numerical values, while real data often has categorical (string) columns — including the class label itself.

- `StringIndexer(inputCol, outputCol)` is an **Estimator**; its `StringIndexerModel` encodes a string column of "labels" to a column of "label indices": each distinct value is mapped to an integer in `[0, num. distinct values)`, casted to a double.
- `IndexToString(inputCol, outputCol, labels)` is the symmetrical **Transformer**: it maps label indices back to the original strings (the mapping is given by the positions inside the `labels` list). Needed because classification models return the *integer* version of predicted labels, and human-readable results require the original ones.

```python
df = spark.createDataFrame([(1, "a"), (2, "b"), (3, "c"), (4, "c"), (5, "a")], ["id", "category"])

indexer = StringIndexer(inputCol="category", outputCol="categoryIndex")
indexerModel = indexer.fit(df)          # analyze data: define the string -> integer mapping
indexedDF = indexerModel.transform(df)  # a->0.0, b->2.0, c->1.0 in the slide example

converter = IndexToString(inputCol="categoryIndex", outputCol="originalCategory",
                          labels=indexerModel.labels)
reconvertedDF = converter.transform(indexedDF)
```

### SQLTransformer

`SQLTransformer(statement)` is a Transformer implementing transformations defined by **SQL queries**: `SELECT attributes, function(attributes) FROM __THIS__ [WHERE …] [HAVING …]`, where `__THIS__` represents the DataFrame on which the transformer is invoked.

```python
from pyspark.ml.feature import SQLTransformer
from pyspark.sql.types import IntegerType

spark.udf.register("countWords", lambda text: len(text.split(" ")), IntegerType())

sqlTrans = SQLTransformer(statement="""
    SELECT *, countWords(text) AS numWords FROM __THIS__""")
newDF = sqlTrans.transform(dfInput)
```

### Text features: Tokenizer, StopWordsRemover, HashingTF, IDF

For document collections the pipeline is: translate text into an attribute per word (a table), remove useless words (stopwords, e.g. conjunctions), and weight the remaining words:

- Words appearing in **almost all documents** do not characterize the data → not important for classification;
- Words appearing in **few documents** distinguish those documents (and their class) → very important;
- Traditionally a weight based on **TF-IDF** assigns a different importance to words based on their frequency in the collection.

The four components used in the slides:

| Component | Role |
|---|---|
| `Tokenizer` | Splits the input text (column `text`) into words, adding a new column (e.g. `words`) |
| `StopWordsRemover` | Removes stopwords from the words column, producing e.g. `filteredWords` |
| `HashingTF` | Maps the set of words to a fixed-length feature vector (column `rawFeatures`); based on a hash function, so two different words can collide — the number of conflicts is influenced by `numFeatures`; each cell holds the term frequency (TF component) |
| `IDF` | Recomputes the weights with the inverse document frequency; the result goes in the standard `features` column |

## Classification with MLlib (slides 16b)

Spark MLlib provides a (limited) set of classification algorithms:

- Logistic regression (binomial and multinomial);
- Decision tree classifier;
- Random forest classifier;
- Gradient-boosted tree classifier;
- Multilayer perceptron classifier;
- Linear Support Vector Machine.

All of them share the same shape:

1. **Model generation** based on a set of training data (records whose class label is known);
2. **Prediction** of the class label of new unlabeled data.

All Spark classification algorithms work **only on numerical attributes** (categorical values must be mapped to integers first) and are trained on a DataFrame with (at least) two columns:

- `label` — the class label to predict, an integer value casted to a double;
- `features` — a vector of doubles (`pyspark.ml.linalg.Vector`, dense or sparse) with the values of the predictive attributes.

Running example from the slides: predict whether customers are good (label `1`) or bad (`0`) from their monthly income and number of children. The raw training file has one record per line, first column = class label:

```
label,attr1,attr2,attr3
1.0,0.0,1.1,0.1
0.0,2.0,1.0,-1.0
0.0,2.0,1.3,1.0
1.0,0.0,1.2,-0.5
```

which must become a `label` + `features` DataFrame. The unlabeled file has the same format with an **empty first column** (label = `null` for all records); after applying the model, the returned DataFrame has the same columns plus `prediction` (and two probability-related columns `rawPrediction` and `probability`, where the i-th cell of `probability` is the probability of belonging to the i-th class).

### Example 1 — numeric labels, assembler + estimator by hand

```python
from pyspark.ml.feature import VectorAssembler
from pyspark.ml.classification import LogisticRegression

trainingData = spark.read.load("ex_data/trainingData.csv",
                               format="csv", header=True, inferSchema=True)

assembler = VectorAssembler(inputCols=["attr1", "attr2", "attr3"], outputCol="features")
trainingDataDF = assembler.transform(trainingData)

lr = LogisticRegression()
lr.setMaxIter(10)      # one set method per parameter
lr.setRegParam(0.01)

classificationModel = lr.fit(trainingDataDF)          # Estimator -> Model

unlabeledData = spark.read.load("ex_data/unlabeledData.csv",
                                format="csv", header=True, inferSchema=True)
unlabeledDataDF = assembler.transform(unlabeledData)  # same assembler!
predictionsDF = classificationModel.transform(unlabeledDataDF)

predictions = predictionsDF.select("attr1", "attr2", "attr3", "prediction")
predictions.write.csv("predictionsLR/", header="true")
```

The returned DataFrame schema (documented in the slides): `attr1, attr2, attr3, features (vector), label (double), rawPrediction (vector), probability (vector), prediction (double)`.

### Example 2 — the same problem as a Pipeline

The preprocessing steps must be applied on **both** training and unlabeled data; a pipeline specifies the common phases once:

```python
from pyspark.ml import Pipeline

pipeline = Pipeline().setStages([assembler, lr])
classificationModel = pipeline.fit(trainingData)
# prediction: the model is associated with the pipeline,
# hence also the assembler stage is executed
predictions = classificationModel.transform(unlabeledData)
```

The decision-tree variant is the same code with `DecisionTreeClassifier()` instead of `LogisticRegression()`, e.g. `dt.setImpurity("gini")` to set the measure deciding whether a node must be split.

### Example 3 — categorical class labels in a 4-stage pipeline

With a string label column (`categoricalLabel` with values `Positive`/`Negative`), the steps are: (1) `StringIndexer` to create the numerical `label` column, (2) `VectorAssembler` for `features`, (3) train the classifier (which uses **only** `label` and `features`), (4) `IndexToString` to convert numerical predictions back to the original labels:

```python
labelIndexer = StringIndexer(inputCol="categoricalLabel", outputCol="label",
                             handleInvalid="keep").fit(trainingData)
labelConverter = IndexToString(inputCol="prediction", outputCol="predictedLabel",
                               labels=labelIndexer.labels)

pipeline = Pipeline().setStages([assembler, labelIndexer, dt, labelConverter])
classificationModel = pipeline.fit(trainingData)
predictions = classificationModel.transform(unlabeledData)
# output schema now also contains predictedLabel (string, the original label)
predictions = predictions.select("attr1", "attr2", "attr3", "predictedLabel")
```

### Example 4 — text classification in a 5-stage pipeline

Input: `Label,Text` lines (slide example: Spark-related documents labeled `1`, "Turin is a beautiful city"-style documents labeled `0`). The pipeline is `tokenizer → remover → hashingTF → idf → lr`:

```python
tokenizer  = Tokenizer().setInputCol("text").setOutputCol("words")
remover    = StopWordsRemover().setInputCol("words").setOutputCol("filteredWords")
hashingTF  = HashingTF().setNumFeatures(1000).setInputCol("filteredWords").setOutputCol("rawFeatures")
idf        = IDF().setInputCol("rawFeatures").setOutputCol("features")
lr         = LogisticRegression().setMaxIter(10).setRegParam(0.01)

pipeline = Pipeline().setStages([tokenizer, remover, hashingTF, idf, lr])
classificationModel = pipeline.fit(trainingData)
predictionsDF = classificationModel.transform(unlabeledData)   # uses only 'features'
predictions = predictionsDF.select("text", "prediction")
```

Only `label` and `features` are considered by the classification algorithm; the intermediate columns (`words`, `filteredWords`, `rawFeatures`, `text`) remain in the DataFrame but are ignored.

## Evaluation (slides 16b)

To test the quality of the inferred models, split the labeled data with `randomSplit` and use the evaluators from `pyspark.ml.evaluator`:

- `BinaryClassificationEvaluator` — for binary problems;
- `MulticlassClassificationEvaluator` — for multiclass problems.

Provided metrics: **accuracy, precision, recall, F-measure**. `MulticlassClassificationEvaluator` parameters: `metricName` (`'accuracy'`, `'f1'`, `'weightedPrecision'`, `'weightedRecall'`), `labelCol` (true label column), `predictionCol` (predicted label column). The `evaluate(DataFrame)` method compares predictions with true labels and returns the metric as a double.

```python
trainDF, testDF = labeledDataDF.randomSplit([0.75, 0.25], seed=10)   # 75% / 25%

classificationModel = pipeline.fit(trainDF)
predictionsDF = classificationModel.transform(testDF)

myEvaluatorAcc = MulticlassClassificationEvaluator(labelCol="label",
                    predictionCol="prediction", metricName='accuracy')
# ... same for 'f1', 'weightedPrecision', 'weightedRecall'
print("Accuracy on test data ", myEvaluatorAcc.evaluate(predictionsDF))

# evaluate ALSO on the training data to check if the model is overfitted
predictionsTrainingDF = classificationModel.transform(trainDF)
print("Accuracy on training data ", myEvaluatorAcc.evaluate(predictionsTrainingDF))
```

## Hyperparameter Tuning with Cross-Validation (slides 16b)

Setting the parameters of an algorithm is difficult; a **brute-force** approach evaluates a grid of settings and keeps the one optimizing a quality index (e.g. prediction accuracy) to build the final model on the whole training dataset. One single split is usually **biased**, hence **cross-validation**: k splits and k models are created, and the setting achieving the best *average* result on the k models is selected.

Spark supports a grid-based search over a pipeline via `pyspark.ml.tuning`:

- Input: an MLlib pipeline, a set of values for each parameter (all combinations are generated and evaluated), and a quality metric (evaluator);
- Output: the model associated with the **best parameter setting**.

```python
from pyspark.ml.tuning import ParamGridBuilder, CrossValidator
from pyspark.ml.evaluation import BinaryClassificationEvaluator

pipeline = Pipeline().setStages([assembler, lr])

# 3 values for maxIter x 2 values for regParam = 6 configurations evaluated
paramGrid = ParamGridBuilder() \
    .addGrid(lr.maxIter, [10, 100, 1000]) \
    .addGrid(lr.regParam, [0.1, 0.01]) \
    .build()

cv = CrossValidator() \
    .setEstimator(pipeline) \
    .setEstimatorParamMaps(paramGrid) \
    .setEvaluator(BinaryClassificationEvaluator()) \
    .setNumFolds(3)

tunedLRmodel = cv.fit(labeledDataDF)      # best model from cross-validation
predictionsDF = tunedLRmodel.transform(unlabeledData)
```

Each `addGrid` call takes the parameter object and the list of values to test; `CrossValidator` wraps the pipeline as an Estimator and requires estimator + param maps + evaluator + number of folds.

## Key Takeaways

- Use `pyspark.ml` (DataFrame-based); `pyspark.mllib` is in maintenance mode.
- Everything is a table: MLlib consumes DataFrames with `label` (double) and `features` (`pyspark.ml.linalg.Vector` of doubles); non-double/categorical values must be encoded first (StringIndexer / IndexToString).
- Vocabulary: **Transformer** = `transform(DataFrame) -> DataFrame`; **Estimator** = `fit(DataFrame) -> Model(Transformer)`; **Pipeline** = chain of stages; parameters via setter methods.
- Feature toolbox: `VectorAssembler` (columns → vector), scalers (`StandardScaler`: unit std dev / zero mean), `SQLTransformer` (`__THIS__`), text chain `Tokenizer → StopWordsRemover → HashingTF → IDF` (TF-IDF weights, word collisions depend on `numFeatures`).
- One classification recipe, three flavors: numeric labels (assembler + classifier), categorical labels (add StringIndexer/IndexToString stages), text documents (5-stage pipeline). The same preprocessing must be applied to training and prediction data — put it in the pipeline.
- Evaluate with `randomSplit` + `MulticlassClassificationEvaluator` (`accuracy`, `f1`, `weightedPrecision`, `weightedRecall`); compare test vs training metrics to detect overfitting.
- Tune with `ParamGridBuilder` + `CrossValidator` (`setNumFolds(k)`): all grid combinations × k folds; `fit` returns the best model.
