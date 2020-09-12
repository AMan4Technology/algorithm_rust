use crate::Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m < 1 || n < 1 { return 0; }

        let (m, n) = (m as usize, n as usize);
        let mut count = Vec::with_capacity(n);
        count.push(vec![1; m]);
        for _ in 1..n {
            let mut row = vec![0; m];
            row[0] = 1;
            count.push(row);
        }

        for i in 1..n {
            for j in 1..m {
                count[i][j] = count[i - 1][j] + count[i][j - 1];
            }
        }
        count[n - 1][m - 1]
    }
}