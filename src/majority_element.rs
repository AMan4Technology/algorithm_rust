use crate::Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut curr, mut count) = (0, 0);
        for &value in nums.iter() {
            if count == 0 {
                curr = value;
                count = 1;
                continue;
            }
            count += if value == curr { 1 } else { -1 };
        }
        curr
    }
}