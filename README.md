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

## 2. Optimized Approach (Euler Tour + Fenwick Tree)

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

The benchmark compares performance on increasing tree sizes.

Example results:
n = 100
Naive: ~818 µs
Optimized: ~4 µs

n = 500
Naive: ~3 ms
Optimized: ~7 µs

n = 1000
Naive: ~8 ms
Optimized: ~8 µs

n = 2000
Naive: ~28 ms
Optimized: ~9 µs


---

## Key Insight

The optimized solution remains almost constant in performance because it avoids traversing the tree for every query.

Instead, it converts subtree queries into range queries on a flattened array.

---

## Conclusion

The Euler Tour + Fenwick Tree approach significantly outperforms the naive DFS method, especially for large inputs.

This demonstrates the importance of preprocessing and advanced data structures in optimizing tree-based queries.

---

## Author

Rust implementation for academic deliverable and algorithmic benchmarking.
