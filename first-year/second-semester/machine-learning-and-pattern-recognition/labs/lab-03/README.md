# Dimensionality Reduction - Laboratory 3 Guide

## Part 1: Principal Component Analysis (PCA)

**Principal Component Analysis (PCA)** is a widely used dimensionality reduction technique. Its core purpose is to reduce the number of features in a dataset while retaining the maximum possible variance. This is achieved by projecting the data onto a new, lower-dimensional subspace. This subspace is defined by **principal components**, which are essentially the eigenvectors of the data's covariance matrix corresponding to its largest eigenvalues. These components, therefore, represent the directions of maximum variance within the data.

### Task 1.1: Utility Functions for Vector Reshaping (`vcol`, `vrow`)

#### Task Objective

Implement two essential helper functions to precisely reshape 1-dimensional NumPy arrays:
1.  **`vcol(x)`:** Reshapes array `x` into a column vector.
2.  **`vrow(x)`:** Reshapes array `x` into a row vector.

#### Step-by-Step Reasoning

NumPy's matrix operations frequently necessitate explicit 2-dimensional shapes. For instance, column vectors require a `(N, 1)` shape, and row vectors require a `(1, N)` shape, unlike typical 1D arrays which have a `(N,)` shape. Using the `reshape` method with `x.size` correctly transforms the array into the desired 2D format.

#### Solution Code (`pca.py`)

```python
import numpy

def vcol(x):
    """ Reshapes a 1D array into a column vector (N, 1). """
    return x.reshape((x.size, 1))

def vrow(x):
    """ Reshapes a 1D array into a row vector (1, N). """
    return x.reshape((1, x.size))
```

#### Expected Output

```
vcol(a) result:
 [[1]
  [2]
  [3]]
vrow(a) result:
 [[1 2 3]]
```

---

### Task 1.2: Mean and Covariance Matrix Calculation (`compute_mu_C`)

#### Task Objective

Implement the `compute_mu_C(D)` function. This function should calculate and return both the **mean vector (`mu`)** and the **covariance matrix (`C`)** for a given dataset `D`, where `D` is structured with features as rows and samples as columns.

#### Step-by-Step Reasoning

1.  **Mean Vector (`mu`):** First, compute the mean for each feature (i.e., along each row) across all samples (columns) using `D.mean(axis=1)`. Subsequently, reshape this result into a column vector using the `vcol` helper function.
2.  **Center Data (`DC`):** Next, create a centered data matrix `DC` by subtracting the `mu` from `D` (`DC = D - mu`). NumPy's broadcasting capabilities efficiently handle this operation.
3.  **Covariance Matrix (`C`):** Finally, calculate the covariance matrix as `C = (DC @ DC.T) / float(D.shape[1])`. Here, `D.shape[1]` represents the total number of samples (`N`). The diagonal elements of `C` will represent the variances of individual features, while the off-diagonal elements will represent their covariances.

#### Solution Code (`pca.py`)

```python
import numpy
# Assuming vcol is defined as in Task 1.1

def compute_mu_C(D):
    """
    Computes the mean vector (column) and covariance matrix of the data D.
    Assumes D has features as rows, samples as columns (D x N).
    """
    mu = vcol(D.mean(1)) # Calculate mean along axis 1 (across samples) -> (D,) -> (D, 1)
    # Center the data: (D, N) - (D, 1) uses broadcasting
    DC = D - mu
    # Compute covariance matrix: (D, N) @ (N, D) -> (D, D)
    C = (DC @ DC.T) / float(D.shape[1])
    return mu, C
```

#### Expected Output

```
Mean:
[[5.84333333]
 [3.05733333]
 [3.758     ]
 [1.19933333]]

Covariance matrix:
[[ 0.68112222 -0.04215111  1.26582     0.51282889]
 [-0.04215111  0.18871289 -0.32745867 -0.12082844]
 [ 1.26582    -0.32745867  3.09550267  1.286972  ]
 [ 0.51282889 -0.12082844  1.286972    0.57713289]]
```

---

### Task 1.3: PCA Projection Matrix Computation (`compute_pca`)

#### Task Objective

Implement `compute_pca(D, m)` to return the **PCA projection matrix `P`**. The columns of `P` should consist of the `m` principal components (which are the eigenvectors of the covariance matrix `C`) that correspond to the `m` largest eigenvalues.

