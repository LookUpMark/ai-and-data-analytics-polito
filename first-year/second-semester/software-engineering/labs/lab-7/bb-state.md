# Lab 7: Black Box Testing - Unit Level (Modules with State)

**Objective:**
The goal of this lab is to practice black box testing techniques on small software modules that **maintain internal state**. For each of the following modules, define test cases by applying:

1.  Equivalence Class Partitioning
2.  Boundary Value Analysis

Your test cases will likely involve sequences of operations to observe state changes and interactions.

**Instructions:**

*   Use the specified documentation structure to outline your test cases.
*   Clearly define the criteria (input conditions, parameters, or relevant aspects of the module's state).
*   For each criterion, identify the relevant conditions or partitions.
*   Develop test cases (often sequences of calls) for each partition, including boundary conditions.
*   This lab must be completed individually (not in teams).

---

### Documentation Structure

Your documentation for each exercise should follow this structure:

#### 1. Criteria Definition:

Identify and describe the input parameters, operations, or relevant aspects of the module's state that will be used as a basis for testing.

<p align="center">

| Criterion ID | Description                                                                 |
| :----------- | :-------------------------------------------------------------------------- |
| Criterion 1  | C1 (e.g., State of the queue: empty, not empty, full)                     |
| Criterion 2  | C2 (e.g., Input to `Push()` operation: valid event, invalid event)         |
| ...          | ...                                                                         |

</p>

#### 2. Predicates (Conditions for each Criterion):

For each criterion, define the distinct logical conditions or states.

<p align="center">

| Criterion ID | Predicate                        |
| :----------- | :------------------------------- |
| Criterion1   | Queue is Empty                   |
|              | Queue is Not Empty and Not Full  |
|              | Queue is Full (if applicable)    |
| Criterion2   | Input event time tag < current min |
|              | Input event time tag > current max |
| ...          | ...                              |

</p>

#### 3. Boundaries:

Identify specific boundary states or input values.

<p align="center">

| Criterion   | Boundary Condition                               |
| :---------- | :----------------------------------------------- |
| C1          | Queue has 0 elements                             |
|             | Queue has 1 element                              |
|             | Queue has (Max Capacity - 1) elements (if appl.) |
|             | Queue has Max Capacity elements (if appl.)       |
| C2 (input value) | Input value at min/max allowed range        |

</p>

#### 4. Equivalence Classes and Test Cases (Sequences):

Test cases will often be sequences of operations. Describe the initial state, the sequence of operations, inputs, and expected outcomes/state changes.

<p align="center">

| Test Case ID | Initial State | Sequence of Operations & Inputs                                                                | Expected Outcome / Final State / Return Value | Valid/Invalid Interaction | Notes (B for Boundary) |
| :----------- | :------------ | :--------------------------------------------------------------------------------------------- | :-------------------------------------------- | :------------------------ | :--------------------- |
| T1           | Empty Queue   | 1. `Push(eventA, time=10)`                                                                     | Queue contains (A,10)                         | Valid                     |                        |
| T2           | Queue: (A,10)| 1. `Push(eventB, time=5)`<br/>2. `Pop()`                                                         | Returns (B,5). Queue contains (A,10)        | Valid                     | Order of pop matters   |
| T3B          | Full Queue    | 1. `Push(eventC, time=15)`                                                                     | Error or specific behavior (e.g., no change)  | Potentially Invalid       | Boundary: Push to full |
| ...          | ...           | ...                                                                                            | ...                                           | ...                       | ...                    |

</p>

---

### Exercise 1: Event Queue

#### Module Description:

A module manages a queue of events. Each event has an associated "time tag" (presumably an integer representing priority or time, where lower is "earlier" or higher priority).

*   It is possible to extract events from the queue.
*   The extraction (pop) operation must **return the event with the lower time tag**.

#### Functions:

1.  `Reset()`: Resets the queue to an empty state.
2.  `Push(event_data, time_tag)`: Adds an event with its data and time tag to the queue. (Assume `event_data` can be a simple identifier or string for testing purposes).
3.  `Pop()`: Extracts and returns the event with the lowest time tag. If multiple events have the same lowest time tag, the order of their extraction might be undefined or FIFO based on insertion of those specific items (clarify if needed, assume any is fine for now). If the queue is empty, `Pop()` should return a specific indicator (e.g., null, error, special event).

#### Solution for Exercise 1:

##### 1. Criteria Definition:

<p align="center">

| Criterion ID | Description                                     |
| :----------- | :---------------------------------------------- |
| C1           | State of the queue                              |
| C2           | Operation being performed                       |
| C3           | Characteristics of event being pushed (time tag)  |
| C4           | Number of events with the same lowest time tag  |

</p>

##### 2. Predicates:

<p align="center">

| Criterion ID | Predicate                                        |
| :----------- | :----------------------------------------------- |
| C1           | Queue is Empty                                   |
|              | Queue is Not Empty (contains one or more events) |
| C2           | `Reset()`                                        |
|              | `Push()`                                         |
|              | `Pop()`                                          |
| C3           | Time tag of new event vs. existing events' tags  |
|              |   - New is lower than all existing               |
|              |   - New is higher than all existing              |
|              |   - New is between existing                      |
|              |   - New is same as an existing lowest            |
|              |   - New is same as an existing non-lowest        |
| C4           | Single event with lowest time tag                |
|              | Multiple events with the same lowest time tag    |

</p>

##### 3. Boundaries:

<p align="center">

| Criterion   | Boundary Condition                                   |
| :---------- | :--------------------------------------------------- |
| C1          | Queue has 0 elements (Empty)                         |
|             | Queue has 1 element                                  |
|             | Queue has N elements (where N is a typical max if defined, or just "many") |
| C3          | Time tag very small (e.g., 0 or min int)             |
|             | Time tag very large (e.g., max int)                  |

</p>

##### 4. Equivalence Classes and Test Cases (Sequences):

*(Assume `Pop()` returns `null` or similar if empty. For simplicity, `event_data` will be 'E1', 'E2', etc.)*

<p align="center">

| TC ID | Initial State     | Sequence of Operations & Inputs                      | Expected Outcome / Return / Final State       | Notes (B for Boundary)      |
| :---- | :---------------- | :--------------------------------------------------- | :-------------------------------------------- | :-------------------------- |
| **Reset Operations** |
| T1.1  | Empty             | 1. `Reset()`                                         | Queue remains Empty                           | Basic reset empty           |
| T1.2  | Contains (E1,10)  | 1. `Reset()`                                         | Queue becomes Empty                           | Reset non-empty             |
| **Push Operations** |
| T2.1  | Empty             | 1. `Push(E1, 10)`                                    | Queue: {(E1,10)}                              | Push to empty               |
| T2.2  | Contains (E1,10)  | 1. `Push(E2, 5)` (lower time tag)                    | Queue: {(E2,5), (E1,10)}                      | Push new lowest             |
| T2.3  | Contains (E1,10)  | 1. `Push(E2, 15)` (higher time tag)                  | Queue: {(E1,10), (E2,15)}                      | Push new highest            |
| T2.4  | Contains (E1,5), (E2,15) | 1. `Push(E3, 10)` (intermediate time tag)            | Queue: {(E1,5), (E3,10), (E2,15)}              | Push intermediate           |
| T2.5  | Contains (E1,10)  | 1. `Push(E2, 10)` (same time tag)                    | Queue: {(E1,10), (E2,10)} or {(E2,10), (E1,10)} | Push same time tag          |
| **Pop Operations** |
| T3.1B | Empty             | 1. `Pop()`                                           | Returns `null` (or error indicator)           | Pop from empty (Boundary)   |
| T3.2B | Contains (E1,10)  | 1. `Pop()`                                           | Returns (E1,10). Queue: Empty                 | Pop last element (Boundary) |
| T3.3  | Contains (E1,5), (E2,10) | 1. `Pop()`                                           | Returns (E1,5). Queue: {(E2,10)}              | Pop lowest of multiple      |
| T3.4  | Contains (E1,10), (E2,10), (E3,5) | 1. `Pop()`                                           | Returns (E3,5). Queue: {(E1,10), (E2,10)}     | Pop lowest (distinct)       |
| T3.5  | Contains (E1,5), (E2,5), (E3,10) | 1. `Pop()`                                           | Returns (E1,5) *or* (E2,5). Queue: {(Eremaining,5), (E3,10)} | Pop one of same lowest    |
| T3.6  | Contains (E1,5), (E2,5) | 1. `Pop()` <br/> 2. `Pop()` <br/> 3. `Pop()`         | 1. (E1,5) or (E2,5) <br/> 2. Other (E,5) <br/> 3. `null` | Pop all same lowest, then empty |
| **Combined Operations** |
| T4.1  | Empty             | 1. `Push(E1,20)` <br/> 2. `Push(E2,10)` <br/> 3. `Pop()` | Returns (E2,10). Queue: {(E1,20)}             | Push, Push, Pop             |
| T4.2  | Contains (E1,10),(E2,20) | 1. `Push(E3,5)` <br/> 2. `Pop()` <br/> 3. `Pop()`      | 1. Q:{(E3,5),(E1,10),(E2,20)} <br/> 2. Ret:(E3,5). Q:{(E1,10),(E2,20)} <br/> 3. Ret:(E1,10). Q:{(E2,20)} | Interleaved Push/Pop        |
| T4.3  | Contains (E1,10)  | 1. `Pop()` <br/> 2. `Push(E2,5)` <br/> 3. `Pop()`      | 1. Ret:(E1,10). Q:Empty <br/> 2. Q:{(E2,5)} <br/> 3. Ret:(E2,5). Q:Empty | Pop to empty, then Push/Pop |
| T4.4  | Empty             | 1. `Push(E1,100)` <br/> 2. `Reset()` <br/> 3. `Pop()` | Returns `null`. Queue: Empty                  | Push, Reset, Pop empty      |

</p>

*(Note: The "Queue:" in Expected Outcome describes the logical order for popping, not necessarily the internal storage order unless specified.)*

---

### Exercise 2: Inventory Management System

#### Module Description:

A retail support system manages an inventory of items. Each item has a descriptor (itemCode, description, name) and the number of available items (quantity).

#### Functions/Constructor:

1.  `public Item(String itemCode, int quantity, String description, String name);`
    *   Creates a new item in the inventory.
    *   **Assumptions:** `itemCode` is unique. `quantity` is non-negative.
2.  `void subtractQtyToItem(String itemCode, int qty_to_sub) throws ItemNotExists, ItemNotAvailable;`
    *   Subtracts `qty_to_sub` from the quantity of the item identified by `itemCode`.
    *   Throws `ItemNotExists` if no item with `itemCode` is found.
    *   Throws `ItemNotAvailable` if `qty_to_sub` is greater than the current quantity of the item (i.e., not enough stock), or if `qty_to_sub` is negative.

#### Solution for Exercise 2:

##### 1. Criteria Definition:

<p align="center">

| Criterion ID | Description                                       |
| :----------- | :------------------------------------------------ |
| C1           | State of the inventory regarding a specific itemCode |
| C2           | Operation being performed                         |
| C3           | Parameters for `Item` constructor                 |
| C4           | Parameters for `subtractQtyToItem`                |
| C5           | Quantity of item vs. quantity to subtract       |

</p>

##### 2. Predicates:

<p align="center">

| Criterion ID | Predicate                                              |
| :----------- | :----------------------------------------------------- |
| C1           | Item with `itemCode` exists in inventory             |
|              | Item with `itemCode` does NOT exist in inventory       |
| C2           | Constructor `Item()`                                   |
|              | `subtractQtyToItem()`                                  |
| C3           | `itemCode` for constructor (unique, duplicate)         |
|              | `quantity` for constructor (>=0, <0)                   |
| C4           | `itemCode` for subtract (exists, not exists)           |
|              | `qty_to_sub` for subtract (positive, zero, negative)   |
| C5           | `item.quantity >= qty_to_sub` (sufficient stock)     |
|              | `item.quantity < qty_to_sub` (insufficient stock)    |

</p>

##### 3. Boundaries:

<p align="center">

| Criterion   | Boundary Condition                                                |
| :---------- | :---------------------------------------------------------------- |
| C3 (qty)    | `quantity = 0` (for constructor)                                  |
| C4 (qty_sub)| `qty_to_sub = 0`                                                  |
|             | `qty_to_sub = 1` (smallest positive subtraction)                  |
|             | `qty_to_sub = item.quantity` (subtract exact available quantity)  |
|             | `qty_to_sub = item.quantity + 1` (subtract just more than available) |

</p>

##### 4. Equivalence Classes and Test Cases (Sequences):

*(Item state will be represented as `(itemCode, quantity)` for brevity. Exceptions are expected outcomes.)*

<p align="center">

| TC ID | Initial Inventory State | Sequence of Operations & Inputs                                                                 | Expected Outcome / Final State / Exception        | Notes (B for Boundary)           |
| :---- | :---------------------- | :---------------------------------------------------------------------------------------------- | :------------------------------------------------ | :------------------------------- |
| **Constructor Operations** |
| T1.1  | Empty                   | 1. `new Item("A001", 10, "Desc1", "Name1")`                                                      | Inventory: {("A001", 10)}                          | Basic item creation              |
| T1.2B | Empty                   | 1. `new Item("A002", 0, "Desc2", "Name2")`                                                       | Inventory: {("A002", 0)}                          | Create item with zero quantity (B) |
| T1.3  | Empty                   | 1. `new Item("A003", -5, "Desc3", "Name3")`                                                      | Error or Qty becomes 0 (depends on constructor logic for invalid input) | Create item with negative qty  |
| T1.4  | {("A001", 10)}           | 1. `new Item("A001", 5, "NewDesc", "NewName")`                                                  | Error: ItemCode duplicate, or overwrites (specify behavior) | Duplicate itemCode             |
| **subtractQtyToItem Operations** |
| T2.1  | {("A001", 10)}           | 1. `subtractQtyToItem("A001", 3)`                                                               | Inventory: {("A001", 7)}                           | Valid subtraction                |
| T2.2B | {("A001", 10)}           | 1. `subtractQtyToItem("A001", 10)`                                                              | Inventory: {("A001", 0)}                           | Subtract exact quantity (B)      |
| T2.3B | {("A001", 10)}           | 1. `subtractQtyToItem("A001", 0)`                                                               | Inventory: {("A001", 10)}                          | Subtract zero quantity (B)       |
| T2.4  | {("A001", 10)}           | 1. `subtractQtyToItem("A001", 15)`                                                              | Throws `ItemNotAvailable`                         | Insufficient stock               |
| T2.5B | {("A001", 10)}           | 1. `subtractQtyToItem("A001", 11)`                                                              | Throws `ItemNotAvailable`                         | Insufficient stock (Boundary)    |
| T2.6  | {("A001", 10)}           | 1. `subtractQtyToItem("A001", -2)`                                                              | Throws `ItemNotAvailable` (or other error for negative qty_to_sub) | Subtract negative quantity     |
| T2.7  | {("A001", 10)}           | 1. `subtractQtyToItem("B001", 5)`                                                               | Throws `ItemNotExists`                            | Item code does not exist         |
| T2.8B | {("A001", 0)}            | 1. `subtractQtyToItem("A001", 1)`                                                               | Throws `ItemNotAvailable`                         | Subtract from zero stock (B)     |
| **Combined Sequences** |
| T3.1  | Empty                   | 1. `new Item("C001", 20, "D", "N")` <br/> 2. `subtractQtyToItem("C001", 5)` <br/> 3. `subtractQtyToItem("C001", 10)` | Final Inventory: {("C001", 5)}                    | Create, subtract, subtract       |
| T3.2  | Empty                   | 1. `new Item("D001", 5, "D", "N")` <br/> 2. `subtractQtyToItem("D001", 3)` <br/> 3. `subtractQtyToItem("D001", 3)`  | Step 3 throws `ItemNotAvailable`. Inv: {("D001",2)} | Create, subtract, subtract to fail |
| T3.3  | Empty                   | 1. `subtractQtyToItem("E001", 5)`                                                               | Throws `ItemNotExists`                            | Subtract non-existent item first |

</p>

---

### Exercise 3: `computeFee` Function (Railway Offer)

*(This exercise was solved in the previous response. The structure of the solution would be similar to Exercise 1 and 2 above, focusing on criteria like `n_passengers`, `n_over18`, `n_under15`, and derived conditions like group size validity and offer eligibility. Test cases would cover valid groups qualifying for the offer, valid groups not qualifying, and invalid group sizes.)*

For Exercise 3, the key criteria and partitions revolve around:

*   `n_passengers`: $<2$, $2..5$, $>5$. Boundaries at 1, 2, 5, 6.
*   `n_over18`: $0$, $>=1$. Boundaries at 0, 1.
*   The interaction between these to determine if the offer applies (group size 2-5 AND `n_over18 >= 1`).
*   The number of `n_under15` to see how many get free travel if the offer applies.
*   Consistency: $ n\_over18 + n\_under15 \le n\_passengers $.

Test cases would then be constructed by picking combinations of these partitions and boundary values, as shown in the previous detailed solution for Exercise 3.