# Exam Question Answers

## Theory Question 1

**Describe and compare `Principal Component Analysis (PCA)` and `Linear Discriminant Analysis (LDA)`, covering the following aspects:**

*   **Objectives of the two models and their formulation**
*   **Training objective function of the two models**
*   **Characteristics of `PCA` `principal components` and `LDA` `discriminant directions`**
*   **How the models can be used in `classification tasks`**

### Answer

#### 1. Objectives and Formulation

*   **`Principal Component Analysis (PCA)`:** An **`unsupervised`** dimensionality reduction technique. Its goal is to find a lower-dimensional representation of the data that captures the maximum possible variance, assuming this variance is the most informative aspect of the data. It is formulated by performing an eigenvalue decomposition of the data's **covariance matrix** ($C$).

*   **`Linear Discriminant Analysis (LDA)`**: A **`supervised`** dimensionality reduction technique designed for classification. Its goal is to find a subspace that maximizes the separability between classes. It achieves this by maximizing the ratio of the **`between-class scatter`** ($S_B$), which measures the separation of class means, to the **`within-class scatter`** ($S_W$), which measures the dispersion of data within each class.

#### 2. Training Objective Function

*   **`PCA`**: The objective is to find a projection that minimizes the **`mean squared reconstruction error`**. This is equivalent to finding the eigenvectors of the covariance matrix that correspond to the largest eigenvalues, thus maximizing the variance of the projected data.

*   **`LDA`**: The objective is to find a projection matrix $W$ that maximizes **`Fisher's discriminant ratio`**, $J(W) = \frac{\det(W^T S_B W)}{\det(W^T S_W W)}$. The solution is found by solving the generalized eigenvalue problem $S_B w = \lambda S_W w$.

#### 3. Characteristics of the Directions

*   **`PCA Principal Components`:** They are the eigenvectors of the data's covariance matrix. They are **`orthogonal`**, capturing uncorrelated dimensions of variance, and are ordered by their corresponding eigenvalues (i.e., by the amount of variance they capture). Their calculation is **`unsupervised`**.

*   **`LDA Discriminant Directions`:** They are the eigenvectors of $S_W^{-1} S_B$. They are generally **`not orthogonal`** and are ordered by their ability to separate the classes. Their calculation is **`supervised`**, and the maximum number of directions is $C - 1$, where $C$ is the number of classes.

#### 4. Use in Classification

*   **`PCA`**: Typically used as a **`pre-processing`** step to reduce dimensionality, which can help combat the "`curse of dimensionality`" and reduce noise. However, being `unsupervised`, it might discard low-variance dimensions that are very useful for classification.

*   **`LDA`**: It is intrinsically a classification-oriented method. The data is projected onto the `LDA` subspace, and a classifier is trained on these new discriminant `features`. A powerful `pipeline` is `PCA+LDA`, where `PCA` is first used for dimensionality reduction and regularization, followed by `LDA` to find the optimal subspace for classification.

---

## Theory Question 2

**Considering the `Linear Discriminant Analysis (LDA)` approach for binary classification and the binary `Tied MVG` classifier, detail:**

*   **Model formulation, training objective, and `inference` procedure (i.e., how to use the model for classification) of the `LDA` classifier**
*   **Model assumptions, training objective, and `inference` procedure of the `Tied MVG` classifier**
*   **The relationship between the two models**
*   **The shape of the `decision rules` of the binary `LDA` and `Tied MVG` classifiers**

### Answer

#### 1. `LDA` Classifier

*   **Formulation and Objective**: For binary classification, `LDA` seeks a single projection vector $w$ that maximizes `Fisher's discriminant ratio`, $J(w) = \frac{w^T S_B w}{w^T S_W w}$. This maximizes the separation between the means of the two projected classes. The optimal projection is $w \propto S_W^{-1}(\mu_1 - \mu_2)$.
*   **`Inference`**: A new sample $x$ is classified by projecting it onto the vector $w$ to get a `score` $s = w^T x$. This `score` is then compared to a threshold $t$. If $s \geq t$, the sample is assigned to one class; otherwise, it is assigned to the other.

#### 2. `Tied MVG` Classifier

*   **Assumptions and Objective**: This is a generative model that assumes the data for each class follows a multivariate Gaussian distribution. Its key assumption is that all classes share the **`same covariance matrix`** ($\Sigma_c = \Sigma$). This means the class distributions have different centers but the same shape and orientation. The training objective is to find the `Maximum Likelihood Estimates (MLE)` for the class means $\mu_c$ and the single shared covariance matrix $\Sigma$.
*   **`Inference`**: It uses Bayes' theorem to classify a new sample $x$, selecting the class $c$ that maximizes the posterior probability $P(C=c|x)$.

#### 3. Relationship between `LDA` and `Tied MVG`

The two models are deeply connected. The decision boundary for a `Tied MVG` classifier is found where the posterior probabilities for two classes are equal. By analyzing the `log-posterior-odds` ratio ($\log \frac{P(C=1|x)}{P(C=0|x)} = \log \frac{P(C=1)f(x|C=1)}{P(C=0)f(x|C=0)}$), the quadratic terms ($x^T \Sigma^{-1} x$) are identical for both classes due to the shared covariance matrix, and thus cancel out. This results in a decision boundary that is a linear function of $x$, which is **`identical`** to the decision boundary found by `LDA`.

#### 4. Shape of `Decision Rules`

*   **`LDA`**: The `decision rule` is explicitly **`linear`**. The decision boundary is a hyperplane in the `feature` space defined by the equation $w^T x - t = 0$.
*   **`Tied MVG`**: The `decision rule` is also **`linear`**. The assumption of a shared covariance matrix is precisely what simplifies the `log-likelihood` ratio to a linear function, resulting in a linear decision boundary.

---

## Theory Question 3

**Describe in detail the `Multivariate Gaussian (MVG)` classifier, covering the following aspects:**

*   **Model assumptions**
*   **Estimation of model parameters**
*   **How the model can be used to perform `inference` (i.e., classify a test sample) for both `multi-class` and binary problems**
*   **The shape of the `decision rules` for binary problems**
*   **`Naive Bayes` and `Tied Covariance` model variants, focusing on:**
    *   **Differences with the standard (unconstrained) model in terms of assumptions and `decision rules`**
    *   **Advantages and limitations compared to the unconstrained model**

### Answer

#### 1. Model Assumptions

The **`Multivariate Gaussian (MVG)`** classifier is a generative model. It assumes that the `feature` data for each class $c$ is drawn from a distinct multivariate Gaussian distribution, characterized by its own **mean vector** $\mu_c$ and **covariance matrix** $\Sigma_c$. This models each class as a hyper-ellipsoid with a unique center, shape, and orientation.

#### 2. Estimation of Model Parameters

The model parameters ($\{\mu_c, \Sigma_c\}$ for all classes) are estimated from the labeled `training` data using **`Maximum Likelihood Estimation (MLE)`**. For each class $c$:

*   **Class Mean**: $\hat{\mu}_c = \frac{1}{N_c} \sum_{i: y_i = c} x_i$, where $N_c$ is the number of samples of class $c$
*   **Class Covariance Matrix**: $\hat{\Sigma}_c = \frac{1}{N_c} \sum_{i: y_i = c} (x_i - \hat{\mu}_c)(x_i - \hat{\mu}_c)^T$
*   **Class Prior**: $\hat{\pi}_c = \frac{N_c}{N}$, where $N$ is the total number of samples

These are the `MLE` estimates that maximize the `log-likelihood` on the `training` data.

#### 3. `Inference`

For both binary and `multi-class` problems, `inference` uses Bayes' theorem. A new sample $x$ is classified by finding the class $c$ that maximizes the posterior probability: $\text{Predicted Class} = \arg\max_c P(C=c) f(x|C=c)$. The class prior probability $P(C=c)$ is usually estimated from the class frequencies, and $f(x|C=c)$ is the class-conditional Gaussian PDF. Calculations are often performed with `log-probabilities` for numerical stability.

#### 4. Shape of `Decision Rules` for Binary Problems

The decision boundary occurs where the posterior probabilities are equal. For the standard `MVG`, the covariance matrices $\Sigma_1$ and $\Sigma_2$ are different. This leads to a `log-likelihood` ratio containing quadratic terms in $x$, which means the decision boundary is **`quadratic`** (e.g., a hyper-paraboloid or a hyper-ellipsoid).

