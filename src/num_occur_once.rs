use crate::Solution;

impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let x = nums.iter().fold(0, |acc, &x| acc ^ x);
        if x == 0 {
            return Vec::new();
        }

        let mut index = 0;
        for i in 0..31 {
            if x & 1 << i != 0 {
                index = 1 << i;
            }
        }

        let (mut a, mut b) = (0, 0);
        nums.iter().for_each(|&num| {
            if num & index == 0 {
                a ^= num;
            } else {
                b ^= num;
            }
        });
        vec![a, b]
    }
}
