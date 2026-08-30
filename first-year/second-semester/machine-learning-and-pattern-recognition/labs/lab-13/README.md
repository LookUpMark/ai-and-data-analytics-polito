# Laboratory 13: Score Calibration and Score Fusion

This guide for **Laboratory 13** focuses on **Score Calibration** and **Score Fusion**, two critical techniques in machine learning model evaluation and improvement. You will learn to transform raw classifier scores into interpretable, calibrated Log-Likelihood Ratios (LLRs) and combine predictions from multiple classifiers to enhance overall performance. The laboratory employs both single-fold and K-fold cross-validation strategies for robust training and validation of calibration models.

## Introduction to Score Calibration

Raw classifier scores are often not directly interpretable as true probabilities or Log-Likelihood Ratios (LLRs). **Score calibration** addresses this by transforming these `s_raw` scores into `s_cal` (calibrated LLRs). This transformation is crucial as it enables **optimal decision-making across varied application scenarios** by ensuring the scores accurately reflect the likelihood of class membership. For this laboratory, calibration will be performed using a **logistic regression model trained on the raw scores** themselves, defining a simple linear transformation: $f_cal(s_raw) = α ⋅ s_raw + β$.

**Validation and Evaluation Sets:**
It is vital to distinguish between two types of datasets used in this lab:
*   **Validation Set:** Comprises `scores_*.npy` and `labels.npy`. This set is used internally for hyperparameter tuning and cross-validation of calibration models.
*   **Evaluation Set:** Consists of `eval_scores_*.npy` and `eval_labels.npy`. This set is strictly reserved for the final, unbiased assessment of the fully trained and calibrated/fused models.

---

## Task 1: Initial Analysis of Raw Classifier Scores

### Task Objective

Analyze the raw, uncalibrated scores from two distinct systems: System 1 (`Data/scores_1.npy`) and System 2 (`Data/scores_2.npy`). These scores are provided for the validation set (`Data/labels.npy`). For each system, you will compute `minDCF` and `actDCF` at a target prior $π_T = 0.2$ with uniform costs ($C_{fn} = 1.0$, $C_{fp} = 1.0$). Additionally, you will generate Bayes error plots to visualize their performance.

### Step-by-Step Reasoning

1.  **Load Data:** Begin by loading the raw scores for System 1 (`scores_sys_1`), System 2 (`scores_sys_2`), and their corresponding true labels (`labels`).
2.  **Understand Metrics:** Recall that `minDCF` quantifies a classifier's intrinsic discriminative power, representing the lowest possible cost achievable by optimally setting a threshold. In contrast, `actDCF` measures the actual performance when a specific, theoretically optimal threshold is applied. The difference `actDCF - minDCF` can thus reveal potential calibration loss (i.e., how much performance is lost due to miscalibration).
3.  **Calculate DCF:** Utilize the `bayesRisk.compute_minDCF_binary_fast` and `bayesRisk.compute_actDCF_binary_fast` functions for both System 1 and System 2. Ensure to pass the specified target prior `prior_target=0.2` and uniform costs `cfn=1.0`, `cfp=1.0`.
4.  **Generate Bayes Error Plots:** Employ the `bayesPlot` helper function. This function will generate the necessary data points (`log_odds`, `actDCF_values`, `minDCF_values`) over a range of prior log-odds. Finally, plot the `minDCF` curves (typically dashed lines) and `actDCF` curves (typically solid lines) for both systems on a single Matplotlib figure to facilitate comparison.

#### Solution Code (`sol_initial_analysis.py`)

```python
import numpy
import bayesRisk
import matplotlib.pyplot as plt

# Helper function to generate data for Bayes error plots
def bayesPlot(S, L, left = -3, right = 3, npts = 21):
    """
    Generates data points for a Bayes error plot (normalized DCF vs. prior log-odds).

    Args:
        S (numpy.ndarray): Scores (LLRs) for the classifier.
        L (numpy.ndarray): True binary labels.
        left (float): Left boundary of prior log-odds range.
        right (float): Right boundary of prior log-odds range.
        npts (int): Number of points to generate in the range.

    Returns:
        tuple: (effPriorLogOdds, actDCF_values, minDCF_values)
    """
    effPriorLogOdds = numpy.linspace(left, right, npts)
    effPriors = 1.0 / (1.0 + numpy.exp(-effPriorLogOdds)) # Convert log-odds to priors
    actDCF_values = []
    minDCF_values = []
    for effPrior in effPriors:
        # Calculate actual DCF and minimum DCF for each effective prior
        actDCF_values.append(bayesRisk.compute_actDCF_binary_fast(S, L, effPrior, 1.0, 1.0))
        minDCF_values.append(bayesRisk.compute_minDCF_binary_fast(S, L, effPrior, 1.0, 1.0))
    return effPriorLogOdds, actDCF_values, minDCF_values

if __name__ == '__main__':
    try: # Attempt to load data from the expected path
        scores_sys_1 = numpy.load('Data/scores_1.npy')
        scores_sys_2 = numpy.load('Data/scores_2.npy')
        labels = numpy.load('Data/labels.npy')
    except FileNotFoundError: # Fallback for alternative directory structure
        scores_sys_1 = numpy.load('../Data/scores_1.npy')
        scores_sys_2 = numpy.load('../Data/scores_2.npy')
        labels = numpy.load('../Data/labels.npy')

    prior_target, cfn, cfp = 0.2, 1.0, 1.0 # Define target application parameters

    print('--- Task 1: Initial Analysis of Raw Classifier Scores ---')
    print('Analysis on Full Validation Set:')

    # Calculate and print DCF metrics for System 1
    minDCF_sys1 = bayesRisk.compute_minDCF_binary_fast(scores_sys_1, labels, prior_target, cfn, cfp)
    actDCF_sys1 = bayesRisk.compute_actDCF_binary_fast(scores_sys_1, labels, prior_target, cfn, cfp)
    print(f'  System 1: minDCF ($\\pi_T$={prior_target}) = {minDCF_sys1:.3f} - actDCF ($\\pi_T$={prior_target}) = {actDCF_sys1:.3f}')

    # Calculate and print DCF metrics for System 2
    minDCF_sys2 = bayesRisk.compute_minDCF_binary_fast(scores_sys_2, labels, prior_target, cfn, cfp)
    actDCF_sys2 = bayesRisk.compute_actDCF_binary_fast(scores_sys_2, labels, prior_target, cfn, cfp)
    print(f'  System 2: minDCF ($\\pi_T$={prior_target}) = {minDCF_sys2:.3f} - actDCF ($\\pi_T$={prior_target}) = {actDCF_sys2:.3f}')

    # Generate and display Bayes error plots
    plt.figure(figsize=(9, 6))
    plt.title('Bayes Error Plots (Raw Scores)')

    # Plot curves for System 1
    logOdds1, actDCF1, minDCF1 = bayesPlot(scores_sys_1, labels)
    plt.plot(logOdds1, minDCF1, color='red', linestyle='--', label='minDCF (System 1)')
    plt.plot(logOdds1, actDCF1, color='red', linestyle='-', label='actDCF (System 1)')

    # Plot curves for System 2
    logOdds2, actDCF2, minDCF2 = bayesPlot(scores_sys_2, labels)
    plt.plot(logOdds2, minDCF2, color='blue', linestyle='--', label='minDCF (System 2)')
    plt.plot(logOdds2, actDCF2, color='blue', linestyle='-', label='actDCF (System 2)')

    plt.xlabel('Prior Log-Odds (log($\\pi_T$/(1-$ \\pi_T $)))')
    plt.ylabel('Normalized DCF')
    plt.ylim([0, 0.8]) # Set y-axis limit for better comparison
    plt.legend() # Display legend to identify curves
    plt.grid(True, linestyle='--', alpha=0.6) # Add a grid for readability
    # plt.show() # Uncomment to display the plot interactively
```