#### 5. `Naive Bayes` and `Tied Covariance` Variants

*   **`Naive Bayes Gaussian Classifier`:**
    *   **Assumptions and `Rules`**: Assumes that the `features` are **`conditionally independent`** given the class, forcing each covariance matrix $\Sigma_c$ to be **`diagonal`**. The decision boundary remains **`quadratic`**, but is constrained to be axis-aligned.
    *   **`Pros & Cons`**: Greatly reduces the number of parameters ($O(D)$ instead of $O(D^2)$), making it efficient and robust against `overfitting` in high dimensions. However, the independence assumption is often violated in practice, which can degrade `performance`.

*   **`Tied Covariance Gaussian Classifier`:**
    *   **Assumptions and `Rules`**: Assumes that all classes share the **`same covariance matrix`** ($\Sigma_c = \Sigma$). This simplifies the decision boundary to be **`linear`**.
    *   **`Pros & Cons`**: Acts as a compromise, reducing parameters compared to the full model for better stability with limited data. However, the assumption can be too restrictive if the classes have naturally different distributions in the `feature` space.

---

## Theory Question 4

**Describe the `binary logistic regression` model for classification, covering the following aspects:**

*   **`Classification rule` of the `binary logistic regression` model**
*   **Probabilistic interpretation of the model and the `classification score`**
*   **Estimation of model parameters and possible interpretations of the `training objective function`**

**Both `logistic regression` and `Support Vector Machines (SVM)` can be interpreted as `risk minimization` approaches.**

*   **Compare the `objective functions` of the two models**
*   **Explain possible approaches to obtain `non-linear decision functions` with these two classifiers**

### Answer

#### 1. `Binary Logistic Regression Model`

*   **`Classification Rule`**: A discriminative classifier that learns a **`linear decision boundary`**. A new sample $x$ is classified based on the sign of a `linear score` $s = w^T x + b$.
*   **Probabilistic Interpretation**: The model provides a direct probabilistic output by passing the `score` $s$ through the **sigmoid function**, $\sigma(s) = \frac{1}{1 + e^{-s}}$. This output, $P(C=1|x) = \sigma(w^T x + b)$, is the posterior probability of the positive class. The `score` $s$ itself can be interpreted as the **`log-posterior-odds ratio`**.
*   **Parameter Estimation**: The parameters ($w, b$) are estimated by minimizing the `negative log-likelihood`, also known as the **`binary cross-entropy loss`**:
    $$L(w, b) = -\frac{1}{N} \sum_{i=1}^{N} \left[ y_i \log(\sigma(w^T x_i + b)) + (1-y_i) \log(1-\sigma(w^T x_i + b)) \right]$$
    where $y_i \in \{0, 1\}$ are the labels and $\sigma(\cdot)$ is the sigmoid function. This is a form of `Maximum Likelihood Estimation` and, having no closed-form solution, requires iterative optimization methods like gradient descent.

#### 2. Comparison with `SVM`

*   **`Objective Functions`:**
    *   **`Logistic Regression`**: Minimizes the **`logistic loss`** (`cross-entropy`). This `loss` is smooth and penalizes all samples, even correctly classified ones, encouraging the model to produce more confident probabilities.
    *   **`SVM`**: Minimizes the **`hinge loss`**. This `loss` is zero for correctly classified points that lie outside the `margin`. It only penalizes points that violate the `margin`, making the model insensitive to points far from the `boundary` and leading to a **`sparse`** solution defined only by the `support vectors`.

*   **`Non-linear Decision Functions`:**
    *   **`Logistic Regression`**: Achieves non-linearity through **`feature expansion`**, where non-linear combinations of the original `features` are manually added to the model. The model learns a `linear boundary` in this expanded space, which is `non-linear` in the original space.
    *   **`SVM`**: Achieves non-linearity more elegantly with the **`kernel trick`**. This technique allows the `SVM` to learn a `boundary` in a very high-dimensional space by replacing `dot products` with a `kernel function` (e.g., `polynomial` or `RBF kernel`), without the computational cost of explicitly transforming the `features`.

---

## Theory Question 5

**Describe the `Support Vector Machine (SVM)` classifier, covering the following aspects:**

*   **`Classification rule` of `SVM` and interpretation of the `SVM score`**
*   **The concept of `margin`**
*   **Primal (both as constrained convex quadratic programming and with `hinge loss`) and dual formulation of the `objective function`, and the relationship between the primal and dual solutions**
*   **`SVM` for non-linear classification**

### Answer

#### 1. `SVM Classification Rule` and `Score Interpretation`

*   **`Rule`**: A `linear SVM` classifies a sample $x$ based on the sign of a `linear score` $s = w^T x + b$. The `decision boundary` is the hyperplane where $s = 0$.
*   **`Score Interpretation`**: The `score` $s$ is not a probability; it represents the **`signed geometric distance`** of the sample from the decision hyperplane. The magnitude of the `score` indicates the confidence in the classification.

#### 2. The Concept of `Margin`

The **`margin`** is the corridor separating the two classes, centered on the decision hyperplane. `SVMs` are **`maximum margin classifiers`**: their objective is to find the hyperplane that maximizes the width of this `margin`. A wider `margin` is believed to lead to better generalization on unseen data. The data points on the edges of the `margin` are the **`support vectors`**.

#### 3. Primal and Dual Formulations

*   **`Primal Formulation`**: The objective is to minimize $\frac{1}{2} ||w||^2$ (to maximize the `margin`) subject to $y_i (w^T x_i + b) \ge 1$ for all samples. The **`soft-margin`** formulation adds a penalty term based on the `hinge loss` ($C \sum_i \xi_i$) to handle non-separable data, allowing for `margin` violations via `slack` variables $\xi_i$.
*   **`Dual Formulation`**: Derived from the primal, it maximizes $\sum_i \alpha_i - \frac{1}{2} \sum_{i,j} \alpha_i \alpha_j y_i y_j x_i^T x_j$ subject to $\sum_i \alpha_i y_i = 0$ and $0 \le \alpha_i \le C$. The `dual objective function` depends only on the **`dot products`** of the `training` samples, not the `feature vectors` themselves.
*   **Relationship**: The primal and dual problems yield the same solution. The KKT conditions link them and reveal that the optimal `weight vector` $w = \sum_i \alpha_i y_i x_i$ is a linear combination of only the **`support vectors`** (samples with $\alpha_i > 0$), making the `SVM` solution sparse.

#### 4. `SVM` for Non-linear Classification

`SVMs` use the **`kernel trick`** for non-linear classification. Since the `dual formulation` only uses `dot products`, we can replace the `standard dot product` $x_i^T x_j$ with a **`kernel function`** $k(x_i, x_j)$ (e.g., `polynomial` or `RBF kernel`), without the computational cost of explicitly transforming the `features`.

---

## Theory Question 6

**Describe `Gaussian Mixture Models (GMM)` in the context of `density estimation` and `pattern classification`, covering the following aspects:**

*   **Model definition, interpretation of model parameters, and formulation of `GMM` as a `latent variable model`**
*   **Estimation of model parameters**
*   **How the model can be used to solve classification problems, including `open-set classification tasks`**
*   **Potential `issues` of `GMMs`, possible ways to address these `issues`, and possible `variations` of the model**

### Answer

#### 1. `GMM` Definition and Formulation

*   **Definition**: A **`Gaussian Mixture Model (GMM)`** is a probabilistic model that represents a data distribution as a weighted sum of multiple `Gaussian components`: $p(x) = \sum_{k=1}^{K} \pi_k \mathcal{N}(x | \mu_k, \Sigma_k)$.
*   **Parameters and Latent View**: The model is defined by the `weights` ($\pi_k$), `means` ($\mu_k$), and `covariances` ($\Sigma_k$) of its $K$ components. It can be viewed as a **`latent variable model`**, where a `hidden variable` $z_i$ for each `data point` $x_i$ specifies which component generated it, leading to a "`soft clustering`" of the data.

#### 2. Parameter Estimation

