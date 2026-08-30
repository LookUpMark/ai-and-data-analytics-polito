# Generative Density Estimation - Laboratory 5 Guide

This guide covers **Generative Density Estimation**, with a primary focus on **Multivariate Gaussian (MVG) distribution** log-density calculation and **Maximum Likelihood (ML) parameter estimation**.

**Prerequisites:**
To effectively follow this guide, familiarity with the following concepts and tools is recommended:
*   Probability density functions (PDFs)
*   Gaussian (Normal) distribution properties
*   Basic statistical concepts (mean, covariance)
*   Fundamental matrix operations (inverse, determinant, multiplication)
*   Logarithm properties (products, powers)
*   NumPy proficiency

## Data Convention for this Lab

Throughout this laboratory, the following data conventions will be used:
*   **Samples (`x`):** Represented as a column vector.
*   **Dataset (`X`):** Structured as a matrix with features arranged in rows and samples in columns (Shape: `M x N`).
*   **Mean (`mu` or `μ`):** An `M x 1` column vector.
*   **Covariance Matrix (`C` or `Σ`):** An `M x M` square matrix.

---

## Task 1: Log-Density of Multivariate Gaussian (Single Sample)

### Task Objective

Implement the function `logpdf_GAU_ND_singleSample(x, mu, C)`. This function should calculate the **log-PDF of a Multivariate Gaussian distribution** for a single given sample.

*   `x`: The input sample, an `M x 1` column vector.
*   `mu`: The mean vector, an `M x 1` column vector.
*   `C`: The covariance matrix, an `M x M` matrix.

### Step-by-Step Reasoning

The M-dimensional MVG PDF for a sample `x` given mean `μ` and covariance `Σ` is defined by the formula:
$$ N(x|μ, Σ) = \frac{1}{\sqrt{(2\pi)^M \cdot |\Sigma|}} \cdot \exp\left( -0.5 \cdot (x - μ)ᵀ \Sigma⁻¹ (x - μ) \right) $$

To improve numerical stability, it is beneficial to work with the logarithm of the PDF (log-PDF). The log-PDF formula is:
$$ \log N(x|μ, Σ) = - \frac{M}{2} \cdot \log(2\pi) - \frac{1}{2} \cdot \log(|\Sigma|) - 0.5 \cdot (x - μ)ᵀ \Sigma⁻¹ (x - μ) $$

To compute this, follow these computational steps:

1.  **Dimensionality (`M`):** Determine the number of features using `x.shape[0]`.
2.  **Inverse Covariance (`P` or `Σ⁻¹`):** Compute the inverse of the covariance matrix `C` using `numpy.linalg.inv(C)`.
3.  **Log-Determinant (`log|C|`):** Calculate the logarithm of the determinant of `C` using `numpy.linalg.slogdet(C)[1]`. This function is preferred for its numerical robustness.
4.  **Difference Vector (`d`):** Compute the difference between the sample and the mean: `d = x - mu`.
5.  **Quadratic Term (Mahalanobis Distance Squared):** Calculate the term `d.T @ P @ d`. This operation results in a `(1, 1)` array representing the squared Mahalanobis distance.
6.  **Combine Terms:** Sum the three main components of the log-PDF formula: `(- (M/2) ⋅ numpy.log(2π))`, `(- 0.5 ⋅ log|C|)`, and `(- 0.5 ⋅ quadratic_term)`.
7.  **Return Scalar:** Apply `.ravel()` to the final sum to convert the result into a `(1,)` 1D array, which represents a single scalar log-density value.

#### Solution Code (`gau_density.py`)