#### Step-by-Step Reasoning

1.  **Covariance Matrix:** First, obtain the covariance matrix `C` for the dataset `D` by calling `compute_mu_C(D)`.
2.  **Singular Value Decomposition (SVD):** Utilize `numpy.linalg.svd(C)`. This function returns `U` (a matrix where columns are eigenvectors), `s` (a 1D array of singular values, which are the square roots of the eigenvalues, sorted in *descending* order), and `Vh` (the conjugate transpose of `V`). The descending order of `s` simplifies the selection of top components.
3.  **Select Components:** Construct the projection matrix `P` by extracting the first `m` columns from `U` using `P = U[:, 0:m]`. These columns correspond to the principal components with the largest variances.

#### Solution Code (`pca.py`)

```python
import numpy
import scipy.linalg # Although svd is in numpy.linalg, good practice to import for broader use

# Assuming compute_mu_C is defined as in Task 1.2

def compute_pca(D, m):
    """
    Computes the PCA projection matrix P for data D, reducing to m dimensions.
    Assumes D has features as rows, samples as columns (D x N).
    m: Number of principal components to keep.
    """
    mu, C = compute_mu_C(D) # Get covariance matrix C
    # Compute SVD: U contains eigenvectors (columns), s contains eigenvalues (sorted descending)
    U, s, Vh = numpy.linalg.svd(C)
    # Select the first m columns of U as the projection matrix P
    P = U[:, 0:m]
    return P
```

#### Expected Output

```
# Example P for m=4 (all principal components)
[[ 0.36138659 -0.65658877  0.58202985 -0.3205007 ]
 [-0.08452251 -0.73016143 -0.59791083  0.30138927]
 [ 0.85667061  0.17337266 -0.07629665 -0.47983995]
 [ 0.3582892   0.07548102  0.54583143  0.75112108]]
```

---

### Task 1.4: Applying PCA Projection (`apply_pca`)

#### Task Objective

Implement `apply_pca(P, D)`. This function should project the original data `D` onto the lower-dimensional subspace defined by the PCA projection matrix `P`, ultimately returning the **projected data matrix `DP`**.

#### Step-by-Step Reasoning

The projected data `DP` is directly obtained through matrix multiplication: `DP = P.T @ D`.
*   `P` has a shape of `(D_features, m)`, with its columns representing the principal components.
*   `P.T`, therefore, has a shape of `(m, D_features)`, serving as the transformation matrix.
*   `D` is the original data matrix, with a shape of `(D_features, N_samples)`.
*   The resulting `DP` will have a shape of `(m, N_samples)`, representing the data after dimensionality reduction.

#### Solution Code (`pca.py`)

```python
import numpy

def apply_pca(P, D):
    """
    Projects the data D onto the subspace defined by projection matrix P.
    P: PCA projection matrix (D x m), columns are principal components.
    D: Data matrix (D x N), features as rows, samples as columns.
    Returns: Projected data DP (m x N).
    """
    # Matrix multiplication: (m, D) @ (D, N) -> (m, N)
    DP = P.T @ D
    return DP
```

#### Expected Output

This function returns the transformed data matrix; there is no direct print output associated with its execution. For instance, `DP` for the Iris dataset with `m=2` would result in a matrix of shape `(2, 150)`.

---

## Part 2: Linear Discriminant Analysis (LDA)

**Linear Discriminant Analysis (LDA)** is a distinct dimensionality reduction technique specifically designed for **classification problems**. In contrast to PCA, which aims to maximize total data variance, LDA's objective is to maximize **separability between classes**. It achieves this by identifying projections that maximize the ratio of **between-class scatter** (which quantifies the separation of class means) to **within-class scatter** (which measures the spread or compactness of data points within each class).

### Task 2.1: Computing Between-Class and Within-Class Scatter Matrices (`compute_Sb_Sw`)

#### Task Objective

Implement `compute_Sb_Sw(D, L)` to calculate and return both the **Between-Class scatter matrix (`SB`)** and the **Within-Class scatter matrix (`SW`)**.

#### Step-by-Step Reasoning

