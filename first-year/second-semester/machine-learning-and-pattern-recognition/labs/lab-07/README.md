# Generative Multinomial Models - Laboratory 7 Guide

This guide details **Generative Multinomial Models** for text classification. Specifically, we will classify three-line stanzas (tercets) from **Dante Alighieri's "Divina Commedia"** based on their originating Cantica (**Inferno**, **Purgatorio**, **Paradiso**). The primary goal is to construct and evaluate a **Multinomial Naive Bayes classifier** to distinguish the linguistic styles of these different sections.

## Dataset Overview

*   **Source**: Dante Alighieri's "Divina Commedia."
*   **Data Files**: The data is provided in `data/inferno.txt`, `data/purgatorio.txt`, and `data/paradiso.txt`. Each file contains one tercet per line.

## Prerequisites

To effectively follow this guide, a foundational understanding of the following concepts and tools is beneficial:
*   Basic Python programming (including functions, loops, and data structures).
*   NumPy (for array manipulation and operations).
*   Core probability concepts (likelihood, prior, posterior).
*   The use of logarithms for numerical stability (particularly for log-likelihood and log-posterior calculations).
*   (Optional but helpful): Familiarity with Bag-of-Words models.

---

## Task 1: Data Loading and Preparation

### Task Objective

1.  Load tercets corresponding to each Cantica from their respective text files.
2.  Subsequently, split each loaded list of tercets into a **training set** (comprising 75% of the data) and a **validation/evaluation set** (comprising the remaining 25%).

### Step-by-Step Reasoning

1.  **Loading Tercets**: Utilize a dedicated function (e.g., `load_data` within `load.py`). This function should open each file using `encoding="ISO-8859-1"`, read its contents line by line, and `strip()` any leading or trailing whitespace from each line. The processed tercets are then stored in separate Python lists.
2.  **Splitting Data**: Employ another utility function (e.g., `split_data(l, n)` in `load.py`) to divide each list. For a 75%/25% split ratio, set `n=4`, meaning every 4th element will be assigned to the test set, while the remaining elements go to the training set. Apply this splitting mechanism to the lists for Inferno, Purgatorio, and Paradiso to generate their respective training and evaluation subsets (e.g., `lInfTrain`, `lInfEval`, etc.).

#### Solution Code (`load.py`)

```python
def load_data():
    """Load tercets from the three Divina Commedia text files."""
    
    # Load Inferno tercets
    inferno_tercets = []
    with open('data/inferno.txt', encoding="ISO-8859-1") as file:
        for line in file:
            tercet = line.strip()  # Remove whitespace
            inferno_tercets.append(tercet)
    
    # Load Purgatorio tercets
    purgatorio_tercets = []
    with open('data/purgatorio.txt', encoding="ISO-8859-1") as file:
        for line in file:
            tercet = line.strip()  # Remove whitespace
            purgatorio_tercets.append(tercet)
    
    # Load Paradiso tercets
    paradiso_tercets = []
    with open('data/paradiso.txt', encoding="ISO-8859-1") as file:
        for line in file:
            tercet = line.strip()  # Remove whitespace
            paradiso_tercets.append(tercet)
    
    return inferno_tercets, purgatorio_tercets, paradiso_tercets

def split_data(tercet_list, split_ratio):
    """
    Split a list of tercets into training and test sets.
    
    Args:
        tercet_list: List of tercets to split
        split_ratio: Every nth element goes to test set (e.g., 4 means 25% test, 75% train)
    
    Returns:
        training_tercets, test_tercets
    """
    training_tercets = []
    test_tercets = []
    
    for index in range(len(tercet_list)):
        if index % split_ratio == 0:
            # Every nth element goes to test set
            test_tercets.append(tercet_list[index])
        else:
            # All other elements go to training set
            training_tercets.append(tercet_list[index])
    
    return training_tercets, test_tercets

# --- Example Usage (in commedia.py's main section) ---
# Load data from files
inferno_data, purgatorio_data, paradiso_data = load_data()

# Split each dataset into training (75%) and evaluation (25%) sets
inferno_train, inferno_eval = split_data(inferno_data, 4)
purgatorio_train, purgatorio_eval = split_data(purgatorio_data, 4)
paradiso_train, paradiso_eval = split_data(paradiso_data, 4)

# Print summary of data loading and preparation
print("Data Loading and Preparation Summary:")
print(f"  Inferno:    Training = {len(inferno_train)} tercets, Evaluation = {len(inferno_eval)} tercets")
print(f"  Purgatorio: Training = {len(purgatorio_train)} tercets, Evaluation = {len(purgatorio_eval)} tercets")
print(f"  Paradiso:   Training = {len(paradiso_train)} tercets, Evaluation = {len(paradiso_eval)} tercets")
```

