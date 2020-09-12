use crate::Solution;

impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        if t.len() < 2 {
            return vec![0; t.len()];
        }

        let mut result = vec![0; t.len()];
        let mut prev = Vec::new();
        let (mut max, mut min) = (30, 100);
        for i in 1..result.len() {
            if t[i - 1] >= t[i] {
                prev.push(i - 1);
                max = max.max(t[i - 1]);
                min = min.min(t[i - 1]);
                continue;
            }

            result[i - 1] = 1;
            if min >= t[i] {
                continue;
            }

            if max < t[i] {
                for j in 0..prev.len() {
                    result[prev[j]] = (i - prev[j]) as i32;
                }
                prev.clear();
                continue;
            }

            for j in (0..prev.len()).rev() {
                if t[prev[j]] < t[i] {
                    result[prev[j]] = (i - prev[j]) as i32;
                    prev.remove(j);
                }
            }
        }
        result
    }
}
