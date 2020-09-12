use crate::Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return nums;
        }

        let mut nums = nums;
        let (start, end) = (0, nums.len() - 1);
        Self::my_sort(&mut nums, start, end);
        nums
    }

    fn my_sort(nums: &mut Vec<i32>, start: usize, end: usize) {
        let mut pivot = end;
        for i in start..end {
            if nums[i] <= nums[end] {
                continue;
            }
            let mut swap = false;
            for j in i + 1..end {
                if nums[j] <= nums[end] {
                    nums.swap(i, j);
                    swap = true;
                    break;
                }
            }
            if !swap {
                nums.swap(i, end);
                pivot = i;
                break;
            }
        }
        if pivot > start + 1 {
            Self::my_sort(nums, start, pivot - 1);
        }
        if pivot < end - 1 {
            Self::my_sort(nums, pivot + 1, end);
        }
    }
}
