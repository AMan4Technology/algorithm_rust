use crate::Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut buy = vec![vec![std::i32::MAX; 5]; days.len()];
        buy[0][0] = costs[0];
        buy[0][1] = costs[1];
        buy[0][2] = costs[2];

        let (mut prev_7, mut prev_30) = (0, 0);
        for i in 1..days.len() {
            while days[i] - days[prev_7] >= 7 {
                prev_7 += 1;
            }
            while days[i] - days[prev_30] >= 30 {
                prev_30 += 1;
            }

            let mut min = std::i32::MAX;
            for &x in buy[i - 1].iter() {
                min = min.min(x);
            }

            for j in 0..3 {
                buy[i][j] = min + costs[j];
            }
            if prev_7 < i {
                buy[i][3] = buy[prev_7][1];
            }
            if prev_30 < i {
                buy[i][4] = buy[prev_30][2];
            }
        }

        let mut min = std::i32::MAX;
        for &x in buy[buy.len() - 1].iter() {
            min = min.min(x);
        }
        min
    }
}
