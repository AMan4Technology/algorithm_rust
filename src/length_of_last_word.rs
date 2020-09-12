use crate::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut start, mut count) = (false, 0);
        for i in (0..s.len()).rev() {
            if s[i] == b' ' {
                if start {
                    return count;
                }
                continue;
            }
            start = true;
            count += 1;
        }
        count
    }
}