```python
import numpy

# Assuming vcol is defined (reshapes 1D array to Nx1 column vector)
# def vcol(x):
#    return x.reshape((x.size, 1))

def logpdf_GAU_ND_singleSample(x, mu, C):
    """
    Computes log-density of Multivariate Gaussian for a single sample.

    Args:
        x (numpy.ndarray): Single data sample (M x 1 column vector).
        mu (numpy.ndarray): Mean vector (M x 1 column vector).
        C (numpy.ndarray): Covariance matrix (M x M).

    Returns:
        numpy.ndarray: 1-element array containing the log-density.
    """
    M = x.shape[0] # Number of features
    
    P = numpy.linalg.inv(C) # Calculate Σ⁻¹
    log_det_C = numpy.linalg.slogdet(C)[1] # Calculate log|Σ|
    
    diff = x - mu # Calculate (x - μ)
    mahalanobis_sq = (diff.T @ P @ diff) # Calculate (x - μ)ᵀ Σ⁻¹ (x - μ)

    # Combine log-PDF formula components
    log_pi_term = -0.5 * M * numpy.log(numpy.pi * 2)
    log_det_term = -0.5 * log_det_C
    mahalanobis_term = -0.5 * mahalanobis_sq

    return (log_pi_term + log_det_term + mahalanobis_term).ravel()
```

### Expected Output

This function is designed to return a single log-density value encapsulated within a 1-element NumPy array.

---

## Task 2: Log-Density of Multivariate Gaussian (Multiple Samples)

### Task Objective

Implement two distinct functions to compute the MVG log-PDF for an entire dataset `X` (shaped `M x N`):
1.  **`logpdf_GAU_ND_slow(X, mu, C)`:** This version should iterate through each sample in the dataset, calling `logpdf_GAU_ND_singleSample` for each.
2.  **`logpdf_GAU_ND_fast(X, mu, C)`:** This version should leverage vectorized NumPy operations to compute the log-PDF for all samples simultaneously, prioritizing computational efficiency.

### Step-by-Step Reasoning

For each function, the following steps are involved:

**For `logpdf_GAU_ND_slow` (Looping Implementation):**
1.  Obtain the number of samples `N = X.shape[1]`.
2.  Iterate with a loop variable `i` from `0` to `N-1`.
3.  For each iteration, extract the current sample `X[:, i:i+1]`, ensuring it retains the `M x 1` column vector shape.
4.  Call `logpdf_GAU_ND_singleSample` for this individual sample.
5.  Collect all individual log-density results into a list. Finally, use `numpy.hstack()` and `.ravel()` to concatenate these results into a single `(N,)` 1D array.

**For `logpdf_GAU_ND_fast` (Vectorized Implementation):**
1.  Retrieve the number of features `M = X.shape[0]` (and `N = X.shape[1]` for context, though `N` is implicitly handled).
2.  Compute the constant terms of the log-PDF formula (`log_pi_term`, `log_det_term`) only once, including the precision matrix `P = numpy.linalg.inv(C)`.
3.  **Vectorized Mahalanobis Term:**
    *   Calculate the difference matrix `D = X - mu`. NumPy's broadcasting capabilities automatically handle the subtraction of the `M x 1` mean vector `mu` from the `M x N` dataset `X`, resulting in an `M x N` difference matrix.
    *   Compute the squared Mahalanobis distances for all samples simultaneously using `mahalanobis_sq_vector = (D * (P @ D)).sum(0)`.
        *   `(P @ D)` performs matrix multiplication: `(M x M) @ (M x N)` resulting in an `(M x N)` matrix.
        *   `D * (...)` performs element-wise multiplication between two `(M x N)` matrices.
        *   `.sum(0)` sums the elements along columns, yielding a `(N,)` 1D array, where each element is the squared Mahalanobis distance for a corresponding sample.
    *   Finally, `mahalanobis_term_vector = -0.5 * mahalanobis_sq_vector`.
4.  Combine the pre-calculated constant terms with the `mahalanobis_term_vector`. Broadcasting will apply the scalar constant terms across all elements of the vector, resulting in the final `(N,)` array of log-densities.

#### Solution Code (`gau_density.py`)