#### Expected Output

```
Data Loading and Preparation Summary:
  Inferno:    Training = 1104 tercets, Evaluation = 368 tercets
  Purgatorio: Training = 1002 tercets, Evaluation = 334 tercets
  Paradiso:   Training = 1002 tercets, Evaluation = 334 tercets
```

---

## Task 2: Multinomial Model Training

### Task Objective

Estimate the **Maximum Likelihood (ML) parameters** for a Multinomial model, specifically for each Cantica. This estimation must incorporate **add-epsilon smoothing** (denoted as $\varepsilon$) to prevent zero probabilities when calculating word log-probabilities ($log \pi_{c,j}$).

The formula for $log \pi_{c,j}$ is:
$$
\log \pi_{c,j} = \log(N_{c,j} + \varepsilon) - \log(N_c + M \cdot \varepsilon)
$$
Where:
*   $N_{c,j}$ represents the count of word `j` occurring in documents (tercets) belonging to class `c`.
*   $N_c$ is the total number of words found in all documents of class `c`.
*   $M$ denotes the total size of the vocabulary.

### Step-by-Step Reasoning (Method 2 - Using Arrays)

1.  **Vocabulary Construction (`hWordDict`):** First, build a comprehensive vocabulary. This involves creating a mapping (`hWordDict`) where each unique word encountered across *all training tercets* is assigned a unique integer index, ranging from 0 to $M-1$. The `S2_buildDictionary` function can be used for this purpose.
2.  **Initialize Word Counts:** For each class `c`, initialize a NumPy array of size $M$ (equal to the vocabulary size). All elements in these arrays should be pre-filled with the smoothing factor $\varepsilon$.
3.  **Count Occurrences ($N_{c,j}$):** Iterate through all training tercets, processing them class by class. For each tercet, split it into individual words. Then, for every word `w`, locate its corresponding index `j` in `hWordDict` and increment the count at `h_clsLogProb[cls][j]` by 1.
4.  **Calculate Log Probabilities:** After counting, for each class `c`, retrieve its word count vector (which now contains $N_{c,j} + \varepsilon$). Sum this vector to obtain the total count $N_c + M \cdot \varepsilon$ (the denominator for normalization). Subsequently, apply the log-probability formula element-wise: `numpy.log(count_vector) - numpy.log(total_count)`.
5.  **Store Parameters:** Store the computed log-probability vectors in the `h_clsLogProb` dictionary, where each class label maps to its corresponding NumPy array of log-probabilities. Finally, return both `h_clsLogProb` and `hWordDict`.

#### Solution Code (`commedia.py` - Method 2 functions)

