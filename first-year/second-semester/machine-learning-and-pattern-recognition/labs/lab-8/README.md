# Laboratory 8: Evaluation of Classification Models

This guide for **Laboratory 8: Evaluation of Classification Models** delves into advanced performance metrics crucial for thoroughly assessing classification model effectiveness. Key topics covered include:
*   **Confusion Matrices:** Visualizing per-class classification performance.
*   **Bayes Risk (Detection Cost Function - DCF):** Quantifying the cost of misclassification under specific prior probabilities and associated costs.
*   **Minimum DCF (minDCF):** Determining the best possible DCF achievable by a classifier, irrespective of the decision threshold.
*   **ROC Curves (Receiver Operating Characteristic):** Graphically representing a classifier's performance across various decision thresholds.
*   **Bayes Error Plots:** Illustrating how DCF varies with different prior probabilities.

This laboratory will leverage pre-computed log-likelihoods and Log-Likelihood Ratios (LLRs) derived from models trained in *Laboratory 6 (Generative Gaussian Classifiers on the Iris dataset)* and *Laboratory 7 (Generative Multinomial Models on the Divina Commedia dataset)*.

### Datasets & Pre-computed Scores

For practical application and evaluation, the following datasets and their associated pre-computed scores will be used:
*   **Iris Dataset:** We will revisit classification results (likely class-conditional log-likelihoods or posteriors) from the MVG and Tied Gaussian models trained in Laboratory 6.
*   **Commedia Dataset (from Lab 7):**
    *   `Data/commedia_ll.npy`: A `3 x N` NumPy matrix containing class-conditional log-likelihoods for the 3-class problem (Class 0: Inferno, Class 1: Purgatorio, Class 2: Paradiso).
    *   `Data/commedia_labels.npy`: A `1D` NumPy array storing the true class labels corresponding to `commedia_ll.npy`.
    *   `Data/commedia_llr_infpar.npy`: A `1D` NumPy array containing LLR scores for the binary classification task of Inferno (Class 0) vs. Paradiso (Class 1), computed using the `ε=0.001` Multinomial model. The LLR is defined as `log[f(x|Paradiso)/f(x|Inferno)]`.
    *   `Data/commedia_labels_infpar.npy`: The true labels corresponding to `commedia_llr_infpar.npy`.
    *   `Data/commedia_llr_infpar_eps1.npy`: LLR scores for the same binary Inferno vs. Paradiso task, but generated using the `ε=1.0` Multinomial model.
    *   `Data/commedia_labels_infpar_eps1.npy`: True labels for `commedia_llr_infpar_eps1.npy`, identical to `commedia_labels_infpar.npy`.

### Prerequisites

To undertake this laboratory successfully, you should be familiar with:
*   Concepts from Laboratory 6 (Generative Gaussian Models) and Laboratory 7 (Generative Multinomial Models), including log-likelihoods.
*   Foundational probability theory, specifically Bayes' theorem, prior probabilities, likelihoods, and posterior probabilities.
*   Proficiency in NumPy for array manipulation and mathematical operations.
*   Basic understanding of Matplotlib for data visualization.

### Helper Functions

Standard utility functions that reshape arrays, such as `vcol(x)` (reshapes to an `N x 1` column vector) and `vrow(x)` (reshapes to a `1 x N` row vector), are assumed to be available and correctly implemented.

