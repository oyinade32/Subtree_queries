use std::io::{self, Read};
use subtree_queries::OptimizedTree;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let q: usize = it.next().unwrap().parse().unwrap();

    let mut values = vec![0i64; n + 1];
    for i in 1..=n {
        values[i] = it.next().unwrap().parse().unwrap();
    }

    let mut edges = Vec::with_capacity(n - 1);
    for _ in 0..(n - 1) {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        edges.push((a, b));
    }

    let mut tree = OptimizedTree::new(n, values, edges);

    let mut out = String::new();

    for _ in 0..q {
        let t: i32 = it.next().unwrap().parse().unwrap();

        if t == 1 {
            let s: usize = it.next().unwrap().parse().unwrap();
            let x: i64 = it.next().unwrap().parse().unwrap();
            tree.update(s, x);
        } else {
            let s: usize = it.next().unwrap().parse().unwrap();
            let ans = tree.subtree_sum(s);
            out.push_str(&format!("{}\n", ans));
        }
    }

    print!("{}", out);
}