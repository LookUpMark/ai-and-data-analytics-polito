---
title: Clustering
aliases: [K-means, Bisecting K-means, Hierarchical Clustering, DBSCAN, Cluster Validity, Silhouette, Rand Index]
tags: [computer-science/databases, note/lecture-notes, level/first-year]
creation_date: 2026-08-30
last_modified: 2026-08-30
status: complete
---
> [!summary] **Document Summary**
> Unsupervised cluster analysis: grouping objects so that intra-cluster distances are minimized and inter-cluster distances are maximized. Covers the taxonomy of clusterings (partitional vs hierarchical, exclusive vs fuzzy) and cluster types (well-separated, center-based, contiguous, density-based, conceptual); K-means in depth (algorithm, SSE, initial-centroid problem, elbow parameter setting, empty clusters, pre/post-processing, bisecting variant, limitations); agglomerative hierarchical clustering (dendrograms, proximity matrix, MIN/MAX/group-average/centroid/Ward linkage with strengths and limitations); DBSCAN (core, border, noise points, parameter tuning); and cluster validity — external/internal/relative indices, cohesion (WSS) vs separation (BSS), Silhouette index and Rand Index.

## What Is Cluster Analysis?

> [!definition] Cluster analysis
> Finding groups of objects such that the objects in a group are **similar (or related) to one another** and **different from (or unrelated to)** the objects in other groups: **intra-cluster distances are minimized, inter-cluster distances are maximized**.

Applications:

- **Understanding** — group related documents for browsing, genes and proteins with similar functionality, stocks with similar price fluctuations (a stock dataset splits e.g. into two "technology" clusters going down, financial down, and oil going up); clustering precipitation in Australia.
- **Summarization** — reduce the size of large data sets.

The notion of cluster can be **ambiguous**: the same point set may be naturally read as 2, 4 or 6 clusters.

## Types of Clusterings

A **clustering** is a set of clusters. Key distinction: **partitional** clustering divides data objects into non-overlapping subsets (each object in exactly one subset); **hierarchical** clustering produces a set of **nested clusters organized as a hierarchical tree**, visualized as a **dendrogram** (a tree-like diagram recording the sequence of merges or splits).

Other distinctions between sets of clusters:

- **Exclusive vs non-exclusive**: in non-exclusive clustering points may belong to multiple clusters.
- **Fuzzy vs non-fuzzy**: in fuzzy clustering a point belongs to **every cluster with some weight between 0 and 1** (weights sum to 1); probabilistic clustering is similar.
- **Partial vs complete**: sometimes only part of the data needs to be clustered.
- **Heterogeneous vs homogeneous**: widely different cluster sizes, shapes, densities.

## Types of Clusters

| Type | Definition |
|---|---|
| **Well-separated** | any point in the cluster is closer (more similar) to every other point of the cluster than to any point not in it |
| **Center-based** | an object is closer to the **center** of its cluster than to the center of any other cluster; the center is a **centroid** (average of the points) or a **medoid** (most representative point) |
| **Contiguous** (nearest neighbor / transitive) | a point is closer (more similar) to **one or more** other points in the cluster than to any point outside |
| **Density-based** | a dense region of points, separated from other high-density regions by low-density regions; used when clusters are irregular or intertwined and when noise/outliers are present |
| **Property or conceptual** | clusters share some common property or represent a particular concept (e.g. two overlapping circles) |

Clustering algorithms covered: **K-means and its variants**, **hierarchical clustering**, **density-based clustering (DBSCAN)**.

## K-means

**Partitional** approach: each cluster is associated with a **centroid** (center point); each point is assigned to the cluster with the **closest centroid**; the number of clusters **K must be specified**. The basic algorithm is very simple:

```
1. select K points as initial centroids
2. repeat
3.     form K clusters by assigning each point to its closest centroid
4.     recompute the centroid of each cluster
5. until centroids do not change (or few points change clusters)
```

Details:

- **Initial centroids are often chosen randomly** — clusters produced vary from one run to another: the slides show the same points clustered **optimally** or **sub-optimally** depending on the starting centroids, and an iteration-by-iteration trace where the assignment of points to the two centroids changes from iteration 1 to iteration 6 before stabilizing.
- The centroid is (typically) the **mean** of the points in the cluster.
- **Closeness**: Euclidean distance, cosine similarity, correlation, etc. K-means converges for the common similarity measures; most convergence happens in the **first few iterations**, so the stopping condition is often relaxed to "until relatively few points change clusters".
- **Complexity O(n · K · I · d)** — n points, K clusters, I iterations, d attributes.

### Evaluating clusters: SSE

Most common measure: **Sum of Squared Errors** — for each point the error is the distance to the nearest cluster representative; square and sum:

```
SSE = Σi Σx∈Ci  dist²(mi, x)
```

