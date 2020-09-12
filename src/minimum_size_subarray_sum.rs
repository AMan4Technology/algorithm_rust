use crate::Solution;

impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        if s <= 0 || nums.is_empty() {
            return 0;
        }

        let (mut left, mut right) = (0, 1);
        let (mut sum, mut len) = (nums[left], std::i32::MAX);
        loop {
            if sum < s {
                if right == nums.len() {
                    break;
                }

                sum += nums[right];
                right += 1;
                continue;
            }
            len = len.min((right - left) as i32);
            sum -= nums[left];
            left += 1;
        }

        if len == std::i32::MAX {
            0
        } else {
            len
        }
    }
}
