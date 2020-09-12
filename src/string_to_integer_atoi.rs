use crate::Solution;

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        if str.is_empty() { return 0; }
        let str = str.as_bytes();
        let mut positive = true;
        let mut start = 0;
        for i in 0..str.len() {
            match str[i] {
                b'-' => {
                    positive = false;
                    start = i + 1;
                    break;
                }
                b'+' => {
                    start = i + 1;
                    break;
                }
                b'0'..=b'9' => {
                    start = i;
                    break;
                }
                b' ' => { continue; }
                _ => { return 0; }
            }
        }

        let mut result: i64 = 0;
        for i in start..str.len() {
            if let b'0'..=b'9' = str[i] {
                result = result * 10 + (str[i] - b'0') as i64;
                if result < std::i32::MAX as i64 + 1 {
                    continue;
                }
                return if positive { std::i32::MAX } else { std::i32::MIN };
            }
            break;
        }
        if positive { result as i32 } else { -result as i32 }
    }
}