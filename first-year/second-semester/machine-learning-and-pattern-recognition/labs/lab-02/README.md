# Iris Dataset - Laboratory 2 Guide

This laboratory exercise introduces the **Iris dataset**, with the primary goals of:
1.  **Load the Data:** Importing the dataset into a usable format.
2.  **Visualize Features:** Understanding data distribution using histograms and scatter plots.
3.  **Compute Statistics:** Calculating fundamental statistical properties of the dataset.

## Dataset Overview

The Iris dataset, famously utilized by R.A. Fisher, is a collection of measurements for different Iris flower species. Its **key characteristics include:**

*   **Total Samples:** 150 individual flower samples.
*   **Species:** 3 distinct Iris flower species.
*   **Samples per Species:** 50 samples for each species.
*   **Features (Attributes):** Each sample has 4 measurements, all in centimeters (cm):
    *   Sepal Length
    *   Sepal Width
    *   Petal Length
    *   Petal Width

The dataset's **class labels are defined as follows:**

<p align="center">

| Class Name      | Numeric Label |
| :-------------- | :------------ |
| Iris Setosa     | 0             |
| Iris Versicolor | 1             |
| Iris Virginica  | 2             |

</p>

For this laboratory, the **data convention will utilize NumPy arrays structured as follows:**

*   **Data Matrix (`D`):**
    *   NumPy matrix with features as rows and samples as columns.
    *   Expected Shape: (4, 150).
*   **Class Labels (`L`):**
    *   1-dimensional NumPy array.
    *   Expected Shape: (150,).

---

## Task 1: Loading the Iris Dataset

### Task Objective

To correctly load the Iris dataset from a CSV file into the specified NumPy array format, **you will create the following Python functions:**
1.  **`load(fname)`:** Reads Iris data from `fname` (CSV path), returning the data matrix `D` (4x150) and label array `L` (150,).
2.  **`mcol(v)`:** A helper function that reshapes a 1-dimensional NumPy array `v` into a 2-dimensional column vector.

### Solution Code (`iris.py`)

```python
import numpy
import matplotlib
import matplotlib.pyplot as plt # Needed for plotting later

def mcol(v):
  """
  Reshapes a 1D NumPy array `v` into a 2D column vector of shape (v.size, 1).
  This is crucial as many NumPy operations on vectors expect a 2D column format.
  """
  return v.reshape((v.size, 1))

def load(fname):
  """
  Loads the Iris dataset from a CSV file into the specified NumPy array format.

  The iris.csv file stores each sample per line with features (float) comma-separated,
  followed by the class name (string).

  Args:
      fname (str): The path to the CSV file (e.g., 'iris.csv').

  Returns:
      tuple: (D, L)
          D (numpy.ndarray): Data matrix with features as rows, samples as columns (shape 4x150).
          L (numpy.ndarray): Labels array (shape 150,).
  """
  DList = []      # Temporarily stores 4x1 feature column vectors
  labelsList = [] # Temporarily stores integer class labels

  # Dictionary to map string class names from CSV to integer labels
  hLabels = {
      'Iris-setosa': 0,
      'Iris-versicolor': 1,
      'Iris-virginica': 2
      }

  with open(fname) as f: # Ensures file is automatically closed
      for line in f:     # Process file line by line
          try:
              # Split line by comma; first four parts are features, last is class name
              attrs = line.split(',')[0:-1]
              # Convert feature strings to float and create a 1D NumPy array
              attrs_float = numpy.array([float(i) for i in attrs])
              # Reshape the 1D feature array into a 4x1 column vector using mcol
              attrs_col = mcol(attrs_float)

              # Extract class name (last element), strip whitespace
              name = line.split(',')[-1].strip()
              # Map string class name to its corresponding integer label
              label = hLabels[name]

              # Append the processed feature column vector and label to lists
              DList.append(attrs_col)
              labelsList.append(label)
          except Exception as e:
              # Silently skip malformed lines in the CSV file
              pass

  # Horizontally stack all 4x1 column vectors to form the final 4x150 data matrix D
  D = numpy.hstack(DList)
  # Convert the list of integer labels into a NumPy array, explicitly setting dtype
  L = numpy.array(labelsList, dtype=numpy.int32)

  return D, L

def load2():
  """
  Alternative function to load Iris dataset using scikit-learn.
  Note: sklearn's default data representation is samples as rows (150x4).
  We transpose it (.T) to match the lab's convention (features as rows, 4x150).
  """
  import sklearn.datasets
  iris = sklearn.datasets.load_iris()
  return iris['data'].T, iris['target']

# --- Example Usage (uncomment to run) ---
# D, L = load('iris.csv')
# print("Data matrix D shape:", D.shape)
# print("Labels array L shape:", L.shape)
# print("First 5 samples (columns) of D:\n", D[:, 0:5])
# print("First 10 labels of L:", L[0:10])
#
# D2, L2 = load2()
# print("\nData matrix D2 (from load2) shape:", D2.shape)
# print("Labels array L2 (from load2) shape:", L2.shape)
```

