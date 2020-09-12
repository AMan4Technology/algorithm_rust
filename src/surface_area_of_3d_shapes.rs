use crate::Solution;

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut surface = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 {
                    continue;
                }
                surface += 6 * grid[i][j] - 2 * (grid[i][j] - 1);
                if i > 0 {
                    surface -= 2 * grid[i][j].min(grid[i - 1][j]);
                }
                if j > 0 {
                    surface -= 2 * grid[i][j].min(grid[i][j - 1]);
                }
            }
        }
        surface
    }
}
