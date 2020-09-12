use crate::Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() || target < nums[0] || nums[nums.len() - 1] < target { return vec![-1, -1]; }

        fn first(nums: &Vec<i32>, target: i32) -> usize {
            let mut result = 0;
            let (mut start, mut end) = (0, nums.len() - 1);
            while start <= end {
                let middle = start + (end - start) / 2;
                if nums[middle] < target {
                    start = middle + 1;
                    result = start;
                } else if target < nums[middle] {
                    result = middle;
                    end = middle - 1;
                } else {
                    result = middle;
                    if result == 0 || nums[result - 1] < target {
                        break;
                    }
                    result -= 1;
                    end = result;
                }
            }
            result
        }
        fn last(nums: &Vec<i32>, target: i32) -> usize {
            let mut result = 0;
            let (mut start, mut end) = (0, nums.len() - 1);
            while start <= end {
                let middle = start + (end - start) / 2;
                if nums[middle] < target {
                    start = middle + 1;
                    result = start;
                } else if target < nums[middle] {
                    result = middle;
                    end = middle - 1;
                } else {
                    result = middle;
                    if result == nums.len() - 1 || target < nums[result + 1] {
                        break;
                    }
                    result += 1;
                    start = result;
                }
            }
            result
        }

        let first = first(&nums, target);
        if nums[first] != target { return vec![-1, -1]; }

        vec![first as i32, last(&nums, target) as i32]
    }
}