#### Expected Output

```
--- Task 1: Initial Analysis of Raw Classifier Scores ---
Analysis on Full Validation Set:
  System 1: minDCF (pi_T=0.2) = 0.204 - actDCF (pi_T=0.2) = 0.298
  System 2: minDCF (pi_T=0.2) = 0.311 - actDCF (pi_T=0.2) = 0.328
```

---

## Calibration: Single-Fold Approach

The **Single-Fold Approach** to score calibration involves splitting the available validation data into two distinct parts: one for training the calibration model and the other for validating its performance.

### Task 2: Training and Evaluating a Single-Fold Calibration Model

### Task Objective

Implement and execute a single-fold calibration strategy for System 1. This involves:
1.  Splitting the validation scores and labels (from Task 1) into dedicated calibration training and calibration validation sets (using a 1/3 for training, 2/3 for validation ratio).
2.  Training a logistic regression calibration model for System 1 exclusively on the calibration training scores.
3.  Applying the trained calibration model to the calibration validation scores.
4.  Evaluating the performance of the calibrated scores on this calibration validation set.

### Step-by-Step Reasoning

1.  **Split Data:** Divide `scores_sys_1` and `labels` into two segments. For the calibration training set, select every third sample (e.g., `SCAL = scores_sys_1[::3]`, `LCAL = labels[::3]`). For the calibration validation set, combine the remaining samples (e.g., `SVAL = numpy.hstack([scores_sys_1[1::3], scores_sys_1[2::3]])`, `LVAL = numpy.hstack([labels[1::3], labels[2::3]])`).
2.  **Train Calibration Model:** Utilize `logReg.trainWeightedLogRegBinary`. Pass `logReg.vrow(SCAL)` (scores reshaped to a row vector for feature format), `LCAL` (labels), `lambda=0` (no regularization, as calibration often prefers minimal smoothing), and `pT=0.2` (the target prior for weighting). This function will return the learned calibration parameters `w_cal` and `b_cal`.
3.  **Apply Calibration:** Transform the raw scores of the calibration validation set (`SVAL`) into calibrated scores. The transformation involves a linear operation and an LLR adjustment: `calibrated_SVAL = (w_cal.T @ logReg.vrow(SVAL) + b_cal - numpy.log(target_prior_cal / (1 - target_prior_cal))).ravel()`.
4.  **Evaluate Performance:** On the `calibrated_SVAL` and `LVAL`, compute the `minDCF` (on raw scores for reference) and `actDCF` (on both raw and calibrated scores) using the `bayesRisk` functions. Finally, plot Bayes error curves to visually compare the raw and calibrated performance.

#### Solution Code (`sol_single_fold.py`)

