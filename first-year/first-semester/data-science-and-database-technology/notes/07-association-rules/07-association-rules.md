---
title: Association Rules
aliases: [Apriori Algorithm, FP-growth, Frequent Itemsets, Market Basket Analysis, Lift]
tags: [computer-science/databases, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> Association rule mining: extraction of frequent correlations from transactional data (market basket, text, structured data). Formalizes itemsets, support and confidence with worked examples, then the two-step mining process: frequent itemset extraction (brute force O(|T|·2^d·w), the Apriori principle and the level-wise Apriori algorithm with a full worked example on 10 transactions, plus FP-growth with FP-tree and conditional pattern bases) and rule generation. Closes with compact representations (maximal, closed itemsets), the lift/correlation measure (basket-vs-cereals example), and generalized/weighted/context-aware extensions with the patient data case study.

## Definitions and Examples

> [!definition] Association rule
> Given a collection of **transactions** (each a set of unordered **items**), an association rule `A, B → C` states a **co-occurrence** (not causality): A, B are the rule **body**, C the rule **head**.

Classic example — tickets at a supermarket counter: the rule *diapers → beer* holds when 2% of transactions contain both items and 30% of transactions containing diapers also contain beer.

| TID | Items |
|---|---|
| 1 | Bread, Coke, Milk |
| 2 | Beer, Bread |
| 3 | Beer, Coke, Diapers, Milk |
| 4 | Beer, Bread, Diapers, Milk |
| 5 | Coke, Diapers, Milk |

Association rule extraction is an **exploratory** technique applicable to any data type where a transaction can be any set of items:

- **Textual data**: a document is a transaction; words are items — `customer, relationship → data, mining`.
- **Structured data**: a table row is a transaction; pairs (attribute, value) are items — `Refund=No, MaritalStatus=Married → Cheat=No`.

## Rule Quality Metrics

| Itemset | a set of one or more items; a **k-itemset** contains k items |
|---|---|
| **Support count** # | frequency of occurrence of an itemset, e.g. #{Beer, Diapers} = 2 |
| **Support** | fraction of transactions containing the itemset, e.g. sup({Beer, Diapers}) = 2/5 |
| **Frequent itemset** | itemset with support ≥ **minsup** threshold |

Given the rule A → B:

- **Support** = `#{A,B} / |T|` — a-priori probability of AB; rule frequency in the database.
- **Confidence** = `sup(A,B) / sup(A)` — conditional probability of finding B having found A; "strength" of the arrow.

Worked example (5 transactions above), from itemset {Milk, Diapers}:

- `Milk → Diapers`: sup = 3/5 = 60%, conf = #{Milk,Diapers}/#{Milk} = 3/4 = 75%;
- `Diapers → Milk`: same support 60%, conf = #{Milk,Diapers}/#{Diapers} = 3/3 = 100%.

**Association rule mining task**: given T, extract **all** rules satisfying support ≥ minsup and confidence ≥ minconf — the result is *complete* (all rules satisfying both constraints) and *correct* (only those rules). More complex constraints may be added.

## Mining Strategy

Brute force — enumerate all 2^d candidate itemsets (d items) and compute support/confidence for each — is computationally unfeasible: complexity ≈ O(|T| · 2^d · w) with |T| = number of transactions and w = transaction length.

Instead, **split the process**:

1. **Extraction of frequent itemsets** — the most computationally expensive step; extraction time is limited by means of the support threshold. Approaches: level-wise (Apriori, …), without candidate generation (FP-growth, …), others.
2. **Extraction of association rules** — generate all binary partitions of each frequent itemset, enforcing a confidence threshold (e.g. from {Milk, Diapers} sup 60%: Milk→Diapers conf 75%, Diapers→Milk conf 100%).

Efficiency levers: reduce the number of candidates (prune the 2^d search space), reduce the number of transactions (prune as itemset size grows), reduce the number of comparisons (efficient data structures such as the **hash-tree**: leaves hold itemsets and counts, interior nodes hash tables; a *subset* function matches transaction subsets against candidates).

## The Apriori Principle

> [!definition] Apriori principle
> "If an itemset is frequent, then all of its subsets must also be frequent." It holds because of the **antimonotone property of support**: if A ⊆ B then sup(A) ≥ sup(B) — the support of an itemset can never exceed the support of any of its subsets.

Effect: if {A,E} is found infrequent, all its supersets ({A,B,E}, {A,C,E}, …, {A,B,C,D,E}) are pruned without scanning the database.

### Level-wise Apriori

Pseudo-code (Ck = candidates of size k, Lk = frequent itemsets of size k):

```
L1 = {frequent items};
for (k = 1; Lk != ∅; k++) do begin
    Ck+1 = candidates generated from Lk;          -- (1) join step + prune step
    for each transaction t in database do
        increment the count of all candidates in Ck+1 contained in t;
    Lk+1 = candidates in Ck+1 satisfying minsup;  -- (2) frequent itemset generation
end
return ∪k Lk;
```

At each level: **(1) candidate generation** — sort Lk lexicographically; self-join candidates sharing the same k−1 prefix (example: from L3 = {abc, abd, acd, ace, bcd}: join gives abcd (abc⋈abd) and acde (acd⋈ace); prune removes acde because ade, cde ∉ L3 → C4 = {abcd}); apply the Apriori principle to drop candidates containing an infrequent subset. **(2)** scan the DB to count support and prune below minsup.

### Worked example (10 transactions, minsup > 1)

DB: {A,B}, {B,C,D}, {A,C,D,E}, {A,D,E}, {A,B,C}, {A,B,C,D}, {B,C}, {A,B,C}, {A,B,D}, {B,C,E}

| Step | Result |
|---|---|
| C1 / **L1** | {A}:7, {B}:8, {C}:7, {D}:5, {E}:3 — all frequent |
| C2 | {AB}:5, {AC}:4, {AD}:4, {AE}:2, {BC}:6, {BD}:3, {BE}:1, {CD}:3, {CE}:2, {DE}:2 |
| **L2** | all except {B,E} (sup 1 < 2) |
| C3 (join) | {ABC},{ABD},{ABE},{ACD},{ACE},{ADE},{BCD},{CDE} → prune {ABE} ({BE} infrequent) |
| C3 counts | {ABC}:3, {ABD}:2, {ACD}:2, {ACE}:1, {ADE}:2, {BCD}:2, {CDE}:1 |
| **L3** | {ABC}, {ABD}, {ACD}, {ADE}, {BCD} (sup 2 each) |
| C4 | {ABCD} (all 3-subsets in L3) — counted: sup 1 → **L4 = ∅**, algorithm stops |

Final frequent itemsets: L1 ∪ L2 ∪ L3 (11 + 5 itemsets).

### Performance issues

- Candidate sets may be huge (2-itemset generation is the critical step; long patterns require generating all frequent subsets); **n+1 database scans** when the longest frequent pattern has length n.
- Factors: minsup threshold (lower → more/larger itemsets), dimensionality, database size, average transaction width (dense data).

Improvements (with references): hash-based counting, **transaction reduction** (a transaction without any frequent k-itemset is useless later), **partitioning** (an itemset potentially frequent in DB is frequent in at least one partition), **sampling**, **dynamic itemset counting**.

## FP-growth (Han00)

Exploits a main-memory compressed representation, the **FP-tree**:

- Only **two database scans**: (1) count item supports; (2) build the FP-tree.
- High compression for dense data distributions, less for sparse ones; complete representation for frequent pattern mining.
- Mining by **recursive visit** of the FP-tree with a **divide-and-conquer** approach.

Construction: build a **Header Table** with items sorted by decreasing support (B:8, A:7, C:7, D:5, E:3); for each transaction, order items as in the header and insert into the tree reusing common prefixes (path counts accumulate; e.g. the 10-transaction example yields root → B:8 → {A:5 → C:3, C:2 → D:1}, A:2 → C:1 → D:1, etc.). Item pointers assist generation.

Mining: scan the Header Table **from the lowest-support item up**; for each item i extract frequent itemsets including i and items preceding it:

1. build the **Conditional Pattern Base** of i (i-CPB): select the prefix-paths of i from the FP-tree;
2. recursively invoke FP-growth on the CPB (build the conditional FP-tree / header table, iterate).

Worked example for item **D** (prefix paths {B,A,C}:1, {B,A}:1, {B,C}:1, {A,C}:1, {A}:1 → D-conditional header A:4, B:3, C:3): frequent itemsets with prefix D: {D}:5, {A,D}:4, {B,D}:3, {C,D}:3, {A,B,D}:2, {A,C,D}:2, {B,C,D}:2; recursion backtracks through DC-CPB, DCB-CPB (A infrequent there → empty), DB-CPB ({A}:2 → DBA), DA-CPB (empty) until the search ends.

**Other approaches**: e.g. vertical data layout (tidset of each item [Zak00]) instead of horizontal (TID → items).

## Compact Representations

Some itemsets are redundant (identical support as their supersets): with items A1..A10, B1..B10, C1..C10 always co-occurring in the same 10 transactions, there are `Σ C(30,k)` frequent itemsets — a compact representation is needed.

- **Maximal frequent itemset**: none of its immediate supersets is frequent. It does **not** carry the support information of its subsets.
- **Closed itemset**: none of its immediate supersets has the **same support** (each closed itemset keeps its exact support).

Containment: Maximal ⊆ Closed ⊆ Frequent itemsets. Example (TIDs 1: ABC, 2: ABCD, 3: BCE, 4: ACDE, 5: DE, minsup 2): closed frequent = 9 itemsets, maximal = 4.

## Beyond Support and Confidence

- **Choosing minsup is not obvious**: too high → rare-but-interesting itemsets lost (jewellery); too low → computationally expensive, too many rules.
- **Interestingness measures**: objective (statistics from data — support/confidence was only the initial framework) and subjective (user interpretation [Silb98]: interesting if it contradicts expectation or is actionable).

> [!warning] Confidence is not always reliable
> 5000 students: 3750 eat cereals, 3000 play basket, 2000 do both.
> Rule `play basket → eat cereals` has sup = 40%, conf = 2000/3000 = 66.7% — misleading, because "eat cereals" alone has support 75% > 66.7%. The rule head is too frequent.

**Correlation / lift**: for rule A → B,

```
Correlation = P(A,B) / (P(A)·P(B)) = conf(r) / sup(B)
```

- = 1 statistical independence; > 1 positive correlation; < 1 negative correlation.
- Example above: corr(basket → cereals) = 0.89 (negative!); corr(basket → not cereals) = 1.34.

**Extensions**:

- **Weighted association rules**: items/transactions have weights (quantity, price, basket discount); quality measures become weighted support/confidence with ad-hoc aggregation (min, max, avg).
- **Hierarchies / taxonomy**: a *taxonomy* is a set of is-a hierarchies aggregating data items into higher-level concepts (time period, product category, location). A **generalized itemset** may mix data items and generalized items; it *covers* a transaction when its generalized items are ancestors of items in the transaction and its data items are in it; support = covered transactions / dataset cardinality. Generalization (time 6.05 p.m. → 6–7 p.m.; user John → employee) can turn a very low support (0.005%) into an interesting one (0.2%) — context-aware analysis (mobile services).
- **Generalized association rules**: X → Y where X, Y are (generalized or not) itemsets; support, confidence, lift defined accordingly. Case study — Italian Local Health Center patient data (95K records, 3.5K patients; diabetes complications; 26 examinations/7 categories, 200 drugs/14 categories, census data; sparse dataset, hard threshold choice):
  - **High-level rules** (only generalized itemsets): general knowledge, e.g. (Examination, Liver) → (Examination, Kidney) — used for examination scheduling;
  - **Cross-level rules** (mixed levels): e.g. (Examination, Liver) → (Examination, Uric acid), conf 74.8% — insight into specific examinations correlated with liver ones;
  - **Low-level rules** (only data items): very detailed, largely covered by higher-level rules, large rule set — explored by drill-down from high/cross-level rules.
