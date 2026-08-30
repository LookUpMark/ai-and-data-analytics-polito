# Support Vector Machines (SVM): A Geometrically Intuitive Classifier

> **Author**
Marc'Antonio Lopez
AI & Data Analytics student at Polytechnic University of Turin

Support Vector Machines (SVMs) are highly effective **binary classifiers** renowned for their distinctive **geometric approach**. Unlike probabilistic models such as Logistic Regression, which estimate class posterior probabilities ($P(C \mid X)$), SVMs operate differently. Their core objective is to identify a specific **decision boundary** (a hyperplane) that creates the **largest possible margin** (or separation gap) between the two distinct classes. This inherent focus on maximizing the margin acts as a powerful, built-in **regularization mechanism**, naturally helping prevent overfitting and promote good generalization.

## Binary Classification and the Need for Regularization

**Binary classification** is a fundamental task in machine learning where data points are assigned to one of two predefined categories. Logistic Regression, a common binary classifier, learns a linear decision rule ($w^Tx + b = 0$) by minimizing a specific loss function called the **logistic loss**.

**Regularization** is a crucial technique used to prevent **overfitting**. Overfitting occurs when a model becomes overly complex, memorizing the intricacies of the training data, including its noise and random fluctuations. While this leads to excellent performance on the training data, it results in poor and unreliable performance on new, unseen data. The risk of overfitting is particularly high when dealing with perfectly linearly separable data or when operating in high-dimensional feature spaces.

**Example: Logistic Regression without Regularization**
Without regularization (i.e., when the regularization parameter $\lambda = 0$), the objective function for Logistic Regression is:
$$ \min_{w,b} \left[ (1/n) \sum_{i=1 \text{ to } n} \log(1 + e^{-z_i(w^Tx_i + b)}) \right] $$
If the data is perfectly linearly separable, the logistic loss can theoretically approach zero as the magnitude of the weight vector $||w||$ approaches infinity. This situation leads to several problems:

*   **Ill-posed optimization problem:** The optimization objective (log-likelihood) would continue to increase indefinitely, and the algorithm might not converge.
*   **Overfitting:** The model becomes overly confident in its training predictions, leading to perfect training accuracy but poor generalization.
*   **No unique solution:** Many hyperplanes could perfectly separate the data, leading to multiple possible solutions for $(w, b)$.

Regularization techniques like L2 regularization (adding $(\lambda/2) ||w||^2$ to the objective) address this by penalizing large weights, ensuring a unique and stable optimal hyperplane.

## Support Vector Machine: A Geometric Approach to Regularization

SVMs offer a distinct and powerful approach to classification, characterized by a clear **geometric interpretation of regularization**. They are exceptionally adept at identifying effective decision boundaries for both **linearly separable** and **non-linearly separable** datasets. The ability to handle non-linearity is achieved elegantly through the use of **kernels**.

**Key characteristics of SVMs include:**

1.  **Generalized Risk Minimization:** SVMs are fundamentally designed to ensure strong **generalization capabilities** on unseen data, with their objective function explicitly formulated to achieve this.
2.  **Efficient Non-linear Separation:** SVMs can find complex non-linear decision boundaries very efficiently by leveraging the innovative **"kernel trick"** (explained later).
3.  **Output Interpretation:** It's important to note that the raw scores produced by an SVM (representing the signed distance from the decision boundary) are **not directly interpretable as class posterior probabilities**. If probabilistic outputs are required, post-processing steps are necessary.

## Linearly Separable Classes: The Problem of Choice

When a dataset is **linearly separable**, it means a straight line (in 2D), a flat plane (in 3D), or a hyperplane (in higher dimensions) can perfectly divide the data points into their respective classes. In such a scenario, an **infinite number** of hyperplanes can achieve this perfect separation.

The challenge then becomes choosing the *best* among these infinitely many hyperplanes to ensure effective generalization to new data. Different hyperplanes, while perfectly separating the training data, will exhibit varying degrees of stability and robustness to unseen data. Unregularized models lack a principled mechanism for making this crucial selection; they might pick any separating hyperplane without considering its robustness.

## Maximizing the Margin: The SVM's Solution

The core principle distinguishing SVMs and solving the "problem of choice" for linearly separable data is to select the hyperplane that separates classes with the **largest possible "margin."**