```python
import numpy
import bayesRisk
import logReg # Assuming logReg.py (from Lab 9) is available
import matplotlib.pyplot as plt

# Assume helper functions are defined from previous tasks/labs
# bayesPlot is defined in Task 1.

if __name__ == '__main__':
    # Load raw scores and labels for the validation dataset (from Task 1)
    try:
        scores_sys_1 = numpy.load('Data/scores_1.npy')
        labels = numpy.load('Data/labels.npy')
    except FileNotFoundError:
        scores_sys_1 = numpy.load('../Data/scores_1.npy')
        labels = numpy.load('../Data/labels.npy')

    print('--- Task 2: Single-Fold Calibration Model Training & Evaluation ---')

    # 1. Split the validation data into calibration training (1/3) and calibration validation (2/3)
    SCAL1 = scores_sys_1[::3] # Every 3rd sample for calibration training
    LCAL = labels[::3]
    SVAL1 = numpy.hstack([scores_sys_1[1::3], scores_sys_1[2::3]]) # Remaining 2/3 for validation
    LVAL = numpy.hstack([labels[1::3], labels[2::3]])

    target_prior_cal = 0.2 # Target prior for calibration
    lambda_reg_cal = 0.0 # Regularization parameter for calibration LR (often 0 for calibration)

    print(f'\n  Splitting data for calibration: {SCAL1.size} samples for training, {SVAL1.size} for validation.')
    print(f'  Target calibration prior (pi_T) = {target_prior_cal}, Lambda for calibration = {lambda_reg_cal}')

    # 2. Train the logistic regression calibration model on the calibration training set
    # logReg.vrow(SCAL1) reshapes the scores to a (1, N) row vector as required by logReg.trainWeightedLogRegBinary
    w_cal, b_cal = logReg.trainWeightedLogRegBinary(logReg.vrow(SCAL1), LCAL, lambda_reg_cal, target_prior_cal)

    print(f'\n  Learned calibration parameters: w_cal = {w_cal[0]:.3f}, b_cal = {b_cal:.3f}')

    # 3. Apply the learned transformation to the calibration validation scores
    # This transforms raw scores into calibrated LLRs suitable for the target prior
    calibrated_SVAL1 = (w_cal.T @ logReg.vrow(SVAL1) + b_cal - numpy.log(target_prior_cal / (1 - target_prior_cal))).ravel()

    # --- 4. Evaluate Performance on Calibration Validation Set ---
    print('\n  Performance on Calibration Validation Set (System 1):')

    # Calculate minDCF and actDCF for raw scores
    minDCF_val_raw = bayesRisk.compute_minDCF_binary_fast(SVAL1, LVAL, target_prior_cal, 1.0, 1.0)
    actDCF_val_raw = bayesRisk.compute_actDCF_binary_fast(SVAL1, LVAL, target_prior_cal, 1.0, 1.0)
    # Calculate actDCF for calibrated scores
    actDCF_val_cal = bayesRisk.compute_actDCF_binary_fast(calibrated_SVAL1, LVAL, target_prior_cal, 1.0, 1.0)

    print(f'\tminDCF(p={target_prior_cal})                      : {minDCF_val_raw:.3f}')
    print(f'\tactDCF(p={target_prior_cal}), raw scores        : {actDCF_val_raw:.3f}')
    print(f'\tactDCF(p={target_prior_cal}), calibrated scores : {actDCF_val_cal:.3f}')

    # --- Visualization ---
    plt.figure(figsize=(9, 6))
    plt.title('System 1 - Single-Fold Calibration Validation')

    # Plot Bayes error curves for raw scores
    logOdds_val_raw, actDCF_val_raw_plot, minDCF_val_raw_plot = bayesPlot(SVAL1, LVAL)
    plt.plot(logOdds_val_raw, minDCF_val_raw_plot, color='red', linestyle='--', label='minDCF')
    plt.plot(logOdds_val_raw, actDCF_val_raw_plot, color='red', linestyle=':', label='actDCF (pre-cal.)')

    # Plot actDCF curve for calibrated scores
    # Re-use logOdds from raw plot as the x-axis for plotting, as it's common for this dataset split
    _, actDCF_val_cal_plot, _ = bayesPlot(calibrated_SVAL1, LVAL)
    plt.plot(logOdds_val_raw, actDCF_val_cal_plot, color='red', linestyle='-', label='actDCF (cal.)')

    plt.xlabel('Prior Log-Odds')
    plt.ylabel('Normalized DCF')
    plt.ylim([0, 0.8])
    plt.legend()
    plt.grid(True, linestyle='--', alpha=0.6)
    # plt.show() # Uncomment to display the plot
```

#### Expected Output

```
--- Task 2: Single-Fold Calibration Model Training & Evaluation ---

  Splitting data for calibration: 137 samples for training, 274 for validation.
  Target calibration prior (pi_T) = 0.2, Lambda for calibration = 0.0

  Learned calibration parameters: w_cal = 0.169, b_cal = -1.026

  Performance on Calibration Validation Set (System 1):
	minDCF(p=0.2)                      : 0.182
	actDCF(p=0.2), raw scores        : 0.274
	actDCF(p=0.2), calibrated scores : 0.206
```

### Task 3: Applying the Single-Fold Model to the Evaluation Set

### Task Objective

Apply the single-fold calibration model trained in Task 2 (represented by `w_cal` and `b_cal`) to the **strictly held-out evaluation set** (`Data/eval_scores_1.npy` and `Data/eval_labels.npy`). This step is crucial for obtaining a final, unbiased performance estimate of the fully calibrated system.

### Step-by-Step Reasoning

1.  **Load Evaluation Data:** Load the raw scores (`eval_scores_sys_1`) and true labels (`eval_labels`) specifically designated for final evaluation.
2.  **Apply Calibration:** Use the `w_cal` and `b_cal` parameters (obtained from training on a *portion* of the validation set in Task 2) to transform the `eval_scores_sys_1` into `calibrated_eval_scores`. The transformation is identical to the one used for the calibration validation set.
3.  **Evaluate Performance:** Compute the `minDCF` (on raw evaluation scores for reference) and `actDCF` (on both raw and calibrated evaluation scores) using the `bayesRisk` functions. Plot Bayes error curves for visual comparison.

#### Solution Code (`sol_single_fold.py`)

