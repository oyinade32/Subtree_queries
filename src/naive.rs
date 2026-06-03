use crate::tree::Tree;

pub struct NaiveTree {
    tree: Tree,
}

impl NaiveTree {
    pub fn new(tree: Tree) -> Self {
        Self { tree }
    }

    pub fn update(&mut self, node: usize, value: i64) {
        self.tree.values[node] = value;
    }

    pub fn subtree_sum(&self, start: usize) -> i64 {
        let mut visited = vec![false; self.tree.n + 1];
        let mut stack = vec![start];
        let mut sum = 0;

        while let Some(node) = stack.pop() {
            if visited[node] {
                continue;
            }
            visited[node] = true;

            sum += self.tree.values[node];

            for &next in &self.tree.adj[node] {
                if !visited[next] {
                    stack.push(next);
                }
            }
        }

        sum
    }
}