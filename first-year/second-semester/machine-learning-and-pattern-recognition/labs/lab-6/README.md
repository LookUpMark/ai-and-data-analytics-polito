# Generative Gaussian Models for Classification - Laboratory 6 Guide

This guide provides a comprehensive overview of applying various **Generative Gaussian Models** for classification tasks, specifically utilizing the well-known Iris dataset.

### Dataset Overview

For this laboratory, we will primarily work with the **Iris dataset**, which possesses the following characteristics:
*   **Classes:** 3 distinct flower species (Setosa, Versicolor, Virginica).
*   **Features:** Each sample includes 4 distinct measurements.
*   **Loading:** The `load_iris()` utility function is provided, which returns the data matrix `D` (shaped `4 x 150`, i.e., 4 features by 150 samples) and its corresponding labels `L` (a `150,` 1D array).

### Prerequisites

To successfully complete this laboratory, you will rely on several utility functions previously introduced in earlier labs. These include:
*   `vcol(x)`: Reshapes a 1D array `x` into an `N x 1` column vector.
*   `vrow(x)`: Reshapes a 1D array `x` into a `1 x N` row vector.
*   `compute_mu_C(D)`: Computes the empirical mean vector and covariance matrix for a given data matrix `D`.
*   `logpdf_GAU_ND(X, mu, C)`: Calculates the log-density for multiple samples (arranged as `Features x Samples`) for a Multivariate Gaussian distribution.
*   `split_db_2to1(D, L, seed=0)`: Splits a dataset `D` and its labels `L` into a 2/3 training set and a 1/3 validation set.
*   `load_iris()`: Loads the Iris dataset.

### Assumed Utility Functions

For convenience and consistency, the necessary utility functions are provided below. These functions will be automatically available for your tasks.

```python
import numpy
import scipy.special  # Used for numerical stability in log-sum-exp operations
import sklearn.datasets  # Used for loading the Iris dataset

def convert_to_column_vector(array):
    """
    Reshape a 1D array into a column vector (N, 1).
    
    Args:
        array: 1D numpy array
    
    Returns:
        Column vector with shape (N, 1)
    """
    return array.reshape((array.size, 1))

def convert_to_row_vector(array):
    """
    Reshape a 1D array into a row vector (1, N).
    
    Args:
        array: 1D numpy array
    
    Returns:
        Row vector with shape (1, N)
    """
    return array.reshape((1, array.size))

def compute_mean_and_covariance(data_matrix):
    """
    Compute the empirical mean vector and covariance matrix of data.
    
    Args:
        data_matrix: Data matrix with shape (num_features x num_samples)
    
    Returns:
        mean_vector: Column vector of feature means
        covariance_matrix: Covariance matrix of features
    """
    # Calculate mean for each feature (row)
    mean_vector = convert_to_column_vector(data_matrix.mean(axis=1))
    
    # Center the data by subtracting the mean
    centered_data = data_matrix - mean_vector
    
    # Calculate covariance matrix
    num_samples = data_matrix.shape[1]
    covariance_matrix = (centered_data @ centered_data.T) / float(num_samples)
    
    return mean_vector, covariance_matrix

def compute_multivariate_gaussian_log_density(data_matrix, mean_vector, covariance_matrix):
    """
    Compute log-density for multiple samples under a Multivariate Gaussian distribution.
    
    Args:
        data_matrix: Data matrix with shape (num_features x num_samples)
        mean_vector: Mean vector of the Gaussian distribution
        covariance_matrix: Covariance matrix of the Gaussian distribution
    
    Returns:
        log_densities: Array of log-density values for each sample
    """
    num_features = data_matrix.shape[0]
    
    # Compute precision matrix (inverse of covariance)
    precision_matrix = numpy.linalg.inv(covariance_matrix)
    
    # Compute log determinant of covariance matrix for normalization
    log_determinant = numpy.linalg.slogdet(covariance_matrix)[1]
    
    # Center the data
    centered_data = data_matrix - mean_vector
    
    # Compute squared Mahalanobis distances for all samples
    mahalanobis_squared = (centered_data * (precision_matrix @ centered_data)).sum(axis=0)
    
    # Compute log-density using the multivariate Gaussian formula
    normalization_constant = -0.5 * num_features * numpy.log(2 * numpy.pi)
    log_densities = normalization_constant - 0.5 * log_determinant - 0.5 * mahalanobis_squared
    
    return log_densities

def split_dataset_2to1(data_matrix, labels, random_seed=0):
    """
    Split dataset into training (2/3) and validation (1/3) sets.
    
    Args:
        data_matrix: Data matrix with shape (num_features x num_samples)
        labels: Array of class labels
        random_seed: Seed for random number generator for reproducibility
    
    Returns:
        training_data: Tuple (training_matrix, training_labels)
        validation_data: Tuple (validation_matrix, validation_labels)
    """
    total_samples = data_matrix.shape[1]
    num_training_samples = int(total_samples * 2.0 / 3.0)
    
    # Set random seed for reproducible results
    numpy.random.seed(random_seed)
    
    # Create random permutation of sample indices
    shuffled_indices = numpy.random.permutation(total_samples)
    
    # Split indices into training and validation
    training_indices = shuffled_indices[:num_training_samples]
    validation_indices = shuffled_indices[num_training_samples:]
    
    # Extract training and validation data
    training_matrix = data_matrix[:, training_indices]
    validation_matrix = data_matrix[:, validation_indices]
    training_labels = labels[training_indices]
    validation_labels = labels[validation_indices]
    
    return (training_matrix, training_labels), (validation_matrix, validation_labels)

def load_iris_dataset():
    """
    Load the Iris dataset using scikit-learn.
    
    Returns:
        data_matrix: Feature matrix with shape (4, 150) - 4 features, 150 samples
        labels: Array of class labels with shape (150,)
    """
    iris_data = sklearn.datasets.load_iris()
    
    # Transpose data to get features as rows and samples as columns
    data_matrix = iris_data['data'].T
    labels = iris_data['target']
    
    return data_matrix, labels

# --- Example: Initial Data Loading and Splitting ---
# Load the complete Iris dataset and split it into training and validation sets
complete_data_matrix, complete_labels = load_iris_dataset()
(training_data, training_labels), (validation_data, validation_labels) = split_dataset_2to1(complete_data_matrix, complete_labels, random_seed=0)

print("Initial Data Load and Split:")
print(f"  Full dataset shapes: Data {complete_data_matrix.shape}, Labels {complete_labels.shape}")
print(f"  Training set shapes: Data {training_data.shape}, Labels {training_labels.shape}")
print(f"  Validation set shapes: Data {validation_data.shape}, Labels {validation_labels.shape}")
print(f"  Unique training labels: {numpy.unique(training_labels)}")
print(f"  Unique validation labels: {numpy.unique(validation_labels)}")
```