```python
import numpy
import bayesRisk
import logReg
import matplotlib.pyplot as plt

if __name__ == '__main__':
    # ... (code from Task 1 and Task 2, ensuring w_cal, b_cal, target_prior_cal are defined) ...

    print('\n--- Task 3: Applying Single-Fold Calibration to Evaluation Set ---')

    # Load raw scores and labels for the evaluation dataset
    try:
        eval_scores_sys_1 = numpy.load('Data/eval_scores_1.npy')
        eval_labels = numpy.load('Data/eval_labels.npy')
    except FileNotFoundError:
        eval_scores_sys_1 = numpy.load('../Data/eval_scores_1.npy')
        eval_labels = numpy.load('../Data/eval_labels.npy')

    # Apply the pre-trained calibration model (w_cal, b_cal from Task 2) to the evaluation scores
    calibrated_eval_scores_sys_1 = (w_cal.T @ logReg.vrow(eval_scores_sys_1) + b_cal - numpy.log(target_prior_cal / (1 - target_prior_cal))).ravel()

    # --- Evaluate Performance on Evaluation Set ---
    print('\n  Performance on Evaluation Set (System 1):')

    # Calculate minDCF and actDCF for raw evaluation scores
    minDCF_eval_raw = bayesRisk.compute_minDCF_binary_fast(eval_scores_sys_1, eval_labels, target_prior_cal, 1.0, 1.0)
    actDCF_eval_raw = bayesRisk.compute_actDCF_binary_fast(eval_scores_sys_1, eval_labels, target_prior_cal, 1.0, 1.0)
    # Calculate actDCF for calibrated evaluation scores
    actDCF_eval_cal = bayesRisk.compute_actDCF_binary_fast(calibrated_eval_scores_sys_1, eval_labels, target_prior_cal, 1.0, 1.0)

    print(f'\tminDCF(p={target_prior_cal})                      : {minDCF_eval_raw:.3f}')
    print(f'\tactDCF(p={target_prior_cal}), raw scores        : {actDCF_eval_raw:.3f}')
    print(f'\tactDCF(p={target_prior_cal}), calibrated scores : {actDCF_eval_cal:.3f}')

    # --- Plotting Evaluation Results ---
    plt.figure(figsize=(9, 6))
    plt.title('System 1 - Single-Fold Evaluation')

    # Plot Bayes error curves for raw evaluation scores
    logOdds_eval_raw_plot, actDCF_eval_raw_plot, minDCF_eval_raw_plot = bayesPlot(eval_scores_sys_1, eval_labels)
    plt.plot(logOdds_eval_raw_plot, minDCF_eval_raw_plot, color='red', linestyle='--', label='minDCF')
    plt.plot(logOdds_eval_raw_plot, actDCF_eval_raw_plot, color='red', linestyle=':', label='actDCF (pre-cal.)')

    # Plot actDCF curve for calibrated evaluation scores
    # Re-use logOdds for x-axis consistency
    _, actDCF_eval_cal_plot, _ = bayesPlot(calibrated_eval_scores_sys_1, eval_labels)
    plt.plot(logOdds_eval_raw_plot, actDCF_eval_cal_plot, color='red', linestyle='-', label='actDCF (cal.)')

    plt.xlabel('Prior Log-Odds')
    plt.ylabel('Normalized DCF')
    plt.ylim([0, 0.8])
    plt.legend()
    plt.grid(True, linestyle='--', alpha=0.6)
    # plt.show() # Uncomment to display the plot
```

#### Expected Output

```
--- Task 3: Applying Single-Fold Calibration to Evaluation Set ---

  Performance on Evaluation Set (System 1):
	minDCF(p=0.2)                      : 0.205
	actDCF(p=0.2), raw scores        : 0.290
	actDCF(p=0.2), calibrated scores : 0.225
```

---

## Calibration: K-Fold Approach

The **K-Fold Cross-Validation Approach** provides a more robust and less biased way to evaluate calibration model performance compared to the single-fold approach. It ensures that all available validation data are used for both training and validation of the calibration model, but never in the same fold.

### Task 4: Training and Evaluating with K-Fold Calibration

### Task Objective

Implement a K-fold cross-validation scheme (specifically with `K=5` folds) for score calibration. The process involves:
1.  In each fold, training a new calibration model on the data from `K-1` folds.
2.  Validating the performance of this model on the held-out `i`-th fold.
3.  Pooling all out-of-sample calibrated scores from each fold.
4.  Evaluating the overall performance (minDCF, actDCF) on these pooled, out-of-sample scores.

### Step-by-Step Reasoning

1.  **Define `KFOLD`:** Set the number of folds, e.g., `KFOLD = 5`.
2.  **Helper `extract_train_val_folds_from_ary`:** Create a helper function that takes a full array (`X_full`, representing scores or labels) and a `foldIdx`. It should return two arrays: one containing data for training (all folds except `foldIdx`) and another for validation (only `foldIdx`).
3.  **K-Fold Loop:** Iterate through `foldIdx` from `0` to `KFOLD-1`. For each `foldIdx`:
    *   Split `scores_sys_1` and `labels` using the `extract_train_val_folds_from_ary` helper to obtain `SCAL_fold`, `SVAL_fold`, `LCAL_fold`, and `LVAL_fold`.
    *   Train a new calibration model (`w_fold`, `b_fold`) on `SCAL_fold`/`LCAL_fold` using `logReg.trainWeightedLogRegBinary` (with `lambda=0`, `pT=0.2`).
    *   Apply the calibrated transformation to the held-out `SVAL_fold` to get `calibrated_SVAL_fold`.
    *   Append `calibrated_SVAL_fold` and `LVAL_fold` to `calibrated_scores_list` and `labels_list` respectively.
4.  **Pooling:** After the loop completes, horizontally stack all elements in `calibrated_scores_list` to form `calibrated_scores_pooled` and do the same for `labels_list` to form `labels_pooled`. These pooled arrays contain out-of-sample predictions for all data points.
5.  **Final Evaluation:** Compute `minDCF` and `actDCF` on the `calibrated_scores_pooled` and `labels_pooled`.

#### Solution Code (`sol_kfold.py`)