*   **Margin Defined:** The margin refers to the **minimum perpendicular distance** from the separating hyperplane ($wᵀx + b = 0$) to any data point from either class. Conceptually, imagine drawing the widest possible "corridor" or "street" between the two classes such that no data points fall within this corridor. The separating hyperplane is then optimally positioned exactly in the middle of this corridor.
*   **Support Vectors (SVs):** These are the critical data points that lie exactly on the edges of this margin (i.e., they are the data points closest to the separating hyperplane). Crucially, these Support Vectors are the **sole points that determine the optimal hyperplane's position and orientation**. All other data points, lying further away from the margin, do not influence the final decision boundary.

**Mathematical Formulation for Maximizing the Margin:**
The perpendicular distance from a data point $x_i$ to the hyperplane $wᵀx + b = 0$ is given by $\frac{|wᵀx_i + b|}{||w||}$. For correctly classified points, the condition $z_i(wᵀx_i + b) > 0$ holds, where $z_i$ is the class label (+1 or -1). The SVM's objective is to maximize this minimum geometric margin across all data points:

$$
\max_{w,b} \left( \min_{i} \left[ \frac{z_i(wᵀx_i + b)}{||w||} \right] \right)
$$

This problem is simplified by fixing the minimum **functional margin** ($z_i(wᵀx_i + b)$) to 1. This scaling can be achieved by appropriately scaling the weight vector $w$ and bias $b$ without changing the hyperplane itself (since if $(w,b)$ defines a hyperplane, then $(\beta w, \beta b)$ defines the same hyperplane for any $\beta > 0$). With this constraint, the maximization problem simplifies to maximizing $1/||w||$, which is mathematically equivalent to **minimizing $(1/2)||w||^2$**. This minimization is subject to the condition that $z_i(wᵀx_i + b) \ge 1$ for all training points $i$.

## The Primal SVM Objective: Hard Margin SVM

For datasets that are **perfectly linearly separable** (i.e., no training points fall inside the margin or are misclassified), the "hard margin" SVM aims to find the hyperplane that maximizes the margin without any violations.

The primal optimization problem for a hard margin SVM is formulated as:

**Minimize:** $ (1/2) ||w||^2 $ (with respect to the parameters $w$ and $b$)

**Subject to:** $ z_i(wᵀx_i + b) \ge 1 $, for all training samples $i=1, \dots, n$.

*   **Convex Quadratic Programming (QP) Problem:** This formulation constitutes a **convex quadratic programming problem**. This type of problem has desirable properties:
    *   It guarantees a **unique global minimum** (i.e., a unique optimal hyperplane).
    *   It can be solved efficiently using specialized optimization algorithms.
    *   This unique solution is guaranteed provided that a feasible solution exists (i.e., the data is indeed linearly separable).

## Solving the SVM Problem: The Dual Formulation and KKT Conditions

The constrained quadratic programming problem defining the SVM is typically solved by transforming it into its **Lagrangian formulation** and then converting that into its **dual problem**. This dual formulation is computationally more tractable, especially when dealing with non-linear SVMs via kernels.

1.  **Lagrangian:** We define the Lagrangian function by introducing Lagrange multipliers $\alpha_i \ge 0$ for each constraint:
    $$ L(w, b, \alpha) = (1/2)||w||^2 - \sum_{i=1}^n \alpha_i [z_i(wᵀx_i + b) - 1] $$
2.  **Minimizing with respect to $w, b$ (Primal Variables):** To find the minimum of the Lagrangian, we take partial derivatives with respect to $w$ and $b$ and set them to zero:
    *   $\frac{\partial L}{\partial w} = w - \sum_{i=1}^n \alpha_i z_i x_i = 0 \quad \Rightarrow \quad \mathbf{w = \sum_{i=1}^n \alpha_i z_i x_i}$
        *   **Insight:** This crucial condition shows that the optimal weight vector $w$ is a linear combination of the training points, where each point's contribution is weighted by its corresponding Lagrange multiplier $\alpha_i$ and its class label $z_i$.
    *   $\frac{\partial L}{\partial b} = - \sum_{i=1}^n \alpha_i z_i = 0 \quad \Rightarrow \quad \mathbf{\sum_{i=1}^n \alpha_i z_i = 0}$
        *   **Insight:** This imposes a linear constraint on the sum of the Lagrange multipliers.