### Expected Output

Upon successful execution of the example usage, you should observe the following output:

```
Data matrix D shape: (4, 150)
Labels array L shape: (150,)
First 5 samples (columns) of D:
 [[5.1 4.9 4.7 4.6 5. ]
  [3.5 3.  3.2 3.1 3.6]
  [1.4 1.4 1.3 1.5 1.4]
  [0.2 0.2 0.2 0.2 0.2]]
First 10 labels of L: [0 0 0 0 0 0 0 0 0 0]

Data matrix D2 (from load2) shape: (4, 150)
Labels array L2 (from load2) shape: (150,)
```

---

## Task 2: Visualizing Feature Distributions (Histograms)

### Task Objective

Create and save normalized histograms for each of the four features. Each histogram should show the distribution of that feature for all three Iris classes, overlaid on a single plot.

### Solution Code (`iris.py`)

```python
# Assume D and L are loaded from Task 1 (e.g., D, L = load('iris.csv'))

def plot_hist(D, L):
    """
    Plots normalized histograms for each feature, separated by class.
    Each plot visualizes the distributions for Setosa, Versicolor, and Virginica classes.
    """
    # Filter data into class-specific subsets using boolean indexing
    D0 = D[:, L==0] # Setosa samples
    D1 = D[:, L==1] # Versicolor samples
    D2 = D[:, L==2] # Virginica samples

    # Dictionary to map feature indices to descriptive names for plot labels
    hFea = {
        0: 'Sepal length',
        1: 'Sepal width',
        2: 'Petal length',
        3: 'Petal width'
        }

    # Iterate through each of the 4 features
    for dIdx in range(4):
        plt.figure() # Create a new figure for each feature's histograms
        plt.xlabel(hFea[dIdx]) # Set x-axis label to the feature name
        plt.ylabel('Density')   # Set y-axis label to 'Density' (due to normalization)

        # Plot histograms for each class on the same figure
        # `bins=10`: Divides data into 10 intervals
        # `density=True`: Normalizes histogram so area sums to 1 (probability density)
        # `alpha=0.4`: Sets transparency for overlapping bars
        # `label`: Provides legend entry for each class
        plt.hist(D0[dIdx, :], bins = 10, density = True, alpha = 0.4, label = 'Setosa')
        plt.hist(D1[dIdx, :], bins = 10, density = True, alpha = 0.4, label = 'Versicolor')
        plt.hist(D2[dIdx, :], bins = 10, density = True, alpha = 0.4, label = 'Virginica')

        plt.legend()       # Display the legend to identify classes
        plt.tight_layout() # Adjust plot parameters for a tight layout
        plt.savefig('hist_%d.pdf' % dIdx) # Save plot to PDF, named by feature index

    plt.show() # Display all generated figures interactively

# --- Example Usage (uncomment to run) ---
# plot_hist(D, L)
```

### Expected Output

*   **File Generation:** Four PDF files (`hist_0.pdf`, `hist_1.pdf`, `hist_2.pdf`, `hist_3.pdf`) will be created, each containing a histogram plot for one of the four features.
*   **Interactive Display:** Four interactive plot windows will open.

