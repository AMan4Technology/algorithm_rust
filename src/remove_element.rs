use crate::Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let (mut i, mut length) = (0, nums.len());
        while i < length {
            if nums[i] != val {
                i += 1;
                continue;
            }
            while i < length && nums[length - 1] == val {
                length -= 1;
            }
            if i == length { break; }
            length -= 1;
            nums.swap(i, length);
            i += 1;
        }
        length as i32
    }
}