where mi is the representative point of cluster Ci — it can be shown that mi corresponds to the **center (mean)** of the cluster. Given two clusterings, choose the one with the smallest error; increasing K trivially reduces SSE, but **a good clustering with smaller K can have a lower SSE than a poor clustering with higher K**.

### Choosing K: elbow graph

**Elbow (knee) approach**: plot the quality measure trend (e.g. SSE) against K and choose the value where **the gain from adding a centroid becomes negligible** — the quality measure reduction is no longer interesting. Examples in the slides: network traffic data and medical records (SSE curve flattening between K = 5 and 10).

### Initial centroids problem and remedies

- **Multiple runs** — helps, but "probability is not on your side".
- **Sample and use hierarchical clustering** to determine initial centroids.
- **Select more than K initial centroids** and then select among these the **most widely separated**.
- **Post-processing** (see below).
- **Bisecting K-means** — variant that can produce a partitional or a hierarchical clustering; **not as susceptible to initialization issues**.

### Empty clusters, pre- and post-processing

Basic K-means can yield **empty clusters**; strategies: choose the point that contributes most to SSE, or a point from the cluster with the highest SSE (repeat for several empty clusters).

- **Pre-processing**: normalize the data; eliminate outliers.
- **Post-processing**: eliminate small clusters that may represent outliers; **split "loose" clusters** (relatively high SSE); **merge clusters that are "close"** and have relatively low SSE; these steps can also be used *during* the clustering process.

### Limitations of K-means

K-means has problems when clusters have **differing sizes**, **differing densities**, or **non-globular shapes**, and when the data contains **outliers** (illustrated in the slides by splitting two globular groups incorrectly: unequal sizes, sparse+dense mix, two ring-shaped clusters). One solution: **use many clusters** — find parts of clusters, then put the parts together.

## Hierarchical Clustering

Produces nested clusters organized as a tree, visualized as a **dendrogram**: any desired number of clusters can be obtained by **cutting the dendrogram at the proper level**, and the levels may correspond to **meaningful taxonomies** (e.g. biological sciences: animal kingdom, phylogeny reconstruction) — no need to fix the number of clusters in advance.

Two main types:

- **Agglomerative** (bottom-up, "the one usually used"): start with the points as individual clusters; at each step **merge the closest pair of clusters** until only one cluster (or k clusters) is left.
- **Divisive** (top-down): start with one all-inclusive cluster; at each step **split** a cluster until each cluster contains a point (or there are k clusters).

Traditional algorithms use a **similarity/distance matrix** (proximity matrix) and merge or split **one cluster at a time**.

### Agglomerative algorithm

```
1. Compute the proximity matrix
2. Let each data point be a cluster
3. Repeat
4.   Merge the two closest clusters
5.   Update the proximity matrix
6. Until only a single cluster remains
```

The **key operation is the computation of the proximity of two clusters**: how to update the proximity matrix after merging C2 and C5 into C2∪C5 distinguishes the different algorithms.

### Cluster similarity measures

| Method | Proximity of two clusters | Strengths | Limitations |
|---|---|---|---|
| **MIN / single link** | the two **most similar (closest) points** in the two clusters (one link in the proximity graph) | can handle **non-elliptical shapes** | **sensitive to noise and outliers** |
| **MAX / complete linkage** | the two **least similar (most distant) points** | **less susceptible to noise and outliers** | tends to **break large clusters**; biased towards globular clusters |
| **Group average** | **average of pairwise proximities** between points of the two clusters: `proximity(Ci,Cj) = Σ proximity(pi,pj) / (|Ci|·|Cj|)` | compromise between MIN and MAX; less susceptible to noise and outliers | biased towards globular clusters |
| **Distance between centroids** | proximity of the two cluster centroids | — | — |
| **Ward's method** | **increase in squared error** when two clusters are merged (similar to group average if distance is squared distance) | less susceptible to noise and outliers; **hierarchical analogue of K-means** (can be used to initialize K-means) | biased towards globular clusters |

Worked example on the similarity matrix of items I1…I5 (1.00 on the diagonal; e.g. sim(I1,I2)=0.90, sim(I1,I3)=0.10, sim(I4,I5)=0.80): with similarity (instead of distance) MIN corresponds to the **maximum** similarity value. For the 6-point dataset (p1…p6) the dendrogram of each method records the sequence of merges with their proximity values (merge heights 0.05 … 0.4): e.g. with MIN the pairs merge bottom-up into {1,2}, {3,6}, {5,4}, … until a single cluster; **the same points yield different nested clusterings and dendrograms for MIN, MAX, group average and Ward's method**.

For group average, note that the **average** connectivity is needed for scalability: total (unnormalized) proximity favors large clusters.

**Time and space requirements**: O(N²) **space** (proximity matrix); O(N³) **time** in many cases (N steps, each updating and searching the N² matrix), reducible to O(N² log N) for some approaches.

## DBSCAN

**Density-based** algorithm: **density = number of points within a specified radius (Eps)**.