```python
import numpy
import itertools

def build_vocabulary_dictionary(tercet_list):
    """
    Build a vocabulary mapping unique words to integer indices.
    
    Args:
        tercet_list: List of tercets (strings)
    
    Returns:
        word_to_index_dict: Dictionary mapping each unique word to an integer index
    """
    word_to_index_dict = {}
    next_word_index = 0
    
    for tercet in tercet_list:
        words = tercet.split()
        for word in words:
            if word not in word_to_index_dict:
                # Assign new index to previously unseen word
                word_to_index_dict[word] = next_word_index
                next_word_index += 1
    
    return word_to_index_dict

def estimate_multinomial_model(training_tercets_by_class, smoothing_epsilon=0.001):
    """
    Estimate Multinomial model parameters (word log-probabilities per class)
    with add-epsilon smoothing to avoid zero probabilities.
    
    Args:
        training_tercets_by_class: Dictionary where keys are class names,
                                 values are lists of training tercets for that class
        smoothing_epsilon: Smoothing factor to add to all word counts
    
    Returns:
        class_log_probabilities: Dictionary mapping class names to arrays of word log-probabilities
        vocabulary_dictionary: Dictionary mapping words to indices
    """
    
    # Step 1: Combine all tercets from all classes to build complete vocabulary
    all_training_tercets = []
    for tercets_list in training_tercets_by_class.values():
        all_training_tercets += tercets_list
    
    # Step 2: Build vocabulary dictionary mapping words to indices
    vocabulary_dictionary = build_vocabulary_dictionary(all_training_tercets)
    vocabulary_size = len(vocabulary_dictionary)
    
    # Step 3: Initialize word count arrays for each class with smoothing
    class_log_probabilities = {}
    for class_name in training_tercets_by_class:
        # Initialize with smoothing epsilon to avoid zero probabilities
        class_log_probabilities[class_name] = numpy.zeros(vocabulary_size) + smoothing_epsilon
    
    # Step 4: Count word occurrences for each class
    for class_name, tercets_for_class in training_tercets_by_class.items():
        for tercet in tercets_for_class:
            words = tercet.split()
            for word in words:
                if word in vocabulary_dictionary:
                    word_index = vocabulary_dictionary[word]
                    class_log_probabilities[class_name][word_index] += 1
    
    # Step 5: Convert counts to log-probabilities
    for class_name in class_log_probabilities.keys():
        word_counts = class_log_probabilities[class_name]
        total_word_count = word_counts.sum()
        
        # Calculate log-probabilities: log(count / total_count)
        class_log_probabilities[class_name] = numpy.log(word_counts) - numpy.log(total_word_count)
    
    return class_log_probabilities, vocabulary_dictionary
```

### Expected Output

This function returns two key components of the trained Multinomial model:
- `class_log_probabilities`: Dictionary mapping each class name to an array of word log-probabilities
- `vocabulary_dictionary`: Dictionary mapping each unique word to its index in the probability arrays

---

## Task 3: Class-Conditional Log-Likelihood Calculation

### Task Objective

Compute the **class-conditional log-likelihood matrix `S`** for a given set of validation tercets. In this matrix, `S[c, t]` specifically represents the log-likelihood $log f(x_t | c)$, which is the likelihood of tercet $x_t$ given that it belongs to class $c$.

### Step-by-Step Reasoning (Method 2 - Using Arrays)

The log-likelihood of a document `y` given class `c` in a Multinomial model is approximated by the sum of the log-probabilities of its words, weighted by their counts:
$$
\log f(y|c) \approx \sum_j y_j \cdot \log \pi_{c,j}
$$
This can be efficiently computed as a dot product.

1.  **Tercet to Count Vector:** Use the `S2_tercet2occurrencies` helper function to convert each tercet string into a word count **column vector** `v` (of shape `M x 1`). Any words not found in the vocabulary (`hWordDict`) should be ignored.
2.  **Parameter Matrix Construction:** Build a matrix `MParameters`. This matrix will have dimensions `N_classes x M`, where each row `c` contains the $log \pi_{c,j}$ values for all words `j` in vocabulary, specific to class `c`.
3.  **Matrix-Vector Product Calculation:** For each validation tercet, perform a matrix-vector product: `STercet = numpy.dot(MParameters, v_tercet_counts)`. This operation `((N_classes x M) @ (M x 1))` will yield a `(N_classes x 1)` column vector. Each element `STercet[c, 0]` within this vector represents the $log f(tercet | c)$ (i.e., the log-likelihood of the current tercet given class `c`).
4.  **Stack Results:** Collect all individual `STercet` column vectors into a list. Finally, use `numpy.hstack()` to horizontally stack these vectors, forming the complete `S` matrix with dimensions `(N_classes x N_tercets)`.

