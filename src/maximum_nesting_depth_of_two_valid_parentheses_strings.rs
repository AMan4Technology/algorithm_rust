use crate::Solution;

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut a_or_b = vec![0; seq.len()];
        let seq = seq.as_bytes();
        for i in 0..seq.len() {
            a_or_b[i] = if seq[i] == b'(' { i & 1 } else { (i + 1) & 1 } as i32
        }
        a_or_b
    }
}
