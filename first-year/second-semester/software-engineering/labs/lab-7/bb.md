# Lab 7: Black Box Testing - Unit Level

**Objective:**
The purpose of this lab is to practice black box testing techniques on small software modules. For each module described below, you will define test cases by applying:

1.  Equivalence Class Partitioning
2.  Boundary Value Analysis

**Instructions:**

*   Use the specified documentation structure to outline your test cases.
*   Clearly define the criteria (input conditions or parameters).
*   For each criterion, identify the relevant conditions or partitions.
*   Develop test cases for each partition, including boundary conditions.
*   This lab must be completed individually (not in teams).

---

### Documentation Structure

Your documentation for each exercise should follow this structure:

#### 1. Criteria Definition:

Identify and describe the input parameters or conditions that will be used as a basis for testing.

<p align="center">

| Criterion ID | Description (e.g., Name of input parameter, or a condition derived from inputs) |
| :----------- | :----------------------------------------------------------------------------- |
| Criterion 1  | C1 (e.g., `carb` value, `total_calories < 1000`)                               |
| Criterion 2  | C2 (e.g., `protein` value, `(carb + protein) / fat > 0.5`)                     |
| ...          | ...                                                                            |

</p>

#### 2. Predicates (Conditions for each Criterion):

For each criterion, define the distinct logical conditions or ranges that partition its possible values.

<p align="center">

| Criterion ID | Predicate (Logical condition based on the criterion) |
| :----------- | :--------------------------------------------------- |
| Criterion1   | `C1 == true`                                         |
|              | `C1 == false`                                        |
| Criterion2   | `C2 < 0` (Example: if C2 is an input value)          |
|              | `C2 > 0` (Example: if C2 is an input value)          |
| ...          | ...                                                  |

</p>

#### 3. Boundaries:

Identify specific boundary values for relevant criteria.

<p align="center">

| Criterion               | Boundary Condition        |
| :---------------------- | :------------------------ |
| C2                      | `C2 == 0` (Example value) |
| *(Other relevant criteria)* | *(Their boundary values)* |

</p>

#### 4. Equivalence Classes and Test Cases:

Combine the predicates to form equivalence classes and define test cases for each. Indicate if a test case represents a valid or invalid input scenario. Mark boundary test cases with a "B".

<p align="center">

| C1    | C2    | ... | Valid/Invalid | Test Case Description / Values                                          |
| :---- | :---- | :-- | :------------ | :---------------------------------------------------------------------- |
| true  | `< 0` | ... | Valid         | `T1 = (values for C1, C2, ...)`                                         |
| true  | `> 0` | ... | Valid         | `T2 = (...)`                                                            |
| true  | `==0` | ... | Valid         | `T3B = (...)` *(B indicates this is specifically a boundary test case)* |
| false | `< 0` | ... | Invalid       | `T4 = (...)`                                                            |
| false | `> 0` | ... | Valid         | `T5 = (...)`                                                            |

</p>

---

### Exercise 1: `acceptableToEat` Function

**Function Signature:**
`boolean acceptableToEat(int carb, int protein, int fat);`

**Function Description:**
The function `acceptableToEat` receives the weight in grams of carbohydrates (`carb`), proteins (`protein`), and fats (`fat`) in a serving of food. It returns `true` if both of the following conditions are met:

1.  The total amount of calories is less than 1000.
    *   Calories are calculated as: $ total\_calories = carb * 4 + protein * 4 + fat * 9 $.
2.  The ratio $ (carb + protein) / fat $ is greater than $ ½ $ (0.5).
    *   **Assumption:** For this exercise, we will assume `fat` must be greater than 0 for the ratio to be valid and calculable. Test cases where `fat <= 0` will be considered invalid inputs for the ratio part. We also assume `carb`, `protein`, `fat` are non-negative integers.

**Examples:**

*   `acceptableToEat(100, 100, 100)` -> `false` (Calories = 1700)
*   `acceptableToEat(1, 1, 10)` -> `false` (Ratio = 0.2)
*   `acceptableToEat(1, 1, 1)` -> `true` (Calories = 17, Ratio = 2)

**Solution for Exercise 1:**

#### 1. Criteria Definition:

<p align="center">

| Criterion ID | Description                                        | Abbreviation |
| :----------- | :------------------------------------------------- | :----------- |
| C1           | Total amount of calories                           | `TCAL`       |
| C2           | Ratio `(carb + protein) / fat`                     | `RATIO`      |
| C3           | Input `fat` value (for ratio denominator validity) | `FAT_VAL`    |

</p>