```python
import numpy

# Assuming logpdf_GAU_ND_singleSample, vcol, vrow are defined (from previous tasks/setup)

# --- Slow version using a loop ---
def logpdf_GAU_ND_slow(X, mu, C):
    """
    Computes log-density for multiple samples using a loop.
    Args:
        X (numpy.ndarray): Dataset (M x N matrix).
        mu (numpy.ndarray): Mean vector (M x 1 column vector).
        C (numpy.ndarray): Covariance matrix (M x M).
    Returns:
        numpy.ndarray: 1D array of log-densities (N,).
    """
    N = X.shape[1]
    ll = [logpdf_GAU_ND_singleSample(X[:, i:i+1], mu, C) for i in range(N)]
    return numpy.hstack(ll).ravel()


# --- Fast version using vectorized operations ---
def logpdf_GAU_ND_fast(X, mu, C):
    """
    Computes log-density for multiple samples using vectorized NumPy operations.
    Args:
        X (numpy.ndarray): Dataset (M x N matrix).
        mu (numpy.ndarray): Mean vector (M x 1 column vector).
        C (numpy.ndarray): Covariance matrix (M x M).
    Returns:
        numpy.ndarray: 1D array of log-densities (N,).
    """
    M = X.shape[0]
    # N = X.shape[1] # N is not directly used in the formula, but useful for understanding

    # Calculate constant terms
    log_pi_term = -0.5 * M * numpy.log(numpy.pi * 2)
    log_det_C = numpy.linalg.slogdet(C)[1]
    log_det_term = -0.5 * log_det_C

    P = numpy.linalg.inv(C) # Precision matrix
    
    D = X - mu # Difference matrix, broadcasted subtraction
    mahalanobis_sq_vector = (D * (P @ D)).sum(0) # Vectorized Mahalanobis distance squared
    mahalanobis_term_vector = -0.5 * mahalanobis_sq_vector

    return log_pi_term + log_det_term + mahalanobis_term_vector

# Set the default log-PDF function for use in other tasks
# Initially set to slow for verification purposes; switch to fast for performance.
logpdf_GAU_ND = logpdf_GAU_ND_slow 
# logpdf_GAU_ND = logpdf_GAU_ND_fast
```

### Expected Output

```
1.1102230246251565e-16  # Max absolute difference for 1D case (using _fast version)
0.0                     # Max absolute difference for 1D case (using _slow version)
4.440892098500626e-16   # Max absolute difference for ND case (using _fast version)
0.0                     # Max absolute difference for ND case (using _slow version)
```

---

## Task 3: Computing Log-Likelihood

### Task Objective

Implement the `compute_ll(X, mu, C)` function to calculate the **total log-likelihood** of a given dataset `X`. This calculation should be based on an MVG model defined by its mean `mu` and covariance `C`.

### Step-by-Step Reasoning

The log-likelihood, denoted as `l(μ, Σ | X)`, for an entire dataset `X` is fundamentally the sum of the individual log-densities of each sample within the dataset. This relationship is expressed as:
$$ l(μ, Σ | X) = \sum_{i=1}^{N} \log N(x_i | μ, Σ) $$

Therefore, the computation involves two straightforward steps:

1.  Compute the individual log-densities for all samples in `X`. It is recommended to use the `logpdf_GAU_ND` function (preferably its `_fast` vectorized implementation) for this step.
2.  Sum all the resulting `N` individual log-density values obtained from the previous step, using `numpy.sum()`. This sum will represent the total log-likelihood.

#### Solution Code (`gau_density.py`)

```python
import numpy
# Assuming logpdf_GAU_ND is defined (either _slow or _fast from Task 2)

def compute_ll(X, mu, C):
    """
    Computes the total log-likelihood of dataset X given MVG parameters mu and C.

    Args:
        X (numpy.ndarray): Dataset (M x N matrix).
        mu (numpy.ndarray): Mean vector (M x 1 column vector).
        C (numpy.ndarray): Covariance matrix (M x M).

    Returns:
        float: The total log-likelihood (a scalar value).
    """
    log_densities = logpdf_GAU_ND(X, mu, C) # Compute log-densities for all samples
    total_log_likelihood = log_densities.sum() # Sum them for total log-likelihood
    return total_log_likelihood
```