```python
import numpy
import scipy.special # Used for logsumexp
import matplotlib.pyplot # Used for plotting

def vcol(x):
    return x.reshape((x.size, 1))

def vrow(x):
    return x.reshape((1, x.size))

def compute_posteriors(log_class_conditional_ll, prior_array):
    """Computes posterior probabilities P(c|x) from log-likelihoods and priors."""
    log_prior_col = vcol(numpy.log(prior_array))
    logJoint = log_class_conditional_ll + log_prior_col
    logMarginal = vrow(scipy.special.logsumexp(logJoint, axis=0))
    logPost = logJoint - logMarginal
    return numpy.exp(logPost)

def uniform_cost_matrix(nClasses):
    """Creates a standard uniform cost matrix where correct classifications cost 0 and all errors cost 1."""
    C = numpy.ones((nClasses, nClasses))
    numpy.fill_diagonal(C, 0)
    return C

def compute_optimal_Bayes(posterior_matrix, cost_matrix):
    """Computes the optimal Bayes decisions for each sample to minimize the expected cost."""
    expectedCosts = cost_matrix @ posterior_matrix
    return numpy.argmin(expectedCosts, axis=0)

def compute_confusion_matrix(predictedLabels, trueLabels):
    """Computes the confusion matrix. M[predicted, actual] = count."""
    nClasses = trueLabels.max() + 1
    M = numpy.zeros((nClasses, nClasses), dtype=numpy.int32)
    for i in range(trueLabels.size):
        M[predictedLabels[i], trueLabels[i]] += 1
    return M

def compute_optimal_Bayes_binary_llr(llr_scores, prior_class1, Cfn, Cfp):
    """Determines optimal binary prediction based on LLRs, prior P(C=1), and costs."""
    prior_class0 = 1.0 - prior_class1
    # Calculate the optimal threshold for the given priors and costs
    optimal_threshold = -numpy.log( (prior_class1 * Cfn) / (prior_class0 * Cfp) )
    # Apply the decision rule: predict 1 if LLR >= threshold, else 0
    predictions = numpy.int32(llr_scores >= optimal_threshold)
    return predictions

def compute_empirical_Bayes_risk_binary(predictedLabels, classLabels, prior, Cfn, Cfp, normalize=True):
    """Computes empirical Bayes risk (DCF) for binary classification."""
    M = compute_confusion_matrix(predictedLabels, classLabels)
    # Calculate Pfn: False Negatives (M[0,1]) / Total Actual Positives (M[0,1] + M[1,1])
    total_actual_positives = M[0, 1] + M[1, 1]
    Pfn = M[0, 1] / total_actual_positives if total_actual_positives > 0 else 0.0
    # Calculate Pfp: False Positives (M[1,0]) / Total Actual Negatives (M[0,0] + M[1,0])
    total_actual_negatives = M[0, 0] + M[1, 0]
    Pfp = M[1, 0] / total_actual_negatives if total_actual_negatives > 0 else 0.0
    
    # Unnormalized Bayes error (DCFu)
    bayesError = prior * Cfn * Pfn + (1.0 - prior) * Cfp * Pfp
    
    if normalize:
        # Calculate Bdummy for binary case
        Bdummy = numpy.minimum(prior * Cfn, (1.0 - prior) * Cfp)
        return bayesError / Bdummy
    else: 
        return bayesError

def compute_empirical_Bayes_risk(predictedLabels, classLabels, prior_array, costMatrix, normalize=True):
    """Computes empirical Bayes risk (DCF) for multiclass (or binary) classification."""
    M = compute_confusion_matrix(predictedLabels, classLabels)
    # Calculate the total number of samples for each true class
    total_actual_per_class = M.sum(0)
    # Calculate error rates: M[predicted, actual] / total actual for that class
    errorRates = numpy.divide(M, vrow(total_actual_per_class),
                              out=numpy.zeros_like(M, dtype=float), # Ensure float output and handle division by zero
                              where=total_actual_per_class!=0)
    
    # Unnormalized Bayes risk (DCFu) for multiclass
    unnormalized_bayes_risk = ( (errorRates * costMatrix).sum(axis=0) @ prior_array )
    
    if normalize:
        # Calculate Bdummy for multiclass: min_i (sum_k (C[i,k] * pi_k))
        Bdummy = numpy.min(costMatrix @ vcol(prior_array))
        return unnormalized_bayes_risk / Bdummy
    return unnormalized_bayes_risk

def compute_Pfn_Pfp_allThresholds_fast(llr_scores, classLabels):
    """Efficiently computes all unique (Pfn, Pfp) pairs for all possible decision thresholds."""
    # Sort LLR scores and corresponding class labels
    llr_sorter_indices = numpy.argsort(llr_scores)
    llrSorted = llr_scores[llr_sorter_indices]
    classLabelsSorted = classLabels[llr_sorter_indices]

    # Calculate total number of true positives (class 1) and true negatives (class 0)
    nTrue = (classLabelsSorted == 1).sum()
    nFalse = (classLabelsSorted == 0).sum()
    
    # Initialize FN (misses) and FP (false alarms) counts.
    # Initially, assuming threshold is +inf, all class 1 samples are FN, and no class 0 samples are FP.
    nFalseNegative = 0 # Number of Class 1 samples misclassified as 0
    nFalsePositive = nFalse # Number of Class 0 samples misclassified as 1

    Pfn_values = []
    Pfp_values = []
    thresholds_out = []
    
    # Add an initial point for threshold = -infinity (predict all as 1)
    Pfn_values.append(nFalseNegative / nTrue if nTrue > 0 else 0.0)
    Pfp_values.append(nFalsePositive / nFalse if nFalse > 0 else 0.0)
    thresholds_out.append(-numpy.inf)

    # Iterate through sorted LLR scores to calculate Pfn and Pfp for each effective threshold
    for i in range(len(llrSorted)):
        # If current sample is Class 1, it's no longer a False Negative if threshold is moved below its LLR
        if classLabelsSorted[i] == 1: nFalseNegative += 1
        # If current sample is Class 0, it's no longer a False Positive if threshold is moved below its LLR
        if classLabelsSorted[i] == 0: nFalsePositive -= 1
        
        # Only add a new (Pfn, Pfp) point if the LLR score changes or if it's the last point
        if i == len(llrSorted) - 1 or llrSorted[i] != llrSorted[i+1]:
            Pfn_values.append(nFalseNegative / nTrue if nTrue > 0 else 0.0)
            Pfp_values.append(nFalsePositive / nFalse if nFalse > 0 else 0.0)
            thresholds_out.append(llrSorted[i])
            
    return numpy.array(Pfn_values), numpy.array(Pfp_values), numpy.array(thresholds_out)


def compute_minDCF_binary_fast(llr_scores, classLabels, prior_class1, Cfn, Cfp, returnThreshold=False):
    """Computes the minimum normalized DCF (minDCF) for a binary classifier efficiently."""
    # Get all (Pfn, Pfp) pairs across all thresholds
    Pfn, Pfp, thresholds = compute_Pfn_Pfp_allThresholds_fast(llr_scores, classLabels)
    
    # Calculate the unnormalized DCF for each (Pfn, Pfp) pair
    numerator_dcf = prior_class1 * Cfn * Pfn + (1.0 - prior_class1) * Cfp * Pfp
    # Calculate the Bdummy for normalization
    denominator_dcf = numpy.minimum(prior_class1 * Cfn, (1.0 - prior_class1) * Cfp)
    # Calculate all normalized DCF values
    all_normalized_dcfs = numerator_dcf / denominator_dcf
    
    # Find the index of the minimum normalized DCF
    min_dcf_index = numpy.argmin(all_normalized_dcfs)
    # Retrieve the minimum DCF value
    min_dcf_value = all_normalized_dcfs[min_dcf_index]

    if returnThreshold: 
        # If requested, also return the optimal threshold that yields minDCF
        return min_dcf_value, thresholds[min_dcf_index]
    else: 
        return min_dcf_value

def compute_actDCF_binary_fast(llr_scores, classLabels, prior_class1, Cfn=1.0, Cfp=1.0):
    """Computes actual normalized DCF for binary classifier with optimal thresholding."""
    # Find the optimal predictions for the given prior and costs
    optimal_predictions = compute_optimal_Bayes_binary_llr(llr_scores, prior_class1, Cfn, Cfp)
    # Compute and return the normalized DCF for these optimal predictions
    return compute_empirical_Bayes_risk_binary(optimal_predictions, classLabels, prior_class1, Cfn, Cfp, normalize=True)
```

