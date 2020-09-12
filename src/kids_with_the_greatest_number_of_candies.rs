use crate::Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        if candies.is_empty() {
            return Vec::new();
        }

        let max = candies.iter().fold(0, |acc, &value| acc.max(value));

        let mut result = vec![false; candies.len()];
        for i in 0..candies.len() {
            if candies[i] + extra_candies >= max {
                result[i] = true;
            }
        }
        result
    }
}