### Expected Output

```
# Output for ll_XND = compute_ll(XND, m_ML_XND, C_ML_XND)
-270.70478023795044

# Output for ll_X1D = compute_ll(X1D, m_ML_X1D, C_ML_X1D)
-23227.077654602715

# Output for non-ML parameters for X1D (values will be lower/more negative)
# Example alternative mean: mu = numpy.array([[1.0]])
-23251.159053334717
# Example alternative variance: C = numpy.array([[2.0]])
-23428.619886017754
# Example small change to ML variance: C = numpy.array([[6.1]])
-23227.17671182687
```

---

## Task 4: Maximum Likelihood Estimation

### Task Objective

This task delves into **Maximum Likelihood (ML) Estimation** for MVG parameters. The core concept is that the ML estimates for the mean (`μ_ML`) and covariance (`Σ_ML`) of a dataset are simply the dataset's **empirical (sample) mean** and **empirical (sample) covariance**, respectively. To achieve this, you will utilize the `compute_mu_C` function to calculate these statistics for both the `XND` (multidimensional) and `X1D` (1-dimensional) sample datasets.

### Step-by-Step Reasoning

The Maximum Likelihood principle aims to find the set of parameters for a model that makes the observed data most probable. For a Multivariate Gaussian distribution, these ML estimates correspond directly to the sample statistics derived from the data:

*   **μ_ML:** This is the sample mean, calculated as:
    $$ \mu_{ML} = \frac{1}{N} \sum_{i=1}^{N} x_i $$
*   **Σ_ML:** This represents the sample covariance, calculated as:
    $$ \Sigma_{ML} = \frac{1}{N} \sum_{i=1}^{N} (x_i - \mu_{ML})(x_i - \mu_{ML})^\intercal $$

Conveniently, the `compute_mu_C` function (which was either developed in Laboratory 3 or provided as a utility) directly performs these calculations, thus providing the empirical mean and covariance matrix.

#### Solution Code (`gau_density.py` - within `if __name__ == '__main__':`)

```python
# Assuming compute_mu_C is defined (e.g., from a shared utils file or copied in)
# Example definition of compute_mu_C (as in Lab 3):
# def vcol(x):
#    return x.reshape((x.size, 1))
# def compute_mu_C(D):
#    mu = vcol(D.mean(1))
#    C = ((D-mu) @ (D-mu).T) / float(D.shape[1])
#    return mu, C

# --- Load the sample datasets for demonstration ---
# XND = numpy.load('XND.npy') # Multidimensional dataset
# X1D = numpy.load('X1D.npy') # 1-dimensional dataset

# --- Example usage for XND dataset ---
# Calculate the ML estimates for mean and covariance of XND
m_ML_XND, C_ML_XND = compute_mu_C(XND)
print("ML Mean (XND):")
print(m_ML_XND)
print("\nML Covariance (XND):")
print(C_ML_XND)

# --- Example usage for X1D dataset ---
# Calculate the ML estimates for mean and covariance of X1D
m_ML_X1D, C_ML_X1D = compute_mu_C(X1D)
print("\nML Mean (X1D):")
print(m_ML_X1D)
print("\nML Covariance (X1D):")
print(C_ML_X1D)
```

### Expected Output

```
ML Mean (XND):
[[-0.07187197]
 [ 0.05979594]]

ML Covariance (XND):
[[0.94590166 0.09313534]
 [0.09313534 0.8229693 ]]

ML Mean (X1D):
[[1.9539157]]

ML Covariance (X1D):
[[6.09542485]]
```

---

## Task 5: Visualization of Estimated Density Fit

