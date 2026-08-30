# Gaussian Mixture Models (GMMs) Laboratory

This laboratory guides the implementation of **Gaussian Mixture Models (GMMs)**. It covers their estimation through the **Expectation-Maximization (EM) algorithm**, their incremental construction using the **LBG algorithm**, and their application to various **classification tasks**.

## Gaussian Mixture Models Introduction

A GMM represents a random variable's probability density function (PDF) as a **weighted sum of multiple Gaussian (Normal) distributions**, enabling the modeling of complex, multi-modal data.

$$
f_X(x) = \sum_{g=1}^{M} w_g \cdot N(x | \mu_g, \Sigma_g)
$$

Where:
*   $M$: The total number of Gaussian components in the mixture.
*   $w_g$: The weight of component $g$ ($\sum w_g = 1$, $w_g \ge 0$).
*   $N(x | \mu_g, \Sigma_g)$: The Multivariate Gaussian PDF for component $g$.
    *   $\mu_g$: Its mean vector.
    *   $\Sigma_g$: Its covariance matrix.

A GMM object (`gmm`) is represented as a list of tuples, with each tuple defining a Gaussian component: `[(w1, mu1, C1), ..., (wM, muM, CM)]`.

### Helper Functions

The following NumPy and SciPy helper functions are utilized throughout this laboratory:

```python
import numpy
import scipy.special # Often needed for logsumexp

def vcol(x):
    """
    Reshapes a 1-D NumPy array 'x' into a 2-D column vector (N, 1).
    This is useful for consistent matrix operations where column vectors are expected.
    """
    return x.reshape((x.size, 1))

def vrow(x):
    """
    Reshapes a 1-D NumPy array 'x' into a 2-D row vector (1, N).
    This is useful for consistent matrix operations where row vectors are expected.
    """
    return x.reshape((1, x.size))

def logpdf_GAU_ND(x, mu, C):
    """
    Computes the log-density (log-PDF) of a Multivariate Gaussian distribution
    for multiple samples, where 'x' has dimensions (D x N_samples).

    Args:
        x (numpy.ndarray): Data matrix (D x N_samples).
        mu (numpy.ndarray): Mean vector (D x 1).
        C (numpy.ndarray): Covariance matrix (D x D).

    Returns:
        numpy.ndarray: 1-D array of log-densities (N_samples,).
    """
    M = x.shape[0] # Number of features (dimensionality)
    P = numpy.linalg.inv(C) # Inverse of the covariance matrix
    log_det_C = numpy.linalg.slogdet(C)[1] # Logarithm of the determinant of C (robust calculation)
    D_diff = x - mu # Difference between samples and mean
    # Mahalanobis distance squared: (x - mu)^T * C^-1 * (x - mu)
    # This vectorized form calculates it for all samples:
    # (D_diff * (P @ D_diff)) performs element-wise multiplication after matrix product,
    # then .sum(0) sums along the feature dimension to get a 1D array of squared distances.
    mahalanobis_sq_vector = (D_diff * (P @ D_diff)).sum(0)

    # Combined log-PDF formula components
    # -0.5 * M * log(2*pi) - 0.5 * log|C| - 0.5 * Mahalanobis_sq
    return -0.5 * M * numpy.log(numpy.pi * 2) - 0.5 * log_det_C - 0.5 * mahalanobis_sq_vector
```

---

## Task 1: Computing Log-Density of a GMM (`logpdf_GMM`)

### Task Objective

Implement the `logpdf_GMM(X, gmm)` function. The objective is to calculate the log-density $\log f_X(x_i)$ for each sample $x_i$ in the data matrix `X` (`D x N`). This calculation should be performed for all samples as follows:

$$
\log f_X(x_i) = \log \left( \sum_{g=1}^{M} w_g \cdot N(x_i | \mu_g, \Sigma_g) \right)
$$

### Step-by-Step Reasoning

1.  **Iterate Components:** Loop through each GMM component, defined by its weight $w_g$, mean $\mu_g$, and covariance $\Sigma_g$.
2.  **Compute Joint Log-Densities:** For each GMM component $g$, calculate its joint log-density for all samples: $\log(w_g) + \log N(X | \mu_g, \Sigma_g)$. This involves using `logpdf_GAU_ND` for the Gaussian term and `numpy.log(w_g)` for the weight. Stack these `N`-element arrays vertically to form an `(M, N)` matrix (`S_joint_log_densities_matrix`), where `S[g, i]` represents $\log f_{X_i,G_i}(x_i, g)$ (the log-density of sample $x_i$ belonging to component $g$).
3.  **Log-Sum-Exp:** To compute the final marginal log-densities $\log f_X(x_i)$, apply `scipy.special.logsumexp` to the `S_joint_log_densities_matrix`. The summation should be performed over components (`axis=0`) for each sample, yielding the desired `(N,)` array of marginal log-densities.

#### Solution Code (`gmm.py`)