**Expected Initial Setup Output:**

```
Initial Data Load and Split:
  Full dataset shapes: Data (4, 150), Labels (150,)
  Training set shapes: Data (4, 100), Labels (100,)
  Validation set shapes: Data (4, 50), Labels (50,)
  Unique training labels: [0 1 2]
  Unique validation labels: [0 1 2]
```

---

## Part 1: Multivariate Gaussian Classifier (MVG)

The **Multivariate Gaussian (MVG) Classifier** is a generative model built on the assumption that data originating from each class $c$ independently follows a distinct Multivariate Gaussian distribution, denoted as $N(x | \mu_c, \Sigma_c)$.

### Task 1.1: MVG - Maximum Likelihood Parameter Estimation

#### Task Objective

Implement the `estimate_mvg_parameters(data_matrix, labels)` function. This function computes the **Maximum Likelihood (ML) estimates** for both the mean vector $\mu_c$ and the covariance matrix $\Sigma_c$ for *each individual class* $c$ present within the provided training data.

#### Reasoning

In the context of Gaussian distributions, the Maximum Likelihood estimates for its parameters (mean and covariance) are simply the empirical (sample) mean and empirical (sample) covariance derived directly from the observed data. To apply this principle for MVG, these parameters must be estimated independently for each class. Therefore, the implementation should iterate through all unique class labels, filter the training data to isolate samples belonging to the current class, and then use the `compute_mean_and_covariance` function to obtain the mean and covariance for that specific class. These estimated parameters should then be stored in a dictionary, mapped to their corresponding class labels.

#### Solution Code (`sol.py`)

```python
import numpy

def estimate_mvg_parameters(data_matrix, labels):
    """
    Compute Maximum Likelihood estimates (mean, covariance) for Multivariate Gaussian (MVG) 
    classifier for each class.
    
    Args:
        data_matrix: Training data with shape (num_features x num_samples)
        labels: Array of class labels for training samples
    
    Returns:
        class_parameters: Dictionary mapping class label to (mean_vector, covariance_matrix) tuple
    """
    unique_class_labels = numpy.unique(labels)
    class_parameters = {}
    
    for class_label in unique_class_labels:
        # Filter data to get samples belonging only to current class
        class_data = data_matrix[:, labels == class_label]
        
        # Compute ML estimates for mean and covariance of current class
        class_mean, class_covariance = compute_mean_and_covariance(class_data)
        
        # Store parameters for current class
        class_parameters[class_label] = (class_mean, class_covariance)
    
    return class_parameters
```

