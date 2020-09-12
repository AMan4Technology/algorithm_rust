use crate::Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 { return nums.len() as i32; }

        let mut slice = Vec::with_capacity(nums.len() / 4);
        slice.push(nums[0]);
        for &num in nums.iter().skip(1) {
            if slice[slice.len() - 1] < num {
                slice.push(num);
                continue;
            }
            let (mut start, mut end) = (0, slice.len() - 1);
            while start < end {
                let middle = start + (end - start) / 2;
                if slice[middle] < num {
                    start = middle + 1;
                } else if num < slice[middle] {
                    end = middle;
                } else {
                    end = middle;
                    break;
                }
            }
            slice[end] = num;
        }
        slice.len() as i32
    }
}