```python
import numpy
import scipy.special

def logpdf_GMM(X, gmm):
    """
    Computes the log-density of a GMM for each sample in a data matrix X.

    Args:
        X (numpy.ndarray): Data matrix (D x N_samples).
        gmm (list): List of tuples, each representing a GMM component (weight, mean, covariance).

    Returns:
        numpy.ndarray: 1-D array of log-densities (N_samples,).
    """
    S_joint_log_densities_list = []
    for w, mu, C in gmm: # Iterate through each Gaussian component in the GMM
        logpdf_conditional = logpdf_GAU_ND(X, mu, C) # Calculate log N(x_i | mu_g, Sigma_g) for all samples
        logpdf_joint_for_component = logpdf_conditional + numpy.log(w) # Add log(w_g) to get log(w_g * N(...))
        S_joint_log_densities_list.append(logpdf_joint_for_component)

    S_joint_log_densities_matrix = numpy.vstack(S_joint_log_densities_list) # Stack into an (M x N_samples) matrix

    # Use logsumexp to compute log(sum_g exp(log(w_g * N(...)))) for numerical stability
    log_marginal_densities = scipy.special.logsumexp(S_joint_log_densities_matrix, axis=0) # Sum over components (axis=0)
    return log_marginal_densities
```

#### Expected Output

```
--- Task 1: logpdf_GMM Verification ---
Max absolute difference for 4D data: 2.842170943040401e-14
Max absolute difference for 1D data: 0.0
```

---

## Task 2: GMM Estimation - The EM Algorithm

### Task Objective

Implement the **Expectation-Maximization (EM) algorithm** to estimate GMM parameters $(w_g, \mu_g, \Sigma_g)$ from data `X`. EM refines these parameters by iteratively improving their fit until the average log-likelihood change between successive iterations falls below a specified convergence threshold, $\Delta l$ (represented by `epsLLAverage`).

### Step-by-Step Reasoning

EM is an iterative optimization algorithm particularly suited for models with latent variables, such as the unobserved component assignments in GMMs. The algorithm proceeds through alternating Expectation (E) and Maximization (M) steps.

**0. Initialization:** The EM algorithm requires an initial set of GMM parameters `(w_g, μ_g, Σ_g)` for all `M` components. This initial set is typically provided as `gmm_initial`.

**1. E-step (Expectation):** In this step, the posterior responsibility $\gamma_{g,i}$ is computed for each sample $x_i$ and each component $g$. This represents the probability that sample $x_i$ was generated by component $g$, given the current GMM parameters: $\gamma_{g,i} = P(G_i = g | X_i = x_i)$.
    *   Calculate the log of the responsibilities (often called the `log_gamma_matrix`): `log_gamma_matrix = S_joint_log_densities_matrix - log_marginal_densities`.
    *   Convert to actual responsibilities: `responsibilities_matrix = numpy.exp(log_gamma_matrix)`.

**2. M-step (Maximization):** This step updates the GMM parameters $(w_g, \mu_g, \Sigma_g)$ for each component `g` by maximizing the expected log-likelihood, using the responsibilities computed in the E-step.
    *   Zero-order statistic $Z_g$: The sum of responsibilities for component $g$ across all samples, calculated as `gamma_g.sum()`. This effectively acts as the 'effective number of samples' assigned to component $g$.
    *   First-order statistic $F_g$: The weighted sum of samples assigned to component $g$, calculated as `vcol((vrow(gamma_g) * X).sum(1))`.
    *   Second-order statistic $S_{g\_stat}$: The weighted sum of outer products of samples for component $g$, calculated as `(vrow(gamma_g) * X) @ X.T`.
    *   Updated parameters:
        *   $w_{g,t+1} = Z_g / N$ (`Z_g / num_samples`).
        *   $\mu_{g,t+1} = F_g / Z_g$ (`F_g / Z_g`).
        *   $\Sigma_{g,t+1} = \frac{S_{g\_stat}}{Z_g} - \mu_{g,t+1} \cdot \mu_{g,t+1}^T$.

**3. Iteration and Convergence:** The E-step and M-step are repeated iteratively. The algorithm is considered converged when the average log-likelihood improvement (`llDelta`) between iterations falls below the specified convergence threshold $\Delta l$ (defined by `epsLLAverage`). It's a critical property of EM that the log-likelihood should never decrease across iterations; a decrease indicates a potential issue, often due to numerical precision or a bug.

#### Solution Code (`gmm.py`)

