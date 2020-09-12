use crate::Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() || matrix[0].is_empty() {
            return;
        }
        Self::rotate_with_range(0, matrix.len() - 1, 0, matrix[0].len() - 1, matrix);
    }

    fn rotate_with_range(
        up: usize,
        down: usize,
        left: usize,
        right: usize,
        matrix: &mut Vec<Vec<i32>>,
    ) {
        if up >= down || left >= right {
            return;
        }

        let mut tmp = vec![0; right - left + 1];
        for i in left..right {
            tmp[i - left] = matrix[up + i - left][right];
            matrix[up + i - left][right] = matrix[up][i]
        }

        for i in up..down {
            let t = matrix[down][right - (i - up)];
            matrix[down][right - (i - up)] = tmp[i - up];
            tmp[i - up] = t;
        }

        for i in (left + 1..=right).rev() {
            let t = matrix[down - (right - i)][left];
            matrix[down - (right - i)][left] = tmp[right - i];
            tmp[right - i] = t;
        }

        for i in (up + 1..=down).rev() {
            matrix[up][left + down - i] = tmp[down - i];
        }

        Self::rotate_with_range(up + 1, down - 1, left + 1, right - 1, matrix);
    }
}
