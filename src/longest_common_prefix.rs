use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = String::new();
        if strs.is_empty() { return result; }

        let mut short = strs[0].len();
        for s in strs.iter().skip(1) {
            if s.len() < short {
                short = s.len();
            }
        }
        for i in 0..short {
            let curr = &strs[0][i..i + 1];
            for other in strs.iter().skip(1) {
                if *curr != other[i..i + 1] {
                    return result;
                }
            }
            result.push_str(curr)
        }
        result
    }
}