---

## Task 1: Confusion Matrices

### Task Objective

Implement functions to accurately compute and display **confusion matrices** from classification results. These functions will then be applied to:
1.  Results from the **Iris dataset** (referencing the classification example from Laboratory 6).
2.  Results from the **3-class Commedia dataset**.

For this task, we initially assume uniform prior probabilities for each class and uniform misclassification costs.

### Step-by-Step Reasoning

A confusion matrix, typically denoted as `M`, is a square matrix where `M[i, j]` represents the count of samples that were *truely* in class `j` but were *predicted* as belonging to class `i`. Consequently, diagonal elements `M[i, i]` indicate correct classifications, while off-diagonal elements `M[i, j]` (where `i ≠ j`) represent misclassifications.

Under the assumptions of uniform prior probabilities and uniform misclassification costs, the optimal decision rule for classification simplifies to predicting the class `c` that maximizes its posterior probability `P(c|x)`. This is further equivalent to maximizing the class-conditional log-likelihood `log f(x|c)`.

1.  **Compute Posteriors `P(c|x)`:** Utilize a function (e.g., `compute_posteriors`) that accepts a matrix of class-conditional log-likelihoods (such as `commedia_ll.npy`) and an array of class priors. This function should compute `P(c|x)` by applying Bayes' theorem in the log-domain (`log P(c|x) = log f(x|c) + log P(c) - log f(x)`), crucially employing the **log-sum-exp trick** for numerical stability, and then exponentiating the result to obtain actual probabilities.
2.  **Derive Predicted Labels:** For each sample, the predicted class label is determined by finding the index of the maximum value along the class dimension of the computed posterior probability matrix. This can be efficiently done using `numpy.argmax(P, axis=0)`.
3.  **Compute Confusion Matrix:** Implement `compute_confusion_matrix(predictedLabels, trueLabels)`. This function should initialize a `K x K` zero matrix `M`, where `K` is the number of classes. Then, for each sample `i`, increment the count at `M[predictedLabels[i], trueLabels[i]]`.

#### Solution Code (`sol.py`)

```python
if __name__ == '__main__':
    print("--- Task 1: Confusion Matrices ---")
    print("\nCommedia Dataset (Multiclass) - Uniform Priors & Costs")
    try: # Load commedia data
        commedia_ll = numpy.load('Data/commedia_ll.npy')
        commedia_labels = numpy.load('Data/commedia_labels.npy')
    except FileNotFoundError:
        # Fallback for alternative directory structure in some environments
        commedia_ll = numpy.load('../Data/commedia_ll.npy')
        commedia_labels = numpy.load('../Data/commedia_labels.npy')

    # Define uniform prior for 3 classes
    uniform_prior_3_classes = numpy.ones(3) / 3.0
    # Compute posterior probabilities using the loaded log-likelihoods and uniform priors
    commedia_posteriors = compute_posteriors(commedia_ll, uniform_prior_3_classes)
    # Create a uniform cost matrix for 3 classes (0 cost for correct, 1 for incorrect)
    uniform_costs_3_classes = uniform_cost_matrix(3)
    # Compute optimal Bayes predictions by minimizing expected cost (equivalent to max posterior for uniform costs)
    commedia_predictions_mvg = compute_optimal_Bayes(commedia_posteriors, uniform_costs_3_classes)
    # Compute the confusion matrix
    confusion_matrix_commedia = compute_confusion_matrix(commedia_predictions_mvg, commedia_labels)
    print("Confusion Matrix (Predicted Row, True Col):")
    print(confusion_matrix_commedia)
```

#### Expected Output

```
--- Task 1: Confusion Matrices ---

Commedia Dataset (Multiclass) - Uniform Priors & Costs
Confusion Matrix (Predicted Row, True Col):
[[205 111  56]
 [145 199 121]
 [ 50  92 225]]
```

---

## Task 2: Optimal Bayes Decisions (Binary Task)

### Task Objective

Implement the function `compute_optimal_Bayes_binary_llr(llr, prior, Cfn, Cfp)`. This function should determine the optimal binary prediction (either 0 or 1) for a set of samples. The decision will be based on the provided **log-likelihood ratio (LLR)** scores, the prior probability of class 1 (`prior`), the cost of a false negative (`Cfn`), and the cost of a false positive (`Cfp`). After implementation, verify its correctness using several example configurations.

### Step-by-Step Reasoning

For binary classification, a decision is typically made by predicting Class 1 if the `llr(x)` score for a sample `x` is greater than or equal to a specific threshold `t`; otherwise, Class 0 is predicted. The optimal threshold `t` is derived to minimize the expected classification cost, considering the costs of false negatives and false positives, as well as the prior probabilities of the classes.

The formula for the optimal threshold `t` is given by:
$$ t = -\log \left[ \frac{C_{fn} \cdot \pi_1}{C_{fp} \cdot \pi_0} \right] $$
Where $\pi_1$ is the prior probability of Class 1 (provided as `prior`), and $\pi_0$ is the prior probability of Class 0, calculated as `1 - prior`.

The implementation should first calculate this optimal threshold `t`, and then apply the decision rule ( `llr_scores >= t` ) to all given `llr` scores to generate the binary predictions.

#### Solution Code (`sol.py`)

