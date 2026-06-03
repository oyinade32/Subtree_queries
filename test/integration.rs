use subtree_queries::naive::NaiveTree;
use subtree_queries::optimized::OptimizedTree;
use subtree_queries::tree::Tree;

fn build_sample_tree() -> (Tree, Vec<(usize, usize)>) {
    let n = 5;

    let values = vec![0, 4, 2, 5, 2, 1];

    let edges = vec![
        (1, 2),
        (1, 3),
        (3, 4),
        (3, 5),
    ];

    let tree = Tree::new(n, values, edges.clone());

    (tree, edges)
}

#[test]
fn test_naive_vs_optimized() {
    let (tree, edges) = build_sample_tree();

    let mut naive = NaiveTree::new(tree.clone());
    let mut opt = OptimizedTree::new(5, tree.values.clone(), edges);

    // initial subtree check
    assert_eq!(naive.subtree_sum(3), opt.subtree_sum(3));
    assert_eq!(naive.subtree_sum(1), opt.subtree_sum(1));

    // update test
    naive.update(5, 3);
    opt.update(5, 3);

    assert_eq!(naive.subtree_sum(3), opt.subtree_sum(3));
    assert_eq!(naive.subtree_sum(1), opt.subtree_sum(1));

    // more updates
    naive.update(2, 10);
    opt.update(2, 10);

    assert_eq!(naive.subtree_sum(1), opt.subtree_sum(1));
}