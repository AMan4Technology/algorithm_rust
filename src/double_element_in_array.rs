use crate::Solution;

impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut counts = vec![0; nums.len()];
        for &num in nums.iter() {
            if counts[num as usize] == 1 {
                return num;
            }
            counts[num as usize] = 1;
        }
        -1
    }
}