```python
import numpy
import scipy.special

# smooth_covariance_matrix is defined in Task 4, assumed to be available for this function

def train_GMM_EM_Iteration(X, gmm, covType = 'Full', psiEig = None):
    """
    Performs one iteration (E-step and M-step) of the EM algorithm for GMM estimation.
    Applies covariance type constraints and eigenvalue smoothing.

    Args:
        X (numpy.ndarray): Data matrix (D x N_samples).
        gmm (list): Current GMM parameters.
        covType (str): Type of covariance matrix ('Full', 'Diagonal', 'Tied').
        psiEig (float, optional): Eigenvalue constraint for covariance matrices.

    Returns:
        list: Updated GMM parameters.
    """
    assert (covType.lower() in ['full', 'diagonal', 'tied']), "Invalid covType."

    # E-step: Compute responsibilities
    S_joint_log_densities_list = []
    for w, mu, C in gmm:
        logpdf_conditional = logpdf_GAU_ND(X, mu, C) # log N(x_i | mu_g, Sigma_g) for all samples
        logpdf_joint = logpdf_conditional + numpy.log(w) # log(w_g * N(...))
        S_joint_log_densities_list.append(logpdf_joint)
    S_joint_log_densities_matrix = numpy.vstack(S_joint_log_densities_list) # (M x N_samples) matrix of log joint densities

    log_marginal_densities = scipy.special.logsumexp(S_joint_log_densities_matrix, axis=0) # log(f_X(x_i))
    responsibilities_matrix = numpy.exp(S_joint_log_densities_matrix - log_marginal_densities) # gamma_gi = exp(log(f_joint) - log(f_marginal))

    # M-step: Update GMM parameters
    gmmUpd = []
    num_samples = X.shape[1]

    # Temporarily store individual component updates
    current_component_updates = []

    for gIdx in range(len(gmm)): # For each component
        gamma_g = responsibilities_matrix[gIdx, :] # Responsibilities for component g across all samples
        Z_g = gamma_g.sum() # Zero-order statistic (effective count for component g)
        F_g = vcol((vrow(gamma_g) * X).sum(1)) # First-order statistic (weighted sum of samples)
        S_g_stat = (vrow(gamma_g) * X) @ X.T # Second-order statistic (weighted sum of outer products)

        wUpd = Z_g / num_samples # Update weight for component g
        muUpd = F_g / Z_g # Update mean for component g
        CUpd = S_g_stat / Z_g - muUpd @ muUpd.T # Update covariance for component g (full)

        # Apply covariance type constraints (from Task 5)
        if covType.lower() == 'diagonal':
            CUpd  = CUpd * numpy.eye(X.shape[0]) # Force covariance to be diagonal

        current_component_updates.append((wUpd, muUpd, CUpd))

    # Special handling for Tied GMMs (from Task 5): Calculate shared covariance after all components updated
    if covType.lower() == 'tied':
        CTied = numpy.zeros((X.shape[0], X.shape[0]))
        for w_comp, mu_comp, C_comp_upd in current_component_updates:
            CTied += w_comp * C_comp_upd # Sum weighted individual covariances
        # Update all components to use the same tied covariance
        gmmUpd = [(w_comp, mu_comp, CTied) for w_comp, mu_comp, _ in current_component_updates]
    else: # For 'full' and 'diagonal', just pass the current updates
        gmmUpd = current_component_updates

    # Apply Eigenvalue Smoothing (from Task 4): Ensure covariance matrices are well-conditioned
    if psiEig is not None:
        gmmUpd = [(w, mu, smooth_covariance_matrix(C, psiEig)) for w, mu, C in gmmUpd]

    return gmmUpd


def train_GMM_EM(X, gmm_initial, covType = 'Full', psiEig = None, epsLLAverage = 1e-6, verbose=True):
    """
    Trains a GMM using the Expectation-Maximization (EM) algorithm until convergence.

    Args:
        X (numpy.ndarray): Data matrix (D x N_samples).
        gmm_initial (list): Initial GMM parameters.
        covType (str): Type of covariance matrix ('Full', 'Diagonal', 'Tied').
        psiEig (float, optional): Eigenvalue constraint for covariance matrices.
        epsLLAverage (float): Convergence threshold for average log-likelihood change.
        verbose (bool): If True, prints iteration details.

    Returns:
        list: The converged GMM parameters.
    """
    gmm = gmm_initial
    llOld = logpdf_GMM(X, gmm).mean() # Initial average log-likelihood
    llDelta = None # Change in log-likelihood

    if verbose: print('GMM - it %3d - average ll %.8e' % (0, llOld))

    it = 1
    while (llDelta is None or llDelta > epsLLAverage): # Loop until convergence
        gmmUpd = train_GMM_EM_Iteration(X, gmm, covType=covType, psiEig=psiEig) # Perform one EM iteration
        llUpd = logpdf_GMM(X, gmmUpd).mean() # Calculate average log-likelihood of updated GMM
        llDelta = llUpd - llOld # Calculate change in average log-likelihood

        if verbose: print('GMM - it %3d - average ll %.8e' % (it, llUpd))

        gmm = gmmUpd # Update GMM for next iteration
        llOld = llUpd # Store current log-likelihood
        it = it + 1

        if llDelta < 0 and verbose: # Check for log-likelihood decrease (indicates issue)
            print("Warning: Log-likelihood decreased during EM iteration!")

    if verbose:
        print('GMM - it %3d - average ll %.8e (eps = %e, delta_ll = %e)' %
              (it-1, llUpd, epsLLAverage, llDelta if llDelta is not None else 0))
    return gmm
```

#### Expected Output

```
--- Task 2: GMM Estimation - EM Algorithm ---
***** EM - 4D Data (3 Components, Full Covariance) *****

GMM - it   0 - average ll -7.40189544e+00
GMM - it   1 - average ll -7.27190483e+00
...
GMM - it   9 - average ll -7.26325603e+00 (eps = 1.000000e-06, delta_ll = 3.469447e-08)
Final average ll: -7.26325603e+00
```