#### Solution Code (`commedia.py` - Method 2 functions)

```python
import numpy

def convert_tercet_to_word_counts(tercet, vocabulary_dictionary):
    """
    Convert a tercet string into a column vector of word counts.
    
    Args:
        tercet: String containing the tercet text
        vocabulary_dictionary: Dictionary mapping words to indices
    
    Returns:
        word_count_vector: Column vector (M x 1) where M is vocabulary size
    """
    vocabulary_size = len(vocabulary_dictionary)
    word_count_vector = numpy.zeros(vocabulary_size)
    
    words = tercet.split()
    for word in words:
        if word in vocabulary_dictionary:
            word_index = vocabulary_dictionary[word]
            word_count_vector[word_index] += 1
    
    # Convert to column vector (reshape to M x 1)
    return word_count_vector.reshape((vocabulary_size, 1))

def compute_log_likelihood_matrix(class_log_probabilities, vocabulary_dictionary, 
                                evaluation_tercets, class_to_index_mapping=None):
    """
    Compute the matrix of class-conditional log-likelihoods.
    
    Args:
        class_log_probabilities: Dictionary mapping class names to word log-probability arrays
        vocabulary_dictionary: Dictionary mapping words to indices
        evaluation_tercets: List of tercet strings to evaluate
        class_to_index_mapping: Optional mapping from class names to matrix row indices
    
    Returns:
        log_likelihood_matrix: Matrix of shape (num_classes x num_tercets)
                             where element [c, t] is log P(tercet_t | class_c)
    """
    
    # Create consistent class-to-index mapping if not provided
    if class_to_index_mapping is None:
        sorted_class_names = sorted(class_log_probabilities.keys())
        class_to_index_mapping = {class_name: idx for idx, class_name in enumerate(sorted_class_names)}
    
    num_classes = len(class_log_probabilities)
    vocabulary_size = len(vocabulary_dictionary)
    
    # Step 1: Build parameter matrix where each row contains log-probabilities for one class
    parameter_matrix = numpy.zeros((num_classes, vocabulary_size))
    for class_name in class_log_probabilities:
        class_index = class_to_index_mapping[class_name]
        parameter_matrix[class_index, :] = class_log_probabilities[class_name]
    
    # Step 2: Compute log-likelihoods for each tercet
    tercet_scores = []
    for tercet in evaluation_tercets:
        # Convert tercet to word count vector
        word_count_vector = convert_tercet_to_word_counts(tercet, vocabulary_dictionary)
        
        # Compute log-likelihood for this tercet: matrix-vector multiplication
        # (num_classes x vocabulary_size) @ (vocabulary_size x 1) = (num_classes x 1)
        tercet_log_likelihoods = numpy.dot(parameter_matrix, word_count_vector)
        tercet_scores.append(tercet_log_likelihoods)
    
    # Step 3: Stack all tercet scores horizontally to form final matrix
    log_likelihood_matrix = numpy.hstack(tercet_scores)
    
    return log_likelihood_matrix
```

### Expected Output

This function returns the log-likelihood matrix with shape `(num_classes x num_evaluation_tercets)`, where:
- Each row corresponds to one class
- Each column corresponds to one evaluation tercet
- Element `[c, t]` contains the log-likelihood of tercet `t` given class `c`

---

## Task 4: Multiclass Classification and Evaluation

### Task Objective