### Task Objective

Visually assess how well an ML-estimated Gaussian PDF fits the `X1D` dataset. This will be achieved by creating a plot that shows the **normalized histogram** of the `X1D` data, with the **computed Gaussian PDF curve** overlaid directly on top.

### Step-by-Step Reasoning

To create this visualization and assess the fit:

1.  Load the `X1D` dataset. Ensure you have its Maximum Likelihood estimates for the mean (`m_ML_X1D`) and covariance (`C_ML_X1D`), as computed in Task 4.
2.  Plot the histogram of `X1D`. Use `plt.hist(X1D.ravel(), bins=50, density=True, ...)`. The `density=True` parameter is crucial as it normalizes the histogram, allowing for a direct comparison with the PDF curve.
3.  Generate a set of points for plotting the PDF curve. This can be done by creating a densely spaced array using `XPlot = numpy.linspace(-8, 12, 1000)` to cover the range of `X1D` values.
4.  Calculate the log-PDF values for these `XPlot` points. Use `log_pdf_values = logpdf_GAU_ND(vrow(XPlot), m_ML_X1D, C_ML_X1D)`. Remember that `vrow(XPlot)` is necessary to reshape the 1D array `XPlot` into the `(1, N)` row vector format expected by `logpdf_GAU_ND`.
5.  Convert the calculated log-PDF values back into actual PDF values by applying the exponential function: `pdf_values = numpy.exp(log_pdf_values)`.
6.  Overlay the PDF curve onto the histogram plot. This is done using `plt.plot(XPlot.ravel(), pdf_values, ...)`.
7.  Enhance the plot with appropriate labels for the x and y axes, a clear title, a legend to distinguish the histogram from the PDF, and a grid for better readability. Optionally, call `plt.show()` to display the plot interactively.

#### Solution Code (`gau_density.py` - within `if __name__ == '__main__':`)

```python
import matplotlib.pyplot as plt
import numpy

# Assuming X1D, m_ML_X1D, C_ML_X1D are available from previous tasks
# Assuming vrow and logpdf_GAU_ND are defined

plt.figure(figsize=(8, 5)) # Create a new figure with a specified size
# 1. Plot normalized histogram of the data
plt.hist(X1D.ravel(), bins=50, density=True, label='Data Histogram (X1D)', alpha=0.7)

# 2. Generate points for plotting the PDF curve
XPlot = numpy.linspace(-8, 12, 1000) # Range from -8 to 12 with 1000 points

# 3. Calculate log-PDF values for these plot points using ML estimates of X1D
# vrow(XPlot) reshapes the 1D linspace array into (1, 1000) as required
log_pdf_values = logpdf_GAU_ND(vrow(XPlot), m_ML_X1D, C_ML_X1D)

# 4. Convert log-PDF values to actual PDF values by exponentiation
pdf_values = numpy.exp(log_pdf_values)

# 5. Plot the PDF curve
plt.plot(XPlot.ravel(), pdf_values, color='red', linewidth=2, label='ML Gaussian PDF Fit')

# Add labels, title, and legend for clarity
plt.xlabel("Feature Value")
plt.ylabel("Density")
plt.legend()
plt.title("Histogram of X1D vs. ML Gaussian Fit")
plt.grid(True, linestyle='--', alpha=0.6) # Add a grid for better readability
# plt.show() # Uncomment this line to display the plot interactively
```

### Expected Output

The execution of this task will produce a plot. This plot will display `X1D`'s normalized histogram (typically shown as blue bars), overlaid by its ML-estimated 1D Gaussian PDF curve (represented by a red line). The red curve should visibly align closely with the overall shape of the histogram, indicating a good fit of the Gaussian model to the empirical data distribution.

---

## Task 6: Project Task - Univariate Gaussian Modeling

### Task Objective

