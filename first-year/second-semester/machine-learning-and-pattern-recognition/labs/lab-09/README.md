# Logistic Regression Models - Laboratory 9 Guide

This guide covers **Logistic Regression models**, focusing on:
*   **Numerical optimization (L-BFGS)** for parameter finding.
*   **Binary Logistic Regression** implementation and evaluation.
*   **Prior-weighted Logistic Regression** for handling imbalanced class priors.
*   **(Optional) Multiclass Logistic Regression**.

---

## Numerical Optimization

Since Logistic Regression's Maximum Likelihood (ML) parameters lack a closed-form solution, **numerical optimization algorithms** are essential for their estimation. This laboratory utilizes `scipy.optimize.fmin_l_bfgs_b`, an efficient **L-BFGS (Limited-memory Broyden–Fletcher–Goldfarb–Shanno)** quasi-Newton method. This method approximates the Hessian matrix using past gradient information, thereby efficiently finding the minimum of an objective function.

### Task 1: Minimizing a Function with Approximate Gradient

**Task Objective:** Minimize the function $f(y, z) = (y + 3)² + \sin(y) + (z + 1)²$ using `scipy.optimize.fmin_l_bfgs_b`. For this task, allow the algorithm to **automatically approximate the gradient**.

**Step-by-Step Reasoning:**
1.  Implement the function `f(x)`, where `x` is expected to be a 1-D NumPy array with `x[0] = y` and `x[1] = z`.
2.  Call `scipy.optimize.fmin_l_bfgs_b`, providing `func=f`, an initial guess `x0=numpy.zeros(2)` (representing `(y,z)=(0,0)`), and setting `approx_grad=True`.
3.  The optimizer will return a tuple containing the `optimized_params` (the `[y, z]` values at the minimum), the `min_value` of the function, and a `details_dict` with optimization information.

#### Solution Code (`sol_optimizer.py`)

```python
# sol_optimizer.py - Example for numerical optimization

import numpy
import scipy.optimize

def f(x):
    """
    The function to be minimized: f(y, z) = (y + 3)² + sin(y) + (z + 1)²
    Args:
        x (numpy.ndarray): A 1D NumPy array where x[0] = y and x[1] = z.
    Returns:
        float: The scalar value of f(y, z).
    """
    y, z = x # Unpack the input array into y and z
    return (y + 3)**2 + numpy.sin(y) + (z + 1)**2

# Call the L-BFGS optimizer
result_approx_grad = scipy.optimize.fmin_l_bfgs_b(func = f, approx_grad = True, x0 = numpy.zeros(2))

print("--- Task 1: Minimizing with Approximate Gradient ---")
print(result_approx_grad)
```

#### Expected Output

```
--- Task 1: Minimizing with Approximate Gradient ---
(array([-2.57747138, -0.99999927]), -0.35614301212286006, {'grad': array([ 1.00000001e-08, -1.38699974e-07]), 'task': b'CONVERGENCE: NORM_OF_PROJECTED_GRADIENT_<=_PGTOL', 'funcalls': 39, 'nit': 7, 'warnflag': 0})
```

### Task 2: Minimizing a Function with Explicit Gradient

**Task Objective:** Minimize the same function $f(y, z)$ as in Task 1, but this time by **explicitly providing its analytically derived gradient** to `scipy.optimize.fmin_l_bfgs_b`. Compare the efficiency (specifically, the `'funcalls'` metric) with the result from Task 1.

**Step-by-Step Reasoning:**
1.  Derive the analytical gradient of $f(y, z)$:
    $$ ∇f(y, z) = [2(y + 3) + \cos(y), 2(z + 1)] $$
2.  Implement a separate function, `fprime(x)`, which computes and returns this 1-D NumPy array representing the gradient.
3.  Call `scipy.optimize.fmin_l_bfgs_b`, now providing both `func=f` and `fprime=fprime`, along with the initial guess `x0=numpy.zeros(2)`.
4.  After execution, compare the `'funcalls'` value obtained from this run with the `'funcalls'` value from Task 1. Providing an explicit analytical gradient is generally more efficient, as numerical approximation methods require multiple function evaluations per iteration to estimate the gradient.

#### Solution Code (`sol_optimizer.py`)

```python
# sol_optimizer.py - Example for numerical optimization

import numpy
import scipy.optimize

def f(x):
    """
    The function to be minimized: f(y, z) = (y + 3)² + sin(y) + (z + 1)²
    Args:
        x (numpy.ndarray): A 1D NumPy array where x[0] = y and x[1] = z.
    Returns:
        float: The scalar value of f(y, z).
    """
    y, z = x
    return (y + 3)**2 + numpy.sin(y) + (z + 1)**2

def fprime(x):
    """
    The analytically derived gradient of f(y, z) = (y + 3)² + sin(y) + (z + 1)².
    Args:
        x (numpy.ndarray): A 1D NumPy array where x[0] = y and x[1] = z.
    Returns:
        numpy.ndarray: A 1D NumPy array representing the gradient [df/dy, df/dz].
    """
    y, z = x
    # Partial derivative w.r.t. y: 2*(y+3) + cos(y)
    # Partial derivative w.r.t. z: 2*(z+1)
    return numpy.array([2 * (y + 3) + numpy.cos(y), 2 * (z + 1)])

# Call the L-BFGS optimizer with the explicit gradient function
result_exact_grad = scipy.optimize.fmin_l_bfgs_b(func = f, fprime = fprime, x0 = numpy.zeros(2))

print("\n--- Task 2: Minimizing with Explicit Gradient ---")
print(result_exact_grad)
```

