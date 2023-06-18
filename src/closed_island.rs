use super::Solution;

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        fn find_head_island_point(
            island: &Vec<Vec<(usize, usize)>>,
            (row, column): (usize, usize),
        ) -> (usize, usize) {
            match island[row][column] {
                (0, 0) => (0, 0),
                (i, j) if i == row && j == column => (row, column),
                _ => find_head_island_point(island, island[row][column]),
            }
        }

        if grid.len() <= 2 || grid[0].len() <= 2 {
            return 0;
        }

        let mut island = vec![vec![(0, 0); grid[0].len()]; grid.len()];
        for row in 1..grid.len() {
            for column in 1..grid[0].len() {
                let curr = grid[row][column];
                if curr == 1 {
                    continue;
                }

                let (left, up) = (grid[row][column - 1], grid[row - 1][column]);
                match (left, up) {
                    (1, 1) => {
                        island[row][column] = (row, column);
                    }
                    (1, 0) => {
                        island[row][column] = find_head_island_point(&island, (row - 1, column));
                    }
                    (0, 1) => {
                        island[row][column] = find_head_island_point(&island, (row, column - 1));
                    }
                    (0, 0) => {
                        let left_point = find_head_island_point(&island, (row, column - 1));
                        let up_point = find_head_island_point(&island, (row - 1, column));
                        if up_point == (0, 0) || left_point == (0, 0) {
                            island[row][column] = (0, 0);
                            island[up_point.0][up_point.1] = (0, 0);
                            island[left_point.0][left_point.1] = (0, 0);
                        } else {
                            island[row][column] = left_point;
                            if left_point != up_point {
                                island[up_point.0][up_point.1] = left_point;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        for row in 1..grid.len() {
            if island[row][island[row].len() - 1] == (0, 0) {
                continue;
            }
            let head_point = find_head_island_point(&island, (row, island[row].len() - 1));
            island[head_point.0][head_point.1] = (0, 0);
        }
        for column in 1..grid[grid.len() - 1].len() {
            if island[grid.len() - 1][column] == (0, 0) {
                continue;
            }
            let head_point = find_head_island_point(&island, (grid.len() - 1, column));
            island[head_point.0][head_point.1] = (0, 0);
        }

        let mut result = 0;
        for row in 1..grid.len() - 1 {
            for column in 1..grid[0].len() - 1 {
                if island[row][column] == (row, column) {
                    result += 1;
                }
            }
        }
        result
    }
}