1.  **Global Mean:** First, calculate the global mean vector `muGlobal` for the entire dataset `D` using `muGlobal = vcol(D.mean(1))`.
2.  **Initialization:** Initialize both `Sb` and `Sw` as zero matrices. Their shape should be `(D_features, D_features)`.
3.  **Per-Class Loop:** Iterate through each unique class `i` present in the labels array `L`:
    *   **Class Data Selection:** Select all samples belonging to the current class `i` into `DCls = D[:, L == i]`.
    *   **Class Sample Count:** Determine the number of samples in this class: `nc = DCls.shape[1]`.
    *   **Class Mean:** Compute the mean vector `mu` for the current class: `mu = vcol(DCls.mean(1))`.
    *   **Accumulate `SB`:** Update `Sb` by adding `nc * (mu - muGlobal) @ (mu - muGlobal).T`. This term represents the contribution of the current class's mean deviation from the global mean.
    *   **Accumulate `SW`:** Update `Sw` by adding `(DCls - mu) @ (DCls - mu).T`. This term captures the variance of samples around their class mean.
4.  **Normalization:** After the loop completes, divide the accumulated `Sb` and `Sw` by the total number of samples `N = D.shape[1]` to normalize them.

#### Solution Code (`lda.py`)

```python
import numpy
# Assuming vcol is defined

def compute_Sb_Sw(D, L):
    """
    Computes Between-Class (SB) and Within-Class (SW) scatter matrices.
    D: Data matrix (D x N), features as rows, samples as columns.
    L: Labels array (N,).
    """
    D_features = D.shape[0] # Number of features

    # Initialize SB and SW as zero matrices of correct shape
    Sb = numpy.zeros((D_features, D_features))
    Sw = numpy.zeros((D_features, D_features))

    muGlobal = vcol(D.mean(1)) # Global mean (D, 1)

    for i in numpy.unique(L): # Iterate through unique class labels (e.g., 0, 1, 2)
        # Select samples belonging to class i
        DCls = D[:, L == i]
        # Compute class mean
        mu = vcol(DCls.mean(1))
        # Number of samples in class i
        nc = DCls.shape[1]

        # Accumulate SB: nc * (class_mean - global_mean) * (class_mean - global_mean).T
        Sb += nc * (mu - muGlobal) @ (mu - muGlobal).T

        # Accumulate SW: Sum of (sample - class_mean) * (sample - class_mean).T for class i
        Sw += (DCls - mu) @ (DCls - mu).T

    # Normalize by total number of samples N
    N = D.shape[1]
    Sb = Sb / N
    Sw = Sw / N
    return Sb, Sw
```

#### Expected Output

```
Expected SB:
[[ 0.42141422 -0.13301778  1.101656    0.47519556]
 [-0.13301778  0.07563289 -0.38159733 -0.15288444]
 [ 1.101656   -0.38159733  2.91401867  1.24516   ]
 [ 0.47519556 -0.15288444  1.24516     0.53608889]]

Expected SW:
[[ 0.259708    0.09086667  0.164164    0.03763333]
 [ 0.09086667  0.11308     0.05413867  0.032056  ]
 [ 0.164164    0.05413867  0.181484    0.041812  ]
 [ 0.03763333  0.032056    0.041812    0.041044  ]]
```

---

### Task 2.2: LDA Projection Matrix via Generalized Eigenvalue Problem (`compute_lda_geig`)

#### Task Objective

Implement `compute_lda_geig(D, L, m)` to compute the **LDA projection matrix `W`** by solving the generalized eigenvalue problem.

#### Step-by-Step Reasoning

1.  **Scatter Matrices:** Begin by obtaining the `Sb` and `Sw` matrices using the `compute_Sb_Sw(D, L)` function.
2.  **Solve Generalized Eigenvalue Problem:** Utilize `scipy.linalg.eigh(Sb, Sw)`. This function is designed to solve problems of the form `Sb @ x = lambda * Sw @ x`. It returns `s` (the eigenvalues, which are sorted in *ascending* order by default) and `U` (a matrix where columns are the corresponding eigenvectors).
3.  **Select Components:** To select the most discriminative directions, reverse the order of columns in `U` (`U[:, ::-1]`) to sort the eigenvectors by their eigenvalues in *descending* order (largest first). Then, select the first `m` columns from this sorted `U` as the LDA projection matrix `W`. These columns represent the principal discriminant directions.

#### Solution Code (`lda.py`)