#### Expected Output

```
--- Task 2: Minimizing with Explicit Gradient ---
(array([-2.57747137, -0.99999927]), -0.35614301212286006, {'grad': array([-0.00000000e+00,  1.49691290e-07]), 'task': b'CONVERGENCE: NORM_OF_PROJECTED_GRADIENT_<=_PGTOL', 'funcalls': 9, 'nit': 7, 'warnflag': 0})
```
**Efficiency Comparison:** Providing the analytical gradient (`fprime`) significantly reduces the number of function calls (`'funcalls'`) (e.g., 9 vs. 39 in the example outputs), demonstrating a substantial gain in computational efficiency.

---

## Binary Logistic Regression

This section applies numerical optimization techniques to **Binary Logistic Regression**. The specific classification task involves distinguishing **Iris Versicolor (which will be re-labeled as Class 1)** from **Iris Virginica (re-labeled as Class 0)**, effectively excluding Iris Setosa from the dataset.

### Data Loading and Preparation

**Task Objective:** Load the complete Iris dataset, then filter it to include only Versicolor (original label 1) and Virginica (original label 2) samples. Subsequently, remap Virginica's label to `0`. Finally, split this processed binary dataset into a 2/3 training set and a 1/3 validation set.

**Step-by-Step Reasoning:**
1.  **Load Iris Data:** Use `sklearn.datasets.load_iris()` to load the full dataset. After loading, `transpose (.T)` the data matrix `D` so that features are in rows and samples are in columns (`features x samples`).
2.  **Filter Classes:** Apply boolean indexing (`L != 0`) to both the data matrix `D` and the labels `L` to remove all samples belonging to Iris Setosa (which originally has label 0).
3.  **Remap Labels:** Modify the labels array `L` by remapping all instances of Virginica (original label 2) to `0` using `L[L == 2] = 0`.
4.  **Split Data:** Utilize the `split_db_2to1(D, L, seed=0)` function to divide the filtered and remapped dataset into a 2/3 training set and a 1/3 validation set. A `seed` value is used for reproducibility.
5.  **Encapsulate Functionality:** Encapsulate all these data loading and preprocessing steps within a single function, `load_iris_binary()`, for modularity and reusability.

#### Solution Code (`sol.py`)

```python
# sol.py - Solution for Logistic Regression Lab

import numpy
import sklearn.datasets # To load the Iris dataset
import scipy.optimize # For fmin_l_bfgs_b
import scipy.special  # For numpy.logaddexp (if using older numpy) / logaddexp from scipy
import bayesRisk # Assuming bayesRisk.py is available (from Lab 8)


# --- Helper functions ---
def vcol(x):
    """ Reshapes a 1D array into a column vector (N, 1). """
    return x.reshape((x.size, 1))

def vrow(x):
    """ Reshapes a 1D array into a row vector (1, N). """
    return x.reshape((1, x.size))

# --- Data splitting function (from Lab 6 or provided) ---
def split_db_2to1(D, L, seed=0):
    """
    Splits dataset D and labels L into training (2/3) and validation (1/3) sets.
    """
    nTotal = D.shape[1]
    nTrain = int(nTotal * 2.0 / 3.0)

    numpy.random.seed(seed)
    idx = numpy.random.permutation(nTotal)

    idxTrain = idx[0:nTrain]
    idxTest = idx[nTrain:]

    DTR = D[:, idxTrain]
    DVAL = D[:, idxTest]
    LTR = L[idxTrain]
    LVAL = L[idxTest]
    return (DTR, LTR), (DVAL, LVAL)

# --- Function to load and preprocess the Iris data for binary classification ---
def load_iris_binary():
    """
    Loads the Iris dataset, filters to Versicolor and Virginica, and remaps labels.
    Versicolor becomes 1, Virginica becomes 0.
    Returns D (features x samples) and L (labels).
    """
    D, L = sklearn.datasets.load_iris()['data'].T, sklearn.datasets.load_iris()['target']
    D = D[:, L != 0] # Filter out Setosa (label 0)
    L = L[L != 0]
    L[L == 2] = 0 # Remap Virginica (label 2) to 0
    return D, L

# --- Main script part for data loading and splitting ---
if __name__ == '__S__main__': # Using __S__main to prevent execution during parsing. Change to __main__ to run.
    print("--- Data Loading and Preparation for Binary Logistic Regression ---")
    D, L = load_iris_binary()
    print(f"Loaded binary Iris data. D shape: {D.shape}, L shape: {L.shape}")

    (DTR, LTR), (DVAL, LVAL) = split_db_2to1(D, L)

    print("\nResulting Data Shapes:")
    print(f"  Training Data (DTR): {DTR.shape}")
    print(f"  Training Labels (LTR): {LTR.shape}")
    print(f"  Validation Data (DVAL): {DVAL.shape}")
    print(f"  Validation Labels (LVAL): {LVAL.shape}")
    print(f"  Unique Labels in Training Set: {numpy.unique(LTR)}")
    print(f"  Unique Labels in Validation Set: {numpy.unique(LVAL)}")
```

#### Expected Output

```
--- Data Loading and Preparation for Binary Logistic Regression ---
Loaded binary Iris data. D shape: (4, 100), L shape: (100,)

Resulting Data Shapes:
  Training Data (DTR): (4, 66)
  Training Labels (LTR): (66,)
  Validation Data (DVAL): (4, 34)
  Validation Labels (LVAL): (34,)
  Unique Labels in Training Set: [0 1]
  Unique Labels in Validation Set: [0 1]
```

