use crate::Solution;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return -1;
        }

        let (row_num, column_num) = (grid.len(), grid[0].len());
        let mut distances = vec![vec![std::i32::MAX; column_num]; row_num];
        let mut points = Vec::new();
        for i in 0..row_num {
            for j in 0..column_num {
                if grid[i][j] == 1 {
                    distances[i][j] = 0;
                    points.push((i, j));
                }
            }
        }

        let mut max = -1;
        while points.len() != 0 {
            let mut new_points = Vec::with_capacity(points.len());
            for &(x, y) in points.iter() {
                let distance = distances[x][y] + 1;
                if x > 0 && grid[x - 1][y] == 0 && distance < distances[x - 1][y] {
                    distances[x - 1][y] = distance;
                    new_points.push((x - 1, y));
                    max = max.max(distance);
                }
                if x < row_num - 1 && grid[x + 1][y] == 0 && distance < distances[x + 1][y] {
                    distances[x + 1][y] = distance;
                    new_points.push((x + 1, y));
                    max = max.max(distance);
                }
                if y > 0 && grid[x][y - 1] == 0 && distance < distances[x][y - 1] {
                    distances[x][y - 1] = distance;
                    new_points.push((x, y - 1));
                    max = max.max(distance);
                }
                if y < column_num - 1 && grid[x][y + 1] == 0 && distance < distances[x][y + 1] {
                    distances[x][y + 1] = distance;
                    new_points.push((x, y + 1));
                    max = max.max(distance);
                }
            }
            points = new_points;
        }
        max
    }
}