The parameters are estimated with the **`Expectation-Maximization (EM) algorithm`**, an iterative method for models with `latent variables`.
*   **`E-step` (Expectation)**: Calculates the posterior probability (`responsibility`) $\gamma_{ik} = \frac{\pi_k \mathcal{N}(x_i|\mu_k, \Sigma_k)}{\sum_{j=1}^{K} \pi_j \mathcal{N}(x_i|\mu_j, \Sigma_j)}$ of each component $k$ for each `data point` $x_i$, given the current parameters. This formula indicates how likely it is that point $x_i$ belongs to component $k$, representing the "`soft assignment`" of each point to each component.
*   **`M-step` (Maximization)**: Updates the parameters using the `responsibilities` as `soft weights`:
    *   Weights: $\pi_k = \frac{1}{N}\sum_i \gamma_{ik}$
    *   Means: $\mu_k = \frac{\sum_i \gamma_{ik} x_i}{\sum_i \gamma_{ik}}$
    *   Covariances: $\Sigma_k = \frac{\sum_i \gamma_{ik} (x_i - \mu_k)(x_i - \mu_k)^T}{\sum_i \gamma_{ik}}$

The process is repeated until convergence (when the change in `log-likelihood` falls below a threshold).

#### 3. `GMM` for Classification

*   **Standard Classification**: A `GMM` is trained for each class to model $f(x|C=c)$. To classify a new sample, Bayes' rule is applied: assign to the class $c$ that maximizes $P(C=c) \cdot f(x|C=c)$.
*   **`Open-set Classification`**: An additional `GMM` is trained on "`unknown`" data. A sample is classified as "`unknown`" if it has the highest `likelihood` under this model, allowing for the rejection of `out-of-distribution` samples.

#### 4. Issues, Solutions, and Variants

**Main Issues:**

*   **`Singularities`**: When a Gaussian component "collapses" onto a single point or small cluster, the covariance matrix becomes singular (zero determinant). This causes infinite `likelihood` values that break the EM algorithm and make the model unstable.

*   **`Local Optima`**: The EM algorithm only guarantees convergence to local maxima of the `likelihood`, not global ones. The quality of the final solution strongly depends on the parameter initialization, potentially leading to suboptimal solutions.

*   **`Model Selection`**: Determining the optimal number of components $K$ is complex. Increasing $K$ always improves the `fit` on the `training` data but risks `overfitting`, degrading performance on new data.

**Solutions and Strategies:**

*   **Regularization against `Singularities`**: A small constant $\epsilon I$ is added to the covariance matrix during the update, ensuring it always remains invertible and numerically stable.

*   **Improving Initialization**: Strategies like `K-means clustering` are used to initialize the component means, or `multiple random restarts` are performed, choosing the solution with the highest `likelihood`.

*   **Selecting the Number of Components**: `Information theory` criteria like `AIC` or `BIC` are applied, which balance `goodness-of-fit` and model complexity, or `cross-validation` techniques are used to evaluate generalization performance.

**Model Variants:**

*   **`Diagonal GMMs`**: Assume diagonal covariance matrices ($\Sigma_k$ is diagonal), drastically reducing the number of parameters to estimate. Particularly useful for high-dimensional data where estimating full matrices would be computationally prohibitive.

*   **`Tied GMMs`**: All components share the same covariance matrix ($\Sigma_k = \Sigma$ for all $k$). Reduces model complexity and improves numerical stability when `training` data is limited.

---

## Theory Question 7

**Describe the `binary logistic regression` model for classification, covering the following aspects:**

*   **`Classification rule` of the model**
*   **Probabilistic interpretation of the model and its `classification score`**
*   **Estimation of model parameters and possible interpretations of the `training objective function`**
*   **How the model can be extended to perform `non-linear classification`**
*   **How the model can be extended and applied to address `score calibration issues`**

### Answer

#### 1. `Classification Rule`

`Binary logistic regression` is a **discriminative classifier** that learns a **`linear decision boundary`**. A new sample $x$ is classified based on a `linear score` $s = w^T x + b$. The `classification rule` is: assign to **Class 1** if $s > 0$, to **Class 0** if $s < 0$. The hyperplane $w^T x + b = 0$ separates the two classes in the `feature` space.

The model assumes that the `log-odds` is linear in the `features`: $\log\frac{P(C=1|x)}{P(C=0|x)} = w^T x + b$, where $w$ determines the direction of separation and $b$ the position of the hyperplane.

#### 2. Probabilistic Interpretation

The model provides a direct probabilistic output by passing the `linear score` $s$ through the **sigmoid function**, $\sigma(s) = \frac{1}{1 + e^{-s}}$.
*   The output, $P(C=1|x) = \sigma(w^T x + b)$, is the **posterior probability** of the positive class (a value between 0 and 1).
*   The `score` $s$ itself represents the **`log-odds ratio`**: $s = \log\frac{P(C=1|x)}{P(C=0|x)}$, where positive values favor Class 1.
*   The sigmoid function maps any `real-valued score` to the interval $[0, 1]$, making it suitable for probabilistic interpretation.

#### 3. Parameter Estimation and `Objective Function`

The parameters ($w, b$) are estimated by minimizing the **`negative log-likelihood`** (also known as **`binary cross-entropy loss`**):

$$L(w, b) = -\frac{1}{N} \sum_{i=1}^{N} \left[ y_i \log(\sigma(w^T x_i + b)) + (1-y_i) \log(1-\sigma(w^T x_i + b)) \right]$$

on the `training` data. This is a form of **`Maximum Likelihood Estimation (MLE)`**, as it finds the parameters that make the observed labels most likely. The `loss function` is **`convex`** but has no closed-form solution, so it is minimized using iterative algorithms like gradient descent.

From a theoretical perspective, this is equivalent to minimizing the **`empirical risk`** using a logarithmic `loss function` that increasingly penalizes wrong predictions.

#### 4. Extension to `Non-linear Classification`

`Logistic regression` can learn `non-linear boundaries` through **`feature expansion`**. This involves creating new `features` from non-linear transformations of the original ones (e.g., polynomial terms like $x_1^2, x_1x_2$). The model learns a `linear boundary` in this expanded space, which corresponds to a **`non-linear boundary`** in the original `feature` space.

Care must be taken to avoid `overfitting` when expanding to high-dimensional `feature` spaces.

#### 5. Application to `Score Calibration`

Because its `training objective` (minimizing `cross-entropy`) encourages accurate probabilistic outputs, `logistic regression` is an excellent tool for **`scores calibration`** of other, less-calibrated models (like `SVMs`). This process, known as **`Platt Scaling`**, works as follows:
1.  Train a `primary model` (e.g., `SVM`) and generate `scores` on a `calibration set`.
2.  Train a `logistic regression` model where the `single input feature` is the `score` from the `primary model`: $P(C=1|s) = \sigma(as + b)$.

This `second model` learns a **`monotonic function`** that maps the `uncalibrated scores` to `well-calibrated posterior probabilities`, preserving the ordering and providing a correct probabilistic interpretation.

---

## Theory Question 8

**Describe the multinomial generative model for classification, covering the following aspects:**

*   **The type of data it is suited for and its main assumptions.**
*   **How the model parameters are estimated using `Maximum Likelihood Estimation (MLE)`.**
*   **The `Naive Bayes` approximation for handling multiple attributes.**
*   **The "`zero probability problem`" and how it can be addressed.**

### Answer

#### 1. Data Suitability and Assumptions

The multinomial generative model is designed for `classification tasks` involving `discrete` or `categorical features`. This is common in applications like text classification, where the `features` are often word counts from a vocabulary (the "`Bag-of-Words`" model).

*   Main Assumption: The model assumes that the `training` samples are `independent and identically distributed (i.i.d.)`. Each observation is considered an independent draw from the same `underlying probability distribution`.

#### 2. Parameter Estimation (`MLE`)

The model parameters are the conditional probabilities $\pi_{c,j} = P(X=j | C=c)$, which represent the probability of observing `feature` value $j$ given class $c$. These are estimated using `Maximum Likelihood Estimation (MLE)`.

*   Objective: The goal is to find the parameters $\pi_c$ for each class that maximize the `log-likelihood` of the observed `training` data.
*   Solution: The `MLE` estimate for $\pi_{c,j}$ is simply the `relative frequency` of `feature` $j$ within class $c$: $\pi_{c,j}^{\text{ML}} = \frac{N_{c,j}}{N_c}$, where $N_{c,j}$ is the `count` of `feature` $j$ in class $c$, and $N_c$ is the `total count` of all `features` in class $c$.

#### 3. `Naive Bayes` for Multiple Attributes

When dealing with multiple `discrete features`, modeling their joint probability is difficult due to the `curse of dimensionality`. The `Naive Bayes` approximation solves this by making a strong simplifying assumption: all `features` are conditionally independent given the class.

