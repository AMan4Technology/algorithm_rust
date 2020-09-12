use crate::Solution;

impl Solution {
    pub fn ways_to_change(n: i32) -> i32 {
        if n < 5 {
            return 1;
        }

        const MAX: i64 = 1000000007;
        let n = n as i64;
        let mut count = 0;
        for i in 0..=n / 25 {
            let rest = n as i64 - 25 * i;
            let (coin_10, coin_5) = (rest / 10, rest % 10 / 5);
            count += (coin_10 + 1) * (coin_10 + coin_5 + 1) % MAX;
            count %= MAX;
        }
        count as i32
    }
}