```python
if __name__ == '__main__':
    print("\n--- Task 2: Optimal Bayes Decisions (Binary Task) ---")
    print("Verification using Commedia Inferno-Paradiso LLRs.")
    try: # Load binary commedia data
        commedia_llr_binary = numpy.load('Data/commedia_llr_infpar.npy')
        commedia_labels_binary = numpy.load('Data/commedia_labels_infpar.npy')
    except FileNotFoundError:
        commedia_llr_binary = numpy.load('../Data/commedia_llr_infpar.npy')
        commedia_labels_binary = numpy.load('../Data/commedia_labels_infpar.npy')

    # Define various configurations for priors and costs for testing
    configurations = [
        (0.5, 1, 1),   # Uniform prior, uniform costs
        (0.8, 1, 1),   # High prior for C1 (Paradiso), uniform costs
        (0.5, 10, 1),  # Uniform prior, high False Negative cost (motivates predicting Class 1 more often)
        (0.8, 1, 10)   # High prior for C1 (Paradiso), high False Positive cost (motivates predicting Class 0 more often)
    ]

    for prior_C1, Cfn, Cfp in configurations:
        print(f'\nConfiguration: P(C=1)={prior_C1}, Cfn={Cfn}, Cfp={Cfp}')
        # Compute predictions using the optimal Bayes decision rule
        predictions_binary = compute_optimal_Bayes_binary_llr(commedia_llr_binary, prior_C1, Cfn, Cfp)
        # Compute and print the confusion matrix for verification
        conf_matrix_binary = compute_confusion_matrix(predictions_binary, commedia_labels_binary)
        print("Confusion Matrix (Predicted Row, True Col):")
        print(conf_matrix_binary)
```

#### Expected Output

```
--- Task 2: Optimal Bayes Decisions (Binary Task) ---
Verification using Commedia Inferno-Paradiso LLRs.

Configuration: P(C=1)=0.5, Cfn=1, Cfp=1
Confusion Matrix (Predicted Row, True Col):
[[293  96]
 [109 304]]

Configuration: P(C=1)=0.8, Cfn=1, Cfp=1
Confusion Matrix (Predicted Row, True Col):
[[271  80]
 [131 320]]

Configuration: P(C=1)=0.5, Cfn=10, Cfp=1
Confusion Matrix (Predicted Row, True Col):
[[257  75]
 [145 325]]

Configuration: P(C=1)=0.8, Cfn=1, Cfp=10
Confusion Matrix (Predicted Row, True Col):
[[302 113]
 [100 287]]
```

---

## Task 3: Empirical Bayes Risk / Detection Cost Function (DCF)

### Task Objective

Implement functions to compute the **empirical Bayes risk**, also known as the **Detection Cost Function (DCF)**. This involves developing specialized functions for both:
1.  **Binary classification**.
2.  **General multiclass classification**.

After implementation, verify their correctness using the configurations from Task 2.

### Step-by-Step Reasoning

The **Unnormalized DCF (`DCFu`)** quantifies the total expected cost.
*   **For Binary Classification:**
    $$ DCF_u = \pi_1 \cdot C_{fn} \cdot P_{fn} + \pi_0 \cdot C_{fp} \cdot P_{fp} $$
    Here, $P_{fn}$ (Probability of False Negative) is calculated as $FN / (FN+TP)$ (i.e., False Negatives divided by total actual positives), and $P_{fp}$ (Probability of False Positive) is calculated as $FP / (FP+TN)$ (i.e., False Positives divided by total actual negatives). These counts are readily available from the confusion matrix.
*   **For Multiclass Classification:**
    $$ DCF_u = \sum_j \pi_j \cdot \sum_i (R[i, j] \cdot C[i, j]) $$
    In this formula, $\pi_j$ is the prior probability of class `j`, $C[i, j]$ is the cost of predicting class `i` when the true class is `j`, and $R[i, j]$ is the rate of predicting class `i` when the true class is `j`, calculated as $M[i,j] / (\text{total samples truly in class j})$.

The **Normalized DCF** (often simply referred to as DCF) is calculated as $DCF_{normalized} = DCF_u / B_{dummy}$. This normalization makes the DCF value easier to interpret, as it compares the classifier's cost to that of a trivial "dummy" system. $B_{dummy}$ represents the minimum cost achievable by a dummy system that always predicts a single class, based on the given priors and costs.
*   **Binary $B_{dummy}$:** $\min(C_{fn} \cdot \pi_1, C_{fp} \cdot (1 - \pi_1))$. This represents the cost of always predicting Class 0 (cost $C_{fp} \cdot \pi_0$) versus always predicting Class 1 (cost $C_{fn} \cdot \pi_1$).
*   **Multiclass $B_{dummy}$:** $\min_i (\sum_k C[i, k] \cdot \pi_k)$. This is the minimum expected cost if the dummy system consistently predicts class `i`, summed over all possible true classes `k` weighted by their priors.

#### Solution Code (`sol.py`)

