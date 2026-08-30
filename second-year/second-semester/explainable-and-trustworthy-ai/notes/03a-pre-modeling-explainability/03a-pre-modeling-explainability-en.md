# Pre-modeling Explainability

> **Course:** Explainable and Trustworthy AI  
> **Lecture:** 3a  
> **Date:** 2026-04-03  
> **Source:** XAI_03a_premodeling.pdf

## Overview

This lecture covers the **pre-modeling explainability** phase, encompassing all activities that precede actual model construction. The goal is to gain better understanding of the data and preprocess it while preserving interpretability, potentially identifying and correcting biases before modeling.

## Content

### Exploratory Data Analysis (EDA)

**Exploratory Data Analysis** is the fundamental first step of pre-modeling explainability. It uses statistical techniques and visualizations to extract a summary of the main characteristics of a dataset:

- Data summary and dataset visualization
- Computing and analyzing statistical properties: mean, standard deviation, percentage of missing samples, feature dimensionality, presence of outliers

Knowing the data enables better understanding of the model that will be trained and **exposing biases** that might exist in the data.

**Common EDA Tools:**

| Tool | Features |
|---|---|
| **Numpy, Pandas, Scikit-learn** | Standard Python libraries for statistical analysis |
| **ydata-profiling** | Univariate analysis (descriptive statistics, visualizations), multivariate analysis (correlations, missing data, pairwise interactions), dataset comparison |
| **FACETS** | Feature-by-feature statistical analysis, data distribution, focus on common issues like missing values, exploration of relationships between data points |

### Dataset Description

Properly documenting a dataset is essential for multiple reasons:

- **Communication**: ensure proper communication between data creators and users
- **Transparency**: clear data origin, characteristics, and potential biases
- **Avoid data misuse**
- **Ethical considerations**: help identify systemic biases in models
- **Reproducibility**: enable reproduction of results and analyses
- **Data governance**: provide guidelines for data management
- **Collaboration and sharing**: documented data can be easily shared
- **Long-term preservation**: maintain accessibility and usability over time
- **Risk management**: identify risks such as privacy concerns, security vulnerabilities, or data quality issues

**Points to address in documentation:**

| Aspect | Description |
|---|---|
| **Motivation** | Reasons for creating the dataset, who created or funded it |
| **Composition** | What the dataset provides, presence of errors, noise, or redundancies |
| **Collection process** | How data were acquired, who was involved |
| **Preprocessing** | Information on preprocessing or cleansing |
| **Uses** | Which tasks the data may or may not be used for |
| **Distribution** | How the dataset will be disseminated, restrictions, and licenses |
| **Maintenance** | Planned updates, support, and user communication |

Several recommendations exist for standardizing descriptions: **Datasheets for Datasets** (Gebru et al., 2021), **Data Statements** (Bender & Friedman, 2018), **Dataset Nutrition Labels** (Holland et al., 2020).

### Interpretable Feature Engineering

Feature selection and preprocessing should preserve interpretability:

**Feature selection:**
- A lower number of features reduces complexity and makes the process and model more interpretable
- Methods: recursive feature elimination
- Interpretable selection processes: driven by domain experts (select most important features), correlation-based (keep only representative features among correlated ones)

**Interpretable feature engineering:**
- Creating or transforming features in a human-understandable way
- **Discretization**: from numerical age to categories (<30, 30-60, >60)
- **Semantic binning**: from age to concepts (young, adult, senior)
- **Window statistics**: from time series to mean, percentiles, standard deviation over windows
- **Domain knowledge integration**: create domain-driven features that are meaningful and interpretable

## Key Concepts

| Concept | Definition | Note |
|---|---|---|
| **EDA** | Exploratory statistical and visual analysis of data | First step to understand data |
| **Datasheets for Datasets** | Standard for documenting datasets | Gebru et al., 2021 |
| **Discretization** | Converting continuous features to categories | Increases interpretability |
| **Semantic binning** | Binning with semantic labels | E.g., age → young/adult/senior |

## Connections

- EDA and data description prepare the ground for the modeling phase (lecture 03b)
- *Datasheets for Datasets* are relevant to the transparency requirement from lecture 01
- Interpretable feature selection is a prerequisite for interpretable models (decision trees, rules)
