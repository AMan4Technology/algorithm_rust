use crate::Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }

        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                return nums[i + 1];
            }
        }
        nums[0]
    }
}