Perform **multiclass classification** for the given tercets by following these steps:
1.  Compute **posterior probabilities** for each class, utilizing the class-conditional log-likelihoods and assuming **uniform class priors**.
2.  Make **predictions** for each tercet by selecting the class with the highest posterior probability.
3.  Evaluate the classifier's performance by calculating both the **overall accuracy** and **per-class accuracy**.

### Step-by-Step Reasoning

1.  **Class Priors**: Since no specific prior information is given, assume uniform class priors. For three classes, $P(c) = 1/3$ for each class.
2.  **Posterior Probabilities**: Utilize a dedicated function (e.g., `compute_classPosteriors`, potentially adapted from Laboratory 6). This function should take the log-likelihood matrix `S` (obtained from Task 3) and the `log` of the class priors as input. It is crucial that this function correctly implements the **log-sum-exp trick** to maintain numerical stability during the computation of posterior probabilities.
3.  **Predictions**: For each tercet, the predicted class label is derived by finding the index corresponding to the maximum value in its posterior probability distribution. This is achieved using `PredictedLabel = numpy.argmax(Posteriors, axis=0)`.
4.  **True Labels**: Create a NumPy array, `labelsEval`, that accurately reflects the true class labels for all tercets in `lTercetsEval`. This array should correspond to the order defined by `hCls2Idx`.
5.  **Overall Accuracy**: Calculate the **overall classification accuracy** by comparing the `PredictedLabel` array against `labelsEval`. The `compute_accuracy` function can be used for this.
6.  **Per-Class Accuracy**: To determine the accuracy for each individual class, iterate through each class. For the current class, create a boolean mask to isolate its true labels within `labelsEval`. Apply this mask to both the `Posteriors` matrix and `labelsEval` to create subsets specific to that class. Then, apply the `compute_accuracy` function to these class-specific subsets.

#### Solution Code (`commedia.py` - main block, multiclass part)