#### 2. Predicates:

<p align="center">

| Criterion ID | Predicate                         |
| :----------- | :-------------------------------- |
| C1 (TCAL)    | `TCAL < 1000`                     |
|              | `TCAL >= 1000`                    |
| C2 (RATIO)   | `RATIO > 0.5`                     |
|              | `RATIO <= 0.5`                    |
| C3 (FAT_VAL) | `FAT_VAL > 0` (Valid for ratio)   |
|              | `FAT_VAL <= 0` (Invalid for ratio)|

</p>

#### 3. Boundaries:

<p align="center">

| Criterion   | Boundary Condition                                                                                                |
| :---------- | :---------------------------------------------------------------------------------------------------------------- |
| C1 (TCAL)   | `TCAL = 999` (just below threshold)                                                                               |
|             | `TCAL = 1000` (at threshold)                                                                                      |
|             | `TCAL = 1001` (just above threshold)                                                                              |
| C2 (RATIO)  | `RATIO` approaches `0.5` from above (e.g., 0.51, using `(c+p)/f = 2.55/5 = 0.51`)                                  |
|             | `RATIO = 0.5` (at threshold, using `(c+p)/f = 2.5/5 = 0.5`)                                                       |
|             | `RATIO` approaches `0.5` from below (e.g., 0.49, using `(c+p)/f = 2.45/5 = 0.49`)                                  |
| C3 (FAT_VAL)| `FAT_VAL = 1` (smallest positive integer for valid ratio)                                                         |
|             | `FAT_VAL = 0` (boundary for invalid ratio)                                                                        |

</p>

#### 4. Equivalence Classes and Test Cases:

*(Note: `c`=carb, `p`=protein, `f`=fat. Output is `true` if (TCAL < 1000) AND (RATIO > 0.5) AND (FAT_VAL > 0))*

Final refined test cases for Exercise 1:

<p align="center">

| TC ID | (carb, protein, fat) | TCAL | Ratio       | Output | Notes                               |
| :---- | :------------------- | :--- | :---------- | :----- | :---------------------------------- |
| T1    | (10, 10, 10)         | 170  | 2.0         | true   | Valid, both conditions met          |
| T2    | (100, 100, 100)      | 1700 | 2.0         | false  | Calories too high                   |
| T3    | (1, 1, 10)           | 44   | 0.2         | false  | Ratio too low                       |
| T4    | (1, 1, 0)            | 8    | Undefined   | false  | Invalid input: fat=0                |
| T5B   | (110, 0, 62)         | 998  | 1.77        | true   | Boundary: TCAL just < 1000          |
| T6B   | (100, 25, 50)        | 1000 | 2.5         | false  | Boundary: TCAL == 1000              |
| T7B   | (2, 1, 5)            | 57   | 0.6         | true   | Boundary: RATIO just > 0.5          |
| T8B   | (1, 1, 4)            | 44   | 0.5         | false  | Boundary: RATIO == 0.5              |
| T9B   | (1, 1, 1)            | 17   | 2.0         | true   | Boundary: FAT_VAL = 1               |
| T10   | (200, 50, 10)        | 1450 | 25.0        | false  | Both conditions fail (high values)  |
| T11   | (0, 0, 1)            | 9    | 0.0         | false  | Min valid inputs, ratio too low     |

</p>

---

### Exercise 2: `computeFee` Function (Bicycle Rental)

**Function Signature:**
`double computeFee(int duration, int minRate, int minRate2);`

**Function Description:**
This function computes the fee (in euros) for a bicycle rental.

*   `duration`: Total minutes the bicycle has been used.
*   `minRate`: Cost per minute (cents) for $ 30 < duration \le 90 $.
*   `minRate2`: Cost per minute (cents) for $ duration > 90 $.

Fee calculation:

1.  First 30 minutes: free.
2.  Minutes 31-90: $ (duration - 30) * minRate $ (if $ duration > 30 $).
3.  Minutes > 90: $ (90 - 30) * minRate + (duration - 90) * minRate2 $.

**Assumptions:** `duration >= 0`. `minRate`, `minRate2 >= 0`. Result is in cents.

**Solution for Exercise 2:**

#### 1. Criteria Definition:

<p align="center">

| Criterion ID | Description            | Abbreviation |
| :----------- | :--------------------- | :----------- |
| C1           | `duration` (in minutes)| `DUR`        |

</p>

#### 2. Predicates:

<p align="center">