```python
import numpy
import bayesRisk
import logReg
import matplotlib.pyplot as plt

# Helper functions for K-fold data extraction
KFOLD = 5

def extract_train_val_folds_from_ary(X_full, foldIdx):
    """
    Extracts training and validation data for a specific fold in K-fold cross-validation.

    Args:
        X_full (numpy.ndarray): The full dataset (scores or labels).
        foldIdx (int): The index of the fold to be used as validation.

    Returns:
        tuple: (train_data, val_data)
    """
    train_folds_list = [X_full[jdx::KFOLD] for jdx in range(KFOLD) if jdx != foldIdx]
    train_data = numpy.hstack(train_folds_list) # Concatenate training folds
    val_data = X_full[foldIdx::KFOLD] # Select the validation fold
    return train_data, val_data

if __name__ == '__main__':
    # Load raw scores and labels (from Task 1)
    try:
        scores_sys_1 = numpy.load('Data/scores_1.npy')
        scores_sys_2 = numpy.load('Data/scores_2.npy') # Used later for fusion
        labels = numpy.load('Data/labels.npy')
    except FileNotFoundError:
        scores_sys_1 = numpy.load('../Data/scores_1.npy')
        scores_sys_2 = numpy.load('../Data/scores_2.npy')
        labels = numpy.load('../Data/labels.npy')

    print('--- Task 4: K-Fold Calibration Model Training & Evaluation ---')

    target_prior_cal, lambda_reg_cal = 0.2, 0.0 # Calibration parameters

    calibrated_scores_list = [] # To store calibrated scores from each validation fold
    labels_list = [] # To store corresponding true labels

    # K-Fold Calibration Loop
    for foldIdx in range(KFOLD):
        print(f'  Processing Fold {foldIdx + 1}/{KFOLD}:')

        # 1. Split scores and labels into training and validation sets for the current fold
        SCAL_fold, SVAL_fold = extract_train_val_folds_from_ary(scores_sys_1, foldIdx)
        LCAL_fold, LVAL_fold = extract_train_val_folds_from_ary(labels, foldIdx)

        # 2. Train a new calibration model on the current training folds
        w_fold, b_fold = logReg.trainWeightedLogRegBinary(logReg.vrow(SCAL_fold), LCAL_fold, lambda_reg_cal, target_prior_cal)

        # 3. Apply the trained model to the held-out validation fold (out-of-sample scores)
        calibrated_SVAL_fold = (w_fold.T @ logReg.vrow(SVAL_fold) + b_fold - numpy.log(target_prior_cal / (1 - target_prior_cal))).ravel()

        # 4. Append the calibrated scores and labels from this fold
        calibrated_scores_list.append(calibrated_SVAL_fold)
        labels_list.append(LVAL_fold)

    # Pooling and Final Evaluation
    calibrated_scores_pooled = numpy.hstack(calibrated_scores_list) # Pool all out-of-sample calibrated scores
    labels_pooled = numpy.hstack(labels_list) # Pool all corresponding labels

    print('\n  Performance on Pooled K-Fold Validation Scores (System 1):')

    # Evaluate performance on the pooled scores
    minDCF_pooled = bayesRisk.compute_minDCF_binary_fast(calibrated_scores_pooled, labels_pooled, target_prior_cal, 1.0, 1.0)
    actDCF_pooled = bayesRisk.compute_actDCF_binary_fast(calibrated_scores_pooled, labels_pooled, target_prior_cal, 1.0, 1.0)

    print(f'\tminDCF($\\pi_T$={target_prior_cal}) : {minDCF_pooled:.3f}')
    print(f'\tactDCF($\\pi_T$={target_prior_cal}) : {actDCF_pooled:.3f}')

    # Reference minDCF for the full raw dataset for comparison
    minDCF_raw_full = bayesRisk.compute_minDCF_binary_fast(scores_sys_1, labels, target_prior_cal, 1.0, 1.0)
    print(f'\tReference minDCF (raw full dataset) : {minDCF_raw_full:.3f}')

    plt.figure(figsize=(9, 6))
    plt.title('System 1 - K-Fold Calibration Validation')

    # Plot Bayes error curves for the full raw dataset (reference)
    logOdds_raw_full, actDCF_raw_plot, minDCF_raw_plot = bayesPlot(scores_sys_1, labels)
    plt.plot(logOdds_raw_full, minDCF_raw_plot, color='red', linestyle='--', label='minDCF (raw full)')
    plt.plot(logOdds_raw_full, actDCF_raw_plot, color='red', linestyle=':', label='actDCF (pre-cal. raw full)')

    # Plot actDCF curve for the pooled calibrated scores
    # logOdds_raw_full is used as x-axis for consistency in plotting range.
    _, actDCF_pooled_plot, _ = bayesPlot(calibrated_scores_pooled, labels_pooled)
    plt.plot(logOdds_raw_full, actDCF_pooled_plot, color='red', linestyle='-', label='actDCF (K-fold cal.)')

    plt.xlabel('Prior Log-Odds')
    plt.ylabel('Normalized DCF')
    plt.ylim([0, 0.8])
    plt.legend()
    plt.grid(True, linestyle='--', alpha=0.6)
    # plt.show() # Uncomment to display the plot
```

#### Expected Output

```
--- Task 4: K-Fold Calibration Model Training & Evaluation ---

  Processing Fold 1/5:
  Weighted Log-reg (pi_T=2.0e-01) - lambda=0.0e+00 - J*(w, b)=3.882065e-02
  Processing Fold 2/5:
  Weighted Log-reg (pi_T=2.0e-01) - lambda=0.0e+00 - J*(w, b)=3.978643e-02
  Processing Fold 3/5:
  Weighted Log-reg (pi_T=2.0e-01) - lambda=0.0e+00 - J*(w, b)=4.032158e-02
  Processing Fold 4/5:
  Weighted Log-reg (pi_T=2.0e-01) - lambda=0.0e+00 - J*(w, b)=3.905622e-02
  Processing Fold 5/5:
  Weighted Log-reg (pi_T=2.0e-01) - lambda=0.0e+00 - J*(w, b)=3.953526e-02

  Performance on Pooled K-Fold Validation Scores (System 1):
	minDCF(p=0.2) : 0.204
	actDCF(p=0.2) : 0.219
	Reference minDCF (raw full dataset) : 0.204
```

