use crate::Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let length = nums.len();
        if length < 2 { return; }

        let mut j = 0;
        for i in 0..length {
            if nums[i] != 0 { continue; }
            j = j.max(i + 1);
            if j == length { break; }
            for k in j..length {
                j = k + 1;
                if nums[k] != 0 {
                    nums.swap(i, k);
                    break;
                }
            }
        }
    }
}