```python
if __name__ == '__main__':
    print("\n--- Task 3: Empirical Bayes Risk / Detection Cost Function (DCF) ---")
    print("Verification using Commedia Inferno-Paradiso LLRs.")
    # (commedia_llr_binary and commedia_labels_binary loaded from Task 2)
    configurations = [(0.5, 1, 1), (0.8, 1, 1), (0.5, 10, 1), (0.8, 1, 10)]

    for prior_C1, Cfn, Cfp in configurations:
        print(f'\nConfiguration: P(C=1)={prior_C1}, Cfn={Cfn}, Cfp={Cfp}')
        # Compute optimal predictions for the current configuration
        predictions_binary = compute_optimal_Bayes_binary_llr(commedia_llr_binary, prior_C1, Cfn, Cfp)
        
        # Compute DCF using the binary-specific function (unnormalized and normalized)
        dcfu_binary_specific = compute_empirical_Bayes_risk_binary(
            predictions_binary, commedia_labels_binary, prior_C1, Cfn, Cfp, normalize=False)
        dcf_norm_binary_specific = compute_empirical_Bayes_risk_binary(
            predictions_binary, commedia_labels_binary, prior_C1, Cfn, Cfp, normalize=True)
        print(f'  DCF (unnormalized, binary specific): {dcfu_binary_specific:.3f}')
        print(f'  DCF (normalized, binary specific): {dcf_norm_binary_specific:.3f}')

        # Prepare data for the general multiclass DCF function (binary case handled as 2 classes)
        binary_cost_matrix = numpy.array([[0, Cfn], [Cfp, 0]]) # Cost matrix for binary task
        prior_array_general = numpy.array([1.0 - prior_C1, prior_C1]) # Prior array for general function
        
        # Compute DCF using the general function (unnormalized and normalized)
        dcfu_general = compute_empirical_Bayes_risk(
            predictions_binary, commedia_labels_binary, prior_array_general, binary_cost_matrix, normalize=False)
        dcf_norm_general = compute_empirical_Bayes_risk(
            predictions_binary, commedia_labels_binary, prior_array_general, binary_cost_matrix, normalize=True)
        print(f'  DCF (unnormalized, general): {dcfu_general:.3f}')
        print(f'  DCF (normalized, general): {dcf_norm_general:.3f}')
```

#### Expected Output

```
--- Task 3: Empirical Bayes Risk / Detection Cost Function (DCF) ---
Verification using Commedia Inferno-Paradiso LLRs.

Configuration: P(C=1)=0.5, Cfn=1, Cfp=1
  DCF (unnormalized, binary specific): 0.256
  DCF (normalized, binary specific): 0.511
  DCF (unnormalized, general): 0.256
  DCF (normalized, general): 0.511

Configuration: P(C=1)=0.8, Cfn=1, Cfp=1
  DCF (unnormalized, binary specific): 0.225
  DCF (normalized, binary specific): 1.126
  DCF (unnormalized, general): 0.225
  DCF (normalized, general): 1.126

Configuration: P(C=1)=0.5, Cfn=10, Cfp=1
  DCF (unnormalized, binary specific): 1.118
  DCF (normalized, binary specific): 2.236
  DCF (unnormalized, general): 1.118
  DCF (normalized, general): 2.236

Configuration: P(C=1)=0.8, Cfn=1, Cfp=10
  DCF (unnormalized, binary specific): 0.724
  DCF (normalized, binary specific): 0.904
  DCF (unnormalized, general): 0.724
  DCF (normalized, general): 0.904
```

---

## Task 4: Minimum DCF

### Task Objective

Compute the **minimum normalized DCF (minDCF)** for binary classifier scores. This involves identifying the optimal decision threshold directly on the evaluation set itself to achieve the lowest possible cost.

### Step-by-Step Reasoning

The minDCF represents the theoretically lowest possible DCF value for a given classifier and evaluation set. It is found by evaluating the DCF across *all possible* decision thresholds and selecting the minimum. This seemingly exhaustive process can be performed efficiently by considering thresholds only at the observed LLR score values.

1.  **`compute_Pfn_Pfp_allThresholds_fast(llr_scores, classLabels)`:** This helper function efficiently generates all unique pairs of `(Pfn, Pfp)` that can be achieved by varying the decision threshold. It typically sorts the LLR scores and their corresponding true labels. Then, it iteratively sweeps a conceptual threshold across these sorted scores, updating the counts of False Positives (FP) and False Negatives (FN) and calculating the corresponding `Pfn` and `Pfp` at each effective threshold point.
2.  **`compute_minDCF_binary_fast(...)`:** This function computes the actual minDCF. It first utilizes `compute_Pfn_Pfp_allThresholds_fast` to obtain all `(Pfn, Pfp)` pairs achievable by the classifier. Subsequently, for *each* of these `(Pfn, Pfp)` pairs, it calculates the normalized DCF given the specified `(prior_class1, Cfn, Cfp)`. Finally, it returns the minimum value found among all these calculated normalized DCFs.

#### Solution Code (`sol.py`)

```python
if __name__ == '__main__':
    print("\n--- Task 4: Minimum DCF ---")
    print("Verification using Commedia Inferno-Paradiso LLRs.")
    # (commedia_llr_binary and commedia_labels_binary loaded from Task 2)
    configurations = [(0.5, 1, 1), (0.8, 1, 1), (0.5, 10, 1), (0.8, 1, 10)]

    for prior_C1, Cfn, Cfp in configurations:
        print(f'\nConfiguration: P(C=1)={prior_C1}, Cfn={Cfn}, Cfp={Cfp}')
        # Compute and print the minDCF value and the corresponding optimal threshold
        minDCF_value, minDCF_threshold = compute_minDCF_binary_fast(
            commedia_llr_binary, commedia_labels_binary, prior_C1, Cfn, Cfp, returnThreshold=True)
        print(f'  MinDCF (normalized, fast): {minDCF_value:.3f} (optimal threshold = {minDCF_threshold:.3e})')
```

#### Expected Output

```
--- Task 4: Minimum DCF ---
Verification using Commedia Inferno-Paradiso LLRs.

Configuration: P(C=1)=0.5, Cfn=1, Cfp=1
  MinDCF (normalized, fast): 0.506 (optimal threshold = -1.614e-01)

Configuration: P(C=1)=0.8, Cfn=1, Cfp=1
  MinDCF (normalized, fast): 0.752 (optimal threshold = 1.386e+00)

Configuration: P(C=1)=0.5, Cfn=10, Cfp=1
  MinDCF (normalized, fast): 0.842 (optimal threshold = -2.303e+00)

Configuration: P(C=1)=0.8, Cfn=1, Cfp=10
  MinDCF (normalized, fast): 0.709 (optimal threshold = 2.773e+00)
```