#### Expected Output

```
MVG - Class 0 Parameters:
Mean (mu_0):
[[4.96129032]
 [3.42903226]
 [1.46451613]
 [0.2483871 ]]
Covariance (Sigma_0):
[[ 0.13140479  0.11370447  0.02862643  0.01187305]
 [ 0.11370447  0.16270552  0.01844953  0.01117586]
 [ 0.02862643  0.01844953  0.03583767  0.00526535]
 [ 0.01187305  0.01117586  0.00526535  0.0108845 ]]

MVG - Class 1 Parameters:
Mean (mu_1):
[[5.91212121]
 [2.78484848]
 [4.27272727]
 [1.33939394]]
Covariance (Sigma_1):
[[0.29267139 0.09340051 0.16503921 0.09706316]
 [0.09340051 0.09395252 0.05436666 0.05269781]
 [0.16503921 0.05436666 0.22271879 0.09673994]
 [0.09706316 0.05269781 0.09673994 0.05777174]]

MVG - Class 2 Parameters:
Mean (mu_2):
[[6.59016393]
 [2.97868852]
 [5.55737705]
 [2.01639344]]
Covariance (Sigma_2):
[[0.4851636  0.08865747 0.35414801 0.0551066 ]
 [0.08865747 0.09321487 0.07663456 0.06377994]
 [0.0551066  0.06377994 0.09886278 0.08182903]]
```

---

### Task 1.2: MVG - Computing Class-Conditional Log-Likelihoods

#### Task Objective

Implement `compute_class_conditional_log_likelihoods(data_matrix, class_parameters)`. This function should calculate the **class-conditional log-likelihoods** $\log f(x|c)$ for *each sample* $x$ in a given dataset (e.g., the validation set). These calculations must be performed for *every class* $c$, utilizing the estimated MVG parameters provided in `class_parameters`.

#### Reasoning

The objective is to produce a score matrix `S`. This matrix will have dimensions `num_classes x num_samples`, where each element `S[c, i]` represents the log-likelihood $\log f(data_matrix[:, i] | \text{class } c)$. To achieve this, the function should iterate through each class for which parameters are stored in `class_parameters`. For each class, it will retrieve its corresponding mean and covariance matrix. Subsequently, the `compute_multivariate_gaussian_log_density` function will be called with the entire dataset and the class-specific parameters. This will compute the log-likelihoods of all samples with respect to that single class. The results are then stored in the appropriate row of the score matrix.

#### Solution Code (`sol.py`)

```python
import numpy

def compute_class_conditional_log_likelihoods(data_matrix, class_parameters):
    """
    Compute class-conditional log-likelihoods for each sample in data_matrix for each class.
    
    Args:
        data_matrix: Data to evaluate with shape (num_features x num_samples)
        class_parameters: Dictionary mapping class labels to (mean_vector, covariance_matrix) tuples
    
    Returns:
        log_likelihood_matrix: Matrix with shape (num_classes x num_samples) where element [c, i] 
                              is log f(sample_i | class_c)
    """
    num_classes = len(class_parameters)
    num_samples = data_matrix.shape[1]
    
    # Initialize the log-likelihood matrix
    log_likelihood_matrix = numpy.zeros((num_classes, num_samples))
    
    # Compute log-likelihoods for each class
    for class_index in range(num_classes):
        class_label = class_index  # Assuming class labels are 0, 1, 2, ...
        
        # Retrieve mean and covariance for current class
        class_mean, class_covariance = class_parameters[class_label]
        
        # Compute log-likelihoods for all samples given current class
        log_likelihood_matrix[class_index, :] = compute_multivariate_gaussian_log_density(
            data_matrix, class_mean, class_covariance
        )
    
    return log_likelihood_matrix
```

#### Expected Output

This function primarily returns the `S` matrix. Its correct computation is implicitly verified by the accurate error rate reported in Task 1.3's output.

---

### Task 1.3: MVG - Computing Log-Posterior Probabilities (Log-Domain)

#### Task Objective

Implement the `compute_log_posterior_probabilities(log_likelihood_matrix, class_priors)` function. Its goal is to compute the **log-posterior probabilities** $\log P(c|x)$ for all classes and all samples. A critical aspect of this implementation is the utilization of the **log-sum-exp trick** to ensure numerical stability.

#### Reasoning

The calculation of log-posterior probabilities is based on Bayes' theorem, applied in the log-domain for numerical robustness: $\log P(c|x) = \log f(x|c) + \log P(c) - \log f(x)$. This can be broken down into three main computational steps:

1.  **Log Joint Probability (`log_joint_probabilities`):** Compute the log joint probability $\log P(x, c)$ for all $x$ and $c$. This is achieved by summing the log-likelihoods and the log prior probabilities for each class: `log_joint_probabilities = log_likelihood_matrix + convert_to_column_vector(numpy.log(class_priors))`.
2.  **Log Marginal Density (`log_marginal_probabilities`):** Calculate the log marginal density $\log f(x)$ (also $\log P(x)$) for each sample. This requires summing $\exp(\log P(x, c))$ over all classes $c$, which is prone to underflow/overflow. Therefore, the `scipy.special.logsumexp` function is used for numerical stability: `log_marginal_probabilities = convert_to_row_vector(scipy.special.logsumexp(log_joint_probabilities, axis=0))`.
3.  **Log Posterior (`log_posterior_probabilities`):** Finally, compute the log posterior $\log P(c|x)$ by subtracting the log marginal density from the log joint probability: `log_posterior_probabilities = log_joint_probabilities - log_marginal_probabilities`.

#### Solution Code (`sol.py`)

```python
import numpy
import scipy.special  # Import for numerical stability in log-sum-exp

def compute_log_posterior_probabilities(log_likelihood_matrix, class_priors):
    """
    Compute log-posterior probabilities from log-likelihoods and class priors,
    using the log-sum-exp trick for numerical stability.
    
    Args:
        log_likelihood_matrix: Matrix of log P(data | class) with shape (num_classes x num_samples)
        class_priors: Array of class prior probabilities with shape (num_classes,)
    
    Returns:
        log_posterior_matrix: Matrix of log-posterior probabilities with shape (num_classes x num_samples)
    """
    
    # Step 1: Convert class priors to log-domain and reshape to column vector
    log_class_priors_column = convert_to_column_vector(numpy.log(class_priors))
    
    # Step 2: Compute log joint probabilities: log P(data, class) = log P(data | class) + log P(class)
    log_joint_probabilities = log_likelihood_matrix + log_class_priors_column
    
    # Step 3: Compute log marginal probabilities: log P(data) = log sum_c P(data, class)
    # Use logsumexp for numerical stability when computing log of sum of exponentials
    log_marginal_probabilities = convert_to_row_vector(
        scipy.special.logsumexp(log_joint_probabilities, axis=0)
    )
    
    # Step 4: Compute log posterior probabilities: log P(class | data) = log P(data, class) - log P(data)
    log_posterior_matrix = log_joint_probabilities - log_marginal_probabilities
    
    return log_posterior_matrix
```

#### Expected Output

```
Max absolute error w.r.t. pre-computed solution - log-posterior matrix:
(A very small number, typically ~1.1102e-16 or 0.0)
```

---

### Task 1.4: MVG - Prediction and Evaluation

#### Task Objective

This task has two objectives:
1.  **Predict class labels** for the validation samples by utilizing the computed MVG log-posterior probabilities.
2.  **Calculate the classification error rate** based on these predictions.

#### Reasoning

For optimal prediction under a 0-1 loss function, the predicted class for each sample $x$ is simply the class $c$ that maximizes the log posterior probability $\log P(c|x)$. This is achieved by finding the `argmax` along the class dimension of the log-posterior matrix. Once predictions are made, the error rate is calculated as the proportion of misclassified samples relative to the total number of validation samples.

#### Solution Code (`sol.py` - within `if __name__ == '__main__':` block for MVG)

```python
# --- Complete MVG Classification Pipeline ---

# Step 1: Estimate MVG parameters from training data
mvg_class_parameters = estimate_mvg_parameters(training_data, training_labels)

# Step 2: Compute class-conditional log-likelihoods for validation data
validation_log_likelihoods = compute_class_conditional_log_likelihoods(validation_data, mvg_class_parameters)

# Step 3: Set uniform class priors (equal probability for each class)
num_classes = len(numpy.unique(training_labels))
uniform_class_priors = numpy.ones(num_classes) / num_classes

# Step 4: Compute log-posterior probabilities
validation_log_posteriors = compute_log_posterior_probabilities(validation_log_likelihoods, uniform_class_priors)

# Step 5: Make predictions by selecting class with maximum log-posterior probability
predicted_labels_mvg = validation_log_posteriors.argmax(axis=0)

# Step 6: Calculate classification error rate
num_errors = (predicted_labels_mvg != validation_labels).sum()
total_samples = validation_labels.size
error_rate_mvg = (num_errors / float(total_samples)) * 100

print("MVG Classifier Error Rate: %.1f%%" % error_rate_mvg)
```

#### Expected Output