| Criterion ID | Predicate                         | Fee Calculation Logic Segment |
| :----------- | :-------------------------------- | :---------------------------- |
| C1 (DUR)     | `0 <= DUR <= 30`                  | Free                          |
|              | `30 < DUR <= 90`                  | Uses `minRate`                |
|              | `DUR > 90`                        | Uses `minRate` and `minRate2` |
|              | `DUR < 0` (optional invalid case) | Invalid Input                 |

</p>

#### 3. Boundaries:

<p align="center">

| Criterion | Boundary Condition |
| :-------- | :----------------- |
| C1 (DUR)  | `DUR = 0`          |
|           | `DUR = 1`          |
|           | `DUR = 29`         |
|           | `DUR = 30`         |
|           | `DUR = 31`         |
|           | `DUR = 89`         |
|           | `DUR = 90`         |
|           | `DUR = 91`         |

</p>

#### 4. Equivalence Classes and Test Cases:

*(Using `minRate=10`, `minRate2=20` for consistent examples, fee in cents)*

<p align="center">

| TC ID | Duration (`DUR`) | Predicate Class             | `minRate` | `minRate2` | Expected Fee (cents)         | Notes                                   |
| :---- | :--------------- | :-------------------------- | :-------- | :--------- | :--------------------------- | :-------------------------------------- |
| T1    | 15               | `0 <= DUR <= 30`            | 10        | 20         | `0`                          | Within free period                      |
| T2B   | 0                | `DUR = 0` (Boundary)        | 10        | 20         | `0`                          | Boundary of free period                 |
| T3B   | 30               | `DUR = 30` (Boundary)       | 10        | 20         | `0`                          | Boundary: end of free, start of rate1   |
| T4B   | 31               | `30 < DUR <= 90` (Boundary) | 10        | 20         | `(31-30)*10 = 10`            | Boundary: start of rate1 period         |
| T5    | 60               | `30 < DUR <= 90`            | 10        | 20         | `(60-30)*10 = 300`           | Within rate1 period                     |
| T6B   | 90               | `DUR = 90` (Boundary)       | 10        | 20         | `(90-30)*10 = 600`           | Boundary: end of rate1, start of rate2  |
| T7B   | 91               | `DUR > 90` (Boundary)       | 10        | 20         | `(90-30)*10 + (91-90)*20 = 600 + 20 = 620` | Boundary: start of rate2 period         |
| T8    | 120              | `DUR > 90`                  | 10        | 20         | `(90-30)*10 + (120-90)*20 = 600 + 600 = 1200` | Within rate2 period                     |
| T9    | -5               | `DUR < 0` (optional)        | 10        | 20         | Error / Undefined behavior   | Invalid input                           |

</p>

*(Note: "Option 2 – prune the composition tree if reasonable to do this" is not strictly necessary here as combinations are linear based on `duration`.)*

---

### Exercise 3: `computeFee` Function (Railway Offer)

**Function Signature:**
`double computeFee(double basePrice, int n_passengers, int n_over18, int n_under15);`

**Function Description:**
Railway company offer for groups (2-5 people): under 15s travel free IF at least one member is >= 18 years old. Others pay `basePrice`. Error if group > 5.
**Assumptions:**

*   `basePrice >= 0`.
*   Input counts (`n_passengers`, `n_over18`, `n_under15`) are non-negative.
*   Inputs are consistent: $ n\_over18 + n\_under15 \le n\_passengers $. (If not, this is an implicit error condition or requires clarification).
*   "Error if groups are composed of more than 5 persons" implies $ n\_passengers \le 5 $ is a validity constraint. The offer itself is for groups 2-5. This means $ n\_passengers = 1 $ might be valid but not get the offer, or also an error. We will assume groups < 2 are valid and don't get offer. The primary error is $ n\_passengers > 5 $.
*   "Others pay the Base Price" refers to $ n\_passengers - n\_under15 $ if the offer conditions are met. If offer conditions are not met, all $ n\_passengers $ pay `basePrice`.

**Solution for Exercise 3:**

#### 1. Criteria Definition:

<p align="center">

| Criterion ID | Description                                  | Abbreviation |
| :----------- | :------------------------------------------- | :----------- |
| C1           | Total number of passengers (`n_passengers`)    | `NP`         |
| C2           | Number of passengers over 18 (`n_over18`)    | `N18`        |
| C3           | Number of passengers under 15 (`n_under15`)  | `NU15`       |
| C4           | Group eligible for offer (derived)           | `OFFER_ELIG` |

</p>

#### 2. Predicates:

<p align="center">