### Task 5: Applying the K-Fold Model to the Evaluation Set

### Task Objective

To obtain an unbiased final performance estimate, train a single, final calibration model. This model should be trained on the *entire* validation set (`scores_sys_1`, `labels`). Subsequently, apply this fully trained model to the held-out `eval_scores_sys_1.npy` data and report the final unbiased performance metrics.

### Step-by-Step Reasoning

1.  **Train Final Calibration Model:** Train a single logistic regression calibration model on the entirety of the validation set (`scores_sys_1` and `labels`). This will yield the `w_final` and `b_final` parameters for the optimal calibration.
2.  **Load Evaluation Data:** Load the raw scores (`eval_scores_sys_1`) and true labels (`eval_labels`) from the evaluation set.
3.  **Apply Final Model:** Apply the `w_final` and `b_final` (from step 1) to the `eval_scores_sys_1` to obtain the `calibrated_eval_scores`. This transformation is consistent with the methodology.
4.  **Evaluate Performance:** Compute the `minDCF` (on raw evaluation scores for reference) and `actDCF` (on both raw and calibrated evaluation scores) on the evaluation set using the `bayesRisk` functions. Plot Bayes error curves for visual comparison.

#### Solution Code (`sol_kfold.py`)

```python
import numpy
import bayesRisk
import logReg
import matplotlib.pyplot as plt

if __name__ == '__main__':
    # ... (code from Task 1, Task 4 for data loading and K-fold calibration) ...

    print('\n--- Task 5: Applying K-Fold Methodology to Evaluation Set ---')

    # 1. Train the FINAL calibration model on the ENTIRE validation set
    # This model uses all available data from the validation set for training
    w_final, b_final = logReg.trainWeightedLogRegBinary(logReg.vrow(scores_sys_1), labels, lambda_reg_cal, target_prior_cal)

    # Load evaluation data
    try:
        eval_scores_sys_1 = numpy.load('Data/eval_scores_1.npy')
        eval_labels = numpy.load('Data/eval_labels.npy')
    except FileNotFoundError:
        eval_scores_sys_1 = numpy.load('../Data/eval_scores_1.npy')
        eval_labels = numpy.load('../Data/eval_labels.npy')

    # 2. Apply this final, trained calibration model to the raw evaluation scores
    calibrated_eval_scores = (w_final.T @ logReg.vrow(eval_scores_sys_1) + b_final - numpy.log(target_prior_cal / (1 - target_prior_cal))).ravel()

    # --- 3. Evaluate Performance on the Evaluation Set ---
    print('\n  Performance on Evaluation Set (System 1):')

    # Calculate minDCF and actDCF for raw evaluation scores
    minDCF_eval_raw = bayesRisk.compute_minDCF_binary_fast(eval_scores_sys_1, eval_labels, target_prior_cal, 1.0, 1.0)
    actDCF_eval_raw = bayesRisk.compute_actDCF_binary_fast(eval_scores_sys_1, eval_labels, target_prior_cal, 1.0, 1.0)
    # Calculate actDCF for calibrated evaluation scores
    actDCF_eval_cal = bayesRisk.compute_actDCF_binary_fast(calibrated_eval_scores, eval_labels, target_prior_cal, 1.0, 1.0)

    print(f'\tminDCF($\\pi_T$={target_prior_cal})                      : {minDCF_eval_raw:.3f}')
    print(f'\tactDCF($\\pi_T$={target_prior_cal}), raw scores        : {actDCF_eval_raw:.3f}')
    print(f'\tactDCF($\\pi_T$={target_prior_cal}), calibrated scores : {actDCF_eval_cal:.3f}')

    # --- Visualization of Evaluation Results ---
    plt.figure(figsize=(9, 6))
    plt.title('System 1 - K-Fold Evaluation on Held-Out Set')

    # Plot Bayes error curves for raw evaluation scores
    logOdds_eval_raw_plot, actDCF_eval_raw_plot, minDCF_eval_raw_plot = bayesPlot(eval_scores_sys_1, eval_labels)
    plt.plot(logOdds_eval_raw_plot, minDCF_eval_raw_plot, color='red', linestyle='--', label='minDCF (raw)')
    plt.plot(logOdds_eval_raw_plot, actDCF_eval_raw_plot, color='red', linestyle=':', label='actDCF (pre-cal.)')

    # Plot actDCF curve for calibrated evaluation scores
    # Re-use logOdds for x-axis consistency
    _, actDCF_eval_cal_plot, _ = bayesPlot(calibrated_eval_scores, eval_labels)
    plt.plot(logOdds_eval_raw_plot, actDCF_eval_cal_plot, color='red', linestyle='-', label='actDCF (cal.)')

    plt.xlabel('Prior Log-Odds')
    plt.ylabel('Normalized DCF')
    plt.ylim([0, 0.8])
    plt.legend()
    plt.grid(True, linestyle='--', alpha=0.6)
    # plt.show() # Uncomment to display the plot
```

#### Expected Output

```
--- Task 5: Applying K-Fold Methodology to Evaluation Set ---

  Performance on Evaluation Set (System 1):
	minDCF(p=0.2)                      : 0.205
	actDCF(p=0.2), raw scores        : 0.290
	actDCF(p=0.2), calibrated scores : 0.225
```

---

## Task 6: Score-Level Fusion

**Score-level fusion** is a technique that combines scores obtained from multiple distinct classifiers to achieve improved overall classification performance. This laboratory extends the logistic regression framework to perform score-level fusion, by treating the combined scores from individual systems as multivariate features for a new logistic regression model.

### Task Objective