```
MVG Classifier Error Rate: 4.0%
```

---

## Part 2: Naive Bayes Gaussian Classifier

The **Naive Bayes Gaussian Classifier** is a simplified generative model. It fundamentally assumes that the features are **conditionally independent given the class**. This crucial assumption leads to a significant constraint: the class-specific covariance matrix $\Sigma_c$ must be a **diagonal matrix**.

### Task 2.1: Naive Bayes - Parameter Estimation

#### Task Objective

Implement `estimate_naive_bayes_parameters(data_matrix, labels)` to compute the Maximum Likelihood (ML) estimates for the Naive Bayes model. This includes determining the **class mean** $\mu_c$ for each class, as well as the **diagonal covariance matrix** $\Sigma_{c\_diag}$ for each class.

#### Reasoning

The class means $\mu_c$ are still derived as the empirical means of the samples belonging to each respective class. However, for the diagonal covariance matrix $\Sigma_{c\_diag}$, the process differs. One approach is to first compute the full empirical covariance matrix for each class independently. Subsequently, this full covariance matrix is converted into a diagonal matrix by retaining only its diagonal elements (setting all off-diagonal elements to zero), which can be achieved efficiently using element-wise multiplication with an identity matrix. Finally, the mean and diagonal covariance pair for each class is stored.

#### Solution Code (`sol.py`)

```python
import numpy

def estimate_naive_bayes_parameters(data_matrix, labels):
    """
    Compute Maximum Likelihood estimates (mean, diagonal covariance) for 
    Naive Bayes Gaussian model for each class.
    
    Args:
        data_matrix: Training data with shape (num_features x num_samples)
        labels: Array of class labels for training samples
    
    Returns:
        class_parameters: Dictionary mapping class label to (mean_vector, diagonal_covariance_matrix) tuple
    """
    unique_class_labels = numpy.unique(labels)
    class_parameters = {}
    num_features = data_matrix.shape[0]
    
    for class_label in unique_class_labels:
        # Filter data to get samples belonging only to current class
        class_data = data_matrix[:, labels == class_label]
        
        # Compute full mean and covariance for current class
        class_mean, full_covariance = compute_mean_and_covariance(class_data)
        
        # Create diagonal covariance matrix by zeroing out off-diagonal elements
        # This enforces the conditional independence assumption of Naive Bayes
        diagonal_covariance = full_covariance * numpy.eye(num_features)
        
        # Store parameters for current class
        class_parameters[class_label] = (class_mean, diagonal_covariance)
    
    return class_parameters
```

#### Expected Output

```
Naive Bayes Gaussian - Class 0 Parameters:
Mean (mu_0):
[[4.96129032]
 [3.42903226]
 [1.46451613]
 [0.2483871 ]]
Covariance (Sigma_0_diagonal):
[[0.13140479 0.         0.         0.        ]
 [0.         0.16270552 0.         0.        ]
 [0.         0.         0.03583767 0.        ]
 [0.         0.         0.         0.0108845 ]]
```
*(Similar output for Classes 1 and 2 will follow, each exhibiting a diagonal covariance matrix.)*

---

### Task 2.2: Naive Bayes - Classification and Evaluation

#### Task Objective

This task involves a complete classification pipeline for the Naive Bayes Gaussian Classifier:
1.  Compute class-conditional log-likelihoods using the Naive Bayes parameters.
2.  Subsequently, compute the log-posterior probabilities.
3.  Predict the class labels for the validation set.
4.  Finally, calculate and report the classification error rate.

#### Reasoning

The overall workflow for classification and evaluation for the Naive Bayes model is functionally identical to that of the MVG classifier. The key distinction lies solely in the set of parameters used: instead of using the full covariance matrices from MVG, we now employ the diagonal covariance matrices derived for the Naive Bayes model.

#### Solution Code (`sol.py` - within `if __name__ == '__main__':` block for Naive Bayes)

```python
# --- Complete Naive Bayes Classification Pipeline ---

# Step 1: Estimate Naive Bayes parameters from training data
naive_bayes_class_parameters = estimate_naive_bayes_parameters(training_data, training_labels)

# Step 2: Compute class-conditional log-likelihoods for validation data
validation_log_likelihoods_nb = compute_class_conditional_log_likelihoods(validation_data, naive_bayes_class_parameters)

# Step 3: Set uniform class priors (equal probability for each class)
uniform_class_priors = numpy.ones(num_classes) / num_classes

# Step 4: Compute log-posterior probabilities
validation_log_posteriors_nb = compute_log_posterior_probabilities(validation_log_likelihoods_nb, uniform_class_priors)

# Step 5: Make predictions by selecting class with maximum log-posterior probability
predicted_labels_naive_bayes = validation_log_posteriors_nb.argmax(axis=0)

# Step 6: Calculate classification error rate
num_errors_nb = (predicted_labels_naive_bayes != validation_labels).sum()
error_rate_naive_bayes = (num_errors_nb / float(total_samples)) * 100

print("Naive Bayes Gaussian Classifier Error Rate: %.1f%%" % error_rate_naive_bayes)
```

