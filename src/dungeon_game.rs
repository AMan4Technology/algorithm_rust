use crate::Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        if dungeon.is_empty() || dungeon[0].is_empty() {
            return 1;
        }

        let (m, n) = (dungeon.len(), dungeon[0].len());
        let mut life = vec![std::i32::MAX; n];
        life[n - 1] = 1.max(1 - dungeon[m - 1][n - 1]);
        for i in (0..n - 1).rev() {
            life[i] = 1.max(life[i + 1] - dungeon[m - 1][i]);
        }

        for i in (0..m - 1).rev() {
            life[n - 1] = 1.max(life[n - 1] - dungeon[i][n - 1]);
            for j in (0..n - 1).rev() {
                life[j] = 1.max(life[j].min(life[j + 1]) - dungeon[i][j]);
            }
        }
        life[0]
    }
}
