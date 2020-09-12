use crate::Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        if nums.len() < k {
            return 0;
        }

        let mut indexes = Vec::new();
        for i in 0..nums.len() {
            if nums[i] & 1 == 1 {
                indexes.push(i);
            }
        }
        if indexes.len() < k {
            return 0;
        }

        indexes.push(nums.len());

        let mut count = (indexes[0] + 1) * (indexes[k] - indexes[k - 1]);
        let (mut start, mut end) = (1, k);
        while end < indexes.len() - 1 {
            count += (indexes[start] - indexes[start - 1]) * (indexes[end + 1] - indexes[end]);
            end += 1;
            start += 1;
        }
        count as i32
    }
}