#### Expected Output

```
Naive Bayes Gaussian Classifier Error Rate: 4.0%
```

---

## Part 3: Tied Covariance Gaussian Classifier

The **Tied Covariance Gaussian Classifier** introduces a specific assumption: **all classes share the *same* covariance matrix ($\Sigma^*$)**. However, each class is still permitted to have its own distinct **class-specific mean vector ($\mu_c$)**.

### Task 3.1: Tied Covariance - Parameter Estimation

#### Task Objective

Implement `estimate_tied_covariance_parameters(data_matrix, labels)` to compute the Maximum Likelihood (ML) estimates for the Tied Covariance model. This includes determining each **class mean** $\mu_c$ and a single, **shared covariance matrix** $\Sigma^*$.

#### Reasoning

Similar to previous models, the class means $\mu_c$ are derived as the empirical means of the samples within each respective class. The calculation of the shared covariance matrix $\Sigma^*$, however, requires a different approach. $\Sigma^*$ is effectively a weighted average of the individual class covariance matrices. More efficiently, it can be computed as the total sum of the unnormalized within-class scatter matrices (which are the sum of centered outer products for each class) divided by the total number of samples across all classes. The implementation should first compute all individual class means, then accumulate the unnormalized scatter, and finally compute the single shared covariance matrix before populating the dictionary with mean and shared covariance pairs for each class.

#### Solution Code (`sol.py`)

```python
import numpy

def estimate_tied_covariance_parameters(data_matrix, labels):
    """
    Compute Maximum Likelihood estimates (class means, shared covariance) for 
    Tied Covariance Gaussian model.
    
    Args:
        data_matrix: Training data with shape (num_features x num_samples)
        labels: Array of class labels for training samples
    
    Returns:
        class_parameters: Dictionary mapping class label to (mean_vector, shared_covariance_matrix) tuple
    """
    unique_class_labels = numpy.unique(labels)
    class_means = {}  # Store individual class means
    total_scatter_matrix = 0  # Accumulate sum of unnormalized within-class scatter matrices
    total_samples = data_matrix.shape[1]

    # Step 1: Compute class means and accumulate scatter matrices
    for class_label in unique_class_labels:
        # Filter data for current class
        class_data = data_matrix[:, labels == class_label]
        
        # Compute mean and normalized covariance for current class
        class_mean, class_covariance_normalized = compute_mean_and_covariance(class_data)
        
        # Store class mean
        class_means[class_label] = class_mean
        
        # Accumulate unnormalized scatter matrix
        # class_covariance_normalized already includes normalization by num_class_samples
        # So we multiply by num_class_samples to get unnormalized scatter
        num_class_samples = class_data.shape[1]
        unnormalized_scatter = class_covariance_normalized * num_class_samples
        total_scatter_matrix += unnormalized_scatter
    
    # Step 2: Compute shared covariance matrix by normalizing total scatter
    shared_covariance_matrix = total_scatter_matrix / total_samples

    # Step 3: Create final parameter dictionary with shared covariance for all classes
    class_parameters = {}
    for class_label in unique_class_labels:
        class_parameters[class_label] = (class_means[class_label], shared_covariance_matrix)
    
    return class_parameters
```

#### Expected Output

