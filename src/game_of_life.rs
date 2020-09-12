use crate::Solution;

impl Solution {
    const DX: [i32; 8] = [0, -1, -1, -1, 0, 1, 1, 1];
    const DY: [i32; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }

        let (row, column) = (board.len() as i32, board[0].len() as i32);
        let (mut live, mut dead) = (Vec::new(), Vec::new());
        for i in 0..row {
            for j in 0..column {
                let mut count = 0;
                for k in 0..8 {
                    let (x, y) = (i + Self::DX[k], j + Self::DY[k]);
                    if 0 <= x
                        && x < row
                        && 0 <= y
                        && y < column
                        && board[x as usize][y as usize] == 1
                    {
                        count += 1;
                    }
                }
                match count {
                    2 => {
                        continue;
                    }
                    3 => {
                        live.push((i, j));
                    }
                    _ => {
                        dead.push((i, j));
                    }
                }
            }
        }

        for &(i, j) in live.iter() {
            board[i as usize][j as usize] = 1;
        }
        for &(i, j) in dead.iter() {
            board[i as usize][j as usize] = 0;
        }
    }
}
