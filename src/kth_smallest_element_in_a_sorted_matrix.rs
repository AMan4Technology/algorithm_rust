use crate::Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let k = k as usize;
        if k < 1 || n * n < k {
            return 0;
        }

        let (mut left, mut right) = (matrix[0][0], matrix[n - 1][n - 1]);
        while left < right {
            let mid = left + (right - left) / 2;
            if Self::check(&matrix, mid, k, n) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    fn check(matrix: &Vec<Vec<i32>>, mid: i32, k: usize, n: usize) -> bool {
        let (mut i, mut j) = (n, 0);
        let mut num = 0;
        while i > 0 && j < n {
            if matrix[i - 1][j] <= mid {
                num += i;
                j += 1;
            } else {
                i -= 1;
            }
        }
        num >= k
    }
}