- **Core point**: has more than a specified number of points (**MinPts**) within Eps — points at the interior of a cluster.
- **Border point**: fewer than MinPts within Eps, but is in the neighborhood of a core point.
- **Noise point**: neither a core nor a border point.

Algorithm: **eliminate noise points**, then perform clustering on the remaining points (the slides classify the points of a dataset with Eps = 10, MinPts = 4 into core, border, noise).

- **Works well**: resistant to noise; handles clusters of **different shapes and sizes**.
- **Does not work well**: **varying densities** (MinPts/Eps tuned on one density fail on another) and **high-dimensional data**.

**Determining Eps and MinPts**: for points in a cluster the kth nearest neighbors are at roughly the same distance, while noise points have the kth nearest neighbor farther away → plot the **sorted distance of every point to its kth nearest neighbor** and look for the knee.

### Choosing an algorithm — recap from strengths/limitations

| Family | Strengths | Weaknesses |
|---|---|---|
| **K-means** (and bisecting variant) | simple, fast O(n·K·I·d); bisecting variant not as susceptible to initialization | K must be specified; fails on differing sizes/densities, non-globular shapes, outliers |
| **Hierarchical** (agglomerative) | no K needed a priori (cut the dendrogram); meaningful taxonomies; linkage choice trades shape flexibility vs robustness | O(N²) space, O(N³) time (O(N² log N) for some approaches); MAX/average/Ward biased to globular clusters; MIN sensitive to noise |
| **DBSCAN** | resistant to noise; arbitrary shapes and sizes; outliers isolated as noise | varying densities; high-dimensional data; Eps/MinPts tuning |

## Cluster Validity

For supervised classification there are measures (accuracy, precision, recall); for clustering the analogous question is how to evaluate the "goodness" of the resulting clusters — but "**clusters are in the eye of the beholder**". Motivations: avoid **finding patterns in noise** (random points still yield clusters under DBSCAN, K-means, complete link), compare clustering algorithms, compare two sets of clusters, compare two clusters.

Aspects of cluster validation:

1. Determining the **clustering tendency** of the data (distinguishing non-random structure from noise).
2. Comparing results to **externally known results** (e.g. class labels).
3. Evaluating how well the results fit the data **without external information** (using only the data).
4. Comparing the results of **two different clusterings**.
5. Determining the **correct number of clusters**.

(For 2–4: distinguish evaluating the entire clustering vs individual clusters.)

### Classes of validity measures

| Index | Use | Examples |
|---|---|---|
| **External** | measure the extent to which cluster labels match externally supplied class labels | entropy, purity |
| **Internal** | measure the goodness of a clustering structure **without external information** | SSE, cluster cohesion, cluster separation, Rand index, adjusted Rand index, Silhouette index |
| **Relative** | compare two different clusterings or clusters (often an external or internal index used for this function) | SSE, entropy |

### Cohesion and separation

- **Cluster cohesion** — how closely related are objects in a cluster; measured by the **within-cluster sum of squares**:

```
WSS = Σi Σx∈Ci (x − mi)²
```

- **Cluster separation** — how distinct a cluster is from the others; measured by the **between-cluster sum of squares**:

```
BSS = Σi |Ci| (m − mi)²          |Ci| = size of cluster i, m = overall mean
```

A **proximity graph** approach is also possible: cohesion = sum of the weights of links **within** a cluster; separation = sum of the weights of links between nodes **inside** and **outside** the cluster.

### Silhouette index

A succinct measure of how well each object lies within its cluster; defined for single points; considers **both cohesion and separation**; computable for individual points, individual clusters and the whole clustering. For each object i:

- **a(i)**: average dissimilarity of i with all other objects in the **same** cluster (the smaller, the better the assignment);
- **b(i)**: **min** over the other clusters of the average dissimilarity of i to that cluster;

```
s(i) = (b(i) − a(i)) / max(a(i), b(i))
```

Ranges between **−1 and +1** (typically between 0 and 1); the closer to 1, the better. The average s(i) over a cluster measures how tightly grouped the cluster is; the average over the whole dataset measures how appropriately the data have been clustered.

### External measures: entropy, purity, Rand Index

**Entropy** and **purity** measure the match of cluster labels against class labels (slide table). The **Rand Index** is based on the idea that any two objects in the same cluster should be in the same class and vice versa. Given:

- f00 = pairs with **different class and different cluster**
- f01 = pairs with **different class and same cluster**
- f10 = pairs with **same class and different cluster**
- f11 = pairs with **same class and same cluster**

```
Rand Index = (f00 + f11) / (f00 + f01 + f10 + f11)
```

> [!warning] Validation is the hard part
> "The validation of clustering structures is the most difficult and frustrating part of cluster analysis. Without a strong effort in this direction, cluster analysis will remain a black art accessible only to those true believers who have experience and great courage." — Jain & Dubes, *Algorithms for Clustering Data*.
