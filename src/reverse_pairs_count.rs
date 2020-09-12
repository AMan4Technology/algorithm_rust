use crate::Solution;

impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        let mut order_nums = Vec::with_capacity(nums.len());
        order_nums.push(nums[0]);
        let mut count = 0;
        for i in 1..nums.len() {
            let position = Self::find_position(&order_nums, nums[i]);
            if position == order_nums.len() {
                order_nums.push(nums[i]);
                continue;
            }
            count += order_nums.len() - position;
            order_nums.insert(position, nums[i]);
        }
        count as i32
    }

    fn find_position(order_nums: &Vec<i32>, target: i32) -> usize {
        let (mut left, mut right) = (0, order_nums.len() - 1);
        if order_nums[right] <= target {
            return right + 1;
        }
        if target < order_nums[left] {
            return 0;
        }

        let mut position = 0;
        while left <= right {
            let middle = left + (right - left) / 2;
            if order_nums[middle] <= target {
                left = middle + 1;
                position = left;
            } else {
                position = middle;
                right = middle - 1;
            }
        }
        position
    }
}