### Task 3: Implement the Logistic Regression Objective Function

**Task Objective:** Implement the **regularized binary logistic regression objective function $J(w, b)$** and its **analytically derived gradient**. The implementation should return a tuple `(objective_value, gradient_vector)`, which is compatible with `scipy.optimize.fmin_l_bfgs_b`.
The objective function is defined as:
$$ J(w, b) = \frac{\lambda}{2} \|w\|^2 + \frac{1}{n} \sum_{i=1}^n \left[ \log(1 + \exp(-z_i (w^T x_i + b))) \right] $$

**Step-by-Step Reasoning:**
1.  **Parameter Packing:** The optimization algorithm (`fmin_l_bfgs_b`) expects a single 1-D NumPy array for parameters. Therefore, pack the weight vector `w` (of dimension `D`) and the scalar bias `b` into a single 1-D array `v = [w_1, ..., w_D, b]`. Unpack these parameters inside the objective function.
2.  **Closure for Data Access:** Define the `logreg_obj_with_grad` function *inside* a training function (e.g., `trainLogRegBinary`). This allows `logreg_obj_with_grad` to access `DTR`, `LTR`, and the regularization parameter `l` from its enclosing scope (a "closure").
3.  **Label Transformation:** Convert the binary labels `LTR` (which are `0` or `1`) into a `ZTR` array with values of `-1` or `1`: `ZTR = LTR * 2.0 - 1.0`. This aligns with the $z_i$ in the objective function.
4.  **Compute Scores `s`:** For each sample $x_i$, calculate the linear score $s_i = w^T x_i + b$. This can be vectorized as `s = (vcol(w).T @ DTR).ravel() + b`.
5.  **Compute Loss Term:** Use `numpy.logaddexp(0, -ZTR * s)` to calculate the $\log(1 + \exp(-z_i s_i))$ term for each sample. This `logaddexp` function is crucial for numerical stability, especially when arguments to `exp` are very large or very small.
6.  **Calculate Objective Value:** Combine the average of the `loss_terms` with the regularization term: `objective = loss_terms.mean() + (l/2) * numpy.linalg.norm(w)**2`.
7.  **Gradient Derivation (Analytical):** The gradients are:
    $$ \nabla_w J = \lambda w + \frac{1}{n} \sum_{i=1}^n \left[ \frac{-z_i x_i}{1 + \exp(z_i s_i)} \right] $$
    $$ \frac{\partial J}{\partial b} = \frac{1}{n} \sum_{i=1}^n \left[ \frac{-z_i}{1 + \exp(z_i s_i)} \right] $$
8.  **Vectorized Gradient Computation:**
    *   First, compute an intermediate term `G` for all samples: `G = -ZTR / (1.0 + numpy.exp(ZTR * s))`. This `G` effectively represents the derivative of the loss with respect to the scores $s_i$.
    *   The gradient with respect to `w` (`GW`) is then `(vrow(G) * DTR).mean(axis=1) + l * w.ravel()`. The `mean(axis=1)` handles the summation over samples and division by `n`.
    *   The gradient with respect to `b` (`Gb`) is simply `G.mean()`.
9.  **Pack Gradient:** Combine `GW` and `Gb` into a single 1-D NumPy array, matching the `v` parameter structure: `gradient = numpy.hstack([GW, numpy.array(Gb)])`.
10. **Return Value:** The `logreg_obj_with_grad` function must return the tuple `(objective, gradient)`.
11. **Training Function `trainLogRegBinary`:** This main function will call `scipy.optimize.fmin_l_bfgs_b`, passing `logreg_obj_with_grad` as the objective and gradient function, and `x0=numpy.zeros(D+1)` as the initial guess. After optimization, it unpacks and returns the optimized `w_opt` and `b_opt`.

#### Solution Code (`sol.py`)

```python
# --- Main Training Function for Binary Logistic Regression ---
def trainLogRegBinary(DTR, LTR, l):
    """
    Trains a binary Logistic Regression model using L-BFGS-B optimization.
    Minimizes the regularized cross-entropy loss J(w, b) and computes its gradient.
    """
    D = DTR.shape[0] # Number of features
    n = DTR.shape[1] # Number of samples
    ZTR = LTR * 2.0 - 1.0 # Convert labels from (0, 1) to (-1, 1)

    # Define the objective function and its gradient computation internally (closure)
    def logreg_obj_with_grad(v):
        w = v[:-1] # Unpack weights (all but the last element of v)
        b = v[-1]  # Unpack bias (the last element of v)

        # 1. Calculate scores: s_i = wᵀ x_i + b for all training samples
        # vcol(w).T ensures w is a row vector for dot product with DTR (features x samples)
        # .ravel() converts the (1, n) result to a 1D array (n,)
        s = numpy.dot(vcol(w).T, DTR).ravel() + b

        # 2. Calculate loss terms: log(1 + exp(-zi*si))
        # numpy.logaddexp(0, x) is numerically stable for log(1 + exp(x))
        loss_terms = numpy.logaddexp(0, -ZTR * s)

        # 3. Calculate average loss and regularization term
        average_loss = loss_terms.mean() # Sum over samples, then divide by n (implicit in mean)
        regularization_term = l / 2 * numpy.linalg.norm(w)**2 # L2 regularization
        objective = average_loss + regularization_term

        # --- Gradient Calculation ---
        # 4. Calculate intermediate term G_i = -zi / (1 + exp(zi*si)) for all samples
        # This is the derivative of the loss term log(1 + exp(-zi*si)) w.r.t. si
        # and incorporates -zi for the gradient w.r.t. w and b
        G = -ZTR / (1.0 + numpy.exp(ZTR * s))

        # 5. Calculate gradient w.r.t. w (dJ/dw)
        # (vrow(G) * DTR) performs element-wise multiplication of G (1 x n) with DTR (D x n) after broadcasting
        # The result is (D x n), summing along axis=1 (across samples) yields (D,)
        # l * w.ravel() adds the regularization gradient
        GW = (vrow(G) * DTR).mean(axis=1) + l * w.ravel()

        # 6. Calculate gradient w.r.t. b (dJ/db)
        Gb = G.mean() # Average of G over all samples

        # 7. Combine gradients into a single 1D array
        gradient = numpy.hstack([GW, numpy.array(Gb)])

        return objective, gradient

    # Call the L-BFGS-B optimizer with the defined objective and gradient function
    # x0 is the initial guess for the parameters (w and b), initialized to zeros
    vf, _, _ = scipy.optimize.fmin_l_bfgs_b(logreg_obj_with_grad, x0 = numpy.zeros(D + 1))

    # Evaluate the final objective value with the optimized parameters for reporting
    final_objective, _ = logreg_obj_with_grad(vf)
    print ("Log-reg - lambda = %e - J*(w, b) = %e" % (l, final_objective))

    w_opt = vf[:-1] # Optimized weights
    b_opt = vf[-1]  # Optimized bias

    return w_opt, b_opt
```

