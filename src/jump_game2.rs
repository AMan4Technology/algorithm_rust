use crate::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let (mut start, mut end) = (0, 1);
        while end < nums.len() {
            count += 1;
            let mut max = 0;
            for i in start..end {
                max = max.max(i + nums[i] as usize);
            }
            start = end;
            end = max + 1;
        }
        count
    }
}