```python
import numpy
import scipy.linalg # Required for generalized eigenvalue problem
# Assuming compute_Sb_Sw is defined

def compute_lda_geig(D, L, m):
    """
    Computes the LDA projection matrix W using generalized eigenvalue decomposition.
    D: Data matrix (D x N).
    L: Labels array (N,).
    m: Number of dimensions to reduce to (m <= D, m < number of classes).
    """
    Sb, Sw = compute_Sb_Sw(D, L)

    # Solve the generalized eigenvalue problem: SB @ W = SW @ W @ lambda
    # scipy.linalg.eigh(A, B) solves A @ x = lambda * B @ x
    # It returns eigenvalues 's' sorted ascendingly, and eigenvectors 'U' as columns
    s, U = scipy.linalg.eigh(Sb, Sw)

    # Sort eigenvectors by eigenvalue in descending order (largest first)
    # Select the first m eigenvectors (principal discriminant directions)
    W = U[:, ::-1][:, 0:m]
    return W
```

#### Expected Output

```
# Example W (or U from solution code) for m=2:
[[-0.18997679 -0.02214645]
 [-0.369685    0.88943343]
 [ 0.59328118 -0.20923783]
 [ 0.69310012  0.39883427]]
```

---

### Task 2.3: LDA Projection Matrix via Joint Diagonalization (`compute_lda_JointDiag`) - Optional

#### Task Objective

Implement `compute_lda_JointDiag(D, L, m)` to find the LDA projection matrix `W` by performing joint diagonalization of `SB` and `SW`.

#### Step-by-Step Reasoning (Joint Diagonalization Method)

1.  **Scatter Matrices:** Compute `Sb` and `Sw` using `compute_Sb_Sw`.
2.  **Whiten `SW`:** This step transforms `SW` into an identity matrix, simplifying the subsequent eigenvalue problem.
    *   Perform SVD on `Sw`: `U_sw, s_sw, _ = numpy.linalg.svd(Sw)`.
    *   Construct the whitening matrix `P = U_sw @ diag(1/sqrt(s_sw)) @ U_sw.T`.
3.  **Transform `SB`:** Transform `Sb` into the whitened space: `Sb2 = P @ Sb @ P.T`.
4.  **PCA on `Sb2`:** Now, the problem of finding directions that maximize between-class scatter while `SW` is identity simplifies to a standard PCA problem on `Sb2`.
    *   Perform SVD on `Sb2`: `U2, s2, _ = numpy.linalg.svd(Sb2)`.
    *   Select the top `m` eigenvectors of `Sb2` as `P2 = U2[:, 0:m]`.
5.  **Combine Transformations:** The final LDA projection matrix `W` is obtained by combining the whitening and subsequent PCA transformations: `W = P.T @ P2`.

#### Solution Code (`lda.py`)

```python
import numpy
# Assuming compute_Sb_Sw, vrow are defined

def compute_lda_JointDiag(D, L, m):
    """
    Computes the LDA projection matrix W using joint diagonalization.
    D: Data matrix (D x N).
    L: Labels array (N,).
    m: Number of dimensions to reduce to (m <= D, m < number of classes).
    """
    Sb, Sw = compute_Sb_Sw(D, L)

    # Whiten SW: P @ SW @ P.T = I
    # SVD of SW: U_sw contains eigenvectors (columns), s_sw contains eigenvalues
    U_sw, s_sw, _ = numpy.linalg.svd(Sw)
    # Construct whitening matrix P = U_sw @ diag(1.0/sqrt(s_sw)) @ U_sw.T
    P = numpy.dot(U_sw * vrow(1.0/(s_sw**0.5)), U_sw.T)

    # Transform Sb to the whitened space: Sb2 = P @ Sb @ P.T
    Sb2 = numpy.dot(P, numpy.dot(Sb, P.T))

    # Perform PCA on transformed Sb (Sb2)
    # SVD of Sb2: U2 contains eigenvectors, s2 contains eigenvalues
    U2, s2, _ = numpy.linalg.svd(Sb2)
    # Select the top m eigenvectors of Sb2 as P2
    P2 = U2[:, 0:m]

    # Combine transformations: W = P.T @ P2
    W = numpy.dot(P.T, P2)

    return W
```

#### Expected Output

This function returns the LDA projection matrix `W`. Importantly, `W` from `compute_lda_JointDiag` should span the same subspace as `W` derived from `compute_lda_geig`.

---

### Task 2.4: Applying LDA Projection (`apply_lda`)

#### Task Objective

