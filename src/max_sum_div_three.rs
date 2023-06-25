use crate::Solution;
use std::cmp::max;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut max_sum = [0; 3];
        for &num in nums.iter() {
            let num = num as usize;
            let a = max_sum[0] + num;
            let b = max_sum[1] + num;
            let c = max_sum[2] + num;

            max_sum[a % 3] = max(a, max_sum[a % 3]);
            max_sum[b % 3] = max(b, max_sum[b % 3]);
            max_sum[c % 3] = max(c, max_sum[c % 3]);
        }

        max_sum[0] as i32
    }
}
