use crate::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.len() < 2 { return true; }

        let s = s.as_bytes();
        let (mut i, mut j) = (0, s.len() - 1);
        while i < j {
            if !s[i].is_ascii_alphanumeric() {
                i += 1;
                continue;
            }
            if !s[j].is_ascii_alphanumeric() {
                j -= 1;
                continue;
            }
            if s[i].to_ascii_lowercase() != s[j].to_ascii_lowercase() {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}