This project task involves applying **univariate Gaussian modeling** to the provided **fingerprint spoofing detection dataset**. This dataset is located at `Project/trainData.txt` and contains 6 features and 2 distinct classes. For each class and for each of the six features, you will perform the following steps:

*   Compute the **Maximum Likelihood (ML) estimates** for the mean (μ) and variance (σ²) of a 1D Gaussian distribution.
*   Plot the **normalized histogram** specifically for that feature's data belonging to the current class.
*   Overlay the **Probability Density Function (PDF)** of the estimated 1D Gaussian onto its corresponding histogram.
*   **Visually analyze the goodness of fit** between the estimated Gaussian PDF and the empirical data distribution.

### Step-by-Step Reasoning

To accomplish this project task, follow these nested iterative steps:

1.  **Load Project Data:** Begin by reading the `Project/trainData.txt` file. Load its contents into a data matrix `D_proj` (which should be `6 x N`) and a labels array `L_proj` (which should be `N,`). Adapt an existing `load` function if necessary to handle the file format.
2.  **Separate Data by Class:** Divide the loaded data into class-specific subsets. Create `D0_proj` for Class 0 and `D1_proj` for Class 1 using boolean indexing (e.g., `D_proj[:, L_proj == 0]`).
3.  **Iterate Through Classes and Features:** Set up nested loops. The outer loop should iterate through `cls_idx` (representing classes 0 and 1). The inner loop should iterate through `feature_idx` (ranging from 0 to 5, covering all six features).
4.  **Extract Feature Data:** Within the innermost loop, extract the 1D data for the current feature and class: `feature_data = class_data[feature_idx, :]`. Ensure `feature_data` remains a 1D array.
5.  **Compute 1D ML Estimates:** Calculate the ML estimates for the 1D Gaussian parameters: `mu_ml_1d = feature_data.mean()` and `var_ml_1d = feature_data.var()`. Crucially, reshape these scalar values into `1 x 1` NumPy arrays (e.g., `numpy.array([[value]])`) to match the expected input shape for `logpdf_GAU_ND`.
6.  **Plot Histogram:** Create a new figure (`plt.figure()`) for each plot. Plot the normalized histogram of the `feature_data` using `plt.hist(feature_data.ravel(), bins=..., density=True, ...)`. The `density=True` argument is essential for overlaying the PDF.
7.  **Plot Estimated 1D Gaussian PDF:**
    *   Generate a range of points (`XPlot`) using `numpy.linspace` that spans the observed range of `feature_data` values.
    *   Calculate the log-PDF values for these `XPlot` points using the `logpdf_GAU_ND` function, passing `vrow(XPlot)` and the `1x1` ML estimates (`m_for_pdf`, `C_for_pdf`).
    *   Convert the log-PDF values back to actual PDF values by exponentiating them: `pdf_values = numpy.exp(log_pdf_values)`.
    *   Overlay this PDF curve onto the histogram using `plt.plot(XPlot.ravel(), pdf_values, ...)`.
8.  **Analyze and Display:** Add appropriate labels (x-axis, y-axis), a descriptive title, and a legend to each plot for clarity. Visually assess the goodness of fit, looking for characteristics such as multi-modality, skewness, or any significant deviations from a Gaussian shape. You may choose to display plots interactively (`plt.show()`) or save them to files and then close the figures.

#### Solution Code

*(The solution code for this project task is not provided as part of this guide; it is expected to be implemented by the student based on the instructions.)*

### Expected Output

Upon successful completion and execution of this task, you should generate a total of 12 plots (6 features × 2 classes). Each plot will present a normalized histogram of the specific feature data for a given class, with its corresponding ML-estimated 1D Gaussian PDF overlaid. A visual inspection of these plots will be critical to understand how well the Gaussian curve approximates the empirical data distribution for each feature and class. This analysis will help identify features that are well-modeled by a Gaussian versus those that might require more complex statistical models (e.g., due to observed multi-modality, skewness, or other non-Gaussian characteristics).