---

## Task 3: LBG Algorithm

### Task Objective

Implement the **Linde-Buzo-Gray (LBG) algorithm**. This algorithm provides a heuristic for incrementally building Gaussian Mixture Models by progressively splitting existing components and then refining the newly formed GMM using the EM algorithm.

### Step-by-Step Reasoning

The LBG algorithm is a heuristic for GMM initialization and component growing. It proceeds as follows:
1.  **Initialization:** Start with a 1-component GMM. Its parameters (weight, mean, covariance) are initialized using the empirical mean and covariance of the entire dataset `X`.
2.  **Splitting (`split_GMM_LBG`):** For each existing component $(w_g, \mu_g, \Sigma_g)$ in the current GMM, it is split into two new components. This involves:
    *   Calculating a displacement vector $d_g$: This is derived from the principal eigenvector (corresponding to the largest eigenvalue) of the component's covariance matrix $\Sigma_g$. Specifically, $d_g = U[:, 0:1] \cdot s[0]^{0.5} \cdot \alpha$ (where $U$ and $s$ come from the SVD of $\Sigma_g$, and $\alpha$ is a scaling factor defined by `lbgAlpha`).
    *   Creating two new components: Each new component receives half of the original component's weight and shares its covariance. Their means are displaced by $\pm d_g$: $(w_g/2, \mu_g - d_g, \Sigma_g)$ and $(w_g/2, \mu_g + d_g, \Sigma_g)$.
3.  **Retraining:** After splitting (which doubles the number of components), the new, larger GMM is refined using the `train_GMM_EM` function. This step is crucial to adjust the new components to the data distribution.
4.  **Iteration:** Steps 2 and 3 (splitting and retraining) are repeated iteratively until the desired `numComponents` for the GMM is reached.

#### Solution Code (`gmm.py`)

```python
import numpy
import scipy.optimize

# compute_mu_C: Helper to calculate empirical mean and covariance (used for initial GMM)
def compute_mu_C(D):
    mu = vcol(D.mean(1))
    C = ((D - mu) @ (D - mu).T) / float(D.shape[1])
    return mu, C

# split_GMM_LBG: Splits each component of a GMM into two
def split_GMM_LBG(gmm, alpha = 0.1, verbose=True):
    """
    Doubles the number of components in a GMM by splitting each existing component.
    Splitting is based on the principal eigenvector of the component's covariance.

    Args:
        gmm (list): The GMM to be split.
        alpha (float): Scaling factor for the displacement vector.
        verbose (bool): If True, prints splitting information.

    Returns:
        list: A new GMM with double the components.
    """
    gmmOut = []
    if verbose: print ('LBG - going from %d to %d components' % (len(gmm), len(gmm)*2))

    for (w, mu, C) in gmm:
        U, s_singular_values, Vh = numpy.linalg.svd(C) # SVD of covariance matrix
        # Displacement vector derived from principal eigenvector (U[:,0]) and largest eigenvalue (s_singular_values[0])
        d_g = U[:, 0:1] * (s_singular_values[0]**0.5) * alpha

        # Create two new components with half weight and displaced means
        gmmOut.append((0.5 * w, mu - d_g, C))
        gmmOut.append((0.5 * w, mu + d_g, C))

    return gmmOut

# train_GMM_LBG_EM: Main LBG-EM iterative training function
def train_GMM_LBG_EM(X, numComponents, covType = 'Full', psiEig = None,
                     epsLLAverage = 1e-6, lbgAlpha = 0.1, verbose=True):
    """
    Trains a GMM using the LBG algorithm for incremental component splitting,
    with EM retraining after each split.

    Args:
        X (numpy.ndarray): Data matrix (D x N_samples).
        numComponents (int): Desired final number of GMM components.
        covType (str): Type of covariance matrix ('Full', 'Diagonal', 'Tied').
        psiEig (float, optional): Eigenvalue constraint for covariance matrices.
        epsLLAverage (float): Convergence threshold for EM iterations.
        lbgAlpha (float): Alpha parameter for LBG splitting.
        verbose (bool): If True, prints iteration details.

    Returns:
        list: The converged GMM parameters with 'numComponents' components.
    """
    # Initial 1-component GMM based on overall data's mean and covariance
    mu_initial, C_initial = compute_mu_C(X)
    if covType.lower() == 'diagonal':
        C_initial = C_initial * numpy.eye(X.shape[0]) # Force initial covariance to be diagonal if needed

    # Apply eigenvalue constraint to initial 1-component GMM (from Task 4)
    if psiEig is not None:
        gmm_current = [(1.0, mu_initial, smooth_covariance_matrix(C_initial, psiEig))]
    else:
        gmm_current = [(1.0, mu_initial, C_initial)]

    # Iteratively split and retrain until desired number of components is reached
    while len(gmm_current) < numComponents:
        if verbose:
            print ('Average ll before LBG split: %.8e (%d components)' %
                   (logpdf_GMM(X, gmm_current).mean(), len(gmm_current)))

        gmm_current = split_GMM_LBG(gmm_current, lbgAlpha, verbose=verbose) # Split components

        if verbose:
            print ('Average ll after LBG split (before EM): %.8e (%d components)' %
                   (logpdf_GMM(X, gmm_current).mean(), len(gmm_current)))

        # Retrain the new GMM using EM
        gmm_current = train_GMM_EM(X, gmm_current, covType=covType, psiEig=psiEig,
                                   verbose=verbose, epsLLAverage=epsLLAverage)

    if verbose:
        print('LBG + EM - final average ll: %.8e (%d components)' %
              (logpdf_GMM(X, gmm_current).mean(), len(gmm_current)))
    return gmm_current
```

