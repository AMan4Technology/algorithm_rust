use crate::Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        if numbers.len() < 2 {
            return Vec::new();
        }

        let (mut left, mut right) = (0, numbers.len() - 1);
        while left < right {
            let count = numbers[left] + numbers[right];
            if count > target {
                right -= 1;
            } else if count < target {
                left += 1;
            } else {
                return vec![left as i32 + 1, right as i32 + 1];
            }
        }
        return Vec::new();
    }
}