*   Simplification: This assumption allows the `joint conditional probability` to be factored into a product of individual `feature` probabilities: $P(x | C=c) \approx \prod_{j=1}^{D} P(x_j | C=c)$. This makes the model `computationally efficient` and robust against `data sparsity`, as each `feature`'s probability can be estimated independently.

#### 4. The "`zero probability problem`"

A `critical issue` arises if a `feature` value present in a test sample was never seen in the `training` data for a particular class. In this case, its `MLE` probability is 0, which forces the entire `class likelihood` to become 0, making classification for that class impossible.

*   Solution: This is addressed using `smoothing`. A common technique is to add a small `pseudo-count` $\alpha > 0$ (e.g., $\alpha=1$ for `Laplace smoothing`) to every `count`: $\pi_{c,j}^{\text{smoothed}} = \frac{N_{c,j} + \alpha}{N_c + m \alpha}$, where $m$ is the number of distinct `feature` values. This ensures that no probability is ever zero. Applying `pseudo-counts` is equivalent to performing `Maximum A Posteriori (MAP)` estimation with a `Dirichlet prior`.

---

## Theory Question 9

**Discuss the principles of Bayesian decisions and robust model evaluation, covering:**

*   **The limitations of accuracy as an `evaluation metric`.**
*   **The role of the `confusion matrix` and `prior-independent metrics (FPR, FNR)`.**
*   **The concept of `Bayes Risk` and the `Detection Cost Function (DCF)` for binary problems.**
*   **How to visualize `classifier performance` across all `decision thresholds`.**

### Answer

#### 1. Limitations of Accuracy

While intuitive, accuracy (the proportion of correct predictions) is often a `poor metric` for evaluating classifiers because:
*   It is highly sensitive to `class imbalance`. A classifier can achieve `high accuracy` on an `imbalanced dataset` by simply always predicting the `majority class`, even if it has no `real discriminative power`.
*   It treats all errors as equally costly, which is rarely true in real-world applications (e.g., a `false negative` in a medical diagnosis is far more severe than a `false positive`).
*   It depends on the `priors` of the `evaluation set`, which may not match the `priors` of the `target application`, making it a `poor estimator` of `future performance`.

#### 2. `Confusion Matrix` and `Prior-Independent Metrics`

The `confusion matrix` provides a `detailed breakdown` of `performance` by tabulating `True Positives (TP)`, `True Negatives (TN)`, `False Positives (FP)`, and `False Negatives (FN)`. From this, we can derive `robust`, `prior-independent metrics`:
*   `True Positive Rate (TPR)` or `Sensitivity`: The proportion of actual positives correctly identified. $TPR = \frac{TP}{TP + FN}$.
*   `True Negative Rate (TNR)` or `Specificity`: The proportion of actual negatives correctly identified. $TNR = \frac{TN}{TN + FP}$.
*   `False Positive Rate (FPR)`: The proportion of actual negatives incorrectly classified as positive. $FPR = \frac{FP}{FP + TN} = 1 - TNR$.
*   `False Negative Rate (FNR)`: The proportion of actual positives incorrectly classified as negative. $FNR = \frac{FN}{FN + TP} = 1 - TPR$.

These `rates` are intrinsic to the `classifier's performance` and are not affected by the `class distribution` in the `evaluation set`.

#### 3. `Bayes Risk` and `DCF`

Optimal decisions should aim to minimize the `Bayes Risk`, which is the `minimum expected cost` over the entire `application population`. For a binary problem, this is quantified by the `Detection Cost Function (DCF)`.
*   `Un-normalized DCF`: $DCF_u = \pi_T C_{FN} P_{fn} + (1 - \pi_T) C_{FP} P_{fp}$, where $\pi_T$ is the `application prior` for the positive class, and $C_{FN}, C_{FP}$ are the costs of `false negatives` and `false positives`.
*   `Normalized DCF`: The `DCF` is normalized by the cost of a "`dummy`" `system` that always predicts the `less costly outcome`. A $\text{DCF} < 1$ means the classifier is better than the `dummy system`.

#### 4. Visualizing `Performance`

To evaluate `classifier's performance` across all possible `decision thresholds`, we use:
*   `Receiver Operating Characteristic (ROC) Curve`: `Plots True Positive Rate` $(TPR = 1 - FNR)$ vs. `FPR`. The ideal point is the top-left corner $(TPR = 1, FPR = 0)$. The `Area Under the Curve (AUC)` summarizes the `overall discriminative power`.
*   `Detection Error Trade-off (DET) Curve`: `Plots FNR` vs. `FPR` on a `probability scale` (e.g., `normal deviate`). This is better for visualizing `performance differences` between `high-accuracy classifiers`, as it expands the `low-error regions`. The ideal point is the bottom-left corner.

---

## Theory Question 10

**Explain the concepts of `score calibration` and `fusion`, covering:**

*   **The problem of `mis-calibrated scores` and its impact on `decision-making`.**
*   **The difference between `minimum DCF` and `actual DCF`.**
*   **Common methods for `score calibration`.**
*   **The motivation and a common approach for `score-level fusion`.**

### Answer

#### 1. `Mis-calibrated Scores`

Many classifiers (e.g., `SVMs`) produce `raw scores` that are not `well-calibrated`, meaning they do not accurately represent true `posterior probabilities` or `Log-Likelihood Ratios (LLR)`. Applying the `theoretical Bayes decision threshold` to these `uncalibrated scores` is suboptimal because the `threshold` is based on `probabilistic assumptions` that the `scores` do not satisfy. This leads to a `higher-than-necessary cost (Bayes Risk)`.

#### 2. `Minimum DCF` vs. `Actual DCF`

The impact of `mis-calibration` is measured by comparing two `DCF` values:
*   `Minimum DCF (minDCF)`: This is the `lowest possible DCF` achievable for a given application, found by empirically searching for the `optimal threshold` on the `evaluation scores`. It measures the `classifier's intrinsic discriminative power`.
*   `Actual DCF (actDCF)`: This is the `DCF` obtained by applying the `theoretical Bayes threshold` (derived from `application priors` and `costs`) directly to the `raw scores`.
The `gap`, `actDCF - minDCF`, is the `calibration loss`. A `large gap` indicates `poor calibration`.

#### 3. `Score Calibration Methods`

The goal of `calibration` is to learn a `monotonic function` that transforms `raw scores` into `well-calibrated LLRs`. This allows the `theoretical Bayes threshold` to be applied effectively. Common methods include:
*   `Prior-Weighted Logistic Regression (Platt Scaling)`: A parametric method that learns an affine transformation $s_{\text{cal}} = as + b$ of the `scores`. It is simple and robust but assumes a linear relationship in the `log-odds` space.
*   `Isotonic Regression`: A non-parametric method that finds the best `non-linear` monotonic transformation. It is more flexible than `Platt Scaling` but can suffer from `overfitting` with small `calibration sets`.

#### 4. `Score-Level Fusion`

**Motivation**: Different classifiers often capture complementary information from the data. By combining their `output scores`, a fused system can achieve better `performance` than any single classifier.

**`Weighted Fusion`**: A common technique that computes a weighted combination of `scores` from multiple systems, typically implemented with `logistic regression`:

1.  The `scores` from the individual classifiers form a `feature vector` $s = [s_1, s_2, ..., s_K]$
2.  A `logistic regression` model learns the optimal weights ($\alpha$) and `bias` ($\gamma$): $s_{fused} = \alpha^T s + \gamma$
3.  This approach simultaneously combines and calibrates the `scores` from `multiple systems`.

---

## Theory Question 11

**Describe the `Principal Component Analysis (PCA)` dimensionality reduction approach, focusing on:**

*   **The model's objective and its formulation**
*   **The `training` objective function**
*   **The relationship between the data distribution and the `PCA` solution**
*   **Practical considerations in employing `PCA`**

### Answer

#### 1. Objective and Formulation

The objective of **`Principal Component Analysis (PCA)`** is to transform data from a high-dimensional `feature` space to a new, lower-dimensional space, while trying to preserve as much information as possible. This information is quantified as the **`variance`** of the data.

The model formulation is based on finding a projection matrix $P$ (of size $n \times m$, where $n$ is the original dimensionality and $m$ is the reduced one), whose columns are orthonormal basis vectors. The projection of a sample $x$ into the new space is given by $y = P^T x$, while its approximate reconstruction in the original space is $\hat{x} = P y$.