```python
import numpy
import scipy.special  # For numerical stability in log-sum-exp

def convert_to_column_vector(array):
    """Convert 1D array to column vector."""
    return array.reshape((array.size, 1))

def convert_to_row_vector(array):
    """Convert 1D array to row vector."""
    return array.reshape((1, array.size))

def compute_posterior_probabilities(log_likelihood_matrix, log_class_priors):
    """
    Compute posterior probabilities from log-likelihoods and log-priors.
    
    Args:
        log_likelihood_matrix: Matrix of log P(data | class) with shape (num_classes x num_samples)
        log_class_priors: Array of log P(class) with shape (num_classes,)
    
    Returns:
        posterior_probabilities: Matrix of P(class | data) with shape (num_classes x num_samples)
    """
    
    # Step 1: Compute joint log-probabilities: log P(data, class) = log P(data | class) + log P(class)
    log_joint_probabilities = log_likelihood_matrix + convert_to_column_vector(log_class_priors)
    
    # Step 2: Compute marginal log-probabilities: log P(data) = log sum_c P(data, class)
    # Use logsumexp for numerical stability
    log_marginal_probabilities = convert_to_row_vector(
        scipy.special.logsumexp(log_joint_probabilities, axis=0)
    )
    
    # Step 3: Compute posterior log-probabilities: log P(class | data) = log P(data, class) - log P(data)
    log_posterior_probabilities = log_joint_probabilities - log_marginal_probabilities
    
    # Step 4: Convert from log-space to actual probabilities
    posterior_probabilities = numpy.exp(log_posterior_probabilities)
    
    return posterior_probabilities

def compute_classification_accuracy(posterior_matrix, true_labels):
    """
    Compute classification accuracy given posterior probabilities and true labels.
    
    Args:
        posterior_matrix: Matrix of posterior probabilities (num_classes x num_samples)
        true_labels: Array of true class labels (integers)
    
    Returns:
        accuracy: Fraction of correctly classified samples
    """
    
    # Get predicted class labels (index of maximum posterior probability)
    predicted_labels = numpy.argmax(posterior_matrix, axis=0)
    
    # Count correct predictions
    correct_predictions = (predicted_labels == true_labels).sum()
    total_samples = true_labels.size
    
    # Calculate accuracy as fraction of correct predictions
    accuracy = correct_predictions / float(total_samples)
    
    return accuracy

# --- Main Multiclass Classification Pipeline ---

# Step 1: Prepare data structures
# Organize training data by class
training_tercets_by_class = {
    'inferno': inferno_train, 
    'purgatorio': purgatorio_train, 
    'paradiso': paradiso_train
}

# Combine all evaluation tercets in order
all_evaluation_tercets = inferno_eval + purgatorio_eval + paradiso_eval

# Create mapping from class names to integer indices
class_to_index = {'inferno': 0, 'purgatorio': 1, 'paradiso': 2}

# Step 2: Create true label array for evaluation data
inferno_labels = numpy.full(len(inferno_eval), class_to_index['inferno'], dtype=int)
purgatorio_labels = numpy.full(len(purgatorio_eval), class_to_index['purgatorio'], dtype=int)
paradiso_labels = numpy.full(len(paradiso_eval), class_to_index['paradiso'], dtype=int)
true_evaluation_labels = numpy.hstack([inferno_labels, purgatorio_labels, paradiso_labels])

print("\n--- Multiclass Classification ---")

# Step 3: Train the Multinomial model
trained_model, vocabulary_dict = estimate_multinomial_model(training_tercets_by_class, smoothing_epsilon=0.001)

# Step 4: Compute class-conditional log-likelihoods for evaluation data
log_likelihood_matrix = compute_log_likelihood_matrix(
    trained_model, vocabulary_dict, all_evaluation_tercets, class_to_index
)

# Step 5: Set uniform class priors (equal probability for each class)
uniform_log_priors = numpy.log(numpy.ones(3) / 3.0)

# Step 6: Compute posterior probabilities
posterior_probabilities = compute_posterior_probabilities(log_likelihood_matrix, uniform_log_priors)

# Step 7: Evaluate overall accuracy
overall_accuracy = compute_classification_accuracy(posterior_probabilities, true_evaluation_labels)
print('Overall Multiclass Accuracy: %.2f%%' % (overall_accuracy * 100))

# Step 8: Evaluate per-class accuracy
print('\nPer-Class Multiclass Accuracy:')
for class_name, class_index in class_to_index.items():
    # Create mask to select only samples from current class
    current_class_mask = (true_evaluation_labels == class_index)
    
    # Compute accuracy for samples belonging to the current class only
    class_posteriors = posterior_probabilities[:, current_class_mask]
    class_true_labels = true_evaluation_labels[current_class_mask]
    class_accuracy = compute_classification_accuracy(class_posteriors, class_true_labels)
    
    print(f'  {class_name.capitalize()} Accuracy: {class_accuracy * 100:.2f}%')
```

#### Expected Output

```
--- Multiclass Classification ---
Overall Multiclass Accuracy: 52.67%

Per-Class Multiclass Accuracy:
  Inferno Accuracy: 53.23%
  Purgatorio Accuracy: 48.34%
  Paradiso Accuracy: 57.08%
```

---

## Task 5: Binary Classification Tasks

### Task Objective

Perform **binary classification** for three specific pairs of Cantiche. For each pair, the task is to distinguish between them and report the resulting classification accuracy. The pairs are:
1.  **Inferno vs. Paradiso**
2.  **Inferno vs. Purgatorio**
3.  **Purgatorio vs. Paradiso**

### Step-by-Step Reasoning

For each of the specified binary pairs, the following steps must be executed:

1.  **Isolate Data**: Create dedicated binary subsets for both training and evaluation. This involves generating `hlTercetsTrain_bin` (containing only the training tercets for the two chosen classes), `lTercetsEval_bin` (the evaluation tercets for these two classes), `hCls2Idx_bin` (a mapping for the two classes to 0 and 1 labels), and `labelsEval_bin` (the corresponding true 0/1 labels for the evaluation set).
2.  **Retrain Model**: It is crucial to train a *new*, pair-specific Multinomial Naive Bayes model. Call `S2_estimateModel` using only the `hlTercetsTrain_bin` data to obtain `S2_model_bin` and `S2_wordDict_bin`. This ensures the model's vocabulary and probabilities are tailored to the specific binary task.
3.  **Compute Log-Likelihoods**: Use the newly trained binary model to compute the class-conditional log-likelihoods for the binary evaluation tercets. Call `S2_compute_logLikelihoodMatrix` with `S2_model_bin`, `S2_wordDict_bin`, `lTercetsEval_bin`, and `hCls2Idx_bin` to get `S_Eval_bin`.
4.  **Compute Posteriors**: Calculate the posterior probabilities for the binary task. Use `compute_classPosteriors` with `S_Eval_bin` and uniform binary priors, specifically `numpy.log(numpy.array([0.5, 0.5]))`, to obtain `BinaryPosteriors`.
5.  **Evaluate Accuracy**: Finally, evaluate the performance of the binary classifier by calling `compute_accuracy` with `BinaryPosteriors` and `labelsEval_bin`.
6.  **Repeat**: Systematically repeat these steps for the remaining two binary pairs.

#### Solution Code (`commedia.py` - main block, binary part)