### Task 4: Train the Model and Compute Validation Scores

**Task Objective:** Train the binary logistic regression model on the training data (`DTR`, `LTR`) for a given regularization parameter `λ`. Subsequently, compute the raw scores, $s(x_{val}) = (w^*)^T x_{val} + b^*$, for all samples within the validation set (`DVAL`).

**Step-by-Step Reasoning:**
1.  **Train the Model:** Invoke the `trainLogRegBinary(DTR, LTR, lamb_example)` function. This call will execute the optimization process (as implemented in Task 3) and return the optimized weight vector `w_opt` and bias `b_opt`.
2.  **Compute Validation Scores:** Calculate the raw scores for the validation set. This is done by performing a dot product of the optimized weights `w_opt` with the validation data `DVAL`, and then adding the optimized bias `b_opt`. Vectorized, this is `sVal = numpy.dot(w_opt.T, DVAL) + b_opt`.

#### Solution Code (`sol.py`)

```python
if __name__ == '__S__main__': # Using __S__main to prevent execution during parsing. Change to __main__ to run.
    # ... (Previous data loading and splitting code from Task 2) ...
    
    print("\n--- Task 4: Train Model and Compute Validation Scores ---")

    lamb_example = 1e-3 # Example regularization parameter (lambda)

    # 1. Train the logistic regression model using the training data and regularization parameter
    w_opt, b_opt = trainLogRegBinary(DTR, LTR, lamb_example)

    print(f"\n  Optimized w: {w_opt}")
    print(f"  Optimized b: {b_opt}")

    # 2. Compute the raw scores for the validation set using the optimized parameters
    sVal = numpy.dot(w_opt.T, DVAL) + b_opt

    print(f"\n  Scores computed for lambda={lamb_example}. Shape: {sVal.shape}")
```

#### Expected Output

```
--- Task 4: Train Model and Compute Validation Scores ---
Log-reg - lambda = 1.000000e-03 - J*(w, b) = 1.100009e-01

  Optimized w: [-0.01502476 -1.82578586  1.86545195  1.71321034]
  Optimized b: -0.011682897686561585

  Scores computed for lambda=1e-03. Shape: (34,)
```

### Task 5: Evaluate Model Performance (Error Rate, DCF)

**Task Objective:** Assess the performance of the binary Logistic Regression model on the validation set (`DVAL`) for various regularization parameter values, specifically `λ = [1e-3, 1e-1, 1.0]`. Specifically, report: the optimized objective value ($J^*$), the classification error rate, and both the `minDCF` and `actDCF` (calculated for an effective prior $\pi_T=0.5$, a false negative cost $C_{fn}=1$, and a false positive cost $C_{fp}=1$). Note that LLRs are derived from raw scores adjusted by the empirical training prior.

**Step-by-Step Reasoning:**
1.  **Iterate through Lambda:** Set up a loop to iterate through the specified `λ` values.
2.  **Train and Score:** Inside each loop iteration, train the logistic regression model (this will also print $J^*$) and then compute the raw scores `sVal` for the validation set.
3.  **Default Classification & Error Rate:** For a quick assessment, derive predicted labels `PVAL` by thresholding `sVal` at 0: `PVAL = (sVal > 0) * 1`. Then, compute the basic classification error rate: `(PVAL != LVAL).sum() / float(LVAL.size) * 100`.
4.  **Empirical Training Prior:** Calculate the empirical prior of Class 1 in the training set once before the loop: `pEmp = (LTR == 1).sum() / LTR.size`. This prior represents the class distribution the model implicitly learned during training.
5.  **LLR Transformation:** Transform the raw scores `sVal` into Log-Likelihood Ratios (LLRs): `sValLLR = sVal - numpy.log(pEmp / (1.0 - pEmp))`. This adjustment ensures the LLRs are properly calibrated for cost-sensitive evaluation, aligning them with the likelihood ratios often used in Bayes decision theory.
6.  **Define Application Parameters:** Specify the target prior and costs for DCF evaluation: `prior_eval = 0.5`, `cfn = 1.0`, `cfp = 1.0`.
7.  **Calculate minDCF:** Compute the `minDCF` using `bayesRisk.compute_minDCF_binary_fast(sValLLR, LVAL, prior_eval, cfn, cfp)`.
8.  **Calculate actDCF:** Compute the `actDCF` using `bayesRisk.compute_actDCF_binary_fast(sValLLR, LVAL, prior_eval, cfn, cfp)`.

