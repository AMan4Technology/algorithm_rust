use crate::Solution;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut important = Vec::new();
        let mut count = 0u8;
        for i in 0..s.len() {
            match s[i] {
                b'a' | b'e' | b'i' | b'o' | b'u' => {
                    important.push(i);
                    count ^= s[i];
                }
                _ => continue,
            }
        }
        Self::visit(s, 0, s.len(), &important, count, 0)
    }

    fn visit(s: &[u8], start: usize, end: usize, important: &[usize], count: u8, max: i32) -> i32 {
        if max > (end - start) as i32 {
            return max;
        }
        if important.len() == 0 || count == 0 {
            return (end - start) as i32;
        }

        Self::visit(
            s,
            start,
            important[important.len() - 1],
            &important[..important.len() - 1],
            count ^ s[important[important.len() - 1]],
            Self::visit(
                s,
                important[0] + 1,
                end,
                &important[1..],
                count ^ s[important[0]],
                max,
            ),
        )
    }
}
