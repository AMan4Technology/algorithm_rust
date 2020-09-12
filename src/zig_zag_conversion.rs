use crate::Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows < 2 { return s; }

        let mut results = vec![vec![]; num_rows];
        let max_row = num_rows - 1;
        let split = 2 * max_row;
        for (i, v) in s.as_bytes().iter().enumerate() {
            let mut row = i % split;
            if row > max_row {
                row = split - row;
            }
            results[row].push(*v);
        }

        let mut result = String::new();
        for row in results {
            result.push_str(String::from_utf8(row).unwrap().as_str());
        }
        result
    }
}