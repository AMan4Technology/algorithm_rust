use crate::Solution;

impl Solution {
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return matrix;
        }

        let (row, column) = (matrix.len(), matrix[0].len());
        let mut result = vec![vec![0; column]; row];

        let mut points = Vec::new();
        for i in 0..row {
            for j in 0..column {
                if matrix[i][j] == 0 {
                    points.push((i, j));
                }
            }
        }

        while points.len() != 0 {
            let mut new_points = Vec::new();
            for &(x, y) in points.iter() {
                if x > 0 && matrix[x - 1][y] == 1 && result[x - 1][y] == 0 {
                    result[x - 1][y] = result[x][y] + 1;
                    new_points.push((x - 1, y));
                }
                if x < row - 1 && matrix[x + 1][y] == 1 && result[x + 1][y] == 0 {
                    result[x + 1][y] = result[x][y] + 1;
                    new_points.push((x + 1, y));
                }
                if y > 0 && matrix[x][y - 1] == 1 && result[x][y - 1] == 0 {
                    result[x][y - 1] = result[x][y] + 1;
                    new_points.push((x, y - 1));
                }
                if y < column - 1 && matrix[x][y + 1] == 1 && result[x][y + 1] == 0 {
                    result[x][y + 1] = result[x][y] + 1;
                    new_points.push((x, y + 1));
                }
            }
            points = new_points;
        }

        result
    }
}