3.  **Dual Problem:** By substituting these conditions back into the Lagrangian expression, we eliminate $w$ and $b$ and obtain the dual problem, which involves maximizing with respect to the Lagrange multipliers $\alpha$:
    **Maximize:** $ W(\alpha) = \sum_{i=1}^n \alpha_i - (1/2) \sum_{i=1}^n \sum_{j=1}^n \alpha_i \alpha_j z_i z_j (x_i^T x_j) $
    **Subject to:** $ \alpha_i \ge 0 $ for all $i$, and $ \sum_{i=1}^n \alpha_i z_i = 0 $.
    *   **Key Characteristic:** The dual problem's dependence solely on **dot products** ($x_i^Tx_j$) between pairs of training data points is a pivotal characteristic, ultimately enabling the powerful "kernel trick."

**Karush-Kuhn-Tucker (KKT) Conditions:**
The KKT conditions provide necessary and sufficient conditions for optimality in constrained optimization problems like the SVM. The **Complementary Slackness** condition is particularly insightful for SVMs:
$$ \alpha_i^* [z_i(w^{*T}x_i + b^*) - 1] = 0 $$
This condition states that for any given training point $i$, either its Lagrange multiplier $\alpha_i^*$ is zero, or the term in the square bracket is zero. This leads to two critical cases:

*   **If $ \alpha_i^* > 0 $:** Then it must be that $ z_i(w^{*T}x_i + b^*) - 1 = 0 $, which means $ z_i(w^{*T}x_i + b^*) = 1 $. These specific data points are precisely the **Support Vectors (SVs)**, as they lie exactly on the margin boundaries. These are the points that constrain the margin.
*   **If $ z_i(w^{*T}x_i + b^*) > 1 $:** (meaning the point is correctly classified and lies strictly outside the margin, thus not affecting the margin boundary), then it must be that $ \alpha_i^* = 0 $. These are non-Support Vectors and consequently do not influence the determination of the decision boundary.

**Implications of KKT Conditions:**
A significant implication of these conditions is that the optimal weight vector $w^*$ is determined **solely by the support vectors**:
$$ w^* = \sum_{i \in \text{SVs}} \alpha_i^* z_i x_i $$
(where the sum is only over those $i$ for which $\alpha_i^* > 0$). The bias term $b^*$ can then be readily found by picking any support vector $x_j$ (where $\alpha_j^* > 0$) and solving $z_j(w^{*T}x_j + b^*) = 1$ for $b^*$.

For making **predictions** on a new data point $x_t$, the decision score $s(x_t)$ is calculated as:
$$ s(x_t) = w^{*T}x_t + b^* = \sum_{i \in \text{SVs}} \alpha_i^* z_i (x_i^T x_t) + b^* $$
This formula further emphasizes SVM's **sparsity**: the prediction depends only on the dot products between the new data point and the support vectors, making SVMs efficient for prediction once trained, especially if the number of support vectors is small.

## Soft Margin SVM for Non-Linearly Separable Data

Most real-world data is **not perfectly linearly separable**. The **Soft Margin SVM** is an extension of the hard margin concept designed to address this reality. It allows for some training points to violate the margin constraints (i.e., fall inside the margin or even be misclassified) but simultaneously introduces a penalty for these violations in the objective function.

1.  **Slack Variables $\xi_i \ge 0$ (Xi):** For each data point $x_i$, a non-negative slack variable $\xi_i$ is introduced, quantifying the extent to which the point violates the margin constraint.
2.  **Relaxed Constraints:** The hard margin constraint is relaxed to:
    $$ z_i(w^Tx_i + b) \ge 1 - \xi_i $$
    The interpretation of $\xi_i$ values is as follows:
    *   If $ \xi_i = 0 $: The point is correctly classified and lies on or outside the margin boundary.
    *   If $ 0 < \xi_i \le 1 $: The point is correctly classified, but falls *inside* the margin (a soft violation).
    *   If $ \xi_i > 1 $: The point is **misclassified** (it lies on the wrong side of the decision boundary).

**Soft Margin SVM Objective:** The primal objective function is modified to minimize both the margin width (by minimizing $||w||^2$) and the total sum of margin violations ($\sum \xi_i$).

**Minimize:** $ (1/2)||w||^2 + C \sum_{i=1 \text{ to } n} \xi_i $ (with respect to $w, b, \xi_i$)
**Subject to:** $ z_i(wᵀx_i + b) \ge 1 - \xi_i $ and $ \xi_i \ge 0 $ for all $i$.

