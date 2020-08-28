use std::iter;

pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
            (0..self.0).map(crate::row).collect()
    }
}

fn row(n: u32) -> Vec<u32> {
    iter::repeat(n)
        .zip(0..=n)
        .map(|(n, k)| combinations(n, k))
        .collect()
}

fn combinations(n: u32, k: u32) -> u32 {
    factorial(n) / (factorial(k) * factorial(n - k))
}

fn factorial(n: u32) -> u32 {
    (1..=n).product()
}