#### 2. `Training` Objective Function

The objective function that `PCA` optimizes is the **`minimization of the mean squared reconstruction error`**: $\frac{1}{K} \sum_{i=1}^K ||x_i - \hat{x}_i||^2$, where $K$ is the number of samples. This criterion ensures that the distance between the original points and their reconstructions is as small as possible.

Minimizing the reconstruction error is mathematically **`equivalent to maximizing the variance`** of the projected data $y$. In other words, `PCA` finds the directions along which the data spreads the most.

#### 3. Relationship between Data Distribution and `PCA` Solution

The `PCA` solution is intrinsically linked to the data distribution, described by its **covariance matrix** $C$. The optimal directions that form the projection matrix $P$ are the **`eigenvectors`** of the covariance matrix $C$ associated with the $m$ largest **`eigenvalues`**.

*   The **`eigenvectors`** (the `principal components`) represent the directions of maximum variance in the data space.
*   The corresponding **`eigenvalues`** quantify the amount of variance captured by each `principal component`.

In essence, `PCA` aligns the new coordinate system with the principal axes of the data's covariance ellipsoid.

#### 4. Practical Considerations

*   **`Data Centering`**: This is a crucial step. The data must be **`mean-centered`** $(x_i - \mu)$ before applying `PCA`. If the data is not centered, the first `principal component` will simply capture the average position of the data in space, rather than the direction of maximum variance.
*   **Selection of `m` (Number of Components)**: The choice of the `target` dimensionality `m` can be guided by two common approaches:
    1.  **`Percentage of Variance Retained`**: Choose the smallest `m` such that the sum of the `m` largest eigenvalues is a significant fraction (e.g., 95%) of the total variance (the sum of all eigenvalues).
    2.  **`Cross-Validation`**: If `PCA` is a `pre-processing` step for a classifier, `m` can be treated as a hyperparameter to be optimized on a `validation set` to maximize the classifier's `performance`.
*   **`Computational Complexity`**: For data with a very large number of `features`, calculating the `n x n` covariance matrix can be prohibitive. In these cases, alternative methods like **`Truncated SVD`** can be used, which calculates only the most important `principal components` more efficiently.

---

## Theory Question 12

**Describe the `Linear Discriminant Analysis (LDA)` dimensionality reduction approach, focusing on:**

*   **The model's objective and its formulation**
*   **The `training` objective function**
*   **The relationship between the data distribution and the `LDA` solution**
*   **Practical considerations in employing `LDA`**

### Answer

#### 1. Objective and Formulation

The objective of **`Linear Discriminant Analysis (LDA)`** is to find a lower-dimensional subspace that **maximizes the separability between classes**. Unlike `PCA`, `LDA` is a **`supervised`** method that uses class labels to find the directions that best discriminate the data.

The model formulation is based on finding a projection matrix $W$ that transforms the original data $x$ into a lower-dimensional space $y = W^T x$. This projection is optimized to maximize the ratio of the `between-class scatter` (the variance of the class means) to the `within-class scatter` (the variance within each class).

#### 2. `Training` Objective Function

The objective function that `LDA` optimizes is **`Fisher's criterion`**, which is the ratio of the `between-class scatter matrix` ($S_B$) to the `within-class scatter matrix` ($S_W$) in the projected space:

$$ J(W) = \frac{\det(W^T S_B W)}{\det(W^T S_W W)} $$

Maximizing this ratio leads to a **generalized eigenvalue problem**: $S_B w = \lambda S_W w$. The columns of the optimal projection matrix $W$ are the eigenvectors of $S_W^{-1} S_B$ corresponding to the largest eigenvalues.

#### 3. Relationship between Data Distribution and `LDA` Solution

The `LDA` solution depends directly on the distribution of the data within each class and between classes, captured by the `scatter` matrices:

*   **`Within-class scatter matrix` ($S_W$)**: It is the sum of the covariance matrices of each class, weighted by the number of samples. It measures how scattered the data is within each class.
    
    $$S_W = \sum_{c=1}^{C} N_c \Sigma_c$$
    
    where $\mu_c$ is the mean of class $c$, $\Sigma_c$ is the covariance matrix of class $c$, and $N_c$ is the number of samples of class $c$.

*   **`Between-class scatter matrix` ($S_B$)**: It measures the separation between the means of the different classes with respect to the global mean of the data.
    
    $$S_B = \sum_{c=1}^{C} N_c (\mu_c - \mu)(\mu_c - \mu)^T$$
    
    where $\mu$ is the global mean of all data.

`LDA` finds the directions that make the class clusters as compact as possible (minimizing $S_W$) and at the same time as far apart as possible from each other (maximizing $S_B$). The maximum number of discriminant directions that `LDA` can find is $C-1$, where $C$ is the number of classes.

#### 4. Practical Considerations

*   **`Data Assumptions`**: `LDA` implicitly assumes that the data of each class follows an approximately Gaussian distribution with similar covariance structures. It works best when the classes are well-separated and have a shape similar to an ellipsoid.
*   **`Singularity of Sw`**: The `within-class scatter` matrix $S_W$ can become singular (non-invertible) if the number of `features` is much larger than the number of samples. This is a common problem in high-dimensional `datasets`.
*   **`PCA+LDA Pipeline`**: A common solution to the singularity problem is to first apply `PCA` to reduce dimensionality and de-correlate the `features`. Subsequently, `LDA` is applied to the `PCA`-transformed data to find the most discriminant directions. This `PCA+LDA` `pipeline` is very powerful and robust.
*   **`Relationship with the Gaussian Classifier`**: `LDA` is closely related to the `Tied Covariance Gaussian` classifier. Both models lead to linear `decision boundaries`, as the assumption of a shared covariance in the Gaussian model is mathematically equivalent to the objective of maximizing the `scatter` ratio of `LDA`.

---

## Project Question Answers

### Project - `question example 1`

**Explain, in light of the characteristics of the classifiers and the characteristics of the project `dataset`:**

**1. The `relative performance` of the `MVG`, `Tied MVG`, and `GMM` models.**
**2. The `relative performance` of the `linear` and `non-linear SVMs`.**

### Answer

#### 1. `Generative Models` (`MVG`, `Tied MVG`, `GMM`):