*   **Regularization Hyperparameter C:** In this formulation, $C > 0$ is a critical **hyperparameter**. It controls the trade-off between:
    *   **Maximizing the margin width** (by keeping $||w||$ small).
    *   **Minimizing the penalty for margin violations** ($\sum \xi_i$).
    *   A **small C** allows more violations (larger margin, more regularization), potentially leading to underfitting.
    *   A **large C** imposes a higher penalty for violations (smaller margin, less regularization), risking overfitting.

This formulation is also known as an **L1-regularized SVM** because the penalty term $C \sum \xi_i$ is equivalent to adding an L1-norm penalty on the slack variables. The term $\xi_i$ can also be expressed as $\max(0, 1 - z_i(w^Tx_i + b))$, which is precisely the **hinge loss function**. Consequently, the primal problem of the Soft Margin SVM is equivalent to minimizing:

$$ (1/2)||w||^2 + C \sum_{i=1 \text{ to } n} \max(0, 1 - z_i(wᵀx_i + b)) $$

### Hinge Loss vs. Logistic Loss: A Comparison

The **Hinge Loss** is central to SVMs, while **Logistic Loss** is fundamental to Logistic Regression. Comparing them highlights their distinct properties regarding how they penalize incorrect predictions and influence model sparsity:

<p align="center">

| Feature           | Hinge Loss: $ \max(0, 1 - s) $               | Logistic Loss: $ \log(1 + e^{-s}) $              |
| :---------------- | :------------------------------------------- | :----------------------------------------------- |
| **Value for $ s \ge 1 $** | 0 (no penalty for correctly classified points outside or on margin) | Positive, asymptotically approaches 0 (continues to penalize, but very lightly, even for highly correct points) |
| **Value for $ s < 1 $** | $ 1 - s $ (linear penalty, increases as $s$ decreases) | Positive, increases more rapidly than linearly as $s$ decreases |
| **Smoothness**      | Non-smooth (not differentiable) at $ s = 1 $ (requires specialized optimization) | Smooth and differentiable (everywhere, allowing gradient-based methods directly) |
| **Sparsity**        | Can lead to sparse solutions ($\alpha_i = 0$ for non-SVs), meaning only SVs influence boundary | Generally does not lead to sparse $\alpha_i$ (all points contribute slightly to parameter updates) |

</p>

**Dual Problem for Soft Margin SVM:** The dual formulation for the Soft Margin SVM is very similar to that of the hard margin case, but with an important added constraint on the Lagrange multipliers $\alpha_i$:

**Maximize:** $ W(\alpha) = \sum_{i=1 \text{ to } n} \alpha_i - (1/2) \sum_{i=1 \text{ to } n} \sum_{j=1 \text{ to } n} \alpha_i \alpha_j z_i z_j (x_i^T x_j) $
**Subject to:** $ 0 \le \alpha_i \le C $ for all $i$, and $ \sum_{i=1 \text{ to } n} \alpha_i z_i = 0 $.
The new upper bound $C$ on $\alpha_i$ is directly related to the hyperparameter $C$ from the primal objective, linking the slack variables to the dual solution.

**Interpretation of $ \alpha_i^* $ in Soft Margin SVM:**
The values of the optimal Lagrange multipliers $ \alpha_i^* $ provide crucial insight into the role of each data point in defining the decision boundary:

*   $ \alpha_i^* = 0 $: These points are **non-support vectors**. They are correctly classified and lie strictly outside the margin. They do not contribute to the definition of the decision boundary.
*   $ 0 < \alpha_i^* < C $: These are **support vectors** that lie precisely on the margin boundary ($z_i(w^{*T}x_i + b^*) = 1$). They are the "critical" points defining the margin.
*   $ \alpha_i^* = C $: These are also **support vectors**, but they are points that actively violate the margin constraint (either lying within the margin or being misclassified). These points are heavily penalized by the $C$ term in the objective.

**Crucially, all points for which $ \alpha_i^* > 0 $ are considered Support Vectors** and contribute directly to defining the optimal decision boundary. The optimal weight vector $w^*$ and the prediction rule for a new point $x_t$ continue to use the same dot-product formulas as in the hard margin case, summing over these identified support vectors.