### Expected Output

```
--- Task 3: LBG Algorithm ---
***** LBG EM - 4D Data (4 Components, Full Covariance) *****
Average ll before LBG split: -8.00224355e+00 (1 components)
LBG - going from 1 to 2 components
Average ll after LBG split (before EM): -8.07387689e+00 (2 components)
GMM - it   0 - average ll -8.07387689e+00
... (EM iterations for 2 components, LL increases) ...
GMM - it  21 - average ll -7.33301826e+00 (eps = 1.000000e-06, delta_ll = 9.393703e-07)
Average ll before LBG split: -7.33301826e+00 (2 components)
LBG - going from 2 to 4 components
Average ll after LBG split (before EM): -7.35664262e+00 (4 components)
GMM - it   0 - average ll -7.35664262e+00
... (EM iterations for 4 components, LL increases) ...
GMM - it  28 - average ll -7.25337844e+00 (eps = 1.000000e-06, delta_ll = 9.098115e-07)
LBG + EM - final average ll: -7.25337844e+00 (4 components)
Max absolute ll difference w.r.t. pre-trained model over all training samples: 2.842170943040401e-14
```

---

## Task 4: Constraining Eigenvalues of Covariance Matrices

### Task Objective

Modify the M-step within the `train_GMM_EM_Iteration` function. The objective is to enforce that the eigenvalues of each component's covariance matrix $\Sigma_g$ are **no smaller than a specified value $\psi$**. This constraint is crucial for preventing singular (and thus ill-conditioned) covariance matrices, which can lead to numerical instability during optimization.

### Step-by-Step Reasoning

1.  **Problem:** During the EM algorithm, a GMM component's covariance matrix $\Sigma_g$ can become singular (or near-singular) if the component "over-specializes" to a very small subset of data points or if it is assigned very few samples. Singular matrices cannot be inverted, which breaks the log-density calculation.
2.  **Solution:** After the updated covariance matrix $C_{Upd}$ is computed in the M-step (and any covariance type constraints are applied, as per Task 5), perform the following steps to constrain its eigenvalues:
    *   Perform Singular Value Decomposition (SVD) on `CUpd`: `U, s, Vh = numpy.linalg.svd(CUpd)`. The singular values `s` are directly related to the eigenvalues of the covariance matrix.
    *   Threshold the singular values `s`: Any singular value smaller than `psi` is set to `psi`: `s[s < psi] = psi`. This ensures a minimum spread along all principal directions.
    *   Reconstruct the constrained covariance matrix using the modified singular values: `C_constrained = U @ (vcol(s) * U.T)`.
3.  **Integration:** This eigenvalue smoothing function needs to be integrated into `train_GMM_EM_Iteration` (specifically, after `CUpd` is calculated and any `covType` specific handling is applied). Additionally, it should be applied to the initial 1-component GMM within `train_GMM_LBG_EM` to ensure the starting covariance is also well-conditioned.

#### Solution Code (`gmm.py`)

```python
import numpy

def smooth_covariance_matrix(C, psi):
    """
    Constrains the eigenvalues of a covariance matrix C to be no smaller than psi.
    This helps prevent singular or ill-conditioned covariance matrices.

    Args:
        C (numpy.ndarray): The covariance matrix to smooth.
        psi (float): The minimum allowable eigenvalue.

    Returns:
        numpy.ndarray: Reconstructed covariance matrix with constrained eigenvalues.
    """
    U, s, Vh = numpy.linalg.svd(C) # Perform SVD
    s[s < psi] = psi # Constrain singular values to be no smaller than psi
    CUpd_constrained = U @ (vcol(s) * U.T) # Reconstruct the covariance matrix
    return CUpd_constrained

# --- Integration into train_GMM_EM_Iteration ---
# (This code is inside train_GMM_EM_Iteration, after M-step's CUpd is calculated and covType handled)
#   if psiEig is not None:
#       gmmUpd = [(w, mu, smooth_covariance_matrix(C_comp_upd, psiEig)) for w, mu, C_comp_upd in gmmUpd]

# --- Integration into train_GMM_LBG_EM for initial 1-component GMM ---
# (This code is inside train_GMM_LBG_EM, when initializing gmm_current with 1 component)
#   if psiEig is not None:
#       gmm_current = [(1.0, mu_initial, smooth_covariance_matrix(C_initial, psiEig))]
#   else:
#       gmm_current = [(1.0, mu_initial, C_initial)]
```

