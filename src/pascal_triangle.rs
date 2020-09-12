use crate::Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut results: Vec<Vec<i32>> = Vec::with_capacity(num_rows);
        for i in 1..=num_rows {
            let mut row = vec![1; i];
            for j in 1..i - 1 {
                row[j] = results[i - 2][j - 1] + results[i - 2][j];
            }
            results.push(row);
        }
        results
    }
}