#### Solution Code (`sol.py`)

```python
if __name__ == '__S__main__': # Using __S__main to prevent execution during parsing. Change to __main__ to run.
    # ... (Previous data loading and splitting code from Task 2) ...
    
    print("\n--- Evaluation Summary ---")
    # Calculate the empirical prior of Class 1 from the training set
    pEmp = (LTR == 1).sum() / LTR.size 
    print(f"  Empirical Prior (pEmp) of Class 1 in Training Set: {pEmp:.4f}")

    print("\n--- Standard Logistic Regression Model Evaluation ---")
    for lamb in [1e-3, 1e-1, 1.0]: # Iterate through different lambda values
        print(f"\nProcessing Lambda = {lamb:.1e}:")
        w, b = trainLogRegBinary(DTR, LTR, lamb) # Train the model (this function prints J*)
        sVal = numpy.dot(w.T, DVAL) + b # Compute raw scores on the validation set

        # Default classification (threshold at 0) and error rate calculation
        PVAL = (sVal > 0) * 1 # Predict 1 if score > 0, else 0
        err = (PVAL != LVAL).sum() / float(LVAL.size)
        print (f"  Validation Error Rate: {err*100:.1f}%")

        # Transform raw scores into Log-Likelihood Ratios (LLRs)
        # This adjustment calibrates the scores based on the empirical prior
        sValLLR = sVal - numpy.log(pEmp / (1.0 - pEmp)) 

        # Define evaluation parameters for DCF calculation
        prior_eval = 0.5 # Target effective prior for evaluation
        cfn = 1.0        # Cost of False Negative
        cfp = 1.0        # Cost of False Positive

        # Calculate minDCF and actDCF using the bayesRisk helper functions
        minDCF_val = bayesRisk.compute_minDCF_binary_fast(sValLLR, LVAL, prior_eval, cfn, cfp)
        actDCF_val = bayesRisk.compute_actDCF_binary_fast(sValLLR, LVAL, prior_eval, cfn, cfp)

        print (f"  minDCF (for $\\pi_T$=0.5): {minDCF_val:.4f}")
        print (f"  actDCF (for $\\pi_T$=0.5): {actDCF_val:.4f}")
```

#### Expected Output

```
--- Evaluation Summary ---
  Empirical Prior (pEmp) of Class 1 in Training Set: 0.5000

--- Standard Logistic Regression Model Evaluation ---

Processing Lambda = 1.0e-03:
Log-reg - lambda = 1.000000e-03 - J*(w, b) = 1.100009e-01
  Validation Error Rate: 8.8%
  minDCF (for $\pi_T$=0.5): 0.0625
  actDCF (for $\pi_T$=0.5): 0.1181

Processing Lambda = 1.0e-01:
Log-reg - lambda = 1.000000e-01 - J*(w, b) = 4.539407e-01
  Validation Error Rate: 11.8%
  minDCF (for $\pi_T$=0.5): 0.0556
  actDCF (for $\pi_T$=0.5): 0.1111

Processing Lambda = 1.0e+00:
Log-reg - lambda = 1.000000e+00 - J*(w, b) = 6.316436e-01
  Validation Error Rate: 14.7%
  minDCF (for $\pi_T$=0.5): 0.1111
  actDCF (for $\pi_T$=0.5): 0.1667
```

---

### Task 6: Prior-Weighted Logistic Regression and Calibration

**Task Objective:** Implement and test **prior-weighted logistic regression**. This approach is particularly useful when the target application prior ($\pi_T$) for a class (e.g., Class 1) differs significantly from its empirical training prior. For this task, use a target prior $\pi_T = 0.8$ for Class 1. Evaluate the model's performance for regularization parameters `λ = [1e-3, 1e-1, 1.0]` by computing $J^*$, `minDCF`, and `actDCF` (calculated for $\pi_T = 0.8$, $C_{\text{fn}} = 1$, $C_{\text{fp}} = 1$). A crucial step is to correctly adjust the LLRs using the specified target prior.

**Step-by-Step Reasoning:**
1.  **Weighted Objective Function:** The objective function for prior-weighted logistic regression includes a weighting factor $\xi_i$ for each sample's loss term.
    *   For samples where $z_i = +1$ (Class 1), the weight is $\xi_i = \pi_T / n_T$, where $n_T$ is the number of samples in Class 1.
    *   For samples where $z_i = -1$ (Class 0), the weight is $\xi_i = (1 - \pi_T) / n_F$, where $n_F$ is the number of samples in Class 0.