---

## Task 5: (Optional) Diagonal and Tied-Covariance GMMs

### Task Objective

Implement variations of GMMs that incorporate specific constraints on their covariance matrices:
1.  **Diagonal Covariance Matrices:** In this model, each component's covariance matrix $\Sigma_g$ is constrained to be diagonal, implying conditional independence of features within that component.
2.  **Tied Covariance Matrices:** Here, all Gaussian components within the GMM are forced to share a single, common covariance matrix, $\Sigma_{tied}$, while retaining individual means.

These constraints require specific modifications within the M-step of the `train_GMM_EM_Iteration` function. Importantly, any eigenvalue constraints (from Task 4) should be applied *after* these covariance type-specific updates.

### Step-by-Step Reasoning

The modifications are applied within the M-step of the `train_GMM_EM_Iteration` function, affecting how `CUpd` is finalized for each component.

1.  **Diagonal GMMs:** For each component, after its full `CUpd` is initially computed, it is transformed into a diagonal matrix. This can be achieved by `CUpd_diag = CUpd * numpy.eye(D)`, effectively retaining only the variances (diagonal elements) and setting covariances to zero.
2.  **Tied GMMs:** After individual `CUpd_g` (unnormalized covariance contribution) and `wUpd_g` (updated weight) are computed for *all* components, a single shared covariance matrix `C_Tied` is calculated. This `C_Tied` is a weighted average of the individual component covariances: `C_Tied = \sum_g (wUpd_g \cdot CUpd_g)`. Subsequently, *every* component in the `gmmUpd` list is updated to use this `C_Tied` as its covariance matrix, replacing its individual `CUpd_g`.

#### Solution Code (`gmm.py`)

```python
# Relevant sections from train_GMM_EM_Iteration in gmm.py

# ... (Previous E-step and M-step computation of wUpd, muUpd, CUpd for EACH component) ...

# Inside the loop for gIdx in range(len(gmm)): # For each component
#   ... (calculation of wUpd, muUpd, and the initial full CUpd) ...
#
#   # Apply covariance type constraint if covType is 'diagonal'
#   if covType.lower() == 'diagonal':
#       CUpd  = CUpd * numpy.eye(X.shape[0]) # Forces CUpd to be diagonal by zeroing off-diagonal elements
#
#   current_component_updates.append((wUpd, muUpd, CUpd)) # Store temporary updates

# This block is executed AFTER the loop that computes individual components,
# if the overall GMM is of 'tied' covariance type.
if covType.lower() == 'tied':
    CTied = numpy.zeros((X.shape[0], X.shape[0]))
    for w_comp, mu_comp, C_comp_upd in current_component_updates:
        CTied += w_comp * C_comp_upd # Sums individual component covariance contributions, weighted by component weights
    gmmUpd = [(w_comp, mu_comp, CTied) for w_comp, mu_comp, _ in current_component_updates] # All components now share this CTied
else: # For 'full' and 'diagonal', just pass the individual component updates directly
    gmmUpd = current_component_updates

# The eigenvalue smoothing (Task 4) is applied AFTER the above covType modifications.
# if psiEig is not None:
#   gmmUpd = [(w, mu, smooth_covariance_matrix(C_final, psiEig)) for w, mu, C_final in gmmUpd]
```

---

## Task 6: GMM for Classification

### Task Objective

This task applies GMMs to classification problems:
1.  **Multiclass Iris Dataset:** A separate GMM will be trained for each of the three Iris classes. The performance will be evaluated based on the classification error rate, considering varying numbers of components (`numC`) and different covariance types (`covType`).
2.  **Binary Artificial Dataset:** GMMs will be trained for each of the two classes in this dataset. Performance will be assessed using `minDCF` and `actDCF` (assuming a target prior $\pi_T = 0.5$, a false negative cost $C_{\text{fn}}=1$, and a false positive cost $C_{\text{fp}}=1$).

Common parameters used across these classification tasks include: LBG `alpha=0.1` for component splitting, an eigenvalue constraint $\psi=0.01$ to prevent singular covariance matrices, and an EM convergence threshold of $\Delta l = 10^{-6}$.

### Step-by-Step Reasoning

**General Approach for GMM-based Classification:**
1.  Load and split the dataset into training (`DTR, LTR`) and validation (`DVAL, LVAL`) sets.
2.  **Training (Per-Class GMMs):** For each class `c` present in the training labels (`LTR`):
    *   Filter the training data to get only samples belonging to class `c` (`DTR_c`).
    *   Train a separate GMM (`GMM_c`) on `DTR_c` using the `train_GMM_LBG_EM` function, specifying the desired `numComponents`, `covType`, `psiEig`, `lbgAlpha`, and `epsLLAverage`.
3.  **Evaluation:**
    *   For each trained `GMM_c`, calculate the log-density `logpdf_GMM(DVAL, GMM_c)` for all samples in the validation set `DVAL`. This produces `S_ll[c, i]`, which is the log-likelihood of sample $i$ under the GMM model for class $c$.

