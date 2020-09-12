use crate::Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid.clone();
        let mut queue = vec![];
        let mut count = 0usize;
        for (i, row) in grid.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                match value {
                    2 => { queue.push((i, j)); }
                    1 => { count += 1; }
                    _ => { continue; }
                }
            }
        }
        let (row_num, column_num) = (grid.len(), grid[0].len());
        let mut time = 0;
        loop {
            let length = queue.len();
            for i in 0..length {
                let (x, y) = queue[i];
                if x > 0 && grid[x - 1][y] == 1 {
                    grid[x - 1][y] = 2;
                    count -= 1;
                    queue.push((x - 1, y));
                }
                if x < row_num - 1 && grid[x + 1][y] == 1 {
                    grid[x + 1][y] = 2;
                    count -= 1;
                    queue.push((x + 1, y));
                }
                if y > 0 && grid[x][y - 1] == 1 {
                    grid[x][y - 1] = 2;
                    count -= 1;
                    queue.push((x, y - 1));
                }
                if y < column_num - 1 && grid[x][y + 1] == 1 {
                    grid[x][y + 1] = 2;
                    count -= 1;
                    queue.push((x, y + 1));
                }
            }
            queue = queue.split_off(length);
            if queue.is_empty() { break; }
            time += 1;
        }
        if count == 0 { time } else { -1 }
    }
}