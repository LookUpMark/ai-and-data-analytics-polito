---
title: Data Mining Process and Preprocessing
aliases: [KDD Process, Data Preprocessing, Similarity and Dissimilarity, Data Preparation]
tags: [computer-science/databases, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> The KDD process and everything that happens before the mining algorithm runs: attribute types (nominal, ordinal, interval, ratio) and dataset types (record, document, transaction, graph, ordered); data quality problems (noise, outliers, missing values, duplicates); and the preparation toolbox — aggregation, sampling, dimensionality reduction, feature selection/creation, discretization, binarization, normalization. Then the proximity measures used by the mining algorithms: Euclidean/Minkowski/Mahalanobis distances, SMC vs Jaccard for binary vectors, cosine similarity, and Pearson correlation with its pitfalls.

## The Data Mining Process (KDD)

> [!definition] Data mining
> Non-trivial extraction of **implicit**, **previously unknown**, **potentially useful** information from available data; extraction is automatic (appropriate algorithms) and the result is represented as abstract models (**patterns**).

```
data → [selection] → target data → [preprocessing] → preprocessed data
     → [transformation] → transformed data → [data mining] → patterns
     → [interpretation/evaluation] → knowledge          (KDD = Knowledge Discovery from Data)
```

- **Descriptive methods** (e.g. clustering): extract interpretable models describing data — example: client segmentation.
- **Predictive methods** (e.g. classification, regression): exploit known variables to predict unknown/future values — example: "spam" email detection.

## Data Types and Properties

- A **data object** (record, point, case, sample, entity, instance) is described by a collection of **attributes** (variable, field, characteristic, dimension, feature). *Attribute values* are the numbers/symbols assigned; the same attribute can map to different values (height in feet or meters) and different attributes can share the same value set (ID and age are both integers — but with different properties).

| Attribute type | Properties | Examples |
|---|---|---|
| **Nominal** | Distinctness (=, ≠) | ID numbers, eye color, zip codes |
| **Ordinal** | Distinctness + order (<, >) | Rankings, grades, height in {tall, medium, short} |
| **Interval** | + addition (+, −) | Calendar dates |
| **Ratio** | + multiplication (×, ÷) | Temperature in Kelvin, length, time, counts |

- **Discrete**: finite or countably infinite values (zip codes, counts; binary is a special case).
- **Continuous**: real numbers (temperature, height, weight) — in practice represented with finite precision.
- Key message: operations should be **meaningful for the data type**; the type you *see* (numbers/strings) may not capture the real properties (an ID is numeric but nominal), and what is meaningful is ultimately decided by the **domain**.

**Dataset types**:

- **Record data**: tabular (fixed set of attributes); **document data** (each document = term vector of occurrence counts — sparse); **transaction data** (each record = a set of items, e.g. market basket).
- **Graph data**: generic graphs, molecules, web pages.
- **Ordered data**: sequences of transactions, genomic sequences, spatio-temporal data (time series).

## Data Quality

Poor data quality negatively affects data processing ("poor data quality costs the typical company at least 10% of revenue" — Redman 2004). Example: a loan-risk classification model built on poor data denies loans to credit-worthy candidates and grants loans to defaulters.

- **Noise**: modification of original values (distortion of a voice on a bad phone line, "snow" on TV).
- **Outliers**: objects considerably different from most data. Case 1: outliers are noise interfering with analysis; Case 2: outliers **are** the goal (credit-card fraud, intrusion detection).
- **Missing values**: information not collected (people decline to give age/weight) or attributes not applicable (annual income for children). Handling: eliminate objects/variables, estimate the missing values (e.g. time series of temperature), ignore during analysis.
- **Duplicate data**: major issue when merging heterogeneous sources (Street vs St.); data cleaning deals with it — but sometimes duplicates should *not* be removed.

## Data Preparation

### Aggregation

Combining two or more attributes (or objects) into one. Purposes: **data reduction** (fewer attributes/objects), **change of scale** (cities → regions → states), **more stable data** (aggregated data has less variability — e.g. average yearly precipitation has lower standard deviation than average monthly precipitation, Australia 1982–1993 example).

### Sampling

Main technique for **data selection**: used when processing the entire dataset is too expensive or time consuming. Key principle: *a sample works almost as well as the entire data set if it is **representative*** (has approximately the same property of interest as the original data).

- **Simple random sampling** — equal probability of selecting any item; **without replacement** (each selected item removed) or **with replacement** (same object can be picked more than once).
- **Stratified sampling** — split data into partitions, then draw random samples from each partition.

Sample size matters: 8000 vs 2000 vs 500 points of the same 2-D dataset show increasingly degraded structure.

### Dimensionality Reduction

- **Curse of dimensionality**: as dimensionality increases, data becomes increasingly sparse; definitions of **density and distance** (critical for clustering and outlier detection) become less meaningful (randomly generated points: the difference between max and min pair distance shrinks toward 0).
- Purposes: avoid the curse, reduce time/memory, enable visualization, eliminate irrelevant features/reduce noise.
- Techniques: **PCA** (Principal Component Analysis), **SVD** (Singular Value Decomposition), supervised and non-linear techniques. Goal: find a projection capturing the largest amount of variation in data.

### Feature Subset Selection

- **Redundant features**: duplicate much of the information of other attributes (purchase price vs sales tax paid).
- **Irrelevant features**: no information for the task (student ID for predicting GPA).
- Techniques: brute force (all subsets — impractical), **embedded** (selection occurs naturally in the algorithm, e.g. random forests), **filter** (select before mining), **wrapper** (use the mining algorithm as a black box to score subsets).

### Feature Creation

Create new attributes capturing the important information more efficiently: **feature extraction** (domain-specific), **mapping data to a new space** (Fourier/wavelet transform — e.g. separating two sine waves from noise in frequency space), **feature construction** (combining features: max, min, avg).

### Discretization

Converting a continuous attribute into an ordinal one: a potentially infinite number of values is mapped to few categories; commonly used in classification (many algorithms work best with few values). Can be **unsupervised** (find breaks in the data values) or **supervised** (use class labels to find breaks — e.g. Iris petal length histogram separates the three classes).

Unsupervised techniques:

1. **N equal-width intervals**: `W = (vmax − vmin)/N` — easy, incremental, but badly affected by outliers and sparse data;
2. **N equal-frequency intervals** (percentiles/quartiles): better fits sparse data and outliers, non incremental;
3. **Clustering** (e.g. k-means on the attribute): fits sparse data and outliers well.

Iris dataset example (UCI Repository, Fisher): 3 classes (Setosa, Versicolour, Virginica), 4 attributes (sepal/petal width and length); discretized petal width/length give the rules "low → Setosa, medium → Versicolour, high → Virginica".

### Binarization and Attribute Transformation

- **Binarization**: map an attribute to one or more binary variables; continuous attributes are first discretized ({low, medium, high}); categorical values use **one-hot encoding** (low/medium/high → 100, 010, 001 — only 1 bit takes value 1).
- **Transformation**: simple functions (x^k, log(x), e^x, |x|); **normalization** scales values into a small range ([0,1] or [−1,1]) — *min-max*, *z-score*, *decimal scaling*; **standardization** in statistics = subtract the mean and divide by the standard deviation (removes unwanted common signals such as seasonality — e.g. Minneapolis/Atlanta/São Paulo Net Primary Production time series become uncorrelated after monthly z-score).

```
min-max:      v' = (v − vmin)/(vmax − vmin) · (newmax − newmin) + newmin
z-score:      v' = (v − mean)/std_dev
decimal scal: v' = v / 10^j     (j smallest integer s.t. max(|v'|) < 1)
```

## Similarity and Dissimilarity

- **Similarity**: numerical measure of how alike two objects are; higher when more alike; often in [0,1].
- **Dissimilarity**: how different; lower when more alike; minimum often 0. *Proximity* = either one.

### Distances

**Euclidean distance** (n = number of dimensions, xk/yk the k-th components):

```
d(x, y) = sqrt( Σ_k (xk − yk)² )
```

Standardization is necessary if scales differ. **Minkowski distance** generalizes it with parameter r:

| r | Name |
|---|---|
| 1 | City block / Manhattan / taxicab (**L1**); Hamming distance = number of differing bits between binary vectors |
| 2 | Euclidean (**L2**) |
| ∞ | Supremum (**Lmax/L∞**): maximum difference between any component |

**Properties of a distance** (a distance satisfying all three is a **metric**): positive definiteness (d(x,y) ≥ 0, = 0 iff x = y), symmetry, triangle inequality. **Properties of a similarity**: s(x,y) = 1 (maximum) only if x = y; symmetry.

**Mahalanobis distance** — `mahalanobis(x,y) = (x−y)ᵀ Σ⁻¹ (x−y)` with Σ the covariance matrix: takes into account the position of the two objects with respect to the other points in the collection (for the red points in the slide example, Euclidean distance is 14.7 but Mahalanobis is 6; with Σ = [[0.3,0.2],[0.2,0.3]], Mahal(A,B) = 5 > Mahal(A,C) = 4).

### Binary vectors: SMC and Jaccard

Given M01, M10 (attributes where the two vectors differ), M00, M11 (matches):

```
SMC = (M11 + M00) / (M01 + M10 + M11 + M00)     -- simple matching
J   = M11 / (M01 + M10 + M11)                    -- Jaccard
```

Example: p = 1000000000, q = 0000000101 → M01=2, M10=1, M00=7, M11=0 → SMC = 7/10 = 0.7 (misleading for sparse data!) but **J = 0** — Jaccard is the accurate evaluation for large sparse data.

### Cosine similarity

For document vectors d1, d2: `cos(d1,d2) = (d1·d2)/(‖d1‖·‖d2‖)`. Example: d1 = 3 2 0 5 0 0 0 2 0 0, d2 = 1 0 0 0 0 0 0 1 0 2 → dot = 5, ‖d1‖ = 6.481, ‖d2‖ = 2.245 → cos = 0.3150. cos = 1 ⇒ identical documents.

**Combining similarities** for heterogeneous attributes: compute sk(x,y) ∈ [0,1] per attribute; define δk = 0 for asymmetric attributes where both are 0 or one value is missing, δk = 1 otherwise; overall similarity = Σ δk·sk / Σ δk; optionally with non-negative weights ωk (also for distances).

## Correlation

**Pearson's correlation** measures the **linear** relationship between two data objects (binary or continuous); useful in data exploration. Correlated features should be removed (simpler analytics, better algorithm performance).

- Scatter plots show similarity from −1 to +1; perfect linear correlation when value is ±1; positive: one variable increases, the other increases; negative: one increases, the other decreases.
- **Drawback**: x = (−3,…,3), y = x² = (9,4,1,0,1,4,9) has corr = 0 — a perfect *non-linear* relationship yields zero correlation.
