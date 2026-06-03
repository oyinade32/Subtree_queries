#[derive(Clone)]
pub struct Tree {
    pub n: usize,
    pub adj: Vec<Vec<usize>>,
    pub values: Vec<i64>,
}

impl Tree {
    pub fn new(n: usize, values: Vec<i64>, edges: Vec<(usize, usize)>) -> Self {
        let mut adj = vec![vec![]; n + 1];

        for (a, b) in edges {
            adj[a].push(b);
            adj[b].push(a);
        }

        Self {
            n,
            adj,
            values,
        }
    }
}