*   **`MVG` vs. `Tied MVG`:** The standard **`Multivariate Gaussian (MVG)`** classifier, which models each class with a separate mean and `full covariance matrix`, consistently outperformed the **`Tied MVG`** model. The main assumption of the `Tied MVG` model is that both classes share a `single covariance matrix`. Its poorer `performance` suggests that this assumption is not valid for our `dataset`; the two classes likely have different variances and `feature correlations`, and the flexibility of the `full MVG` model in capturing these `distinct distributions` is crucial for better `performance`.
*   **`GMM` vs. `MVG`:** The **`Gaussian Mixture Model (GMM)`**, particularly with an `optimized number of components` and `diagonal covariance`, significantly outperformed the `single-component MVG model`. This indicates that the `underlying data distribution` for at least one of the classes is not a simple `unimodal Gaussian`. The `GMM`'s ability to model more complex, `multi-modal distributions` allowed it to capture the `data's structure` more accurately, leading to superior `classification performances`.

#### 2. `SVM Models` (`Linear` vs. `Non-linear`):

*   **`Linear` vs. `Non-linear SVM`:** The **`linear SVM`** provided a `reasonable baseline`, but was significantly outperformed by `SVMs` with `non-linear kernels`. This is strong evidence that the `dataset` **`is not linearly separable`** and that a more complex `decision boundary` is needed.
*   **`Polynomial` vs. `RBF SVM`:** Among the `non-linear kernels`, the **`RBF SVM`** achieved the `best performance`, surpassing the `polynomial kernel SVM`. Although the `polynomial kernel` confirmed the `benefit` of going beyond a `linear model`, the greater flexibility of the `RBF kernel` in creating highly complex, `localized decision boundaries` proved to be the `best match` for the intricate structure of the `project dataset`.

---

### Project - `question example 2`

**Explain the `relative performance` on the `project validation set` of the different `SVM kernels` (including linear models), in light of the `kernel`'s characteristics and the `dataset`'s characteristics. Briefly analyze the effects of `regularization` on `model performance`.**

### Answer

The `performance` of the different `SVM kernels` on the `project dataset` reveals a clear hierarchy, which is directly related to the complexity of the `decision boundary`.

*   **`Linear SVM`:** This model learns a simple hyperplane as a `decision boundary`. Its `performance` was the lowest among the tested `SVMs`, which strongly indicates that the two classes in our `dataset` **`are not linearly separable`**. It serves as a useful `baseline` but is insufficient for this `task`.
*   **`Polynomial SVM`:** Using a `polynomial kernel`, the `SVM` can learn a `non-linear`, `curved decision boundary`. This model showed a `significant improvement` over the `linear SVM`, confirming that a `non-linear` approach is necessary.
*   **`RBF SVM`:** The `RBF kernel` is the most flexible, capable of creating `complex`, `non-linear boundaries` of arbitrary shape. This model consistently achieved the **`best performance`** of all `SVMs`. Its superiority implies that the `optimal decision boundary` is highly complex and not well-approximated by simpler shapes like lines or parabolas.

**Effects of `Regularization`:**
`Regularization`, controlled by the hyperparameter $C$, manages the `trade-off` between maximizing the `margin` and minimizing classification errors on the `training` data.
*   **Very low $C$** (`strong regularization`): Causes `underfitting` as the model is overly constrained and fails to capture the `patterns` in the data.
*   **Very high $C$** (`weak regularization`): Risks `overfitting` on the `training data`, although it often improves `score calibration` by allowing the model to fit better.
*   **Intermediate $C$**: Optimal `performance` is achieved with intermediate values of $C$, which effectively balance `fitting` the `training data` with maintaining a wide `margin` for good generalization.

---

### Project - `question example 3`

**Consider the `SVM` and `logistic regression` classifiers. In light of the characteristics of the `datasets` and the classifiers, explain the `gap` between `minimum` and `actual DCF` for each model and, if necessary, the method you employed to reduce this `gap` for the `project dataset`. Also, analyze the effects of `regularization` on the `miscalibration error` for both models.**

### Answer

The `gap` between `minimum DCF` and `actual DCF` is a direct indicator of the **`quality of score calibration`**.

*   **`Logistic Regression`:** This model inherently produces well-calibrated `scores`. Its `training objective` (minimizing `negative log-likelihood`) directly encourages the model outputs to be accurate posterior probabilities. Consequently, the `gap` between `minDCF` and `actDCF` was consistently small.

*   **`SVM`:** In contrast, the `SVM`'s objective is to maximize the `margin`, not to produce probabilities. Its `scores` represent the signed distance from the decision hyperplane. These `scores` are generally **`poorly calibrated`**, resulting in a **`large gap`** between `minDCF` and `actDCF`. Applying the theoretical threshold to these raw `scores` leads to suboptimal decisions.

**Method to Reduce the `Gap` (Calibration):**
To address the `SVM`'s poor calibration, we trained a `prior-weighted logistic regression` on the `validation set`, using the `raw scores` from the `SVM` as input. This technique, called `Platt Scaling`, learns a simple monotonic function that maps the uncalibrated `SVM` `scores` to well-calibrated `log-likelihood ratios`. After this step, the `actDCF` of the calibrated `SVM` became much closer to its `minDCF`, significantly reducing the `calibration error` and making the classifier's outputs reliable for `decision-making`.

**Effects of `Regularization` on `Miscalibration`:**
*   For the **`SVM`**, `regularization` (parameter $C$) has a noticeable effect. We observed that as $C$ increased, the model tried to correctly classify the `training points`, and its `actDCF` tended to approach its `minDCF`, improving calibration.
*   For **`Logistic Regression`**, `regularization` (parameter $\lambda$) primarily serves to prevent `overfitting`. While extreme values could harm `performance`, its effect on the already good calibration was less pronounced compared to the `SVM`.

---

### Project - `question example 4`

**Given the following functions (assume these functions are already implemented unless specified):**

*   `trainPCA`: trains a `PCA` model
*   `applyPCA`: applies a `PCA` model to some data
*   `trainClassifier(D, L)`: trains a given classifier from the `data matrix D` and the `label vector L`; returns an object containing the trained `model parameters`
*   `scoreClassifier(clsModel, D)`: computes the `array of scores` for classifier `clsModel` (as returned by the function `trainClassifier`) for the `samples` in `data matrix D`
*   `evaluateScores(S, L)`: computes a `performance metric` (e.g. `minimum DCF`) over the `score array S` with `label vector L`

**a) Provide a possible `signature` and an `implementation` of the functions `trainPCA` and `applyPCA`, briefly explaining also the function parameters and the return value.**
**b) Using these functions, write the `Python code` to:**
*   **Train the classifier on a `training set`, optimizing the `PCA dimension` with respect to the provided `metric function` using a `single-fold cross-validation approach`**
*   **Evaluate its `performance` on an `evaluation set`.**

Assume that you have at your disposal a `training set`, already divided in `model training data (DTR, LTR)` and `validation data (DVAL, LVAL)`, and an `evaluation set (DTE, LTE)`. `DTR`, `DVAL` and `DTE` are `data matrices`, with `samples` organized as `column vectors`, whereas `LTR`, `LVAL` and `LTE` are `arrays` containing the corresponding `labels`. To select the `PCA dimension m` consider all possible values of `m` that are compatible with the dimension of the `feature vectors`. Assume that the classifier is `invariant to affine transformations`, that it does not include `hyper-parameters` to tune, and that `PCA` is the `only kind of pre-processing` to analyze.

### Answer

#### a) `Signature` and `Implementation` of `trainPCA` and `applyPCA`

```python
import numpy

def trainPCA(D, m):
    """
    Trains a PCA model by finding the top m principal components.
    
    Args:
        D (numpy.ndarray): The training data matrix, with samples as columns.
        m (int): The number of principal components to retain.
        
    Returns:
        tuple: A tuple containing:
            - P (numpy.ndarray): The projection matrix (m eigenvectors).
            - mu (numpy.ndarray): The mean vector of the training data.
    """
    mu = D.mean(1).reshape((D.shape[0], 1))
    DC = D - mu
    C = numpy.dot(DC, DC.T) / D.shape[1]
    s, U = numpy.linalg.eigh(C)
    P = U[:, ::-1][:, 0:m]
    return P, mu

def applyPCA(P, mu, D):
    """
    Applies a trained PCA model to project data.
    
    Args:
        P (numpy.ndarray): The projection matrix from trainPCA.
        mu (numpy.ndarray): The mean vector from the training data.
        D (numpy.ndarray): The data matrix to project, with samples as columns.
        
    Returns:
        numpy.ndarray: The projected data matrix.
    """
    DC = D - mu
    DP = numpy.dot(P.T, DC)
    return DP
```

#### b) `Code` for `PCA Optimization` and `Evaluation`

```python
best_m = -1
best_metric_val = float('inf')  # Assuming lower is better (e.g., minDCF)
best_P = None
best_mu = None

# --- Hyper-parameter (m) selection using validation set ---
for m in range(1, DTR.shape[0] + 1):
    # Train PCA on the training set
    P, mu = trainPCA(DTR, m)
    
    # Project both training and validation sets
    DTR_p = applyPCA(P, mu, DTR)
    DVAL_p = applyPCA(P, mu, DVAL)
    
    # Train classifier on projected training data
    clsModel = trainClassifier(DTR_p, LTR)
    
    # Score on projected validation data
    S_val = scoreClassifier(clsModel, DVAL_p)
    
    # Evaluate performance
    current_metric = evaluateScores(S_val, LVAL)
    
    # Check if this m is the best so far
    if current_metric < best_metric_val:
        best_metric_val = current_metric
        best_m = m
        best_P = P
        best_mu = mu

print(f"Optimal PCA dimension is m={best_m} with validation metric={best_metric_val}")

# --- Final model training and evaluation ---
# Train a new classifier on the full training data (DTR) projected with the optimal PCA
DTR_p_best = applyPCA(best_P, best_mu, DTR)
final_model = trainClassifier(DTR_p_best, LTR)

# Project the evaluation data (DTE) with the optimal PCA model
DTE_p_best = applyPCA(best_P, best_mu, DTE)

# Score the final model on the projected evaluation data
S_test = scoreClassifier(final_model, DTE_p_best)