---

## Task 5: ROC Curves

### Task Objective

Generate and display the **Receiver Operating Characteristic (ROC) curve** for the binary Inferno-vs-Paradiso classification task.

### Step-by-Step Reasoning

An ROC curve is a graphical plot that illustrates the diagnostic ability of a binary classifier as its decision threshold is varied. It plots the **True Positive Rate (TPR)** against the **False Positive Rate (FPR)** for all possible threshold settings.
*   **True Positive Rate (TPR):** Also known as Sensitivity or Recall, $TPR = 1 - P_{fn}$. It represents the proportion of actual positives that are correctly identified.
*   **False Positive Rate (FPR):** $FPR = P_{fp}$. It represents the proportion of actual negatives that are incorrectly identified as positive.

To generate the ROC curve, you can reuse the `compute_Pfn_Pfp_allThresholds_fast` function (developed in Task 4) to efficiently obtain the `Pfn` and `Pfp` arrays across all thresholds. Then, simply plot `Pfp` (on the x-axis) against `(1 - Pfn)` (on the y-axis) using Matplotlib. It's good practice to also include a diagonal line (`y=x`) on the plot, which serves as a baseline representing the performance of a random classifier.

#### Solution Code (`sol.py`)

```python
if __name__ == '__main__':
    print("\n--- Task 5: ROC Curves ---")
    print("Generating ROC curve for Commedia Inferno-Paradiso classification.")

    # (commedia_llr_binary and commedia_labels_binary loaded from Task 2)
    # Compute Pfn and Pfp values across all thresholds
    Pfn_roc, Pfp_roc, _ = compute_Pfn_Pfp_allThresholds_fast(commedia_llr_binary, commedia_labels_binary)
    # Calculate True Positive Rate (TPR)
    TPR_roc = 1 - Pfn_roc

    matplotlib.pyplot.figure(0, figsize=(7, 6)) # Create a new figure
    matplotlib.pyplot.plot(Pfp_roc, TPR_roc, color='blue', label='ROC Curve') # Plot ROC curve
    matplotlib.pyplot.plot([0, 1], [0, 1], color='gray', linestyle='--', label='Random Classifier') # Plot diagonal baseline
    matplotlib.pyplot.xlabel("False Positive Rate (FPR / Pfp)")
    matplotlib.pyplot.ylabel("True Positive Rate (TPR / (1-Pfn))")
    matplotlib.pyplot.title("Receiver Operating Characteristic (ROC) Curve")
    matplotlib.pyplot.grid(True, linestyle='--', alpha=0.7) # Add grid for readability
    matplotlib.pyplot.legend() # Display legend
    matplotlib.pyplot.xlim([-0.05, 1.05]) # Set x-axis limits for better visualization
    matplotlib.pyplot.ylim([-0.05, 1.05]) # Set y-axis limits
    # matplotlib.pyplot.show() # Uncomment to display the plot interactively
```

#### Expected Output

A graphical plot will be generated, displaying an ROC curve. This curve should start at coordinates (0,0) and end at (1,1), typically bending towards the top-left corner of the plot. Crucially, a good classifier's ROC curve will lie above the `y=x` diagonal line, which represents the performance of a random classifier.

---

## Task 6: Bayes Error Plots

### Task Objective

Generate **Bayes error plots**. These plots should illustrate both the normalized actual DCF (actDCF) and the minDCF as a function of **prior log-odds $p = \log(\pi / (1-\pi))$**. The task specifically requires comparing these plots for two different Multinomial models: one trained with a smoothing parameter `ε=0.001` and another with `ε=1.0`.

### Step-by-Step Reasoning

1.  **Prior Log-Odds:** The x-axis of the Bayes error plot represents the prior log-odds, defined as $p = \log(\pi / (1 - \pi))$, where $\pi$ is the prior probability of Class 1. Generate a range of `p` values, for instance, `numpy.linspace(-3, 3, 21)`.
2.  **Effective Prior `~π`:** Convert each `p` value back into an effective prior probability `~π` using the inverse logistic function: $\tilde{\pi} = 1 / (1 + \exp(-p))$.
3.  **Calculate DCFs:** For each `~π` (effective prior):
    *   **`actDCF`:** Compute the actual normalized DCF. Use a helper function like `compute_actDCF_binary_fast` (which internally calls `compute_optimal_Bayes_binary_llr` to find the optimal threshold for the given `~π` and uniform costs `Cfn=Cfp=1`, then uses `compute_empirical_Bayes_risk_binary` to calculate the DCF at that threshold).
    *   **`minDCF`:** Compute the minimum normalized DCF. Use the `compute_minDCF_binary_fast` function (from Task 4), again for the current `~π` and uniform costs (`Cfn=1.0, Cfp=1.0`).
4.  **Plotting:** Using Matplotlib, plot the `actDCF` values and `minDCF` values against their corresponding `p` (prior log-odds) values. This should be done for both the `ε=0.001` and `ε=1.0` models on the *same figure* to facilitate direct comparison. Observe that the `ε=1.0` model, due to its stronger smoothing, should generally exhibit lower DCF values, indicating a more robust performance, especially under conditions where data might be sparse or skewed.

#### Solution Code (`sol.py`)