Implement `apply_lda(U, D)`. This function should project the original data `D` onto the lower-dimensional subspace defined by the LDA projection matrix `U`, subsequently returning the **projected data matrix `DP`**.

#### Step-by-Step Reasoning

The projected data `DP` is obtained through matrix multiplication: `DP = U.T @ D`.
*   `U` has a shape of `(D_features, m)`, with its columns representing the discriminant directions.
*   `U.T`, therefore, has a shape of `(m, D_features)`, serving as the transformation matrix.
*   `D` is the original data matrix, with a shape of `(D_features, N_samples)`.
*   The resulting `DP` will have a shape of `(m, N_samples)`, representing the data after LDA transformation.

#### Solution Code (`lda.py`)

```python
import numpy

def apply_lda(U, D):
    """
    Projects the data D onto the subspace defined by LDA projection matrix U.
    U: LDA projection matrix (D x m), columns are discriminant directions.
    D: Data matrix (D x N), features as rows, samples as columns.
    Returns: Projected data DP (m x N).
    """
    # Matrix multiplication: (m, D) @ (D, N) -> (m, N)
    DP = U.T @ D
    return DP
```

#### Expected Output

This function returns the transformed data; there is no direct print output associated with its execution. For instance, `DP` for the Iris dataset with `m=2` would result in a matrix of shape `(2, 150)`. It is worth noting that LDA projections typically achieve more pronounced class separation compared to PCA.

---

## Part 3: PCA and LDA for Classification

This section applies PCA and LDA in a classification context. Specifically, we will classify **Iris Versicolor (class 1) from Iris Virginica (class 2)** using a straightforward pipeline that incorporates a train/validation data split.

### Task 3.1: Data Preparation and Splitting (`load_iris`, Filtering, `split_db_2to1`)

#### Task Objective

This task involves three key steps for data preparation:
1.  Load the complete Iris dataset.
2.  Filter the dataset to include only samples belonging to Versicolor (Class 1) and Virginica (Class 2).
3.  Implement the `split_db_2to1(D, L, seed=0)` function, which will randomly divide the filtered data into a 2/3 training set and a 1/3 validation set.

#### Step-by-Step Reasoning

1.  **Load Data:** Begin by loading the full Iris dataset, typically using `load_iris()` (often a wrapper around `sklearn.datasets.load_iris`).
2.  **Filter Data:** Create a boolean mask, `LIris != 0`, to identify and exclude samples of Iris Setosa (Class 0). Apply this mask to both `DIris` and `LIris` to obtain `D` (which will be `4x100`) and `L` (which will be `100,`), containing exclusively samples from classes 1 and 2.
3.  **Split Data (`split_db_2to1`):**
    *   **Calculate Training Size:** Determine the number of training samples as `nTrain = int(N * 2.0 / 3.0)`.
    *   **Reproducibility:** Set `numpy.random.seed(seed)` to ensure that the random split is reproducible.
    *   **Shuffle Indices:** Generate a shuffled array of indices `idx = numpy.random.permutation(N)`.
    *   **Slice Indices:** Divide `idx` into `idxTrain = idx[0:nTrain]` for the training set and `idxTest = idx[nTrain:]` for the validation set.
    *   **Create Subsets:** Use these index arrays to construct the training (`DTR`, `LTR`) and validation (`DVAL`, `LVAL`) data and label subsets from `D` and `L`.

#### Solution Code (`classify.py`)

