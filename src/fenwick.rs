pub struct Fenwick {
    pub n: usize,
    pub bit: Vec<i64>,
}

impl Fenwick {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            bit: vec![0; n + 1],
        }
    }

    pub fn add(&mut self, mut i: usize, delta: i64) {
        while i <= self.n {
            self.bit[i] += delta;
            i += i & (!i + 1);
        }
    }

    pub fn sum(&self, mut i: usize) -> i64 {
        let mut res = 0;
        while i > 0 {
            res += self.bit[i];
            i -= i & (!i + 1);
        }
        res
    }

    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.sum(r) - self.sum(l - 1)
    }
}