use crate::Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut counts = vec![0; n + 1];
        counts[0] = 1;
        counts[1] = 1;

        for i in 2..counts.len() {
            for j in 0..i {
                counts[i] += counts[j] * counts[i - 1 - j]
            }
        }
        counts[n]
    }
}