2.  **Training Function `trainWeightedLogRegBinary(DTR, LTR, l, pT)`:**
    *   First, calculate `nTrue` and `nFalse` (counts of positive and negative class samples in the training set).
    *   Compute the per-sample weights: `wTrue_xi = pT / nTrue` and `wFalse_xi = (1 - pT) / nFalse`.
    *   **Modifications within `logreg_obj_with_grad` (the nested objective function):**
        *   **Objective Calculation:** Compute the `base_loss_terms` (unweighted) as before. Then, for the objective calculation, assign specific weights to the `base_loss_terms`: `wTrue_xi` for positive class samples (`ZTR > 0`) and `wFalse_xi` for negative class samples (`ZTR < 0`). The total objective is then the sum of these `weighted_loss_terms` plus the `regularization` term.
        *   **Gradient `G`:** Similarly, compute the `base_G` (unweighted gradient factor). Apply these weights (`wTrue_xi`, `wFalse_xi`) to the `base_G` term (which represents the unweighted gradient factor) to obtain `weighted_G`.
        *   The gradients `GW` and `Gb` are then calculated by summing `weighted_G` (for `Gb`) or by multiplying `weighted_G` by `DTR` and summing (for `GW`), and finally adding the regularization gradient `l * w.ravel()` to `GW`.
3.  **Evaluation:**
    *   Set the training target prior: `pT_train = 0.8`.
    *   Call `trainWeightedLogRegBinary(...)` to train the model.
    *   Compute raw scores `sVal` on the validation set.
    *   **LLR Adjustment (Crucial for Calibration):** Crucially, adjust the raw scores `sVal` into LLRs: `sValLLR = sVal - numpy.log(pT_train / (1.0 - pT_train))`. This step calibrates the LLRs for the target prior, ensuring they are appropriate for cost-sensitive evaluation under $\pi_T$.
    *   Set `prior_eval` to `pT_train` for consistency in evaluation.
    *   Compute `minDCF_val` and `actDCF_val` using the `bayesRisk` functions.

#### Solution Code (`sol.py`)

```python
# --- Training function for Prior-Weighted Binary Logistic Regression ---
def trainWeightedLogRegBinary(DTR, LTR, l, pT):
    """
    Trains a prior-weighted binary Logistic Regression model.
    Minimizes the weighted regularized cross-entropy loss J(w, b) and its gradient.
    """
    D = DTR.shape[0]
    ZTR = LTR * 2.0 - 1.0 # Convert labels from (0, 1) to (-1, 1)

    nTrue = (ZTR > 0).sum() # Number of samples in the positive class (z_i = +1)
    nFalse = (ZTR < 0).sum() # Number of samples in the negative class (z_i = -1)

    # Calculate per-sample weights based on the target prior (pT)
    wTrue_xi = pT / nTrue # Weight for positive class samples
    wFalse_xi = (1 - pT) / nFalse # Weight for negative class samples

    def logreg_obj_with_grad(v):
        w = v[:-1]
        b = v[-1]

        s = numpy.dot(vcol(w).T, DTR).ravel() + b # Raw scores for all training samples

        base_loss_terms = numpy.logaddexp(0, -ZTR * s) # Unweighted loss for each sample
        
        # Apply specific weights to the loss terms based on class label
        weighted_loss_terms = numpy.array(base_loss_terms) # Create a mutable copy
        weighted_loss_terms[ZTR > 0] *= wTrue_xi # Apply weight for positive class samples
        weighted_loss_terms[ZTR < 0] *= wFalse_xi # Apply weight for negative class samples

        regularization = l / 2 * numpy.linalg.norm(w)**2 # L2 Regularization term
        objective = weighted_loss_terms.sum() + regularization # Total weighted objective value

        # --- Gradient Calculation ---
        base_G = -ZTR / (1.0 + numpy.exp(ZTR * s)) # Unweighted gradient factor for each sample
        
        # Apply specific weights to the gradient factors
        weighted_G = numpy.array(base_G) # Create a mutable copy
        weighted_G[ZTR > 0] *= wTrue_xi
        weighted_G[ZTR < 0] *= wFalse_xi

        # Calculate gradients for w and b, scaled by sample weights
        # Note: Summing (weighted_G * DTR) replaces the mean() used in unweighted version
        GW = (vrow(weighted_G) * DTR).sum(axis=1) + l * w.ravel() 
        Gb = weighted_G.sum() 
        gradient = numpy.hstack([GW, numpy.array(Gb)])

        return objective, gradient

    # Call the L-BFGS-B optimizer
    vf, _, _ = scipy.optimize.fmin_l_bfgs_b(logreg_obj_with_grad, x0 = numpy.zeros(D + 1))

    # Evaluate the final objective value with optimized parameters
    final_objective, _ = logreg_obj_with_grad(vf)
    print (f"Weighted Log-reg ($\\pi_T$={pT:.1e}) - $\\lambda$={l:.1e} - J*(w, b)={final_objective:.6e}")

    w_opt = vf[:-1]
    b_opt = vf[-1]
    return w_opt, b_opt

if __name__ == '__S__main__': # Using __S__main to prevent execution during parsing. Change to __main__ to run.
    # ... (Previous data loading and splitting code from Task 2) ...

    print("\n--- Task 6: Prior-Weighted Logistic Regression Evaluation ---")

    pT_train = 0.8 # Target prior for training and evaluation

    print(f"  Training Weighted LogReg with Target Prior ($\\pi_T$) = {pT_train}:")
    for lamb in [1e-3, 1e-1, 1.0]:
        # Train the weighted logistic regression model
        w, b = trainWeightedLogRegBinary(DTR, LTR, lamb, pT = pT_train) 

        sVal = numpy.dot(w.T, DVAL) + b # Compute raw scores on validation set

        # Crucially, adjust raw scores to LLRs using the *target* prior pT_train for calibration
        sValLLR = sVal - numpy.log(pT_train / (1.0 - pT_train)) 

        prior_eval = pT_train # Evaluate using the same target prior
        cfn = 1.0
        cfp = 1.0

        # Calculate minDCF and actDCF
        minDCF_val = bayesRisk.compute_minDCF_binary_fast(sValLLR, LVAL, prior_eval, cfn, cfp)
        actDCF_val = bayesRisk.compute_actDCF_binary_fast(sValLLR, LVAL, prior_eval, cfn, cfp)

        print (f"  minDCF (for $\\pi_T$={prior_eval:.1f}): {minDCF_val:.4f}")
        print (f"  actDCF (for $\\pi_T$={prior_eval:.1f}): {actDCF_val:.4f}")
        print()
```

