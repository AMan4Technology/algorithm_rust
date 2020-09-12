use crate::Solution;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 || grid[0].len() == 0 { return 0; }

        fn find_head(islands: &Vec<usize>, i: usize) -> usize {
            let mut result = i;
            while islands[result] != result {
                result = islands[result];
            }
            result
        }

        let (mut counts, mut islands) = (vec![0], vec![0]);
        let mut island_grid = vec![vec![0; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 { continue; }
                let left = if j > 0 && grid[i][j - 1] == 1 {
                    find_head(&islands, island_grid[i][j - 1])
                } else { 0 };
                let up = if i > 0 && grid[i - 1][j] == 1 {
                    find_head(&islands, island_grid[i - 1][j])
                } else { 0 };
                if left == 0 && up == 0 {
                    let num = islands.len();
                    islands.push(num);
                    counts.push(1);
                    island_grid[i][j] = num;
                    continue;
                }
                if up == 0 || left == up {
                    counts[left] += 1;
                    island_grid[i][j] = left;
                    continue;
                }
                if left == 0 {
                    counts[up] += 1;
                    island_grid[i][j] = up;
                    continue;
                }
                islands[up] = left;
                counts[left] += 1 + counts[up];
                island_grid[i][j] = left;
            }
        }

        let mut result = 0;
        for &count in counts.iter().skip(1) {
            result = result.max(count);
        }
        result
    }
}