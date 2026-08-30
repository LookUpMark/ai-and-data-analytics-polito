# Post-modeling Global Explainability

> **Course:** Explainable and Trustworthy AI  
> **Lecture:** 4  
> **Date:** 2026-04-03  
> **Source:** XAI_04_posthoc_global.pdf

## Overview

This lecture introduces **post-modeling** explainability with focus on **global** and **model agnostic** methods. Three main approaches are presented: global surrogate models, permutation feature importance, and partial dependence plots (PDP).

## Content

### Model Agnostic Solutions

Model agnostic methods are applicable to any model, treating it as an oracle providing predictions and output probabilities.

**Advantages of model agnostic solutions:**

| Advantage | Description |
|---|---|
| **Model flexibility** | Explain complex, high-performing models |
| **Explanation flexibility** | Adopt the most suitable format for users |
| **Representation flexibility** | Explanation representation can differ from model's |
| **Lower switching cost** | Change the model while preserving the explanation |
| **Model comparison** | Easier when representation is the same |

### Global Surrogate Models

A **global surrogate model** is an interpretable model approximating a complex (black box) model. It is trained on the original model's predictions:

**Goal:** approximate the prediction function $f$ with an interpretable surrogate model $g$, under the constraint that $g$ is interpretable (e.g., decision tree, linear regression, rules).

**Procedure:**
1. Training data $X$ (same as used for $f$ or new with same distribution)
2. Labeling: get predictions of $f$ for $X$
3. Choose an interpretable model $g$
4. Train $g$ on $(X, f(X))$
5. Evaluation: measure how well $g$ replicates $f$ (MSE, accuracy, AUC-ROC)
6. Interpretation: interpret $g$ to gain insight into $f$'s behavior

**Variants:** **TREPAN** uses trees considering fidelity to the original model in the construction process, with *best-first* expansion prioritizing nodes with greatest potential to increase fidelity. Node evaluation: $reach(n) \times (1 - fidelity(n))$.

**Advantages:** simplified representation, diverse explanation forms, enables both global and local explainability, easy to build, flexibility in choice of $g$.

**Limitations:** it is an approximation, risk of oversimplification, does not capture local nuances, depends on training data quality, surrogate may still be hard to interpret.

### Permutation Feature Importance

**Permutation feature importance** estimates feature importance by evaluating the impact of permuting (shuffling) feature values on model performance.

**Procedure:**
1. Compute reference score (e.g., accuracy) on $D$
2. For each feature:
   - Permute the feature (random shuffling on $D$)
   - Evaluate model performance
   - Compute importance score = difference between original and permuted performance
3. Rank features by importance score
4. Repeat $N$ times to stabilize results

The idea: **the higher the performance drop when a feature is permuted, the more important that feature is.**

**Advantages:** model agnostic, intuitive interpretation, compressed global insight, simple implementation, no linearity assumption, performance ratio enables cross-model comparison, no retraining required.

**Limitations:** feature independence assumption (if correlated, creates unrealistic instances and importance is shared), linked to model performance, requires ground truth, depends on shuffling randomness.

### Partial Dependence Plots (PDP)

**PDPs** are a visualization tool to understand the relationship between model predictions and specific input features. They show the dependence between the outcome and a set of features of interest, marginalizing over other features.

For feature of interest $X_S$ and other features $X_C$, partial dependence is:

$$pd_S(x_S) = \mathbb{E}_{X_C}[f(x_S, X_C)] = \int f(x_S, x_C) \, dP(x_C)$$

In practice, approximated by the mean over data:

$$pd_S(x_S) \approx \frac{1}{n} \sum_{i=1}^{n} f(x_S, x_C^{(i)})$$

The PDP at value $x_S$ is the **average prediction if we force all data points to assume that feature value**.

**Advantages:** intuitive computation, visual explanation, easy to implement.

**Limitations:** feature independence assumption (if correlated, creates unrealistic data), typically analyzes one feature at a time, does not show feature distribution (risk of overinterpreting sparse regions), average marginal effect may hide heterogeneous effects.

## Key Concepts

| Concept | Definition | Note |
|---|---|---|
| **Global surrogate** | Interpretable model approximating the black box | Trained on $f(X)$ |
| **TREPAN** | Tree-based surrogate with best-first expansion | Considers model fidelity |
| **Permutation importance** | Impact of feature shuffling on performance | More drop → more important |
| **PDP** | Average relationship between feature and prediction | Marginalizes over other features |

## Connections

- Global surrogate models are the global counterpart of local surrogates (LIME, lecture 05)
- Permutation importance is a global method sharing principles with explaining by removing (lecture 06)
- PDPs are complementary to ICE plots (Individual Conditional Expectation) mentioned in the taxonomy (lecture 02)
