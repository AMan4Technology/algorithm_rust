use crate::Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return vec![0; nums.len()];
        }

        let mut counts = vec![0; nums.len()];
        let mut order_array = vec![nums[nums.len() - 1]];
        for i in (0..nums.len() - 1).rev() {
            counts[i] = Self::insert(&mut order_array, nums[i]);
        }
        counts
    }

    fn insert(order_array: &mut Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, order_array.len() - 1);
        if target <= order_array[left] {
            order_array.insert(0, target);
            return 0;
        }
        if order_array[right] < target {
            order_array.push(target);
            return order_array.len() as i32 - 1;
        }

        while left < right {
            let middle = left + (right - left) / 2;
            if order_array[middle] < target {
                left = middle + 1;
            } else if target < order_array[middle] {
                right = middle;
            } else {
                if order_array[middle - 1] < target {
                    right = middle;
                    break;
                }
                right = middle - 1;
            }
        }
        order_array.insert(right, target);
        right as i32
    }
}