```python
import numpy
import sklearn.datasets # Used in load_iris

# Assuming load_iris is available (often a simple wrapper around sklearn.datasets.load_iris)
def load_iris():
    """ Loads Iris dataset using scikit-learn and transposes to (features x samples). """
    iris = sklearn.datasets.load_iris()
    return iris['data'].T, iris['target']

def split_db_2to1(D, L, seed=0):
    """
    Splits dataset D and labels L into training (2/3) and validation (1/3) sets.
    D: Data matrix (Features x Samples).
    L: Labels array (Samples,).
    seed: Random seed for reproducibility.
    Returns: Tuple ((DTR, LTR), (DVAL, LVAL)).
    """
    nTotal = D.shape[1]
    nTrain = int(nTotal * 2.0 / 3.0)

    numpy.random.seed(seed) # Set seed for reproducible permutation
    idx = numpy.random.permutation(nTotal) # Get shuffled indices [0, ..., N-1]

    idxTrain = idx[0:nTrain] # First nTrain indices for training
    idxTest = idx[nTrain:]  # Remaining indices for validation

    DTR = D[:, idxTrain] # Select training columns from D
    DVAL = D[:, idxTest] # Select validation columns from D
    LTR = L[idxTrain]    # Select training labels from L
    LVAL = L[idxTest]    # Select validation labels from L

    return (DTR, LTR), (DVAL, LVAL)

# --- Usage Example (in if __name__ == '__main__') ---
# # Load the full Iris dataset
# DIris, LIris = load_iris()
#
# # Filter data for classes 1 and 2 (Versicolor and Virginica)
# D = DIris[:, LIris != 0] # Select columns where label is not 0
# L = LIris[LIris != 0]    # Select labels where label is not 0
#
# # Split the filtered data into training and validation sets
# (DTR, LTR), (DVAL, LVAL) = split_db_2to1(D, L, seed=0)
#
# print("DTR shape:", DTR.shape)
# print("LTR shape:", LTR.shape)
# print("DVAL shape:", DVAL.shape)
# print("LVAL shape:", LVAL.shape)
# print("Unique training labels:", numpy.unique(LTR))
# print("Unique validation labels:", numpy.unique(LVAL))
```

#### Expected Output

```
DTR shape: (4, 66)
LTR shape: (66,)
DVAL shape: (4, 34)
LVAL shape: (34,)
Unique training labels: [1 2]
Unique validation labels: [1 2]
```

---

### Task 3.2: LDA Classification (No PCA Pre-processing)

#### Task Objective

Implement a direct LDA-based classification pipeline as follows:
1.  Train a 1D LDA model using the training data (`DTR`, `LTR`).
2.  Project both the training (`DTR`) and validation (`DVAL`) datasets onto this derived LDA direction.
3.  Handle the LDA sign ambiguity to ensure that Class 2 (Virginica) is consistently placed on the "right" (i.e., corresponds to higher projected values).
4.  Determine the classification threshold based on the projected training data (`DTR_lda`).
5.  Classify the projected validation data (`DVAL_lda`) using this threshold.
6.  Compute and report the validation error rate.

#### Step-by-Step Reasoning

1.  **Train LDA:** Compute the 1-dimensional LDA projection matrix `ULDA` using `ULDA = lda.compute_lda_JointDiag(DTR, LTR, m=1)` (or `lda.compute_lda_geig`).
2.  **Project Data:** Apply `ULDA` to both the original training and validation datasets: `DTR_lda = lda.apply_lda(ULDA, DTR)` and `DVAL_lda = lda.apply_lda(ULDA, DVAL)`.
3.  **Handle Sign Ambiguity:** Calculate the mean of projected Versicolor samples (`mean_versicolor_tr`) and projected Virginica samples (`mean_virginica_tr`) from `DTR_lda`. If `mean_versicolor_tr` is greater than `mean_virginica_tr`, it implies the LDA direction is inverted. To correct this, flip the sign of `ULDA` (`ULDA = -ULDA`) and re-project `DTR_lda` (and `DVAL_lda` will be re-projected later with the corrected `ULDA`). Recalculating means after the flip is crucial for an accurate threshold.
4.  **Determine Threshold:** Set the classification threshold as the midpoint between the two class means: `threshold = (mean_versicolor_tr + mean_virginica_tr) / 2.0`.
5.  **Classify Validation Data:** For each sample `i` in `DVAL_lda`, if its projected value `DVAL_lda[0, i]` is greater than or equal to the `threshold`, predict it as Class 2; otherwise, predict it as Class 1.
6.  **Compute Error Rate:** Calculate the number of misclassifications by counting where `PVAL` does not equal `LVAL` (`(PVAL != LVAL).sum()`), then divide this count by the total number of validation samples (`LVAL.size`) and multiply by 100 for a percentage.

#### Solution Code (`classify.py` - First main block)