```python
if __name__ == '__main__':
    print("\n--- Task 6: Bayes Error Plots ---")
    print("Generating Bayes error plots for different smoothing parameters (eps) and prior log-odds.")

    # Define a range of effective prior log-odds
    effPriorLogOdds = numpy.linspace(-3, 3, 21)
    # Convert log-odds to effective prior probabilities (P(C=1))
    effPriors = 1.0 / (1.0 + numpy.exp(-effPriorLogOdds))

    # --- Data and DCF calculation for eps = 0.001 model ---
    try:
        commedia_llr_binary_eps001 = numpy.load('Data/commedia_llr_infpar.npy')
        commedia_labels_binary_eps001 = numpy.load('Data/commedia_labels_infpar.npy')
    except FileNotFoundError:
        commedia_llr_binary_eps001 = numpy.load('../Data/commedia_llr_infpar.npy')
        commedia_labels_binary_eps001 = numpy.load('../Data/commedia_labels_infpar.npy')

    actDCF_eps001, minDCF_eps001 = [], []
    for effPrior in effPriors:
        actDCF_eps001.append(compute_actDCF_binary_fast(commedia_llr_binary_eps001, commedia_labels_binary_eps001, effPrior, 1.0, 1.0))
        minDCF_eps001.append(compute_minDCF_binary_fast(commedia_llr_binary_eps001, commedia_labels_binary_eps001, effPrior, 1.0, 1.0))

    # --- Data and DCF calculation for eps = 1.0 model ---
    try:
        commedia_llr_binary_eps1 = numpy.load('Data/commedia_llr_infpar_eps1.npy')
        commedia_labels_binary_eps1 = numpy.load('Data/commedia_labels_infpar_eps1.npy')
    except FileNotFoundError:
        commedia_llr_binary_eps1 = numpy.load('../Data/commedia_llr_infpar_eps1.npy')
        commedia_labels_binary_eps1 = numpy.load('../Data/commedia_labels_infpar_eps1.npy')

    actDCF_eps1, minDCF_eps1 = [], []
    for effPrior in effPriors:
        actDCF_eps1.append(compute_actDCF_binary_fast(commedia_llr_binary_eps1, commedia_labels_binary_eps1, effPrior, 1.0, 1.0))
        minDCF_eps1.append(compute_minDCF_binary_fast(commedia_llr_binary_eps1, commedia_labels_binary_eps1, effPrior, 1.0, 1.0))

    # --- Plotting Bayes Error Curves ---
    matplotlib.pyplot.figure(1, figsize=(10, 7))
    matplotlib.pyplot.plot(effPriorLogOdds, actDCF_eps001, label='Actual DCF ($\epsilon=0.001$)', color='red', linestyle='-')
    matplotlib.pyplot.plot(effPriorLogOdds, minDCF_eps001, label='Min DCF ($\epsilon=0.001$)', color='darkred', linestyle='--')
    matplotlib.pyplot.plot(effPriorLogOdds, actDCF_eps1, label='Actual DCF ($\epsilon=1.0$)', color='blue', linestyle='-')
    matplotlib.pyplot.plot(effPriorLogOdds, minDCF_eps1, label='Min DCF ($\epsilon=1.0$)', color='darkblue', linestyle='--')
    
    # Set plot limits and labels for clarity
    matplotlib.pyplot.ylim([0, 1.1])
    matplotlib.pyplot.xlim([-3, 3])
    matplotlib.pyplot.xlabel("Prior Log-Odds (log($\\tilde{\\pi}/(1-\\tilde{\\pi})$))")
    matplotlib.pyplot.ylabel("Normalized DCF")
    matplotlib.pyplot.title("Bayes Error Plot: Normalized DCF vs. Prior Log-Odds")
    matplotlib.pyplot.legend()
    matplotlib.pyplot.grid(True, linestyle='--', alpha=0.7)
    # matplotlib.pyplot.show() # Uncomment to display the plot interactively
```

#### Expected Output

A plot will be generated, displaying four distinct curves. These curves represent the `actDCF` and `minDCF` for both the `ε=0.001` and `ε=1.0` models. All four curves will be plotted against the prior log-odds on the x-axis. As expected, the curves corresponding to the `ε=1.0` model should generally appear lower than those of the `ε=0.001` model, indicating superior performance due to better handling of sparse data through stronger smoothing.

---

## Task 7: Multiclass Bayes Risk Evaluation (Non-Uniform)

### Task Objective

Evaluate multiclass classifiers (specifically, the `ε=0.001` and `ε=1.0` Multinomial models from Laboratory 7) under a realistic scenario involving a **specific non-uniform prior distribution** and a **non-uniform cost matrix**. The goal is to compute both the unnormalized and normalized empirical Bayes risk (DCF) for these conditions.

### Step-by-Step Reasoning

1.  **Define Scenario Parameters:** Set the specific non-uniform prior distribution (e.g., `prior_application = numpy.array([0.3, 0.4, 0.3])` for classes 0, 1, 2 respectively). Also, define the non-uniform cost matrix (e.g., `costMatrix_application = numpy.array([[0, 1, 2], [1, 0, 1], [2, 1, 0]])`), where `C[i, j]` is the cost of predicting class `i` when the true class is `j`.
2.  **Load Scores:** Load the pre-computed multiclass log-likelihoods for both `ε=0.001` (`commedia_ll.npy`) and `ε=1.0` (`commedia_ll_eps1.npy`) models, along with their corresponding true labels (`commedia_labels.npy`).
3.  **Compute Posteriors:** For each model (`ε=0.001` and `ε=1.0`), compute the posterior probabilities by calling `compute_posteriors` with the respective `log_likelihoods` and the defined `prior_application`.
4.  **Compute Optimal Predictions:** Using the calculated `posteriors` and the `costMatrix_application`, determine the optimal class predictions for each sample. This is done by calling `compute_optimal_Bayes`, which minimizes the expected cost.
5.  **Compute Confusion Matrix:** Generate the confusion matrix using `compute_confusion_matrix` with the optimal predictions and the true labels. This provides a detailed breakdown of correct and incorrect classifications under the specified cost and prior.
6.  **Compute DCF:** Calculate both the unnormalized and normalized empirical Bayes risk. Use the general `compute_empirical_Bayes_risk` function, passing in the optimal predictions, true labels, `prior_application`, and `costMatrix_application`. Ensure to call it once with `normalize=False` for the unnormalized DCF and once with `normalize=True` for the normalized DCF.
7.  **Repeat for Both Models:** Execute steps 3 through 6 for both the `ε=0.001` and `ε=1.0` models to compare their performance under the same non-uniform scenario.

