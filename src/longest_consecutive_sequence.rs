use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let mut number = HashSet::with_capacity(nums.len());
        for &num in nums.iter() {
            number.insert(num);
        }

        let mut count = 0;
        for num in nums.iter() {
            if !number.contains(num) {
                continue;
            }
            let mut curr_count = 1;

            let mut i = num - 1;
            while number.contains(&i) {
                curr_count += 1;
                number.remove(&i);
                i -= 1;
            }

            i = num + 1;
            while number.contains(&i) {
                curr_count += 1;
                number.remove(&i);
                i += 1;
            }
            count = count.max(curr_count);
        }
        count
    }
}
