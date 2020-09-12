use crate::Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let (mut left, mut right) = (0, s.len() - 1);
        for i in 0..s.len() / 2 + 1 {
            left = i;
            right = s.len() - 1 - i;
            if s[left] != s[right] {
                break;
            }
        }
        if left >= right {
            return true;
        }

        let mut valid = true;
        for i in left + 1..left + 1 + (right - left) / 2 {
            let j = s.len() - i;
            println!("{}:{}", i, j);
            if s[i] != s[j] {
                valid = false;
                break;
            }
        }
        if valid {
            return true;
        }

        valid = true;
        'inner: for i in left..left + (right - left) / 2 {
            let j = s.len() - 2 - i;
            println!("{}:{}", i, j);

            if s[i] != s[j] {
                valid = false;
                break 'inner;
            }
        }
        valid
    }
}
