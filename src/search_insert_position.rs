use crate::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() { return 0; }
        let (mut start, mut end) = (0, nums.len() - 1);
        if target <= nums[start] { return start as i32; } else if nums[end] < target { return end as i32 + 1; }

        let mut result = 0;
        while start <= end {
            result = start + (end - start) / 2;
            if nums[result] < target {
                result += 1;
                start = result;
            } else if target < nums[result] {
                end = result-1;
            } else {
                break;
            }
        }
        result as i32
    }
}