use std::time::Instant;

use subtree_queries::naive::NaiveTree;
use subtree_queries::optimized::OptimizedTree;
use subtree_queries::tree::Tree;

fn generate_tree(n: usize) -> (Tree, Vec<(usize, usize)>) {
    let mut edges = Vec::new();
    let mut values = vec![0; n + 1];

    for i in 1..=n {
        values[i] = (i % 100) as i64;
        if i > 1 {
            edges.push((1, i)); // star-shaped tree
        }
    }

    let tree = Tree::new(n, values, edges.clone());
    (tree, edges)
}

fn main() {
    let sizes = [100, 500, 1000, 2000];
    let queries = 1000;

    for &n in &sizes {
        let (tree, edges) = generate_tree(n);

        let naive = NaiveTree::new(tree.clone());
        let opt = OptimizedTree::new(n, tree.values.clone(), edges);

        let node = n / 2;

        let start_naive = Instant::now();
        for _ in 0..queries {
            let _ = naive.subtree_sum(node);
        }
        let naive_time = start_naive.elapsed();

        let start_opt = Instant::now();
        for _ in 0..queries {
            let _ = opt.subtree_sum(node);
        }
        let opt_time = start_opt.elapsed();

        println!("n = {}", n);
        println!("Naive: {:?}", naive_time);
        println!("Optimized: {:?}", opt_time);
        println!("------------------------");
    }
}