# Compute final performance on the evaluation set
final_metric = evaluateScores(S_test, LTE)
print(f"Performance on evaluation set: {final_metric}")
```

---

### Project - `question example 5`

**You are given the following functions (assume these functions are already implemented unless specified):**

*   `trainRBFKernelSVM(D, L, C, gamma)`: trains an `SVM model` with an `RBF kernel` with `hyper-parameter gamma` and returns an object containing the trained `model information`; `D` is the `training data matrix`, `L` is the corresponding `label array`, and `C` is the `SVM cost-vs-margin trade-off coefficient`
*   `scoreRBFKernelSVM(svmModel, D)`: computes the `classification scores` for `samples` in the `data matrix D` for an `SVM model svmModel` (as returned by the function `trainRBFKernelSVM`) and returns an `array of scores`
*   `evaluateScores(S, L)`: computes an `evaluation metric` (e.g. `minimum DCF`) over the `array of scores S` with associated `array of labels L`

**Write the `Python code` to train and apply an `SVM classifier`. In particular, the `code` should**

*   **Train an `SVM classifier`, optimizing the `value` of the `hyper-parameters` with respect to the `metric function evaluateScores` using a `single-fold cross-validation approach`.**
*   **Evaluate the selected `SVM model` on the `evaluation data`, using the provided `metric`.**

Write an `implementation` of `scoreRBFKernelSVM(svmModel, D)`. Assume that `svmModel` is an object with the following fields: `sv`, `alpha`, `labels`, `gamma`. You can assume that you have at your disposal a function `RBFKernel(D1, D2, gamma)` that returns the `matrix of kernel values k(x, y)` for all pairs of `samples x, y` of `2-D sample matrices D1, D2`.

### Answer

#### `Code` for `SVM Optimization` and `Evaluation`

```python
C_values = [1e-3, 1e-2, 1e-1, 1.0]
gamma_values = [1e-3, 1e-2, 1e-1]

best_C = None
best_gamma = None
best_metric_val = float('inf') # Assuming lower metric is better (e.g., minDCF)

# --- Hyper-parameter tuning using validation set ---
for C in C_values:
    for gamma in gamma_values:
        # Train the SVM model on the training data
        svmModel = trainRBFKernelSVM(DTR, LTR, C, gamma)
        
        # Compute scores on the validation data
        S_val = scoreRBFKernelSVM(svmModel, DVAL)
        
        # Evaluate the performance
        current_metric = evaluateScores(S_val, LVAL)
        
        # Update the best hyperparameters if performance improved
        if current_metric < best_metric_val:
            best_metric_val = current_metric
            best_C = C
            best_gamma = gamma

# --- Final model evaluation on test set ---
final_svm_model = trainRBFKernelSVM(DTR, LTR, best_C, best_gamma)
S_test = scoreRBFKernelSVM(final_svm_model, DTE)
final_performance = evaluateScores(S_test, LTE)
```

#### `Implementation` of `scoreRBFKernelSVM`

```python
import numpy

def scoreRBFKernelSVM(svmModel, D):
    """
    Computes classification scores for an RBF SVM model.
    The score is calculated as f(x) = sum(alpha_i * y_i * K(sv_i, x)).
    
    Args:
        svmModel (object): An SVM model with fields: sv, alpha, labels, gamma.
        D (numpy.ndarray): The data matrix to score (features x samples).
    
    Returns:
        numpy.ndarray: A 1-D array of scores.
    """
    # Create the kernel matrix between support vectors (from training) and new data D
    # sv shape: (num_features, num_sv)
    # D shape: (num_features, num_test_samples)
    # kernel_matrix shape: (num_sv, num_test_samples)
    kernel_matrix = RBFKernel(svmModel.sv, D, svmModel.gamma)
    
    # The dual solution weights are alpha_i * y_i for each support vector
    # alpha shape: (num_sv,)
    # labels shape: (num_sv,)
    # weighted_alphas shape: (num_sv,)
    weighted_alphas = svmModel.alpha * svmModel.labels
    
    # Compute the final scores by summing the weighted kernel values for each test sample
    # dot product of (1, num_sv) with (num_sv, num_test_samples) -> (1, num_test_samples)
    scores = numpy.dot(weighted_alphas, kernel_matrix)
    
    return scores.flatten()
```

---

### Project - `question example 6`

**Consider a `binary classification problem`, with classes labeled as 1 and 0, respectively. Let `(DTR, LTR)`, `(DVAL, LVAL)` represent a `labeled training set` and a `labeled validation set`. Let also `DTE` represent the `dataset matrix` containing the `samples` that our `application` should `classify`. Write a `Python code fragment` that:**

1.  **trains a `calibrated binary classifier`**
2.  **performs `inference` (i.e. computes predicted `labels`) on the `evaluation data`**

**You can assume that the following functions have been defined:**

*   `trainClassifier(D, L)`: train a `non-calibrated classification model` (e.g., an `SVM` or an `LDA classifier`) on the `training matrix D` with associated `labels array L`, and return a `python object` containing the trained `model`
*   `scoreClassifier(model, D)`: compute the `non-calibrated classification scores` for `model model` for the `samples` in `data matrix D` and return a `1-D array of scores`
*   `trainCalibrationModel(S, L, prior)`: train a `calibration model` on the `1-D array of scores S`, with associated `array of labels L`, for a `binary application` with `prior prior` for `class 1`, and return a `python object` containing the trained `model`
*   `applyCalibrationModel(calModel, S)`: apply the `calibration model calModel` to the `1-D array of scores S`, and return a `1-D array of calibrated scores`

NOTE: assume that the `target application` is characterized by an `effective prior p` for `class 1`. You are not required to tune the `calibration model hyper-parameter prior`, but you can assume that the `calibration model` can be trained using the `target application prior p`.

### Answer

```python
import numpy

# Assume DTR, LTR, DVAL, LVAL, DTE are pre-defined.
# Assume p (prior for class 1) is a known float.

# --- 1. Train the calibrated binary classifier ---

# First, train the base (non-calibrated) classifier on the training set.
# The validation set (DVAL, LVAL) is kept separate for calibration.
base_model = trainClassifier(DTR, LTR)

# Second, generate scores on the validation set using the base model.
scores_for_calibration = scoreClassifier(base_model, DVAL)

# Third, train the calibration model. It learns to map the raw scores
# to calibrated log-likelihood ratios, using the validation scores,
# their true labels, and the application prior.
calibration_model = trainCalibrationModel(scores_for_calibration, LVAL, p)

# The "calibrated classifier" is the combination of base_model and calibration_model.

# --- 2. Perform inference on the evaluation data ---

# First, get the raw scores for the evaluation data (DTE) from the base model.
raw_scores_test = scoreClassifier(base_model, DTE)

# Second, apply the trained calibration model to these raw scores to get
# calibrated log-likelihood ratios.
calibrated_scores_test = applyCalibrationModel(calibration_model, raw_scores_test)

# Finally, compute the predicted labels. For calibrated scores (log-likelihood ratios),
# the optimal decision threshold for a given application is derived from the prior.
# The threshold is -numpy.log(p / (1 - p)).
threshold = -numpy.log(p / (1 - p))

# Assign label 1 if score > threshold, otherwise 0.
predicted_labels = (calibrated_scores_test > threshold).astype(int)

# `predicted_labels` now contains the final class predictions for the DTE samples.
```

---

### Project - `question example 7`

**Given the following functions (assume these functions are already implemented unless specified):**

*   `trainPCA`, `applyPCA`, `trainLDA`, `applyLDA`
*   `evaluateScores(S, L)`: computes a `performance metric`

**1. Provide possible `signatures` (prototype) for these functions, briefly explaining the function parameters and the return value.**
**2. Using these functions, write a short `Python program` to train and apply an `LDA binary classifier` with `PCA pre-processing`. The `program` should employ the provided data to train the `model` and to select an `optimal value` for the `PCA dimensionality`.**

### Answer

#### 1. `Function Signatures` (`Prototypes`)

```python
def trainPCA(D, m):
    """
    Trains a PCA model.
    Args:
        D (numpy.ndarray): Training data matrix (features x samples).
        m (int): The number of principal components to keep.
    Returns:
        tuple: The projection matrix (P) and the mean vector (mu).
    """
    pass

def applyPCA(P, mu, D):
    """
    Applies a trained PCA model to project data.
    Args:
        P (numpy.ndarray): Projection matrix from trainPCA.
        mu (numpy.ndarray): Mean vector from the training data.
        D (numpy.ndarray): Data matrix to project (features x samples).
    Returns:
        numpy.ndarray: The projected data matrix.
    """
    pass

