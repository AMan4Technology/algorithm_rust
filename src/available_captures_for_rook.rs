use crate::Solution;

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == 'R' {
                    for k in (0..i).rev() {
                        match board[k][j] {
                            'B' => {
                                break;
                            }
                            'p' => {
                                count += 1;
                                break;
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                    for k in j + 1..board[i].len() {
                        match board[i][k] {
                            'B' => {
                                break;
                            }
                            'p' => {
                                count += 1;
                                break;
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                    for k in i + 1..board.len() {
                        match board[k][j] {
                            'B' => {
                                break;
                            }
                            'p' => {
                                count += 1;
                                break;
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                    for k in (0..j).rev() {
                        match board[i][k] {
                            'B' => {
                                break;
                            }
                            'p' => {
                                count += 1;
                                break;
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                    return count;
                }
            }
        }
        count
    }
}