Implement and evaluate the score-level fusion of System 1 and System 2. This will involve two sub-approaches:
1.  **Single-Fold Fusion (Implicit):** This will be an implicit part of the K-fold methodology by considering the training of a single fusion model.
2.  **K-Fold Fusion (Robust Approach):** A more robust cross-validation strategy will be employed for training and evaluating the fusion model.

### Step-by-Step Reasoning (Focus on K-Fold Fusion as the Robust Approach)

1.  **K-Fold Loop:** Iterate through `foldIdx` from `0` to `KFOLD-1`. For each fold:
    *   **Data Extraction:** Split the scores for System 1 (`scores_sys_1`), System 2 (`scores_sys_2`), and the `labels` using the `extract_train_val_folds_from_ary` helper. This yields `SCAL1_fold`, `SCAL2_fold`, `SVAL1_fold`, `SVAL2_fold`, `LCAL_fold`, and `LVAL_fold`.
    *   **Stack Training Scores:** Vertically stack the training scores from System 1 and System 2 to form a 2-dimensional feature matrix for logistic regression: `SCAL_fused_fold = numpy.vstack([SCAL1_fold, SCAL2_fold])`.
    *   **Train Fusion Model:** Train a logistic regression fusion model on `SCAL_fused_fold` and `LCAL_fold` using `logReg.trainWeightedLogRegBinary` (with `lambda=0`, `pT=0.2`). This provides the fold-specific `w_fuse_fold` and `b_fuse_fold`.
    *   **Stack Validation Scores:** Similarly, vertically stack the held-out validation scores: `SVAL_fused_fold = numpy.vstack([SVAL1_fold, SVAL2_fold])`.
    *   **Apply Fusion Model:** Apply the trained fusion model to the stacked validation scores: `calibrated_fused_SVAL = (w_fuse_fold.T @ SVAL_fused_fold + b_fuse_fold - numpy.log(target_prior_fuse / (1.0 - target_prior_fuse))).ravel()`.
    *   **Append Results:** Append the `calibrated_fused_SVAL` and `LVAL_fold` to `fused_scores_kfold_list` and `fused_labels_kfold_list` respectively.
2.  **Pooling:** After the loop, horizontally stack all elements in `fused_scores_kfold_list` to form `fused_scores_pooled` and do the same for `fused_labels_kfold_list` to form `fused_labels_pooled`. These pooled arrays represent the out-of-sample predictions for the entire validation set.
3.  **Evaluate Pooled Performance:** Compute `minDCF` and `actDCF` on `fused_scores_pooled` and `fused_labels_pooled` to get an overall performance estimate of the K-fold fused system on the validation set.
4.  **Final Fusion Model for Evaluation Set:**
    *   Train a single, final fusion model on the *entire* validation dataset (System 1 and System 2 scores stacked together with all validation labels). This yields `w_fuse_final` and `b_fuse_final`.
    *   Load the evaluation scores for both System 1 and System 2 (`eval_scores_sys_1`, `eval_scores_sys_2`, `eval_labels`).
    *   Stack these evaluation scores (`numpy.vstack([eval_scores_sys_1, eval_scores_sys_2])`).
    *   Apply the `w_fuse_final` and `b_fuse_final` to this stacked evaluation data to get the `fused_eval_scores`.
    *   Compute the final `minDCF` and `actDCF` on the `fused_eval_scores` and `eval_labels` to obtain the unbiased performance of the fused system.

#### Solution Code (`sol_kfold.py`)