def trainLDA(D, L, m_lda=1):
    """
    Trains an LDA model.
    Args:
        D (numpy.ndarray): Training data matrix (features x samples).
        L (numpy.ndarray): Training labels array.
        m_lda (int): Number of discriminant directions (for binary case, it's 1).
    Returns:
        numpy.ndarray: The LDA projection matrix (W).
    """
    pass

def applyLDA(W, D):
    """
    Applies a trained LDA model to project data, yielding scores.
    Args:
        W (numpy.ndarray): LDA projection matrix from trainLDA.
        D (numpy.ndarray): Data matrix to project (features x samples).
    Returns:
        numpy.ndarray: The 1-D array of classification scores.
    """
    pass

def evaluateScores(S, L):
    """
    Computes a performance metric for given scores and labels.
    Args:
        S (numpy.ndarray): 1-D array of classification scores.
        L (numpy.ndarray): 1-D array of true labels.
    Returns:
        float: The computed performance metric (e.g., minDCF).
    """
    pass
```

#### 2. `Python Program` for `PCA+LDA` with Dimensionality Selection

```python
best_pca_dim = -1
best_metric = float('inf')  # Assuming lower metric is better (e.g., minDCF)

# Iterate through possible PCA dimensions
for m in range(1, DTR.shape[0] + 1):
    # Train PCA on the training set
    P, mu = trainPCA(DTR, m)

    # Apply the same PCA transformation to both training and validation sets
    DTR_pca = applyPCA(P, mu, DTR)
    DVAL_pca = applyPCA(P, mu, DVAL)

    # Train LDA on the PCA-projected training data
    # For a binary problem, we only seek 1 discriminant direction.
    W_lda = trainLDA(DTR_pca, LTR, 1)

    # Compute scores by applying LDA to the PCA-projected validation data
    S_val = applyLDA(W_lda, DVAL_pca)

    # Evaluate the performance for this value of m
    current_metric = evaluateScores(S_val, LVAL)

    # Update the best PCA dimension if performance improved
    if current_metric < best_metric:
        best_metric = current_metric
        best_pca_dim = m
```

---

### Project - `question example 8` (`Variation` of `PQ4`: `LDA` for Dimensionality Reduction)

**Given a `multi-class classification problem`, consider the following functions:**

*   `trainLDA`: trains an `LDA model` for `dimensionality reduction`.
*   `applyLDA`: applies a trained `LDA model` to project data.
*   `trainClassifier(D_reduced, L)`: trains a `generic classifier` on the `reduced-dimension data`.
*   `scoreClassifier(clsModel, D_reduced)`: computes `classification scores` using the `classifier`.
*   `evaluateScores(S, L)`: computes a `performance metric` (e.g., `multi-class accuracy`).

**a) Provide a possible `signature` and an `implementation` for the functions `trainLDA` and `applyLDA` for `dimensionality reduction`, briefly explaining the function parameters and the return value.**
**b) Using these functions, write the `Python code` to train a `classifier`, optimizing the `dimensionality of LDA m` with a `single-fold validation approach`. Assume the `number of classes C` is known.**

### Answer

#### a) `Signature` and `Implementation` of `trainLDA` and `applyLDA`

```python
import numpy
import scipy.linalg

def trainLDA(D, L, m):
    """
    Trains an LDA model for dimensionality reduction by computing the
    m directions that maximize the ratio of between-class to within-class scatter.
    
    Args:
        D (numpy.ndarray): The training data matrix (features x samples).
        L (numpy.ndarray): The training labels vector.
        m (int): The number of discriminant directions to retain.
        
    Returns:
        numpy.ndarray: The LDA projection matrix W of shape (num_features, m).
    """
    # 1. Calculate scatter matrices with optimized single loop
    mu_total = D.mean(1).reshape(D.shape[0], 1)
    SW = numpy.zeros((D.shape[0], D.shape[0]))
    SB = numpy.zeros((D.shape[0], D.shape[0]))
    
    # Single loop to calculate both SW and SB
    for i in numpy.unique(L):
        D_class = D[:, L == i]
        mu_class = D_class.mean(1).reshape(D.shape[0], 1)
        nc = D_class.shape[1]
        
        # Within-class scatter for this class
        centered_data = D_class - mu_class
        SW += centered_data @ centered_data.T
        
        # Between-class scatter for this class
        diff_mean = mu_class - mu_total
        SB += nc * (diff_mean @ diff_mean.T)
        
    # 2. Solve the generalized eigenvalue problem: SB * v = lambda * SW * v
    # We use scipy.linalg.eigh which solves A*x = lambda*B*x
    # Note: eigh returns eigenvalues in ascending order.
    eigenvalues, eigenvectors = scipy.linalg.eigh(SB, SW)
    
    # 3. Select the m eigenvectors with the largest eigenvalues
    # We flip the order to get descending eigenvalues.
    W = eigenvectors[:, ::-1][:, 0:m]
    
    return W

def applyLDA(W, D):
    """
    Applies a trained LDA model to project data into a lower-dimensional space.
    
    Args:
        W (numpy.ndarray): The LDA projection matrix from trainLDA.
        D (numpy.ndarray): The data matrix to project (features x samples).
        
    Returns:
        numpy.ndarray: The projected data matrix with m features.
    """
    # Project the data: D_projected = W^T * D
    return numpy.dot(W.T, D)
```

#### b) `Code` for `LDA Dimensionality Optimization` and `Evaluation`

```python
best_m_lda = -1
best_metric_val = -float('inf') # Assuming higher metric is better (e.g., accuracy)

# The maximum number of LDA dimensions is C-1.
max_lda_dims = C - 1

# Iterate through possible LDA dimensions
for m_lda in range(1, max_lda_dims + 1):
    # 1. Train LDA for dimensionality reduction on the training set
    lda_reducer_model_W = trainLDA(DTR, LTR, m_lda)
    
    # 2. Project both training and validation sets
    DTR_reduced = applyLDA(lda_reducer_model_W, DTR)
    DVAL_reduced = applyLDA(lda_reducer_model_W, DVAL)
    
    # 3. Train a generic classifier on the reduced-dimension training data
    classifier = trainClassifier(DTR_reduced, LTR)
    
    # 4. Score the classifier on the reduced-dimension validation data
    S_val = scoreClassifier(classifier, DVAL_reduced)
    
    # 5. Evaluate the performance
    current_metric = evaluateScores(S_val, LVAL)
    
    # 6. Check if this m_lda is the best so far
    if current_metric > best_metric_val:
        best_metric_val = current_metric
        best_m_lda = m_lda
```

---

### Project - `question example 9` (`Variation` of `PQ5`: `Logistic Regression`)

**You are given the following functions (assume these functions are already implemented unless specified):**

*   `trainLogisticRegression(D, L, lambda_reg)`: trains a `binary logistic regression model`.
*   `scoreLogisticRegression(lrModel, D)`: computes `classification scores` for the `model`.
*   `evaluateScores(S, L)`: computes an `evaluation metric` (e.g. `minimum DCF`).

**Write the `Python code` to train and apply a `Logistic Regression classifier`, optimizing the `hyper-parameter lambda_reg` using `single-fold cross-validation`. Then, evaluate the selected `model` on the `test data`. Additionally, provide an `implementation` for `scoreLogisticRegression`, assuming `lrModel` is an object with fields `w` (`weights`) and `b` (`bias`).**

### Answer

#### `Code` for `Logistic Regression Optimization` and `Evaluation`

```python
lambda_values = [1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1.0]

best_lambda = None
best_metric_val = float('inf') # Assuming lower metric is better (e.g., minDCF)

# --- Hyper-parameter tuning using validation set ---
for l in lambda_values:
    # Train the logistic regression model on the training data
    lrModel = trainLogisticRegression(DTR, LTR, l)
    
    # Compute scores on the validation data
    S_val = scoreLogisticRegression(lrModel, DVAL)
    
    # Evaluate the performance
    current_metric = evaluateScores(S_val, LVAL)
    
    # Update the best hyperparameter if performance improved
    if current_metric < best_metric_val:
        best_metric_val = current_metric
        best_lambda = l
```

#### `Implementation` of `scoreLogisticRegression`

```python
import numpy

def scoreLogisticRegression(lrModel, D):
    w = lrModel.w
    b = lrModel.b
    
    # s = w^T * D + b
    scores = numpy.dot(w.T, D) + b
    
    # Return a flat 1-D array of scores
    return scores.flatten()
```