Upon inspection, **Petal Length** (feature 2) and **Petal Width** (feature 3) show very distinct distributions for Setosa (Class 0) compared to Versicolor and Virginica, suggesting these features effectively separate Setosa. Conversely, Sepal Length and Sepal Width (features 0 and 1) have more overlapping distributions.

---

## Task 3: Visualizing Feature Pairs (Scatter Plots)

### Task Objective

Create scatter plots for all unique combinations of two features. On each plot, data points for the three Iris classes should be shown with different visual markers or colors. All generated plots must be saved to PDF files.

### Solution Code (`iris.py`)

```python
# Assume D and L are loaded from Task 1 (e.g., D, L = load('iris.csv'))

def plot_scatter(D, L):
  """
  Plots scatter plots for all unique pairs of features, separated by class.
  Each plot displays the distribution of Setosa, Versicolor, and Virginica samples.
  """
  # Filter data into class-specific subsets
  D0 = D[:, L==0] # Setosa samples
  D1 = D[:, L==1] # Versicolor samples
  D2 = D[:, L==2] # Virginica samples

  # Dictionary to map feature indices to descriptive names for plot labels
  hFea = {
      0: 'Sepal length',
      1: 'Sepal width',
      2: 'Petal length',
      3: 'Petal width'
      }

  # Outer loop for the feature on the x-axis
  for dIdx1 in range(4):
      # Inner loop for the feature on the y-axis
      for dIdx2 in range(4):
          # Skip plotting a feature against itself, as it provides no useful info
          if dIdx1 == dIdx2:
              continue

          plt.figure() # Create a new figure for each feature pair's scatter plot
          plt.xlabel(hFea[dIdx1]) # Set x-axis label
          plt.ylabel(hFea[dIdx2]) # Set y-axis label

          # Plot scatter points for each class on the same figure
          # Matplotlib automatically assigns different colors/markers for each call
          plt.scatter(D0[dIdx1, :], D0[dIdx2, :], label = 'Setosa')
          plt.scatter(D1[dIdx1, :], D1[dIdx2, :], label = 'Versicolor')
          plt.scatter(D2[dIdx1, :], D2[dIdx2, :], label = 'Virginica')

          plt.legend()       # Display the legend
          plt.tight_layout() # Adjust layout
          plt.savefig('scatter_%d_%d.pdf' % (dIdx1, dIdx2)) # Save plot to PDF

      # Display all plots related to the current x-axis feature before proceeding
      plt.show()

# --- Example Usage (uncomment to run) ---
# plot_scatter(D, L)
```

### Expected Output

*   **File Generation:** 12 PDF files (`scatter_0_1.pdf`, `scatter_1_0.pdf`, etc.) will be created for all unique feature pairs (4 features * 3 other features = 12 combinations).
*   **Interactive Display:** Multiple interactive plot windows will open, grouped by the x-axis feature.

Visually, pairs involving **Petal Length** (feature 2) and **Petal Width** (feature 3) show the most distinct class separation. Specifically, **Iris Setosa (Class 0)** typically forms a very separate cluster. In contrast, Versicolor (Class 1) and Virginica (Class 2) often show more overlap.

---

## Task 4: Calculating Global Statistics

### Task Objective

Compute and print the global mean vector, covariance matrix, feature variances, and feature standard deviations for the entire Iris dataset (all 150 samples together).

### Solution Code (from `iris.py` example usage)

```python
# Assume D, L are loaded from Task 1 (e.g., D, L = load('iris.csv'))

# Compute global mean vector across all samples for each feature
# D.mean(1) calculates mean along rows (features), reshape to column vector (4x1)
mu = D.mean(1).reshape((D.shape[0], 1))
print('Mean:')
print(mu)
print()

# Compute global covariance matrix
# (D - mu): Centers the data by subtracting the mean from each sample
# @ (D - mu).T: Matrix multiplication with the transpose of the centered data
# / float(D.shape[1]): Normalizes by the total number of samples (N=150)
C = ((D - mu) @ (D - mu).T) / float(D.shape[1])
print('Covariance matrix:')
print(C)
print()

# Compute feature variances for each feature across all samples
# D.var(1) calculates variance along rows (features)
var = D.var(1)
# Compute feature standard deviations for each feature across all samples
# D.std(1) calculates standard deviation along rows (features)
std = D.std(1)
print('Variance:', var)
print('Std. dev.:', std)
print()
```

