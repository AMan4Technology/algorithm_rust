use crate::Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, w: i32) -> f64 {
        let start = n - w + 1;
        if k <= start || k <= 0 {
            return 1f64;
        }

        let mut count = (1 + k - start) * (k - start) / 2;
        if start < 0 {
            count -= (1 - start) * -start / 2;
        }
        let mut all = (1 + w) * w / 2;
        if k - w < 0 {
            all -= (1 + w - k) * (w - k) / 2;
        }

        1f64 - count as f64 / all as f64
    }
}