```python
import numpy
import bayesRisk
import logReg
import matplotlib.pyplot as plt

# Assume helper functions are defined (vrow, extract_train_val_folds_from_ary, bayesPlot)

if __name__ == '__main__':
    # ... (Code from Task 1, 4, 5 for System 1 analysis and K-fold calibration) ...

    print('\n--- Task 6: Score-Level Fusion ---')

    target_prior_fuse = 0.2 # Target prior for fusion model
    lambda_reg_fuse = 0.0 # Regularization for fusion model (often 0)
    KFOLD = 5 # Number of folds for cross-validation

    fused_scores_kfold_list = [] # To store calibrated fused scores from each fold
    fused_labels_kfold_list = [] # To store corresponding labels

    print('\n  Performing K-Fold Fusion:')
    for foldIdx in range(KFOLD):
        print(f'    Processing Fold {foldIdx + 1}/{KFOLD} for Fusion:')

        # 1. Extract scores and labels for current fold (calibration training/validation) for both systems
        SCAL1_fold, SVAL1_fold = extract_train_val_folds_from_ary(scores_sys_1, foldIdx)
        SCAL2_fold, SVAL2_fold = extract_train_val_folds_from_ary(scores_sys_2, foldIdx)
        LCAL_fold, LVAL_fold = extract_train_val_folds_from_ary(labels, foldIdx)

        # 2. Stack scores vertically to form the 2-D feature matrix for Logistic Regression training
        SCAL_fused_fold = numpy.vstack([SCAL1_fold, SCAL2_fold])

        # 3. Train the fusion model on K-1 folds (SCAL_fused_fold is DTR, LCAL_fold is LTR)
        w_fuse_fold, b_fuse_fold = logReg.trainWeightedLogRegBinary(
            SCAL_fused_fold, LCAL_fold, lambda_reg_fuse, target_prior_fuse)

        # 4. Stack scores for the held-out validation fold (SVAL1_fold is DVAL for system 1, etc.)
        SVAL_fused_fold = numpy.vstack([SVAL1_fold, SVAL2_fold])

        # 5. Apply the fusion model to the held-out validation fold (out-of-sample scores)
        # This also includes the LLR adjustment for the target prior
        calibrated_fused_SVAL_fold = (w_fuse_fold.T @ SVAL_fused_fold + b_fuse_fold - \
                                      numpy.log(target_prior_fuse / (1.0 - target_prior_fuse))).ravel()

        # 6. Append calibrated fused scores and labels to lists for pooling
        fused_scores_kfold_list.append(calibrated_fused_SVAL_fold)
        fused_labels_kfold_list.append(LVAL_fold)

    # --- Evaluate on Pooled K-Fold Fused Scores (Validation Set) ---
    fused_scores_pooled = numpy.hstack(fused_scores_kfold_list) # Pool all calibrated scores
    fused_labels_pooled = numpy.hstack(fused_labels_kfold_list) # Pool all labels

    print('\n  Performance on Pooled K-Fold Fused Scores (Validation Set):')

    minDCF_fused_pooled = bayesRisk.compute_minDCF_binary_fast(
        fused_scores_pooled, fused_labels_pooled, target_prior_fuse, 1.0, 1.0)
    actDCF_fused_pooled = bayesRisk.compute_actDCF_binary_fast(
        fused_scores_pooled, fused_labels_pooled, target_prior_fuse, 1.0, 1.0)
    print(f'\tFusion minDCF($\\pi_T$={target_prior_fuse}) : {minDCF_fused_pooled:.3f}')
    print(f'\tFusion actDCF($\\pi_T$={target_prior_fuse}) : {actDCF_fused_pooled:.3f}')

    # --- Final Fusion Model Training and Application to Evaluation Set ---
    print('\n  Applying Final Fusion Model to Evaluation Set:')

    # Train the FINAL fusion model on the ENTIRE validation set for both systems
    SMatrix_full_train = numpy.vstack([scores_sys_1, scores_sys_2])
    w_fuse_final, b_fuse_final = logReg.trainWeightedLogRegBinary(
        SMatrix_full_train, labels, lambda_reg_fuse, target_prior_fuse)

    # Load evaluation data for both systems
    eval_scores_sys_1 = numpy.load('Data/eval_scores_1.npy')
    eval_scores_sys_2 = numpy.load('Data/eval_scores_2.npy')
    eval_labels = numpy.load('Data/eval_labels.npy')

    # Stack evaluation scores to form the feature matrix for the fusion model
    SMatrix_eval_fuse = numpy.vstack([eval_scores_sys_1, eval_scores_sys_2])

    # Apply the final fusion model to the evaluation set
    fused_eval_scores = (w_fuse_final.T @ SMatrix_eval_fuse + b_fuse_final - \
                         numpy.log(target_prior_fuse / (1.0 - target_prior_fuse))).ravel()

    # Evaluate performance on the evaluation set
    minDCF_fused_eval = bayesRisk.compute_minDCF_binary_fast(
        fused_eval_scores, eval_labels, target_prior_fuse, 1.0, 1.0)
    actDCF_fused_eval = bayesRisk.compute_actDCF_binary_fast(
        fused_eval_scores, eval_labels, target_prior_fuse, 1.0, 1.0)
    print(f'\tFusion minDCF($\\pi_T$={target_prior_fuse}) : {minDCF_fused_eval:.3f}')
    print(f'\tFusion actDCF($\\pi_T$={target_prior_fuse}) : {actDCF_fused_eval:.3f}')

    # --- Visualization of Fused System Performance ---
    plt.figure(figsize=(9, 6))
    plt.title('Fused System Performance (K-Fold Evaluation)')

    # Plot Bayes error curves for System 1 (raw) for comparison
    logOdds_raw_full, actDCF_raw_plot, minDCF_raw_plot = bayesPlot(scores_sys_1, labels)
    plt.plot(logOdds_raw_full, minDCF_raw_plot, color='red', linestyle='--', label='minDCF (Sys 1 raw)')
    plt.plot(logOdds_raw_full, actDCF_raw_plot, color='red', linestyle=':', label='actDCF (Sys 1 raw)')

    # Plot Bayes error curves for the Fused system (calibrated)
    logOdds_fused_eval_plot, actDCF_fused_eval_plot, minDCF_fused_eval_plot = bayesPlot(fused_eval_scores, eval_labels)
    plt.plot(logOdds_fused_eval_plot, minDCF_fused_eval_plot, color='blue', linestyle='--', label='minDCF (Fused cal.)')
    plt.plot(logOdds_fused_eval_plot, actDCF_fused_eval_plot, color='blue', linestyle='-', label='actDCF (Fused cal.)')

    plt.xlabel('Prior Log-Odds')
    plt.ylabel('Normalized DCF')
    plt.ylim([0, 0.8])
    plt.legend()
    plt.grid(True, linestyle='--', alpha=0.6)
    # plt.show() # Uncomment to display the plot
```

#### Expected Output

```
--- Task 6: Score-Level Fusion ---

  Performing K-Fold Fusion:
    Processing Fold 1/5 for Fusion:
    Weighted Log-reg (pi_T=2.0e-01) - lambda=0.0e+00 - J*(w, b)=7.893140e-02
    Processing Fold 2/5 for Fusion:
    Weighted Log-reg (pi_T=2.0e-01) - lambda=0.0e+00 - J*(w, b)=8.125950e-02
    Processing Fold 3/5 for Fusion:
    Weighted Log-reg (pi_T=2.0e-01) - lambda=0.0e+00 - J*(w, b)=8.314959e-02
    Processing Fold 4/5 for Fusion:
    Weighted Log-reg (pi_T=2.0e-01) - lambda=0.0e+00 - J*(w, b)=7.940562e-02
    Processing Fold 5/5 for Fusion:
    Weighted Log-reg (pi_T=2.0e-01) - lambda=0.0e+00 - J*(w, b)=8.067332e-02

  Performance on Pooled K-Fold Fused Scores (Validation Set):
	Fusion minDCF(p=0.2) : 0.177
	Fusion actDCF(p=0.2) : 0.185

  Applying Final Fusion Model to Evaluation Set:

  Performance of Final Fusion Model on Evaluation Set:
	Fusion minDCF(p=0.2) : 0.178
	Fusion actDCF(p=0.2) : 0.189
```