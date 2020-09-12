use crate::Solution;

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        let n = n as usize;
        let mut winner = vec![false; n + 1];

        'o: for i in 2..=n {
            for j in 1..i {
                if i % j == 0 && !winner[i - j] {
                    winner[i] = true;
                    continue 'o;
                }
            }
        }
        winner[n]
    }
}
