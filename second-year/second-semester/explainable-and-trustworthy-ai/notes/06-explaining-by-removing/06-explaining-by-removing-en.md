# Explaining by Removing (Occlusion/Perturbation)

> **Course:** Explainable and Trustworthy AI  
> **Lecture:** 6  
> **Date:** 2026-04-03  
> **Source:** XAI_06_local_explaining_by_removing.pdf

## Overview

This lecture covers explainability methods based on **feature removal** (occlusion/perturbation), starting from the base approach PredDiff, through **Shapley Values** from game theory, to **SHAP** (SHapley Additive exPlanations). A unified framework for all removal-based methods is also presented.

## Content

### The Basic Principle

The idea is to **remove one or more input features** (or simulate the removal) to quantify the feature's influence on the prediction:

$$f(y=c|x) \neq f(y=c|x \setminus \{gender=female, nation=Italy\})$$

### PredDiff — Prediction Difference

PredDiff (Robnik-Šikonja & Kononenko, 2008) is the base approach. The importance of attribute $A_i$ is:

$$predDiff_{f_i}(x) = f(x) - f(x \setminus A_i)$$

**Two ways to evaluate the difference:**

- **Probability difference:** $predDiff_{f_i}(x) = P(y=c|x) - P(y=c|x \setminus A_i)$
- **Information difference:** $infoDiff_{f_i}(x) = \log_2 P(y=c|x) - \log_2 P(y=c|x \setminus A_i)$

**How to simulate removal?** Use an "average value":

$$P(y|x \setminus A_i) = \sum_{j=1}^{m_i} P(A_i = a_j) \cdot P(y|x \leftarrow A_i = a_j)$$

For categorical features: replace with all possible values, weighted by prior probability. For numerical features: discretize and use midpoints of sub-intervals as representative values.

**Interpretation:** higher importance → the feature impacts the prediction more. Positive contribution → pushes toward the predicted class. Negative contribution → pushes against.

**Advantages:** model agnostic, local explanations, feature attributions, direct interpretation. **Limitations:** defined only for structured data, unrealistic perturbations, requires data access, does not consider feature interactions.

### Considering Interactions: Shapley Values

To consider the contribution of multiple features simultaneously (removing $A_i$ and removing $A_i$ and $A_j$), we need a way to aggregate importance scores into a single attribution. The answer: **Shapley values**.

### Shapley Values

Shapley values come from **game theory**. The idea is to assign a relevance score to each player on a collaborating team, analogous to features of a model:

$$\phi_i(v) = \sum_{S \subseteq N \setminus \{i\}} \frac{|N| - |S|! \cdot (|S| - 1)!}{|N|!} (v(S \cup \{i\}) - v(S))$$

where $N$ is the set of players (features), $S$ is a coalition of players, and $v(S)$ is the total payoff of coalition $S$.

**Properties of Shapley Values:**

| Property | Description |
|---|---|
| **Efficiency** | Sum of all Shapley values equals the team's total value: $\sum_{i \in N} \phi_i(v) = v(N)$ |
| **Symmetry** | Players with same marginal contribution have the same $\phi$ |
| **Linearity** | $\phi_i(v + w) = \phi_i(v) + \phi_i(w)$ |
| **Null player** | Player with zero marginal contribution has $\phi_i = 0$ |

Shapley values are the **only** assignment method satisfying all four properties.

### Applying Shapley Values to XAI

**Analogy:**

| Game Theory | XAI |
|---|---|
| Players | Feature values |
| Total score $v(N)$ | Probability difference vs average prediction |
| Coalition $S$ | Features present, others "removed" |
| $v(S)$ | Prediction probability marginalizing over features not in $S$ |
| $\phi_i$ | Feature attribution |

The value function $v$ for XAI is defined as:

$$v(S) = f_S - \mathbb{E}[f(X)]$$

where $f_S$ is the model prediction marginalizing over features not in $S$.

