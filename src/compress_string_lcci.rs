use crate::Solution;

impl Solution {
    pub fn compress_string(s: String) -> String {
        if s.len() < 3 { return s; }

        let mut result = String::with_capacity(s.len() / 2);
        let mut chars = s.chars();
        let (mut curr, mut count) = (chars.next().unwrap(), 1);
        while let Some(char) = chars.next() {
            if char == curr {
                count += 1;
                continue;
            }
            result.push(curr);
            result.push_str(count.to_string().as_str());
            curr = char;
            count = 1;
        }
        result.push(curr);
        result.push_str(count.to_string().as_str());

        if result.len() < s.len() { result } else { s }
    }
}