```python
import pca # For PCA functions
import lda # For LDA functions and helper functions like vcol, vrow
import numpy
import matplotlib.pyplot as plt # For potential plotting if needed

# Assuming load_iris and split_db_2to1 are defined as in Task 3.1
# and DTR, LTR, DVAL, LVAL are available after calling split_db_2to1

# --- Solution for LDA Classification (No PCA Pre-processing) ---

# 1. Train 1-dimensional LDA on training data
ULDA = lda.compute_lda_JointDiag(DTR, LTR, m=1) # Or lda.compute_lda_geig

# 2. Project training data onto the LDA direction
DTR_lda = lda.apply_lda(ULDA, DTR)

# 3. Handle LDA sign ambiguity: Ensure Virginica (class 2) is on the right (larger values)
mean_versicolor_tr = DTR_lda[0, LTR==1].mean() # Mean of projected Versicolor training samples
mean_virginica_tr = DTR_lda[0, LTR==2].mean() # Mean of projected Virginica training samples

if mean_versicolor_tr > mean_virginica_tr:
    print("Flipping LDA sign to orient Class 2 (Virginica) to the right.")
    ULDA = -ULDA # Flip the direction of the LDA vector
    # If ULDA was flipped, re-project training data for accurate threshold calculation
    DTR_lda = lda.apply_lda(ULDA, DTR)
    # Recalculate means after flip (essential for correct threshold)
    mean_versicolor_tr = DTR_lda[0, LTR==1].mean()
    mean_virginica_tr = DTR_lda[0, LTR==2].mean()

# 4. Project validation data using the FINAL (potentially flipped) ULDA vector
DVAL_lda  = lda.apply_lda(ULDA, DVAL)

# 5. Calculate threshold based on the means of projected TRAINING data
threshold = (mean_versicolor_tr + mean_virginica_tr) / 2.0

# 6. Predict labels for validation data
# Initialize an array for predictions with zeros, then fill based on threshold
PVAL = numpy.zeros(shape=LVAL.shape, dtype=numpy.int32)
# If a sample's projected value is >= threshold, predict it as Class 2
PVAL[DVAL_lda[0] >= threshold] = 2
# Otherwise (if < threshold), predict it as Class 1
PVAL[DVAL_lda[0] < threshold] = 1

# 7. Evaluate the classification results
print('\n--- LDA Only Classification Results ---')
print('True Validation Labels:  ', LVAL)
print('Predicted Validation Labels:', PVAL)

num_errors = (PVAL != LVAL).sum() # Count mismatches (errors)
error_rate = num_errors / float(LVAL.size) * 100 # Calculate percentage error

print('Number of errors:', num_errors, '(out of %d validation samples)' % (LVAL.size))
print('Error rate: %.1f%%' % error_rate)
```

#### Expected Output

```
--- LDA Only Classification Results ---
True Validation Labels:   [1 1 2 1 2 1 2 1 1 1 2 1 2 2 2 1 2 1 1 2 2 1 1 1 2 2 2 2 1 1 2 1 2 2]
Predicted Validation Labels: [1 1 2 1 2 1 2 1 1 1 2 1 2 2 2 1 2 1 1 2 2 1 1 1 2 2 1 2 1 1 2 1 2 1]
Number of errors: 2 (out of 34 validation samples)
Error rate: 5.9%
```

---

### Task 3.3: PCA Pre-processing + LDA Classification

#### Task Objective

Implement a two-stage classification pipeline as follows:
1.  **PCA Pre-processing:** Apply PCA (trained on `DTR`) to reduce the data dimensionality to a specified `m_pca`.
2.  **LDA Classification:** Perform LDA-based classification using the PCA-transformed data (trained and applied on this reduced-dimension data).
Evaluate the validation error rate for `m_pca` values of 4, 3, and 2.

#### Step-by-Step Reasoning

A loop will iterate through the specified `m_pca` values (`[4, 3, 2]`):
1.  **Train PCA:** For the current `m_pca`, compute the PCA projection matrix `UPCA` by training on the original training data: `UPCA = pca.compute_pca(DTR, m = m_pca)`.
2.  **Apply PCA:** Project both the original training and validation data onto the PCA subspace: `DTR_pca = pca.apply_pca(UPCA, DTR)` and `DVAL_pca = pca.apply_pca(UPCA, DVAL)`.
3.  **Train LDA (on PCA data):** Train a 1-dimensional LDA model using the PCA-transformed training data: `ULDA = lda.compute_lda_JointDiag(DTR_pca, LTR, m = 1)`.
4.  **Apply LDA (for orientation/threshold):** Project the PCA-transformed training data onto the LDA direction: `DTR_lda = lda.apply_lda(ULDA, DTR_pca)`.
5.  **Handle Sign Ambiguity:** Apply the same logic as in Task 3.2 to `DTR_lda` to ensure consistent orientation of classes. If `ULDA` is flipped, it implicitly affects the subsequent projection of `DVAL_pca`.
6.  **Determine Threshold:** Calculate the classification threshold as the midpoint of the means of the two classes within the *final projected training data* (`DTR_lda`).
7.  **Classify Validation Data:** Project `DVAL_pca` using the (potentially flipped) `ULDA` to get `DVAL_lda`, then classify samples based on the calculated `threshold`.
8.  **Compute Error Rate:** Calculate the percentage of misclassifications on the validation set for the current `m_pca` configuration.