```
Tied Gaussian - Class 0 Parameters:
Mean (mu_0):
[[4.96129032]
 [3.42903226]
 [1.46451613]
 [0.2483871 ]]
Shared Covariance (Sigma*):
[[0.23637589 0.09525344 0.1364944  0.03614529]
 [0.09525344 0.11618517 0.05768855 0.0357726 ]
 [0.1364944  0.05768855 0.14992811 0.03746458]
 [0.03614529 0.0357726  0.03746458 0.04291763]]
```
*(The Shared Covariance matrix will be identical for Classes 1 and 2, as per the model's assumption.)*

---

### Task 3.2: Tied Covariance - Classification and Evaluation

#### Task Objective

This task requires completing the classification pipeline for the Tied Covariance Gaussian Classifier:
1.  Compute the class-conditional log-likelihoods using the estimated Tied Covariance parameters.
2.  Compute the log-posterior probabilities based on these likelihoods.
3.  Predict the class labels for the validation set.
4.  Finally, calculate and report the classification error rate.

#### Reasoning

The classification workflow for the Tied Covariance model is conceptually identical to both the MVG and Naive Bayes models. The primary difference lies in the specific parameter set utilized: here, we use class-specific means but a single, shared covariance matrix.

#### Solution Code (`sol.py` - within `if __name__ == '__main__':` block for Tied Gaussian)

```python
# --- Complete Tied Covariance Classification Pipeline ---

# Step 1: Estimate Tied Covariance parameters from training data
tied_covariance_class_parameters = estimate_tied_covariance_parameters(training_data, training_labels)

# Step 2: Compute class-conditional log-likelihoods for validation data
validation_log_likelihoods_tied = compute_class_conditional_log_likelihoods(validation_data, tied_covariance_class_parameters)

# Step 3: Set uniform class priors (equal probability for each class)
uniform_class_priors = numpy.ones(num_classes) / num_classes

# Step 4: Compute log-posterior probabilities
validation_log_posteriors_tied = compute_log_posterior_probabilities(validation_log_likelihoods_tied, uniform_class_priors)

# Step 5: Make predictions by selecting class with maximum log-posterior probability
predicted_labels_tied = validation_log_posteriors_tied.argmax(axis=0)

# Step 6: Calculate classification error rate
num_errors_tied = (predicted_labels_tied != validation_labels).sum()
error_rate_tied = (num_errors_tied / float(total_samples)) * 100

print("Tied Gaussian Classifier Error Rate: %.1f%%" % error_rate_tied)
```

#### Expected Output

```
Tied Gaussian Classifier Error Rate: 2.0%
```

---

## Part 4: Binary Task (Versicolor vs Virginica) Revisited

This section revisits a **binary classification task**: specifically, distinguishing **Iris Versicolor (Class 1)** from **Iris Virginica (Class 2)**. We will re-evaluate the performance of both **MVG** and **Tied Covariance** models in this context, utilizing the **log-likelihood ratio (LLR)** for decision making.

### Task 4.1: Binary Classification with LLR

#### Task Objective

This task involves a series of steps to perform binary classification using LLR:
1.  Filter the full Iris dataset to include only samples from Class 1 and Class 2.
2.  Split this newly filtered binary dataset into training and validation sets.
3.  Train both binary MVG and binary Tied Covariance models on the training portion of this dataset.
4.  Compute the Log-Likelihood Ratio (LLR) for each sample in the validation set for both models.
5.  Classify samples using a simple LLR threshold of 0 (which implies an assumption of uniform class priors).
6.  Finally, report the classification error rates for both the MVG and Tied Covariance models on this binary task.

#### Reasoning

1.  **Filter and Split Data:** Start by filtering the complete dataset to create binary datasets containing only classes 1 and 2 (e.g., `binary_data = complete_data[:, complete_labels != 0]`, `binary_labels = complete_labels[complete_labels != 0]`). Then, split this binary dataset into training and validation sets.
2.  **Train Binary Models:** Train class-specific parameters for both MVG and Tied Covariance models using only the binary training data. This means calling the appropriate parameter estimation functions with the binary training data.
3.  **Compute LLR:** The Log-Likelihood Ratio (LLR) for a sample $x$ is defined as $\text{llr}(x) = \log f(x|C=2) - \log f(x|C=1)$. To compute this for all validation samples, first calculate the log-likelihoods for each sample under Class 2 and Class 1 using the `compute_multivariate_gaussian_log_density` function with the respective binary model parameters. Then, simply subtract the Class 1 log-likelihoods from the Class 2 log-likelihoods element-wise.
4.  **Classify:** With uniform priors, the decision boundary for LLR is 0. Thus, if `LLR >= 0`, predict Class 2; otherwise, if `LLR < 0`, predict Class 1.
5.  **Evaluate Performance:** Calculate the error rate by counting the number of samples where the predictions do not match the true labels and dividing by the total samples in the validation set.

#### Solution Code (`sol.py` - within `if __name__ == '__main__':` block for 2-Class task)

```python
# --- Part 4: Binary Classification (Versicolor vs Virginica) ---

print("\n--- Part 4: Binary Classification (Versicolor vs Virginica) ---")

# Step 1: Filter data to include only classes 1 and 2 (Versicolor and Virginica)
binary_data_matrix = complete_data_matrix[:, complete_labels != 0]
binary_labels = complete_labels[complete_labels != 0]

# Step 2: Split the filtered binary dataset into training and validation sets
(binary_training_data, binary_training_labels), (binary_validation_data, binary_validation_labels) = split_dataset_2to1(binary_data_matrix, binary_labels, random_seed=0)

print(f"  Binary Training set shapes: Data {binary_training_data.shape}, Labels {binary_training_labels.shape}")
print(f"  Binary Validation set shapes: Data {binary_validation_data.shape}, Labels {binary_validation_labels.shape}")
print(f"  Unique binary training labels: {numpy.unique(binary_training_labels)}")
print(f"  Unique binary validation labels: {numpy.unique(binary_validation_labels)}")

# --- MVG Model for Binary Classification ---
print("\n  ---- MVG Classifier for Binary Task ----")

# Step 3: Train MVG parameters using only binary training data
binary_mvg_parameters = estimate_mvg_parameters(binary_training_data, binary_training_labels)

# Step 4: Compute log-likelihoods for each class (1 and 2) on validation set
class_2_mean_mvg, class_2_covariance_mvg = binary_mvg_parameters[2]
log_likelihood_class_2_mvg = compute_multivariate_gaussian_log_density(
    binary_validation_data, class_2_mean_mvg, class_2_covariance_mvg
)

class_1_mean_mvg, class_1_covariance_mvg = binary_mvg_parameters[1]
log_likelihood_class_1_mvg = compute_multivariate_gaussian_log_density(
    binary_validation_data, class_1_mean_mvg, class_1_covariance_mvg
)

# Step 5: Compute Log-Likelihood Ratio (LLR): log f(x|C=2) - log f(x|C=1)
llr_mvg_binary = log_likelihood_class_2_mvg - log_likelihood_class_1_mvg

# Step 6: Classify based on LLR threshold of 0 (assuming uniform priors)
llr_threshold = 0
predicted_labels_mvg_binary = numpy.zeros(binary_validation_data.shape[1], dtype=numpy.int32)
predicted_labels_mvg_binary[llr_mvg_binary >= llr_threshold] = 2  # If LLR >= 0, predict Class 2
predicted_labels_mvg_binary[llr_mvg_binary < llr_threshold] = 1   # Otherwise, predict Class 1

# Step 7: Calculate and print error rate for MVG binary classifier
num_errors_mvg_binary = (predicted_labels_mvg_binary != binary_validation_labels).sum()
error_rate_mvg_binary = (num_errors_mvg_binary / float(binary_validation_labels.size)) * 100
print("  MVG (Binary) Classifier Error Rate: %.1f%%" % error_rate_mvg_binary)

# --- Tied Covariance Model for Binary Classification ---
print("\n  ---- Tied Covariance Classifier for Binary Task ----")

# Step 3: Train Tied Covariance parameters using only binary training data
binary_tied_parameters = estimate_tied_covariance_parameters(binary_training_data, binary_training_labels)

# Step 4: Compute log-likelihoods for each class (1 and 2) on validation set
class_2_mean_tied, shared_covariance_tied = binary_tied_parameters[2]
log_likelihood_class_2_tied = compute_multivariate_gaussian_log_density(
    binary_validation_data, class_2_mean_tied, shared_covariance_tied
)

class_1_mean_tied, _ = binary_tied_parameters[1]  # Shared covariance is the same for both classes
log_likelihood_class_1_tied = compute_multivariate_gaussian_log_density(
    binary_validation_data, class_1_mean_tied, shared_covariance_tied
)

# Step 5: Compute Log-Likelihood Ratio (LLR)
llr_tied_binary = log_likelihood_class_2_tied - log_likelihood_class_1_tied

# Step 6: Classify based on LLR threshold of 0
predicted_labels_tied_binary = numpy.zeros(binary_validation_data.shape[1], dtype=numpy.int32)
predicted_labels_tied_binary[llr_tied_binary >= llr_threshold] = 2
predicted_labels_tied_binary[llr_tied_binary < llr_threshold] = 1

# Step 7: Calculate and print error rate for Tied Covariance binary classifier
num_errors_tied_binary = (predicted_labels_tied_binary != binary_validation_labels).sum()
error_rate_tied_binary = (num_errors_tied_binary / float(binary_validation_labels.size)) * 100
print("  Tied (Binary) Classifier Error Rate: %.1f%%" % error_rate_tied_binary)
```

#### Expected Output

```
--- Part 4: Binary Classification (Versicolor vs Virginica) ---
  Binary Training set shapes: Data (4, 66), Labels (66,)
  Binary Validation set shapes: Data (4, 34), Labels (34,)
  Unique binary training labels: [1 2]
  Unique binary validation labels: [1 2]

  ---- MVG Classifier for Binary Task ----
  MVG (Binary) Classifier Error Rate: 8.8%

  ---- Tied Covariance Classifier for Binary Task ----
  Tied (Binary) Classifier Error Rate: 2.9%
```