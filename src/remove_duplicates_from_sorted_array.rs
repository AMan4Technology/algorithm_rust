use crate::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() { return 0; }

        let mut pivot = 0;
        for i in 1..nums.len() {
            if nums[pivot] < nums[i] {
                pivot += 1;
                nums.swap(pivot, i);
            }
        }
        (pivot + 1) as i32
    }
}