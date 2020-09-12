use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.len() < 2 {
            return 0;
        }

        let (mut left, mut right) = (0, height.len() - 1);
        let mut result = 0;
        while left < right {
            result = result.max(height[left].min(height[right]) * (right - left) as i32);
            match height[left].cmp(&height[right]) {
                Ordering::Less => {
                    left += 1;
                }
                Ordering::Greater => {
                    right -= 1;
                }
                Ordering::Equal => {
                    left += 1;
                    right -= 1;
                }
            }
        }
        result
    }
}