#### Solution Code (`classify.py` - Second main block)

```python
import pca # For PCA functions
import lda # For LDA functions

# Assuming DTR, LTR, DVAL, LVAL are available from Task 3.1 split

# --- Solution with PCA Pre-processing + LDA Classification ---

# Iterate through different numbers of principal components (m_pca)
for m_pca in [4, 3, 2]: # Test PCA dimensions 4, 3, and 2
    print(f'\n--- PCA (m={m_pca}) + LDA Classification Results ---')

    # 1. Train PCA only on the original training data
    UPCA = pca.compute_pca(DTR, m = m_pca)

    # 2. Apply PCA to both original training and validation data
    DTR_pca = pca.apply_pca(UPCA, DTR)   # Training data transformed to m_pca dimensions
    DVAL_pca = pca.apply_pca(UPCA, DVAL) # Validation data transformed to m_pca dimensions

    # 3. Train 1-dimensional LDA only on the PCA-transformed training data
    ULDA = lda.compute_lda_JointDiag(DTR_pca, LTR, m = 1) # LDA always reduces to 1 dim here

    # 4. Apply LDA to the PCA-transformed training data (needed for orientation/threshold)
    DTR_lda = lda.apply_lda(ULDA, DTR_pca)

    # 5. Handle LDA sign ambiguity on the final projected training data
    mean_versicolor_tr = DTR_lda[0, LTR==1].mean()
    mean_virginica_tr = DTR_lda[0, LTR==2].mean()

    if mean_versicolor_tr > mean_virginica_tr:
        print(f"  Flipping LDA sign for PCA m={m_pca} to orient Class 2 to the right.")
        ULDA = -ULDA # Flip the direction of the LDA vector
        # If ULDA was flipped, re-project training data (essential for threshold)
        DTR_lda = lda.apply_lda(ULDA, DTR_pca)
        # Recalculate means after flip for threshold
        mean_versicolor_tr = DTR_lda[0, LTR==1].mean()
        mean_virginica_tr = DTR_lda[0, LTR==2].mean()

    # 6. Apply FINAL LDA to the PCA-transformed validation data
    DVAL_lda = lda.apply_lda(ULDA, DVAL_pca)

    # 7. Calculate threshold based on the means of the FINAL projected TRAINING data
    threshold = (mean_versicolor_tr + mean_virginica_tr) / 2.0

    # 8. Predict labels for validation data
    PVAL = numpy.zeros(shape=LVAL.shape, dtype=numpy.int32)
    PVAL[DVAL_lda[0] >= threshold] = 2 # Samples >= threshold are Class 2
    PVAL[DVAL_lda[0] < threshold] = 1  # Samples < threshold are Class 1

    # 9. Evaluate the classification results for this m_pca configuration
    print('True Validation Labels:  ', LVAL)
    print('Predicted Validation Labels:', PVAL)
    num_errors = (PVAL != LVAL).sum() # Count mismatches
    error_rate = num_errors / float(LVAL.size) * 100 # Percentage error

    print('Number of errors:', num_errors, '(out of %d validation samples)' % (LVAL.size))
    print('Error rate: %.1f%%' % error_rate)
```

#### Expected Output

```
--- PCA (m=4) + LDA Classification Results ---
True Validation Labels:   [...]
Predicted Validation Labels: [...]
Number of errors: 2 (out of 34 validation samples)
Error rate: 5.9%

--- PCA (m=3) + LDA Classification Results ---
True Validation Labels:   [...]
Predicted Validation Labels: [...]
Number of errors: 2 (out of 34 validation samples)
Error rate: 5.9%

--- PCA (m=2) + LDA Classification Results ---
True Validation Labels:   [...]
Predicted Validation Labels: [...]
Number of errors: 2 (out of 34 validation samples)
Error rate: 5.9%
```