```python
import numpy
import scipy.special  # For numerical stability

# --- Binary Classification Task 1: Inferno vs Paradiso ---
print('\n--- Binary Classification: Inferno vs Paradiso ---')

# Step 1: Prepare binary datasets and mappings
binary_class_mapping_inf_par = {'inferno': 0, 'paradiso': 1}
binary_training_data_inf_par = {'inferno': inferno_train, 'paradiso': paradiso_train}
binary_evaluation_tercets_inf_par = inferno_eval + paradiso_eval

# Step 2: Create binary true labels
inferno_binary_labels = numpy.full(len(inferno_eval), binary_class_mapping_inf_par['inferno'], dtype=int)
paradiso_binary_labels = numpy.full(len(paradiso_eval), binary_class_mapping_inf_par['paradiso'], dtype=int)
binary_true_labels_inf_par = numpy.hstack([inferno_binary_labels, paradiso_binary_labels])

# Step 3: Train binary model specifically for Inferno vs Paradiso
binary_model_inf_par, binary_vocab_inf_par = estimate_multinomial_model(
    binary_training_data_inf_par, smoothing_epsilon=0.001
)

# Step 4: Compute log-likelihoods for binary classification
binary_log_likelihood_inf_par = compute_log_likelihood_matrix(
    binary_model_inf_par, binary_vocab_inf_par, binary_evaluation_tercets_inf_par, binary_class_mapping_inf_par
)

# Step 5: Compute posterior probabilities with uniform binary priors
binary_uniform_priors = numpy.log(numpy.array([1./2., 1./2.]))
binary_posteriors_inf_par = compute_posterior_probabilities(binary_log_likelihood_inf_par, binary_uniform_priors)

# Step 6: Evaluate binary classification accuracy
binary_accuracy_inf_par = compute_classification_accuracy(binary_posteriors_inf_par, binary_true_labels_inf_par)
print(f'  Binary [Inferno vs Paradiso] Accuracy: {binary_accuracy_inf_par * 100:.2f}%')


# --- Binary Classification Task 2: Inferno vs Purgatorio ---
print('\n--- Binary Classification: Inferno vs Purgatorio ---')

# Step 1: Prepare binary datasets and mappings
binary_class_mapping_inf_pur = {'inferno': 0, 'purgatorio': 1}
binary_training_data_inf_pur = {'inferno': inferno_train, 'purgatorio': purgatorio_train}
binary_evaluation_tercets_inf_pur = inferno_eval + purgatorio_eval

# Step 2: Create binary true labels
inferno_binary_labels_2 = numpy.full(len(inferno_eval), binary_class_mapping_inf_pur['inferno'], dtype=int)
purgatorio_binary_labels = numpy.full(len(purgatorio_eval), binary_class_mapping_inf_pur['purgatorio'], dtype=int)
binary_true_labels_inf_pur = numpy.hstack([inferno_binary_labels_2, purgatorio_binary_labels])

# Step 3: Train binary model specifically for Inferno vs Purgatorio
binary_model_inf_pur, binary_vocab_inf_pur = estimate_multinomial_model(
    binary_training_data_inf_pur, smoothing_epsilon=0.001
)

# Step 4: Compute log-likelihoods for binary classification
binary_log_likelihood_inf_pur = compute_log_likelihood_matrix(
    binary_model_inf_pur, binary_vocab_inf_pur, binary_evaluation_tercets_inf_pur, binary_class_mapping_inf_pur
)

# Step 5: Compute posterior probabilities with uniform binary priors
binary_posteriors_inf_pur = compute_posterior_probabilities(binary_log_likelihood_inf_pur, binary_uniform_priors)

# Step 6: Evaluate binary classification accuracy
binary_accuracy_inf_pur = compute_classification_accuracy(binary_posteriors_inf_pur, binary_true_labels_inf_pur)
print(f'  Binary [Inferno vs Purgatorio] Accuracy: {binary_accuracy_inf_pur * 100:.2f}%')


# --- Binary Classification Task 3: Purgatorio vs Paradiso ---
print('\n--- Binary Classification: Purgatorio vs Paradiso ---')

# Step 1: Prepare binary datasets and mappings
binary_class_mapping_pur_par = {'purgatorio': 0, 'paradiso': 1}
binary_training_data_pur_par = {'purgatorio': purgatorio_train, 'paradiso': paradiso_train}
binary_evaluation_tercets_pur_par = purgatorio_eval + paradiso_eval

# Step 2: Create binary true labels
purgatorio_binary_labels_2 = numpy.full(len(purgatorio_eval), binary_class_mapping_pur_par['purgatorio'], dtype=int)
paradiso_binary_labels_2 = numpy.full(len(paradiso_eval), binary_class_mapping_pur_par['paradiso'], dtype=int)
binary_true_labels_pur_par = numpy.hstack([purgatorio_binary_labels_2, paradiso_binary_labels_2])

# Step 3: Train binary model specifically for Purgatorio vs Paradiso
binary_model_pur_par, binary_vocab_pur_par = estimate_multinomial_model(
    binary_training_data_pur_par, smoothing_epsilon=0.001
)

# Step 4: Compute log-likelihoods for binary classification
binary_log_likelihood_pur_par = compute_log_likelihood_matrix(
    binary_model_pur_par, binary_vocab_pur_par, binary_evaluation_tercets_pur_par, binary_class_mapping_pur_par
)

# Step 5: Compute posterior probabilities with uniform binary priors
binary_posteriors_pur_par = compute_posterior_probabilities(binary_log_likelihood_pur_par, binary_uniform_priors)

# Step 6: Evaluate binary classification accuracy
binary_accuracy_pur_par = compute_classification_accuracy(binary_posteriors_pur_par, binary_true_labels_pur_par)
print(f'  Binary [Purgatorio vs Paradiso] Accuracy: {binary_accuracy_pur_par * 100:.2f}%')
```

#### Expected Output

```
--- Binary Classification: Inferno vs Paradiso ---
  Binary [Inferno vs Paradiso] Accuracy: 75.03%

--- Binary Classification: Inferno vs Purgatorio ---
  Binary [Inferno vs Purgatorio] Accuracy: 61.68%

--- Binary Classification: Purgatorio vs Paradiso ---
  Binary [Purgatorio vs Paradiso] Accuracy: 65.33%
```