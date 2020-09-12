use crate::Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }

        let mut count = 0;
        let (mut left, mut right) = (0, height.len() - 1);
        let (mut left_height, mut right_height) = (height[left], height[right]);
        while left < right {
            if left_height < height[left] {
                left_height = height[left];
                continue;
            }
            if height[right] > right_height {
                right_height = height[right];
                continue;
            }

            if height[left] <= height[right] {
                count += left_height - height[left];
                left += 1;
            } else {
                count += right_height - height[right];
                right -= 1;
            }
        }
        count
    }
}
