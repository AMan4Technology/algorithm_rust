use crate::Solution;

impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        let mut results = vec![];
        let (mut start, mut end) = (1, 2);
        let max = (target + 1) / 2;
        let mut sum = 3;

        while end <= max {
            if sum < target {
                end += 1;
                sum += end;
            } else if sum > target {
                sum -= start;
                start += 1;
            } else {
                let mut result = Vec::with_capacity((end - start + 1) as usize);
                for i in start..=end {
                    result.push(i);
                }
                results.push(result);
                sum -= start + start + 1;
                start += 2;
            }
        }

        results
    }
}