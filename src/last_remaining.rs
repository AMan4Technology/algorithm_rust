use crate::Solution;

impl Solution {
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        if n < 1 || m < 1 {
            return -1;
        }
        if n == 1 || m == 1 {
            return n - 1;
        }

        let mut ans = 0;
        for i in 2..=n {
            ans = (m + ans) % i;
        }
        ans
    }
}
