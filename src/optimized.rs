use crate::fenwick::Fenwick;

pub struct OptimizedTree {
    adj: Vec<Vec<usize>>,
    values: Vec<i64>,

    tin: Vec<usize>,
    size: Vec<usize>,
    flat: Vec<i64>,
    timer: usize,

    bit: Fenwick,
}

impl OptimizedTree {
    pub fn new(n: usize, values: Vec<i64>, edges: Vec<(usize, usize)>) -> Self {
        let mut adj = vec![vec![]; n + 1];

        for (a, b) in edges {
            adj[a].push(b);
            adj[b].push(a);
        }

        let mut tree = Self {
            adj,
            values,
            tin: vec![0; n + 1],
            size: vec![0; n + 1],
            flat: vec![0; n + 1],
            timer: 1,
            bit: Fenwick::new(n),
        };

        tree.dfs(1, 0);

        for i in 1..=n {
            tree.bit.add(i, tree.flat[i]);
        }

        tree
    }

    fn dfs(&mut self, node: usize, parent: usize) {
        self.tin[node] = self.timer;
        self.flat[self.timer] = self.values[node];
        self.timer += 1;

        self.size[node] = 1;

        let neighbors = self.adj[node].clone();

        for next in neighbors {
            if next != parent {
                self.dfs(next, node);
                self.size[node] += self.size[next];
            }
        }
    }

    pub fn update(&mut self, node: usize, value: i64) {
        let pos = self.tin[node];
        let delta = value - self.values[node];

        self.values[node] = value;
        self.bit.add(pos, delta);
    }

    pub fn subtree_sum(&self, node: usize) -> i64 {
        let l = self.tin[node];
        let r = self.tin[node] + self.size[node] - 1;

        self.bit.range_sum(l, r)
    }
}