## The Kernel Trick: Enabling Efficient Non-linear SVMs

The elegant dependency of the SVM's dual formulation (and its prediction rule) exclusively on **dot products** between data points ($x_i^Tx_j$) is what enables the incredibly powerful **kernel trick**.

**The Trick:** The kernel trick involves replacing every instance of a dot product $x_i^Tx_j$ in the SVM's dual problem and prediction rule with a **kernel function** $k(x_1, x_2)$. This kernel function implicitly computes the dot product of the transformed feature vectors in a higher-dimensional feature space $ \mathcal{H} $ (i.e., $k(x_1, x_2) = \Phi(x_1)^T\Phi(x_2)$), without ever needing to explicitly compute or even know the high-dimensional mapping $ \Phi(x) $.

**How it works:**
1.  Instead of explicitly transforming each data point $x_i$ into $\Phi(x_i)$ and then computing their dot product, we directly use a function $k(x_i, x_j)$ that, by definition, is equivalent to $(\Phi(x_i))^T \Phi(x_j)$.
2.  This means computations are performed in the original, lower-dimensional space, but the mathematical effect is as if we are operating in a much higher (potentially infinite) dimensional space.

**Benefits of the Kernel Trick:**

*   **Computational Efficiency:** It bypasses the computationally expensive process of explicitly computing and storing high-dimensional feature maps $ \Phi(x) $, making working with complex, non-linear transformations feasible.
*   **Non-linear Decision Boundaries:** By effectively operating in a transformed, higher-dimensional space, the SVM learns decision boundaries that are linear in $ \mathcal{H} $ but translate into complex **non-linear boundaries in the original data space**.
*   **Decoupling of Complexity:** The training complexity of the SVM (typically between $O(N^2)$ and $O(N^3)$, where $N$ is the number of samples) does not explicitly depend on the potentially infinite dimensionality of the implicit feature space $ \mathcal{H} $. It depends only on the number of training samples.

**Valid Kernel Functions:** For a function $k(x_1, x_2)$ to be a valid kernel (i.e., to correspond to an implicit dot product in some feature space), it must satisfy **Mercer's condition**. This mathematical condition stipulates that the **Gram matrix** $K$ (also known as the kernel matrix), where $K_{ij}=k(x_i,x_j)$ is formed by computing the kernel function for all pairs of training data points, must be **symmetric and positive semi-definite**.

**Common Kernel Functions:**
Here are some of the most widely used and effective kernel functions:

<p align="center">

| Kernel Type                          | Formula                                                      | Key Parameter(s)                           | Description                                                                                                                                                                                                                         |
| :----------------------------------- | :----------------------------------------------------------- | :----------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Linear Kernel**                    | $ k(x_1, x_2) = x_1^T x_2 $                                | None                                       | This kernel simply computes the standard dot product. Using it effectively implements a standard linear SVM. It is particularly efficient and effective for very high-dimensional data or when data is roughly linearly separable. |
| **Polynomial Kernel**                | $ k(x_1, x_2) = (\gamma x_1^T x_2 + c)^d $                  | $ d $ (degree), $ c $ (constant), $ \gamma $ (scaling factor) | This kernel allows SVMs to implicitly create polynomial decision boundaries in the original feature space. The degree $d$ determines the complexity of the non-linearity. |
| **Radial Basis Function (RBF) Kernel** (or Gaussian Kernel) | $ k(x_1, x_2) = \exp(-\gamma \|x_1 - x_2\|^2) $             | $ \gamma $ (gamma)                         | This is an extremely popular and versatile kernel. It is highly effective at handling complex non-linear relationships and is often a good default choice. The parameter $\gamma$ controls the "reach" or influence of a single training sample. A small $\gamma$ leads to a broad influence and a smoother decision boundary, while a large $\gamma$ results in a more localized influence, a more complex boundary, and an increased risk of overfitting. |

</p>

---

## Practical Considerations for Effectively Using SVMs

To effectively train and deploy Support Vector Machines, several practical considerations and best practices are paramount:

1.  **Model Selection (Kernel and Hyperparameters):**
    *   This is a crucial and often iterative step in SVM development, involving two main parts:
        *   **Selecting the appropriate kernel function:** The RBF (Radial Basis Function) kernel is often a good default due to its flexibility. However, if data is known to be linearly separable or very high-dimensional, a linear kernel might be more efficient.
        *   **Meticulously tuning hyperparameters:** This includes the regularization parameter $C$ (for Soft Margin SVM) and any kernel-specific parameters like $\gamma$ (for RBF) or $d$ (for polynomial). This tuning should be performed using rigorous **cross-validation** on the training data to find the combination that generalizes best.

2.  **Linear SVM vs. Kernel SVM:**
    *   **Linear SVMs:** Generally more efficient for very **high-dimensional** datasets (e.g., text data with many sparse features) or when the number of samples ($N$) significantly exceeds the feature dimensionality ($d$). Linear SVMs often benefit from specialized fast primal solvers.
    *   **Kernel SVMs:** Necessary when the underlying data relationship is inherently **non-linear**. While powerful, they typically have higher training complexity (often $O(N^2)$ to $O(N^3)$), and their prediction speed depends on the number of support vectors identified.

3.  **Feature Pre-processing (Scaling):**
    *   SVMs are notoriously **highly sensitive to feature scaling**. This is because margin optimization involves distance calculations (Euclidean norm for $||w||$) and dot products, where features with larger numerical ranges disproportionately contribute.
    *   It is **imperative to center and scale features** (e.g., using z-score normalization, min-max scaling, or simply dividing by maximum value for positive features). This ensures all features contribute equitably to distance calculations and, consequently, to margin optimization.

4.  **Probabilistic Interpretation:**
    *   As noted earlier, the raw scores (signed distances from the hyperplane) produced by SVMs **do not directly represent probabilities**.
    *   If calibrated probabilistic outputs are required (e.g., for decision-making with explicit costs or for combining with other probabilistic models), post-processing techniques must be applied. Common methods include:
        *   **Platt Scaling:** Involves training a separate logistic regression model on the SVM's raw scores (or distances) to output probabilities.
        *   **Isotonic Regression:** A non-parametric method that learns a monotonic mapping from scores to probabilities.

5.  **Handling Unbalanced Data:**
    *   When training data classes are highly imbalanced (e.g., one class has far more samples), standard SVMs can exhibit a **bias towards the majority class**, potentially achieving a large margin that favors the more numerous class.
    *   This can be mitigated by using **different penalty coefficients** for each class: $C^+$ for the positive class and $C^-$ for the negative class. These are often set inversely proportional to the class frequencies (e.g., $C_{minority} > C_{majority}$) to give more importance to correctly classifying the minority class.

## Multiclass Extensions and Broader Kernel Methods

The standard SVM formulation is fundamentally designed as a **binary classifier**. To extend its capabilities to **multiclass problems** (i.e., classification with $K > 2$ classes), common strategies involve combining multiple binary SVMs:

*   **One-versus-All (OvA) / One-versus-Rest (OvR):**
    *   **Approach:** This strategy involves training $K$ separate binary SVMs. For each SVM, one specific class is designated as the "positive" class, and all other $K-1$ classes are grouped together as the "negative" class.
    *   **Decision:** For a new sample, each of the $K$ SVMs outputs a score. The final classification decision is then made by selecting the class whose corresponding SVM outputs the highest score (or most positive distance from its boundary).

*   **One-versus-One (OvO):**
    *   **Approach:** This strategy trains a separate binary SVM for every unique pair of classes. For $K$ classes, this results in training $K(K-1)/2$ individual SVMs.
    *   **Decision:** For a new sample, each of these pairwise SVMs makes a prediction (a "vote"). The class that accumulates the most pairwise "wins" or votes is ultimately chosen as the final classification. OvO can be more computationally intensive during training for a large number of classes but can be more accurate.

**Kernel Methods as a Broader Paradigm:**
It's important to recognize that "Kernel Methods" represent a broader and powerful paradigm in machine learning, not just limited to SVMs. In this context, the **kernel trick** can be flexibly applied to **any algorithm whose core computations rely solely on dot products** between data points. This powerful conceptual framework allows inherently linear algorithms to be extended to efficiently address inherently non-linear problems without explicitly transforming the data into a high-dimensional space.

**Examples of other kernel methods include:**
*   **Kernel Logistic Regression:** A non-linear extension of Logistic Regression.
*   **Kernel PCA:** A non-linear version of Principal Component Analysis.
*   **Gaussian Processes:** A powerful non-parametric probabilistic model that uses kernels to define similarity.