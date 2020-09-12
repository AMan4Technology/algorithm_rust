use crate::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let (mut counts, mut count) = (vec![0; (b'z' - b'A' + 1) as usize], 0);
        let s = s.as_bytes();
        for &x in s {
            let i = (x - b'A') as usize;
            if counts[i] == 0 {
                counts[i] = 1;
                continue;
            }
            counts[i] = 0;
            count += 2;
        }

        for &value in counts.iter() {
            if value > 0 {
                return count + 1;
            }
        }
        count
    }
}