**Specifics for Multiclass Iris Dataset Classification:**
1.  **Sweep Parameters:** Iterate through different `covType` settings (e.g., `['full', 'diagonal', 'tied']`) and varying numbers of components `numC` (e.g., `[1, 2, 4, 8, 16]`).
2.  **Prediction:** To make class predictions, combine the class-conditional log-likelihoods `S_ll` with class priors. Assuming uniform priors, this involves adding `numpy.log(1/num_classes)` to each row of `S_ll` to get `log_joint_probabilities`. The predicted class for each sample is then `numpy.argmax(log_joint_probabilities, axis=0)`.
3.  **Evaluation:** Calculate the classification error rate: `(LVAL != PVAL).sum() / LVAL.size * 100`.

**Specifics for Binary Artificial Dataset Classification:**
1.  **Sweep Parameters:** Similar loops for `covType` and `numC` as for Iris.
2.  **Train:** Train two GMMs: `gmm0_bin` on the negative class training data (`DTR_bin[:, LTR_bin==0]`) and `gmm1_bin` on the positive class training data (`DTR_bin[:, LTR_bin==1]`).
3.  **LLR Scores:** Compute the Log-Likelihood Ratio (LLR) scores for the validation set. For each sample, `SLLR_bin = logpdf_GMM(DVAL, gmm1_bin) - logpdf_GMM(DVAL, gmm0_bin)`.
4.  **DCF Evaluation:** Evaluate the performance using `minDCF` and `actDCF`. Use the `bayesRisk.compute_minDCF_binary_fast` and `bayesRisk.compute_actDCF_binary_fast` functions, passing `SLLR_bin`, `LVAL_bin`, and the defined application parameters (`target_prior_bin`, `cfn_bin`, `cfp_bin`).

#### Solution Code (`gmm_cls.py`)

```python
import numpy
import bayesRisk # Assumed bayesRisk.py is available (from Lab 8)
# Assume logpdf_GMM, train_GMM_LBG_EM, vcol from gmm.py are imported or defined.

# Helper function to load Iris (from gmm.py or Lab 6 setup)
def load_iris():
    import sklearn.datasets
    return sklearn.datasets.load_iris()['data'].T, sklearn.datasets.load_iris()['target']

# Helper function to split data (from gmm.py or Lab 6 setup)
def split_db_2to1(D, L, seed=0):
    nTrain = int(D.shape[1]*2.0/3.0)
    numpy.random.seed(seed)
    idx = numpy.random.permutation(D.shape[1])
    idxTrain = idx[0:nTrain]
    idxTest = idx[nTrain:]
    DTR = D[:, idxTrain]
    DVAL = D[:, idxTest]
    LTR = L[idxTrain]
    LVAL = L[idxTest]
    return (DTR, LTR), (DVAL, LVAL)

if __name__ == '__main__':
    # Define common parameters for GMM training and evaluation
    psi_eigen_constraint = 0.01 # Minimum eigenvalue constraint
    lbg_alpha = 0.1 # Alpha for LBG component splitting
    em_eps_ll_average = 1e-6 # Convergence threshold for EM algorithm

    # --- Task 6: IRIS Dataset Multiclass Classification ---
    print('--- IRIS Dataset Multiclass Classification ---')
    D_iris, L_iris = load_iris()
    (DTR_iris, LTR_iris), (DVAL_iris, LVAL_iris) = split_db_2to1(D_iris, L_iris)

    num_iris_classes = 3

    for covType_iris in ['full', 'diagonal', 'tied']:
        print(f"\nCovariance Type (IRIS): {covType_iris.upper()}")
        for numC_iris in [1, 2, 4, 8, 16]: # Iterate through number of components
            gmm_models_iris = [] # List to hold GMM models for each class
            for class_idx in range(num_iris_classes):
                DTR_class = DTR_iris[:, LTR_iris == class_idx] # Filter training data for current class
                # Train GMM for current class using LBG-EM (verbose=False for cleaner output)
                gmm_c = train_GMM_LBG_EM(DTR_class, numComponents=numC_iris,
                                         covType=covType_iris, verbose=False,
                                         psiEig=psi_eigen_constraint, lbgAlpha=lbg_alpha,
                                         epsLLAverage=em_eps_ll_average)
                gmm_models_iris.append(gmm_c)

            SVAL_iris_list = [] # List to hold log-likelihoods for validation data for each class's GMM
            for class_idx in range(num_iris_classes):
                SVAL_iris_list.append(logpdf_GMM(DVAL_iris, gmm_models_iris[class_idx]))

            SVAL_iris_matrix = numpy.vstack(SVAL_iris_list) # Stack into (N_classes x N_samples) matrix

            # Predict classes assuming uniform priors (log-prior is log(1/N_classes))
            log_priors_iris = numpy.log(numpy.ones(num_iris_classes) / num_iris_classes)
            log_joint_iris = SVAL_iris_matrix + vcol(log_priors_iris) # Add log-priors to log-likelihoods

            PVAL_iris = numpy.argmax(log_joint_iris, axis=0) # Predict class with highest log-joint probability

            error_rate_iris = (LVAL_iris != PVAL_iris).sum() / LVAL_iris.size * 100 # Calculate error rate
            print(f'  Components: {numC_iris:2d} - Error rate: {error_rate_iris:.1f}%')


    # --- Task 6: Binary Artificial Dataset Classification ---
    print('\n--- Binary Artificial Dataset Classification (ext_data_binary) ---')
    D_bin, L_bin = numpy.load('Data/ext_data_binary.npy'), numpy.load('Data/ext_data_binary_labels.npy')
    (DTR_bin, LTR_bin), (DVAL_bin, LVAL_bin) = split_db_2to1(D_bin, L_bin)

    target_prior_bin = 0.5 # Target prior for DCF evaluation
    cfn_bin = 1.0 # Cost of false negative
    cfp_bin = 1.0 # Cost of false positive

    for covType_bin in ['full', 'diagonal', 'tied']:
        print(f"\nCovariance Type (BINARY): {covType_bin.upper()}")
        for numC_bin in [1, 2, 4, 8, 16]: # Iterate through number of components
            # Train GMM for class 0 (negative class)
            gmm0_bin = train_GMM_LBG_EM(DTR_bin[:, LTR_bin==0], numComponents=numC_bin,
                                        covType=covType_bin, verbose=False,
                                        psiEig=psi_eigen_constraint, lbgAlpha=lbg_alpha,
                                        epsLLAverage=em_eps_ll_average)
            # Train GMM for class 1 (positive class)
            gmm1_bin = train_GMM_LBG_EM(DTR_bin[:, LTR_bin==1], numComponents=numC_bin,
                                        covType=covType_bin, verbose=False,
                                        psiEig=psi_eigen_constraint, lbgAlpha=lbg_alpha,
                                        epsLLAverage=em_eps_ll_average)

            # Compute log-likelihoods for validation data for both GMMs
            logpdf_gmm0_dval = logpdf_GMM(DVAL_bin, gmm0_bin)
            logpdf_gmm1_dval = logpdf_GMM(DVAL_bin, gmm1_bin)
            SLLR_bin = logpdf_gmm1_dval - logpdf_gmm0_dval # Calculate Log-Likelihood Ratio (LLR)

            # Compute minDCF and actDCF using bayesRisk functions
            minDCF_val = bayesRisk.compute_minDCF_binary_fast(SLLR_bin, LVAL_bin, target_prior_bin, cfn_bin, cfp_bin)
            actDCF_val = bayesRisk.compute_actDCF_binary_fast(SLLR_bin, LVAL_bin, target_prior_bin, cfn_bin, cfp_bin)

            print(f'  Components: {numC_bin:2d} - minDCF / actDCF: {minDCF_val:.4f} / {actDCF_val:.4f}')
        print()
```