**Computational problem:** exact computation requires $2^{|N|}$ coalitions — exponential in the number of features. **Solution:** approximation via **Monte Carlo sampling**.

### Monte Carlo Approximation of Shapley Values

1. For $m = 1, \ldots, M$ iterations:
   - Sample a random instance $z$ from the dataset
   - Randomly select a permutation of feature values
   - Compute $x_{+j}$ (values of $x$ before $j$-th in permutation + $j$) and $x_{-j}$ (values of $z$ after $j$-th)
   - Compute marginal contribution: $\phi_i^m = f(x_{+j}) - f(x_{-j})$
2. Compute Shapley values as average: $\phi_i = \frac{1}{M} \sum_{m=1}^{M} \phi_i^m$

### SHAP — SHapley Additive exPlanations

SHAP (Lundberg & Lee, NeurIPS 2017) unifies several explanation methods under the Shapley value framework, proposing:

- **KernelSHAP**: kernel-based estimation, model agnostic
- **TreeSHAP**: efficient estimation for tree-based models (not model agnostic)
- Aggregation of local explanations for global insights

**SHAP as a linear surrogate model:**

$$g(x') = \phi_0 + \sum_{i=1}^{M} \phi_i x'_i$$

where $x'_i \in \{0, 1\}$ models the presence/absence of interpretable features, and $\phi_i$ are Shapley values.

**Properties of additive feature attribution:**

| Property | Description |
|---|---|
| **Local accuracy** | $f(x) = g(x') = \phi_0 + \sum \phi_i x'_i$ when $x = h_x(x')$ |
| **Missingness** | Missing features ($x'_i = 0$) have zero attribution |
| **Consistency** | If a feature's marginal contribution increases, its attribution does not decrease |

Shapley values are the **only** explanation model $g$ satisfying the definition of additive feature attribution methods and these three properties.

**KernelSHAP** estimates SHAP values by training a weighted linear model with a specific kernel (Shapley kernel) on sampled coalitions:

$$\pi_{x'}(z') = \frac{M-1}{M \binom{M}{|z'|} |z'|(M - |z'|)}$$

**Global insights with SHAP:**

- **Feature importance**: mean of absolute Shapley values per feature across the dataset: $I_j = \frac{1}{n} \sum_{i=1}^{n} |\phi_j^{(i)}|$
- **Summary plot**: density scatter plot with Shapley values for feature and instance
- **Dependence plot**: feature value vs Shapley value, colored by another feature to highlight interactions

### Unified Framework for Removal-Based Methods

Covert, Lundberg & Lee (2021) propose a unified framework characterizing these methods along three dimensions:

1. **Feature removal**: how the method removes features (zeroing, default values, blurring, marginalization)
2. **Model behavior**: what is explained (class probability, prediction loss, dataset loss)
3. **Summary technique**: how each feature's influence is summarized (individual removal, additive model, Shapley values)

## Key Concepts

| Concept | Definition | Note |
|---|---|---|
| **PredDiff** | Prediction difference with/without feature | Base approach, explaining by removing |
| **Shapley value** | Weighted average marginal contribution | Game theory, unique satisfying 4 axioms |
| **Efficiency** | Sum of attributions = difference from mean | Shapley value property |
| **SHAP** | SHapley Additive exPlanations | Unified framework, KernelSHAP + TreeSHAP |
| **KernelSHAP** | SHAP estimation via weighted linear kernel | Model agnostic |
| **TreeSHAP** | Efficient SHAP estimation for trees | Not model agnostic |
| **Shapley kernel** | Kernel for weighting coalitions in KernelSHAP | Used in modified LIME loss |

## Connections

- PredDiff is the base case motivating more sophisticated methods
- Shapley values connect game theory and XAI
- SHAP unifies LIME (lecture 05) and removal-based methods in a common framework
- SHAP's efficiency property connects to partial dependence plots (lecture 04)
- The feature independence assumption is shared with PDP and permutation importance (lecture 04)