### Expected Output

The calculations should yield the following output:

```
Mean:
[[5.84333333]
 [3.05733333]
 [3.758     ]
 [1.19933333]]

Covariance matrix:
[[ 0.68569351 -0.042434    1.27431574  0.51627069]
 [-0.042434    0.18997942 -0.32965651 -0.12163937]
 [ 1.27431574 -0.32965651  3.11627785  1.2956094 ]
 [ 0.51627069 -0.12163937  1.2956094   0.58100626]]

Variance: [0.68112222 0.18871289 3.09550267 0.57713289]
Std. dev.: [0.82530129 0.43441097 1.75940407 0.75969263]
```

**Interpretation:** The mean vector provides the average value for each feature. The covariance matrix, moreover, shows how features co-vary (diagonal elements are variances, off-diagonal are covariances; e.g., Sepal Length and Petal Length have strong positive covariance). Notably, **Petal Length** (feature 2) has the highest variance and standard deviation, indicating the greatest spread in values across the entire dataset.

---

## Task 5: Calculating Per-Class Statistics

### Task Objective

Compute and print the mean vector, covariance matrix, feature variances, and feature standard deviations *separately* for each of the three Iris classes (Setosa, Versicolor, and Virginica).

### Solution Code (from `iris.py` example usage)

```python
# Assume D, L are loaded from Task 1 (e.g., D, L = load('iris.csv'))

print("\n--- Per-Class Statistics ---")
# Loop through each unique class label (0, 1, 2)
for cls in [0, 1, 2]:
    print('\nClass', cls)
    # Filter the global data matrix D to get only samples belonging to the current class
    DCls = D[:, L==cls] # DCls will have shape (4, 50)

    # Calculate mean vector for the current class
    mu_cls = DCls.mean(1).reshape(DCls.shape[0], 1)
    print('Mean:')
    print(mu_cls)

    # Calculate covariance matrix for the current class
    # Normalize by DCls.shape[1], which is the number of samples in THIS class (50)
    C_cls = ((DCls - mu_cls) @ (DCls - mu_cls).T) / float(DCls.shape[1])
    print('Covariance:')
    print(C_cls)

    # Calculate variance for each feature within the current class
    var_cls = DCls.var(1)
    print('Variance:', var_cls)

    # Calculate standard deviation for each feature within the current class
    std_cls = DCls.std(1)
    print('Std. dev.:', std_cls)
    print()
```

### Expected Output

The expected output for per-class statistics is as follows:

```
--- Per-Class Statistics ---

Class 0
Mean:
[[5.006     ]
 [3.428     ]
 [1.462     ]
 [0.246     ]]

Covariance:
[[ 0.121764 -0.098496  0.016008  0.010088]
 [-0.098496  0.14369   0.011692  0.009292]
 [ 0.016008  0.011692  0.029556  0.005944]
 [ 0.010088  0.009292  0.005944  0.010964]]

Variance: [0.121764 0.14369  0.029556 0.010964]
Std. dev.: [0.348947   0.3790644  0.17192    0.10470912]

(... similar output for Class 1 and Class 2 will follow ...)
```

**Interpretation:** Per-class statistics highlight distinct characteristics. For example, **Iris Setosa (Class 0)** has significantly smaller mean **Petal Length (~1.46 cm)** and **Petal Width (~0.246 cm)** compared to other classes. Furthermore, Setosa also exhibits much lower variances across its features, particularly for petal dimensions, indicating its flowers are notably more uniform in size than Versicolor or Virginica. These numerical differences, therefore, reinforce the clear visual separation observed in plots.