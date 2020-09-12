use crate::Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() < 2 { return; }

        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}