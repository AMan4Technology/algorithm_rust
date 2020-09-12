use crate::Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() { return 0; }

        let (mut yes, mut no) = (vec![0; nums.len()], vec![0; nums.len()]);
        yes[0] = nums[0];
        for i in 1..nums.len() {
            yes[i] = no[i - 1] + nums[i];
            no[i] = yes[i - 1].max(no[i - 1]);
        }
        yes[nums.len() - 1].max(no[nums.len() - 1])
    }
}