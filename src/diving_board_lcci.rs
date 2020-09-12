use crate::Solution;

impl Solution {
    pub fn diving_board(shorter: i32, longer: i32, k: i32) -> Vec<i32> {
        if k < 1 {
            return Vec::new();
        }
        if shorter == longer {
            return vec![shorter * k];
        }

        let k = k as usize;
        let mut result = vec![0; k + 1];
        let step = longer - shorter;
        result[0] = k as i32 * shorter;
        for i in 1..=k {
            result[i] = result[i - 1] + step;
        }
        result
    }
}
