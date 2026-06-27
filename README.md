# Subtree Queries – Tree Optimization Project (Rust)

## Problem Overview

We are given a rooted tree with `n` nodes, where each node has a value. The system must support two operations efficiently:

1. Update the value of a node.
2. Query the sum of values in a subtree.

Because `n` and `q` can be large (up to 200,000), a naive approach becomes too slow.

---

## Approaches Implemented

This project implements and compares two different algorithms:

---

## 1. Naive DFS Approach

### Idea
For every subtree query, we perform a Depth-First Search (DFS) starting from the target node and sum all reachable nodes.

### Complexity
- Update: O(1)
- Subtree Query: O(n)

### Limitation
This approach becomes slow for large trees because each query may traverse a large portion of the tree.

---

## 2. Optimized Euler Tour + Fenwick Tree Approach

### Idea
The tree is flattened using an Euler Tour technique so that each subtree corresponds to a continuous range in an array.

A Fenwick Tree (Binary Indexed Tree) is then used to:
- Support fast point updates
- Support fast range sum queries

### Complexity
- Update: O(log n)
- Subtree Query: O(log n)

---

## Data Structures Used

- Adjacency List (Tree representation)
- DFS Traversal (Euler Tour flattening)
- Fenwick Tree (Binary Indexed Tree)

---

## Project Structure
- Cargo.toml (project configuration and dependencies)

src/
├── main.rs              # CSES submission entry point
├── lib.rs               # Module exports
├── tree.rs              # Shared tree structure
├── naive.rs             # Naive DFS implementation
├── optimized.rs         # Euler Tour + Fenwick Tree solution
├── fenwick.rs           # Fenwick Tree implementation

tests/
└── integration_test.rs  # Correctness comparison tests

benches/
└── benchmark.rs         # Performance comparison (benchmarking)


---

## Benchmark Results

The benchmark compares the performance of the naive and optimized implementations for both update and subtree query operations using different tree sizes.

| Tree Size | Operation     |        Naive | Optimized |
| --------- | ------------- | -----------: | --------: |
| 100       | Update        |     4.324 µs |  8.785 µs |
| 100       | Subtree Query |   645.838 µs | 10.187 µs |
| 500       | Update        |     4.195 µs | 11.992 µs |
| 500       | Subtree Query |  3.896713 ms | 19.802 µs |
| 1000      | Update        |     4.800 µs | 14.229 µs |
| 1000      | Subtree Query |  6.290066 ms | 21.161 µs |
| 2000      | Update        |     5.014 µs | 15.838 µs |
| 2000      | Subtree Query | 14.894762 ms | 21.439 µs |


## Key Insight

The benchmark shows that the naive implementation performs node updates slightly faster because it only changes a single value in the array.

However, subtree queries become significantly slower as the tree grows because the algorithm performs a Depth-First Search (DFS) for every query.

The optimized implementation spends slightly more time updating values because it must also update the Fenwick Tree. In return, subtree queries remain very fast since the Euler Tour converts each subtree into a continuous range, allowing the Fenwick Tree to answer range-sum queries efficiently.


---
## Conclusion

This project compares a naive DFS approach with an optimized Euler Tour and Fenwick Tree approach for solving subtree queries.

The benchmark demonstrates that although the naive implementation performs updates faster, its subtree queries become much slower as the input size increases because each query traverses the tree.

The optimized implementation performs updates in **O(log n)** time due to Fenwick Tree maintenance, while both updates and subtree queries scale efficiently for large datasets. This makes it the preferred solution when processing many queries on large trees.

Overall, the project shows how preprocessing and efficient data structures can significantly improve the performance of tree-based algorithms.


---

## Author

Rust implementation for academic deliverable and algorithmic benchmarking.
