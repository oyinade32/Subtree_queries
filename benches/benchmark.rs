use std::time::Instant;

use subtree_queries::naive::NaiveTree;
use subtree_queries::optimized::OptimizedTree;
use subtree_queries::tree::Tree;

fn generate_tree(n: usize) -> (Tree, Vec<(usize, usize)>) {
    let mut edges = Vec::new();
    let mut values = vec![0; n + 1];

    // Assign values to each node
    for i in 1..=n {
        values[i] = (i % 100) as i64;
    }

    for i in 2..=n {
        edges.push((i / 2, i));
    }

    let tree = Tree::new(n, values, edges.clone());
    (tree, edges)
}

fn main() {
    let sizes = [100, 500, 1000, 2000];
    let queries = 1000;

    for &n in &sizes {
        let (tree, edges) = generate_tree(n);

        let mut naive = NaiveTree::new(tree.clone());
        let mut opt = OptimizedTree::new(n, tree.values.clone(), edges);

        let start_naive_update = Instant::now();

        for i in 0..queries {
            let node = (i % n) + 1;
            naive.update(node, (i % 100) as i64);
        }

        let naive_update = start_naive_update.elapsed();

        let start_opt_update = Instant::now();

        for i in 0..queries {
            let node = (i % n) + 1;
            opt.update(node, (i % 100) as i64);
        }

        let opt_update = start_opt_update.elapsed();

     
        let start_naive_query = Instant::now();

        for i in 0..queries {
            let node = (i % n) + 1;
            let _ = naive.subtree_sum(node);
        }

        let naive_query = start_naive_query.elapsed();

        let start_opt_query = Instant::now();

        for i in 0..queries {
            let node = (i % n) + 1;
            let _ = opt.subtree_sum(node);
        }

        let opt_query = start_opt_query.elapsed();

        println!("====================================");
        println!("Tree Size: {}", n);

        println!("\nUpdate Benchmark");
        println!("Naive:     {:?}", naive_update);
        println!("Optimized: {:?}", opt_update);

        println!("\nSubtree Query Benchmark");
        println!("Naive:     {:?}", naive_query);
        println!("Optimized: {:?}", opt_query);

        println!("====================================\n");
    }
}