#### Expected Output

```
--- Prior-Weighted Logistic Regression Evaluation ($\pi_T$ = 0.8) ---

  Training Weighted LogReg with Target Prior ($\pi_T$) = 0.8:

Weighted Log-reg ($\pi_T$=8.0e-01) - $\lambda$=1.0e-03 - J*(w, b)=9.401035e-02
  minDCF (for $\pi_T$=0.8): 0.1667
  actDCF (for $\pi_T$=0.8): 0.2222

Weighted Log-reg ($\pi_T$=8.0e-01) - $\lambda$=1.0e-01 - J*(w, b)=3.606261e-01
  minDCF (for $\pi_T$=0.8): 0.0556
  actDCF (for $\pi_T$=0.8): 0.7222

Weighted Log-reg ($\pi_T$=8.0e-01) - $\lambda$=1.0e+00 - J*(w, b)=4.724715e-01
  minDCF (for $\pi_T$=0.8): 0.1111
  actDCF (for $\pi_T$=0.8): 1.0000
```

---

## [Optional] Multiclass Logistic Regression

This section introduces **Multiclass Logistic Regression**, also known as **Softmax Regression**. This extends the binary logistic regression framework to handle classification problems involving more than two distinct classes.

### Task 7: Implement Multiclass Logistic Regression

**Task Objective:** Implement the **multiclass logistic regression objective function** and its **analytically derived gradient**. Then, train this model on the full Iris dataset (which has 3 classes) and evaluate its optimized objective value ($J^*$) and validation error rate for different regularization parameters (`λ`).

**Step-by-Step Reasoning:**
1.  **Parameter Structure:** For multiclass regression, we have a weight matrix `W` (of shape `D x K`, where `D` is the number of features and `K` is the number of classes) and a bias vector `b` (of shape `K x 1`). These parameters need to be packed into a single 1-D array `v` for the optimizer. A common packing scheme is `v = [W.ravel(), b.ravel()]`. Unpacking involves `W = v[:D*K].reshape((D, K))` and `b = v[D*K:].reshape((K, 1))` (or `K,` if a 1D array is desired).
2.  **Compute Scores `S`:** Calculate the scores for all classes and samples: `S = W.T @ DTR + vcol(b)`. The resulting `S` matrix will have dimensions `(K x n)`, where `n` is the number of samples.
3.  **Compute Probabilities (Softmax):** Apply the softmax function in the log-domain for numerical stability. This yields `LogProbs = S - vrow(scipy.special.logsumexp(S, axis=0))`. `LogProbs` will also be `(K x n)`.
4.  **1-of-K Labels `T`:** Convert the integer labels `LTR` (e.g., 0, 1, 2) into a 1-of-K (one-hot) encoding target matrix `T`. This matrix will have dimensions `(K x n)`: `T = numpy.eye(K)[LTR].T`.
5.  **Objective Function:** The multiclass objective function is:
    $$ J(W, b) = \frac{\lambda}{2} \|W\|_F^2 - \frac{1}{n} (T * LogProbs).sum() $$
    The `(T * LogProbs).sum()` efficiently sums only the log-probabilities corresponding to the true classes due to the one-hot encoding of `T`.
6.  **Gradient Derivation (Recommended Vectorized Form):**
    *   Compute the predicted probabilities `Y` using `Y = numpy.exp(LogProbs)`. `Y` will be `(K x n)`.
    *   Calculate the error term: `error_term = Y - T`. This `error_term` is `(K x n)`.
    *   The gradient with respect to `W` ($\nabla_W J$) is:
        $$ \nabla_W J = \frac{1}{n} DTR @ error\_term.T + \lambda W $$
        This results in a `(D x K)` matrix.
    *   The gradient with respect to `b` ($\nabla_b J$) is:
        $$ \nabla_b J = \frac{1}{n} error\_term.sum(axis=1) $$
        This results in a `(K,)` 1D array.
7.  **Training Function `trainLogRegMulti`:** Call `scipy.optimize.fmin_l_bfgs_b` with the packed parameter vector `v`, the combined objective/gradient function, and an initial guess `x0`. After optimization, unpack the returned parameters to get `W_opt` and `b_opt`.
8.  **Prediction:** To predict labels for validation data (`DVAL`), compute scores `s_val_multi = numpy.dot(W_opt.T, DVAL) + vcol(b_opt)`. The predicted labels are then `predicted_labels = numpy.argmax(s_val_multi, axis=0)`.
9.  **Evaluation:** Calculate the classification error rate: `(predicted_labels != LVAL).sum() / LVAL.size`.

#### Solution Code