#### Solution Code (`sol.py`)

```python
if __name__ == '__main__':
    print("\n" + "-"*40)
    print("\n--- Task 7: Multiclass Bayes Risk Evaluation (Non-Uniform) ---")

    # Define the non-uniform prior distribution and cost matrix for the application scenario
    prior_application = numpy.array([0.3, 0.4, 0.3]) # Priors for classes 0, 1, 2 respectively
    costMatrix_application = numpy.array([[0, 1, 2], # Cost matrix C[predicted_i, actual_j]
                                           [1, 0, 1], 
                                           [2, 1, 0]])

    # --- Evaluate for Epsilon = 0.001 Model ---
    print("\n  Evaluating model with Epsilon = 0.001:")
    try:
        commedia_ll_eps001_multi = numpy.load('Data/commedia_ll.npy')
        commedia_labels_eps001_multi = numpy.load('Data/commedia_labels.npy')
    except FileNotFoundError:
        commedia_ll_eps001_multi = numpy.load('../Data/commedia_ll.npy')
        commedia_labels_eps001_multi = numpy.load('../Data/commedia_labels.npy')

    # Compute posteriors, optimal predictions, and confusion matrix
    commedia_posteriors_eps001_multi = compute_posteriors(commedia_ll_eps001_multi, prior_application)
    commedia_predictions_eps001_multi = compute_optimal_Bayes(commedia_posteriors_eps001_multi, costMatrix_application)
    conf_matrix_eps001_multi = compute_confusion_matrix(commedia_predictions_eps001_multi, commedia_labels_eps001_multi)
    print("    Confusion Matrix (Predicted Row, True Col):")
    print(conf_matrix_eps001_multi)

    # Compute and print unnormalized and normalized DCF
    DCFu_eps001_multi = compute_empirical_Bayes_risk(commedia_predictions_eps001_multi, commedia_labels_eps001_multi, prior_application, costMatrix_application, normalize=False)
    DCF_norm_eps001_multi = compute_empirical_Bayes_risk(commedia_predictions_eps001_multi, commedia_labels_eps001_multi, prior_application, costMatrix_application, normalize=True)
    print(f'    Empirical Bayes risk (Unnormalized DCF): {DCFu_eps001_multi:.3f}')
    print(f'    Normalized empirical Bayes risk (Normalized DCF): {DCF_norm_eps001_multi:.3f}')

    # --- Evaluate for Epsilon = 1.0 Model ---
    print("\n  Evaluating model with Epsilon = 1.0:")
    try:
        commedia_ll_eps1_multi = numpy.load('Data/commedia_ll_eps1.npy')
        commedia_labels_eps1_multi = numpy.load('Data/commedia_labels_eps1.npy')
    except FileNotFoundError:
        commedia_ll_eps1_multi = numpy.load('../Data/commedia_ll_eps1.npy')
        commedia_labels_eps1_multi = numpy.load('../Data/commedia_labels_eps1.npy')
    
    # Compute posteriors, optimal predictions, and confusion matrix for eps=1.0 model
    commedia_posteriors_eps1_multi = compute_posteriors(commedia_ll_eps1_multi, prior_application)
    commedia_predictions_eps1_multi = compute_optimal_Bayes(commedia_posteriors_eps1_multi, costMatrix_application)
    conf_matrix_eps1_multi = compute_confusion_matrix(commedia_predictions_eps1_multi, commedia_labels_eps1_multi)
    print("    Confusion Matrix (Predicted Row, True Col):")
    print(conf_matrix_eps1_multi)

    # Compute and print unnormalized and normalized DCF for eps=1.0 model
    DCFu_eps1_multi = compute_empirical_Bayes_risk(commedia_predictions_eps1_multi, commedia_labels_eps1_multi, prior_application, costMatrix_application, normalize=False)
    DCF_norm_eps1_multi = compute_empirical_Bayes_risk(commedia_predictions_eps1_multi, commedia_labels_eps1_multi, prior_application, costMatrix_application, normalize=True)
    print(f'    Empirical Bayes risk (Unnormalized DCF): {DCFu_eps1_multi:.3f}')
    print(f'    Normalized empirical Bayes risk (Normalized DCF): {DCF_norm_eps1_multi:.3f}')
    print("\n" + "-"*40)
```

#### Expected Output

```
--- Task 7: Multiclass Bayes Risk Evaluation (Non-Uniform) ---

  Evaluating model with Epsilon = 0.001:
    Confusion Matrix (Predicted Row, True Col):
    [[205 111  56]
     [145 199 121]
     [ 50  92 225]]
    Empirical Bayes risk (Unnormalized DCF): 0.560
    Normalized empirical Bayes risk (Normalized DCF): 0.933

  Evaluating model with Epsilon = 1.0:
    Confusion Matrix (Predicted Row, True Col):
    [[216  77  31]
     [146 236 143]
     [ 38  89 228]]
    Empirical Bayes risk (Unnormalized DCF): 0.485
    Normalized empirical Bayes risk (Normalized DCF): 0.808
```