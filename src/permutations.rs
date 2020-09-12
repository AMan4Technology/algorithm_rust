use crate::Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 { return vec![vec![]]; }

        fn next(nums: &Vec<i32>, indexes: Vec<usize>, results: &mut Vec<Vec<i32>>, mut result: Vec<i32>) {
            if indexes.len() == 1 {
                result.push(nums[indexes[0]]);
                results.push(result);
                return;
            }

            for i in 0..indexes.len() {
                let (mut indexes, mut result) = (indexes.clone(), result.clone());
                result.push(nums[indexes[i]]);
                indexes.remove(i);
                next(nums, indexes, results, result);
            }
        }

        let mut cap = 1;
        let mut indexes = vec![0; nums.len()];
        for i in 0..nums.len() {
            cap *= i + 1;
            indexes[i] = i;
        }
        let mut results = Vec::with_capacity(cap);
        next(&nums, indexes, &mut results, vec![]);
        results
    }
}