```python
# Conceptual Multiclass Logistic Regression implementation
# This code is illustrative and would need to be properly integrated and tested.

# import numpy
# import scipy.optimize
# import scipy.special # For logsumexp

# # Assume vcol, vrow, split_db_2to1 are defined
# # Assume load_iris (full 3-class data) is defined

# def trainLogRegMulti(DTR, LTR, l):
#     """
#     Trains a multiclass Logistic Regression (Softmax Regression) model.
#     Minimizes the regularized cross-entropy loss and computes its gradient.
#     """
#     D = DTR.shape[0] # Number of features
#     n = DTR.shape[1] # Number of samples
#     K = LTR.max() + 1 # Number of classes (assuming labels are 0, ..., K-1)

#     # Convert labels to 1-of-K encoding (Target matrix T)
#     T = numpy.eye(K)[LTR].T # shape K x n

#     def logreg_multi_obj_with_grad(v_packed):
#         # Unpack parameters: W (D x K) and b (K,)
#         W = v_packed[:D*K].reshape((D, K))
#         b = v_packed[D*K:] # K,

#         # 1. Calculate scores: S_ik = w_k^T x_i + b_k
#         # W.T is (K x D), DTR is (D x n) => dot product is (K x n)
#         S = numpy.dot(W.T, DTR) + vcol(b) # S is (K x n)

#         # 2. Calculate log-probabilities (Softmax in log-domain for numerical stability)
#         log_sum_exp_scores = vrow(scipy.special.logsumexp(S, axis=0)) # (1 x n) - for normalization
#         LogProbs = S - log_sum_exp_scores # (K x n) - log-probabilities P(C=k|x_i)

#         # 3. Calculate regularization term: (lambda/2 * ||W||_F^2)
#         # ||W||_F^2 is Frobenius norm squared, which is sum of squares of all elements
#         regularization = l / 2 * numpy.linalg.norm(W)**2

#         # 4. Calculate loss term: - (1/n) * sum_i sum_k (T_ik * log y_ik)
#         # (T * LogProbs) performs element-wise multiplication, then sum() sums all elements
#         loss = -(1.0 / n) * (T * LogProbs).sum()

#         # 5. Total objective function value
#         objective = loss + regularization

#         # --- Gradient Calculation ---
#         # 6. Compute predicted probabilities: Y_ik = P(C=k|x_i)
#         Y = numpy.exp(LogProbs) # (K x n)

#         # 7. Compute the "error" term for gradients: Y - T
#         # This error term is (predicted probability - true one-hot label)
#         error_term = Y - T # (K x n)

#         # 8. Gradient w.r.t. W (dJ/dW)
#         # DTR (D x n) @ error_term.T (n x K) => (D x K)
#         GW = (1.0 / n) * (DTR @ error_term.T) + l * W # (D x K)

#         # 9. Gradient w.r.t. b (dJ/db)
#         # Sum error_term along samples (axis=1) to get (K,) vector
#         Gb = (1.0 / n) * error_term.sum(axis=1) # (K,)

#         # 10. Pack gradients into a single 1D array
#         gradient_packed = numpy.hstack([GW.ravel(), Gb])

#         return objective, gradient_packed

#     # Call the optimizer with the defined objective and gradient function
#     # x0 is initial guess for all parameters (W_ravel and b)
#     v_packed_opt, _, _ = scipy.optimize.fmin_l_bfgs_b(logreg_multi_obj_with_grad, x0 = numpy.zeros(D * K + K))

#     # Unpack optimized parameters
#     W_opt = v_packed_opt[:D*K].reshape((D, K))
#     b_opt = v_packed_opt[D*K:]

#     # Evaluate the final objective for reporting
#     final_objective, _ = logreg_multi_obj_with_grad(v_packed_opt)
#     print (f"Multiclass Log-reg - lambda = {l:.1e} - J*(W, b) = {final_objective:.6e}")

#     return W_opt, b_opt

# if __name__ == '__S__main__': # Using __S__main to prevent execution during parsing. Change to __main__ to run.
#     # Load full Iris dataset (for 3 classes)
#     def load_iris_full():
#         iris = sklearn.datasets.load_iris()
#         return iris['data'].T, iris['target']

#     D_full, L_full = load_iris_full()
#     (DTR_multi, LTR_multi), (DVAL_multi, LVAL_multi) = split_db_2to1(D_full, L_full)

#     print("\n--- Task 7: Multiclass Logistic Regression Evaluation ---")
#     for lamb in [1e-3, 1e-1, 1.0]:
#         print(f"\nProcessing Lambda = {lamb:.1e}:")
#         W_opt, b_opt = trainLogRegMulti(DTR_multi, LTR_multi, lamb)

#         # Compute scores on the validation set using optimized parameters
#         s_val_multi = numpy.dot(W_opt.T, DVAL_multi) + vcol(b_opt) 
#         # Predict labels by taking the class with the highest score
#         predicted_labels_multi = numpy.argmax(s_val_multi, axis=0) 

#         # Calculate validation error rate
#         err_multi = (predicted_labels_multi != LVAL_multi).sum() / float(LVAL_multi.size)
#         print (f"  Validation Error Rate: {err_multi*100:.1f}%")
```

#### Expected Output

```
--- Task 7: Multiclass Logistic Regression Evaluation ---

Processing Lambda = 1.0e-03:
Multiclass Log-reg - lambda = 1.0e-03 - J*(W, b) = 9.691763e-02
  Validation Error Rate: 4.0%

Processing Lambda = 1.0e-01:
Multiclass Log-reg - lambda = 1.0e-01 - J*(W, b) = 5.000329e-01
  Validation Error Rate: 6.0%

Processing Lambda = 1.0e+00:
Multiclass Log-reg - lambda = 1.0e+00 - J*(W, b) = 8.216263e-01
  Validation Error Rate: 18.0%
```