| Criterion ID | Predicate                          | Notes                                     |
| :----------- | :--------------------------------- | :---------------------------------------- |
| C1 (NP)      | `NP > 5`                           | Error condition                           |
|              | `2 <= NP <= 5`                     | Valid size for offer eligibility          |
|              | `NP < 2`                           | Valid size, but not eligible for offer    |
| C2 (N18)     | `N18 >= 1`                         | Adult present for offer eligibility       |
|              | `N18 == 0`                         | No adult present                          |
| C4 (OFFER_ELIG)| `OFFER_ELIG` is true               | $ (2 <= NP <= 5) \text{ AND } (N18 >= 1) $           |
|              | `OFFER_ELIG` is false              | Not meeting NP or N18 criteria for offer  |

</p>

#### 3. Boundaries:

<p align="center">

| Criterion | Boundary Condition                                |
| :-------- | :------------------------------------------------ |
| C1 (NP)   | `NP = 1` (below offer range)                      |
|           | `NP = 2` (min for offer)                          |
|           | `NP = 5` (max for offer, max valid)               |
|           | `NP = 6` (error)                                  |
| C2 (N18)  | `N18 = 0` (no adult)                              |
|           | `N18 = 1` (min adult for offer)                   |
| C3 (NU15) | `NU15 = 0` (no under 15s)                         |
|           | `NU15 = 1` (one under 15)                         |
|           | `NU15 = NP - N18` (max possible under 15s if N18 present) |

</p>

#### 4. Equivalence Classes and Test Cases:

*(Using `basePrice = 10.0` for fee calculation)*

<p align="center">

| TC ID | (BP, NP, N18, NU15) | `NP > 5`? | `2 <= NP <= 5`? | `N18 >= 1`? | OFFER_ELIG? | Paying Passengers | Expected Fee | Notes                                                              |
| :---- | :------------------ | :-------- | :-------------- | :---------- | :---------- | :---------------- | :----------- | :----------------------------------------------------------------- |
| T1    | (10, 6, 1, 1)       | Yes       | No              | Yes         | No          | N/A (Error)       | Error        | Group size > 5 (Error)                                             |
| T2B   | (10, 5, 1, 2)       | No        | Yes             | Yes         | Yes         | 5 - 2 = 3         | 30.0         | Max offer size, Adult, U15 free (Matches corrected example)        |
| T3B   | (10, 2, 1, 1)       | No        | Yes             | Yes         | Yes         | 2 - 1 = 1         | 10.0         | Min offer size, Adult, U15 free                                      |
| T4    | (10, 3, 0, 1)       | No        | Yes             | No          | No          | 3                 | 30.0         | Offer size, No Adult, All pay (Matches example `computeFee(20.0, 3, 0, 1) -> 60.0` if BP=20) |
| T5    | (10, 3, 1, 0)       | No        | Yes             | Yes         | Yes         | 3 - 0 = 3         | 30.0         | Offer size, Adult, No U15                                          |
| T6B   | (10, 1, 0, 0)       | No        | No              | No          | No          | 1                 | 10.0         | Group size < 2, No offer, All pay (1 person)                       |
| T7B   | (10, 1, 1, 0)       | No        | No              | Yes         | No          | 1                 | 10.0         | Group size < 2, Adult, No offer, All pay (1 person)                |
| T8    | (10, 4, 2, 2)       | No        | Yes             | Yes         | Yes         | 4 - 2 = 2         | 20.0         | Mid-range offer valid case                                         |
| T9    | (10, 5, 0, 5)       | No        | Yes             | No          | No          | 5                 | 50.0         | Offer size, No adult, all under 15 (but must pay)                  |
| T10   | (10, 2, 2, 0)       | No        | Yes             | Yes         | Yes         | 2 - 0 = 2         | 20.0         | Min offer size, all adults                                         |
| T11B  | (10, 5, 5, 0)       | No        | Yes             | Yes         | Yes         | 5 - 0 = 5         | 50.0         | Max offer size, all adults                                         |
| T12B  | (10, 6, 0, 0)       | Yes       | No              | No          | No          | N/A (Error)       | Error        | Boundary `NP=6` (Error)                                            |

</p>

*(Note: The number of paying passengers calculation: If `OFFER_ELIG` is true, it's $ NP - NU15 $. Otherwise, it's $ NP $.)*
*(The example discrepancy in the problem description (150.0 vs 90.0) for `computeFee(30.0, 5, 1, 2)` is significant. The provided solution tests for a fee of 90.0, assuming the stated rules where under-15s are free if offer conditions (group size 2-5 AND adult present) are met. The 150.0 would imply all 5 passengers pay, contradicting the "under 15s free" part of the offer.)*