---
title: Classification
aliases: [Decision Trees, Random Forest, Rule-Based Classification, k-Nearest Neighbors, Naive Bayes, SVM, Confusion Matrix, ROC Curve]
tags: [computer-science/databases, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> Supervised classification: predicting a class label for unlabelled records and building an interpretable model of a phenomenon. Covers decision tree induction in depth (Hunt's algorithm, test conditions per attribute type, Gini/entropy/gain ratio impurity measures with worked examples on the "Cheat" dataset, stopping criteria and pre/post-pruning, overfitting), then the other techniques of the course: random forests, rule-based and associative classification, k-NN, Naïve Bayes, SVM, and neural networks (FFNN, CNN, RNN/LSTM, autoencoders, word embeddings). Closes with model evaluation: partitioning techniques (holdout, cross-validation, bootstrap), confusion matrix, accuracy pitfalls, precision/recall/F-measure and ROC analysis.

## Classification: Definition

> [!definition] Classification
> Given a collection of **class labels** and a collection of data objects **labelled** with a class label, find a **descriptive profile of each class** which allows assigning unlabelled objects to the appropriate class.

- **Training set** — collection of labelled data objects used to *learn* the classification model.
- **Test set** — collection of labelled data objects used to *validate* the classification model.

Applications: customer propensity to leave a company (**churn**/attrition), fraud detection, classification of pathology types. Classification techniques covered: decision trees, classification rules, association rules, neural networks, Naïve Bayes and Bayesian networks, k-Nearest Neighbours (k-NN), Support Vector Machines (SVM).

### Evaluation criteria for classification techniques

| Criterion | Content |
|---|---|
| **Accuracy** | quality of the prediction |
| **Interpretability** | model interpretability, model compactness |
| **Incrementality** | model update in presence of newly labelled records |
| **Efficiency** | model building time, classification time |
| **Scalability** | training set size, attribute number |
| **Robustness** | noise, missing data |

## Decision Trees

Running example (training data, 10 records):

| Tid | Refund | Marital Status | Taxable Income | Cheat |
|---|---|---|---|---|
| 1 | Yes | Single | 125K | No |
| 2 | No | Married | 100K | No |
| 3 | No | Single | 70K | No |
| 4 | Yes | Married | 120K | No |
| 5 | No | Divorced | 95K | Yes |
| 6 | No | Married | 60K | No |
| 7 | Yes | Divorced | 220K | No |
| 8 | No | Single | 85K | Yes |
| 9 | No | Married | 75K | No |
| 10 | No | Single | 90K | Yes |

Model: a tree splitting first on `Refund` (Yes → NO), then `Marital Status` (Married → NO; Single/Divorced → `Taxable Income`), then `Taxable Income` (< 80K → NO, ≥ 80K → YES). **More than one tree** may fit the same data (e.g. a tree splitting first on `Marital Status`). Classification of a new record: start from the root and follow the branches (e.g. Refund=No, Married → Cheat = No).

### Hunt's Algorithm

Many induction algorithms exist: Hunt's (one of the earliest), CART, ID3, C4.5, C5.0, SLIQ, SPRINT. With Dt = set of training records reaching node t:

- If Dt contains records of **more than one class**: select the "best" attribute A on which to split, label node t as A, split Dt into smaller subsets and **recurse** on each subset.
- If all records in Dt belong to the **same class** yt: t is a **leaf** labelled yt (pure node).
- If Dt is **empty**: t is a leaf labelled with the **default (majority) class** yd.

Tree induction adopts a **greedy strategy** — the "best" attribute is selected locally at each step, *not* a global optimum. Issues: structure of the test condition, selection of the best attribute, stopping condition.

### Structure of test conditions

Depends on the **attribute type** (nominal, ordinal, continuous) and the **number of outgoing edges** (binary vs multi-way split):

- **Nominal** (e.g. CarType ∈ {Family, Luxury, Sports}): multi-way split uses as many partitions as distinct values; binary split divides values into two subsets (e.g. {Sports, Luxury} vs {Family}) — the optimal partitioning must be found.
- **Ordinal** (e.g. Size ∈ {Small, Medium, Large}): multi-way as above; binary split must respect the order — {Small, Medium} | {Large} or {Medium, Large} | {Small} are valid, {Small, Large} | {Medium} is not (grouping non-consecutive values).
- **Continuous** (e.g. Taxable Income): either **discretization** into an ordinal attribute — *static* (once at the beginning) or *dynamic* (during induction); ranges via equal interval bucketing, equal frequency (percentiles) or clustering — or a **binary decision** `(A < v) / (A ≥ v)` considering all possible splits and choosing the best cut (more computationally intensive).

> [!warning] Splitting on an identifier overfits
> Splitting on a key such as `Student ID` produces pure leaves (one record each) but the tree does not generalize — a classic **overfitting** situation. Attributes with homogeneous class distribution are preferred; a **node impurity** measure is needed to choose among them.

## Measures of Node Impurity

Three measures: **Gini index**, **entropy**, **misclassification error**. Different algorithms rely on different measures. A split is evaluated by the (weighted) impurity of the children: gain = M0 − M12 vs M0 − M34 for competing attributes A and B — choose the attribute that reduces the parent's impurity most.

### Gini index (used by CART, SLIQ, SPRINT)

```
GINI(t) = 1 − Σj [p(j|t)]²          p(j|t) = relative frequency of class j at node t
GINI_split = Σi (ni/n) · GINI(i)    n records at parent, ni at child i
```

Maximum `1 − 1/nc` when records are equally distributed among the nc classes (highest impurity); minimum 0 when all records belong to one class. Worked values (6-record nodes, 2 classes):

| Node | C1 | C2 | Gini |
|---|---|---|---|
| t1 | 0 | 6 | 0.000 |
| t2 | 1 | 5 | 0.278 |
| t3 | 2 | 4 | 0.444 |
| t4 | 3 | 3 | 0.500 |

**Boolean attribute B** — parent (C1:6, C2:6), Gini 0.500. Children N1 (C1:5, C2:2), N2 (C1:1, C2:4):

- Gini(N1) = 1 − (5/7)² − (2/7)² = 0.408; Gini(N2) = 1 − (1/5)² − (4/5)² = 0.32
- Gini(split on B) = 7/12 · 0.408 + 5/12 · 0.32 = **0.371** — larger and purer partitions are sought.

**Categorical attribute** (CarType) — count matrix per class, then evaluate the alternatives: multi-way split Gini 0.393; binary {Sports, Luxury}|{Family} Gini 0.400; {Sports}|{Family, Luxury} Gini 0.419.

**Continuous attribute** (Taxable Income) — sort the values, linearly scan them updating the count matrix and computing the Gini index for each candidate cut; choose the position with least Gini. For the Cheat table (sorted incomes 60…220, candidate cuts 55…230): Gini ranges 0.420 … 0.300; the best split is at cut **97** (Gini 0.300), i.e. `Taxable Income > 97` (Yes:3/No:0 vs Yes:0/No:7).

### Entropy and Information Gain (ID3, C4.5)

```
Entropy(t) = − Σj p(j|t) log2 p(j|t)
GAIN = Entropy(parent) − Σi (ni/n) · Entropy(i)
```

Entropy is maximum (log nc) for uniform distribution, 0 for a pure node; computations mirror the Gini examples: (0,6) → 0; (1,5) → 0.65; (2,4) → 0.92.

- **Information Gain** (ID3, C4.5) measures the entropy reduction achieved by the split; disadvantage: **tends to prefer splits yielding a large number of small but pure partitions**.
- **Gain Ratio** (C4.5, designed to fix this) adjusts GAIN by the entropy of the partitioning:

```
SplitINFO = − Σi (ni/n) log2(ni/n)        GainRATIO = GAIN_split / SplitINFO
```

Higher-entropy partitionings (many small partitions) are penalized.

### Stopping criteria, pruning, overfitting

Stopping: stop when all records belong to the same class, or all records have similar attribute values, or by **early termination**:

- **Pre-pruning** (early stopping rule): stop before the fully-grown tree — e.g. stop if instances < user threshold, if class distribution is independent of available features (χ² test), or if expanding the node does not improve the impurity measure.
- **Post-pruning**: grow the tree to entirety, then trim nodes **bottom-up**; if generalization error improves, replace the sub-tree with a leaf labelled with the majority class of its instances.

> [!definition] Underfitting and Overfitting
> **Underfitting**: the model is too simple — both training and test errors are large. **Overfitting**: the model learned the training set "by heart" and does not generalize — training error keeps decreasing while test error increases. With **noise**, an over-accurate decision boundary gets distorted by noisy points.

Other practical issues: **data fragmentation** (the number of instances shrinks going down the tree; leaves may hold too few instances for statistically significant decisions) and **missing attribute values**, which affect (1) impurity computation, (2) distribution of instances to child nodes, (3) classification of test instances.

### Decision boundary and evaluation

The **decision boundary** between regions of different classes is **parallel to the axes** because each test involves one attribute at a time. **Oblique decision trees** use test conditions on multiple attributes (e.g. x + y < 1): more expressive, but finding the optimal test is computationally expensive.

Evaluation of decision trees: accuracy comparable to other techniques on simple datasets; interpretable for small trees (single predictions interpretable); **not incremental**; fast model building, very fast classification; scalable in training set size and attribute number; **difficult management of missing data**.

## Random Forest

**Ensemble learning** technique: multiple base models are combined to improve accuracy and stability and to avoid overfitting. A random forest is a **set of decision trees** built at training time; the class is assigned by **majority voting**.

- **Bootstrap aggregation (bagging)**: given a training set D of n instances, for b = 1..B sample **with replacement** n′ ≤ n examples, generate Db, train a classification tree on it.
- **Feature bagging**: for each candidate split select a **random subset of features** — typically √p of the p features. Trees are **decorrelated**: random subsets let different features be selected as best split attributes (an attribute hidden by a stronger one may grant better performance).

Evaluation: **higher accuracy than decision trees**; model and prediction not interpretable (a prediction may come from hundreds of trees — but **global feature importance** is provided); not incremental; fast building, very fast classification; scalable; **robust to noise and outliers**.

## Rule-Based Classification

> [!definition] Classification rule
> A rule `(Condition) → y` where Condition is a **conjunction of simple predicates** (LHS, rule antecedent) and y is the class label (RHS, rule consequent). A rule **covers** an instance x if the attributes of x satisfy the condition. Unlike association rules, classification rules are **not symmetric** in X–Y.

Example (vertebrates, attributes Blood Type / Give Birth / Can Fly / Live in Water):

- R1: (Give Birth = no) ∧ (Can Fly = yes) → Birds
- R2: (Give Birth = no) ∧ (Live in Water = yes) → Fishes
- R3: (Give Birth = yes) ∧ (Blood Type = warm) → Mammals
- R4: (Give Birth = no) ∧ (Can Fly = no) → Reptiles
- R5: (Live in Water = sometimes) → Amphibians

A hawk triggers only R1 → Bird; a grizzly bear triggers R3 → Mammal; a **lemur triggers only R3**; a **turtle triggers both R4 and R5**; a **dogfish shark triggers none**.

- **Mutually exclusive** rules: two rule conditions cannot be true at the same time — every record covered by at most one rule.
- **Exhaustive** rules: the classifier accounts for every combination of attribute values — every record covered by at least one rule.

### From decision trees to rules

Each root-to-leaf path becomes a rule, e.g. from the Cheat tree:

```
(Refund=Yes) ==> No
(Refund=No) ∧ Marital Status∈{Single,Divorced} ∧ Taxable Income<80K) ==> No
(Refund=No) ∧ Marital Status∈{Single,Divorced} ∧ Taxable Income>80K) ==> Yes
(Refund=No) ∧ Marital Status=Married) ==> No
```

Tree-derived rules are mutually exclusive and exhaustive and carry as much information as the tree — but they **can be simplified**: `(Refund=No) ∧ (Status=Married) → No` becomes `(Status=Married) → No` (rule set used to generate a post-processed model with better performances, reducing overfitting). Effects of simplification:

- rules no longer mutually exclusive (a record may trigger several) → **ordered rule set** (**decision list**: assign the class of the highest-ranked triggered rule) or unordered rules with voting schemes;
- rules no longer exhaustive (a record may trigger none) → use a **default class**.

**Building rules**: *direct methods* extract rules directly from data (RIPPER, CN2, Holte's 1R); *indirect methods* extract them from other models such as decision trees (C4.5rules). Evaluation: accuracy **higher than decision trees**; model and prediction interpretable; not incremental; fast building and classification; scalable; **robust to outliers**.

## Associative Classification

The classification model is defined by means of **association rules** `(Condition) → y` where the rule body is an itemset. Model generation:

1. **Rule selection & sorting** based on support, confidence and correlation thresholds;
2. **Rule pruning**;
3. **Database coverage**: the training set is covered by selecting the topmost rules according to the previous sort.

Evaluation: accuracy **higher than decision trees and rule-based classifiers** (correlation among attributes is considered); interpretable; rule generation may be **slow** (depends on the support threshold) while classification is very fast; scalable in training set size, **reduced scalability in attribute number** (rule generation may become unfeasible); not incremental; **unaffected by missing data**, robust to outliers.

## k-Nearest Neighbours (k-NN)

**Instance-based classifiers** store the training records and use them directly to predict the class of unseen cases. Examples: the **rote-learner** (classifies only if a record matches a training example exactly) and the **nearest neighbour** classifier, which uses the k "closest" points.

Requirements: the set of stored records, a **distance metric** (typically Euclidean, `d(p,q) = Σ (pi − qi)²`), and the value of k. To classify an unknown record: compute the distance to all training records, identify the k nearest neighbours, determine the class by **majority vote** (optionally weighting the vote by distance, w = 1/d²).

- **Choosing k is critical**: too small → sensitive to noise points; too large → the neighborhood may include points from other classes. For k = 1 the decision regions form a **Voronoi diagram**.
- **Scaling**: attribute domains should be **normalized** to prevent one attribute from dominating the distance (height ∈ [1.5, 2.0] m vs income ∈ [10K, 1M] $).
- **High-dimensional data**: suffers the **curse of dimensionality**.

Evaluation: accuracy comparable to other techniques on simple datasets; model not interpretable (single predictions "described" by their neighbours); **incremental** (training set must remain available); almost no model building but **slower classification** (requires computing distances); weakly scalable in training set size; robustness depends on the distance computation.

## Naïve Bayes

Bayes theorem: from P(C,X) = P(C|X)P(X) = P(X|C)P(C) follows `P(C|X) = P(X|C)·P(C) / P(X)`.

> [!definition] Bayesian classification
> Let C be the class attribute and X = ⟨x1,…,xk⟩ the record to classify (all attributes are random variables). Compute **P(C|X) for all classes** and assign X to the class with maximal P(C|X). P(X) is constant for all C (disregarded); P(C) = Nc/N is the a-priori probability of class C.

Estimating P(X|C) = P(x1,…,xk|C) uses the **naïve hypothesis** of statistical independence: `P(x1,…,xk|C) = P(x1|C)·P(x2|C)·…·P(xk|C)` — not always true, so model quality may be affected (accurate especially with not very large training sets). For **discrete** attributes P(xk|C) = |xkC| / Nc; for **continuous** attributes a probability distribution is used. **Bayesian networks** allow specifying a subset of dependencies among attributes.

Worked example (weather dataset, 14 records: P(p)=9/14, P(n)=5/14; per-class frequencies such as P(sunny|p)=2/9, P(sunny|n)=3/5, P(high|p)=3/9, P(false|p)=6/9 …). Label X = ⟨rain, hot, high, false⟩:

- P(X|p)·P(p) = 3/9 · 2/9 · 3/9 · 6/9 · 9/14 = **0.010582**
- P(X|n)·P(n) = 2/5 · 2/5 · 4/5 · 2/5 · 5/14 = **0.018286** → assign class **n**.

Evaluation: accuracy similar or lower than decision trees (the naïve hypothesis simplifies the model); not interpretable (weights of contributions may explain single predictions); **fully incremental** and does not require the training data; fast building, very fast classification; scalable; **affected by attribute correlation**.

## Support Vector Machines (SVM)

Find a **linear hyperplane** (decision boundary) separating the data: among the possible solutions, choose the one that **maximizes the margin** between the two classes. If the decision boundary is not linear, **transform data into a higher-dimensional space** and separate them there (kernel trick idea).

Evaluation: among the **best performers** in accuracy; **black box** (model and prediction not interpretable); not incremental; model building requires significant parameter tuning, classification is very fast; medium scalability in training set size and attribute number; robust to noise and outliers.

## Artificial Neural Networks

Inspired by the human brain: neurons as elaboration units, synapses as connection network. Architectures per task: **CNN** for image understanding, **RNN** for time series, **feed-forward NN (FFNN)** for numerical vector classification, **autoencoders** for denoising.

**Neuron structure**: weighted sum of the input vector plus offset, followed by an **activation function** — it simulates biological activation, provides non-linearity, and may saturate outputs in fixed ranges:

| Activation | Properties |
|---|---|
| Sigmoid, tanh | saturate input in a fixed range; non-linear on the whole scale; FFNN hidden and output layers (sigmoid output ∈ (0,1) read as likelihood) |
| Binary step | outputs 1 for non-zero input; binary outputs; **not appropriate for gradient descent** (derivative undefined at 0, zero elsewhere) |
| ReLU | deep networks (CNNs); avoids vanishing gradient, does not saturate, activates linearly for positive input |
| Softmax | output layer only, considers all neurons of the layer; output is a discrete **probability distribution** (e.g. class probabilities) |

**FFNN training**: assign random weights/offsets; iterate over training instances — forward propagation to the output, compare with the expected output (error), **backpropagation** of the error updating each neuron's weights. Stop when accuracy exceeds a threshold, parameter variation falls below a threshold, or the maximum number of epochs is reached. Evaluation: among best performers; black box; not incremental; **very complex parameter tuning** (significant time); very fast classification; medium scalability; robust to noise and outliers but requires a **large training set** (otherwise unstable when tuning parameters).

**Convolutional Neural Network (CNN)** — classification with softmax on top of learned features:

- **Tensors**: data flows as N-dimensional vectors; rank = number of dimensions; shape = elements per dimension. Images are rank-3 tensors `[d, h, w]` (d = depth: 1 grayscale, 3 RGB).
- **Convolution stage**: feature extraction by hundreds-to-thousands of **sliding filters** containing the trainable weights; convolving `[d,h,w]` with N filters yields `[N,h,w]` (one depth layer per filter).
- **Activation**: ReLU typically used (faster training, no vanishing gradients, fast derivatives for backpropagation).
- **Pooling**: tensor downsampling by a sliding filter replacing values with a summary statistic — **maxpool** (maximum) is the most common.
- Shallow filters learn textures/edges; deeper filters learn objects and parts. **Semantic segmentation** CNNs (e.g. SegNet) assign a class to each pixel: encoder network (convolutional layers, abstract features) + decoder network (deconvolutional layers → output image).

**Recurrent NN (RNN)** — process sequential data x(t) keeping a **state** that evolves over time (input x(t) plus previous state s(t−1)); applications: machine translation, time series prediction, speech recognition, POS tagging. Training with **Backpropagation Through Time** (error propagated through time steps). Issue: **vanishing gradient** (error gradient decreases rapidly over time → no long-term memory); solution **LSTM** (gates that encourage state information to flow through long intervals).

**Autoencoders** compress input data into compact representations and reconstruct the input from them — used for **feature extraction** (compressed representation as feature set) and **image/signal denoising**.

**Word embeddings** associate words with n-dimensional vectors trained on big text collections; words with similar meaning share similar vectors. Semantic relationships are captured by vector positions: `king − man = queen − woman`, i.e. `king − man + woman = queen`.

## Model Evaluation

Methods: partitioning techniques for training/test sets + metrics for performance evaluation + techniques for model comparison (ROC). Performance depends on factors besides the learning algorithm: class distribution, cost of misclassification, size of training and test sets. The **learning curve** shows accuracy vs training sample size (requires a sampling schedule: arithmetic or geometric); small samples cause bias and variance of the estimate.

### Partitioning techniques

- **Holdout**: fixed partitioning, typically 80% training / 20% test (proportions depend on dataset size); appropriate for large datasets; may be repeated (repeated holdout).
- **Cross-validation**: k disjoint subsets (folds); k-fold trains on k−1 partitions and tests on the remaining one, repeated for all folds — reliable estimate, not for very large datasets. **Leave-one-out** (k = n) only for very small datasets.
- **Stratified sampling** without replacement to generate partitions; **Bootstrap** = sampling with replacement.

Typical usage: **training 60% / validation 20% / test 20%** of labelled data — holdout to split (training+validation) vs test; cross validation to split training vs validation (validation for hyperparameter tuning and algorithm selection; test for the final performance estimate).

### Confusion matrix and metrics

Binary confusion matrix: a = TP (true positive), b = FN (false negative), c = FP (false positive), d = TN (true negative).

```
Accuracy = (TP + TN) / (TP + TN + FP + FN) = (a + d) / (a + b + c + d)
Precision p = a / (a + c)        Recall r = a / (a + b)
F-measure = 2rp / (r + p) = 2a / (2a + b + c)
```

> [!warning] Accuracy is not always reliable
> With class 0 = 9900 objects and class 1 = 100, a trivial model predicting always class 0 has **99% accuracy** while detecting no class-1 object. Accuracy is inappropriate for **unbalanced class distributions** and **different class relevance** (e.g. ill patients erroneously classified as healthy). Evaluate each class C separately instead: **recall** = correctly assigned to C / belonging to C; **precision** = correctly assigned to C / assigned to C; maximize their harmonic mean (F-measure).

### ROC curve

Developed in the 1950s for signal detection theory; characterizes the trade-off between **positive hits and false alarms**, plotting:

- **TPR** = TP/(TP+FN) on the y-axis against **FPR** = FP/(FP+TN) on the x-axis.

Reference points: (0,0) declare everything negative; (1,1) declare everything positive; (0,1) **ideal**; the diagonal = random guessing; below the diagonal the prediction is the opposite of the true class. Construction: use a classifier producing posterior probabilities P(+|A), sort instances by decreasing P(+|A), apply a threshold at each unique value, count TP/FP/TN/FN. Worked example on 10 instances (P(+|A) from 0.95 down to 0.25) produces the (FPR, TPR) points from (1,1) to (0,0).

**Model comparison**: no model consistently outperforms the other — M1 better for small FPR, M2 better for large FPR. **Area under the ROC curve (AUC)**: ideal = 1.0, random guess = 0.5.