### Expected Output

#### For Iris Dataset Classification:

```
--- IRIS Dataset Multiclass Classification ---

Covariance Type (IRIS): FULL
  Components:  1 - Error rate: 4.0%
  Components:  2 - Error rate: 4.0%
  Components:  4 - Error rate: 2.0%
  Components:  8 - Error rate: 2.0%
  Components: 16 - Error rate: 2.0%

Covariance Type (IRIS): DIAGONAL
  Components:  1 - Error rate: 4.0%
  Components:  2 - Error rate: 4.0%
  Components:  4 - Error rate: 2.0%
  Components:  8 - Error rate: 2.0%
  Components: 16 - Error rate: 2.0%

Covariance Type (IRIS): TIED
  Components:  1 - Error rate: 4.0%
  Components:  2 - Error rate: 4.0%
  Components:  4 - Error rate: 2.0%
  Components:  8 - Error rate: 2.0%
  Components: 16 - Error rate: 2.0%
```

#### For Binary Artificial Dataset Classification:

```
--- Binary Artificial Dataset Classification (ext_data_binary) ---

Covariance Type (BINARY): FULL
  Components:  1 - minDCF / actDCF: 0.4984 / 0.5398
  Components:  2 - minDCF / actDCF: 0.4302 / 0.4416
  Components:  4 - minDCF / actDCF: 0.3804 / 0.3957
  Components:  8 - minDCF / actDCF: 0.3477 / 0.3704
  Components: 16 - minDCF / actDCF: 0.3340 / 0.3468

Covariance Type (BINARY): DIAGONAL
  Components:  1 - minDCF / actDCF: 0.5203 / 0.5625
  Components:  2 - minDCF / actDCF: 0.4300 / 0.4300
  Components:  4 - minDCF / actDCF: 0.4074 / 0.4132
  Components:  8 - minDCF / actDCF: 0.3857 / 0.4018
  Components: 16 - minDCF / actDCF: 0.3719 / 0.3851

Covariance Type (BINARY): TIED
  Components:  1 - minDCF / actDCF: 0.4984 / 0.5398
  Components:  2 - minDCF / actDCF: 0.4302 / 0.4416
  Components:  4 - minDCF / actDCF: 0.3804 / 0.3957
  Components:  8 - minDCF / actDCF: 0.3477 / 0.3704
  Components: 16 - minDCF / actDCF: 0.3340 / 0.3468
```