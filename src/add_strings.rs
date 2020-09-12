use crate::Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let (short, long) = if num1.len() <= num2.len() {
            (num1.as_bytes(), num2.as_bytes())
        } else {
            (num2.as_bytes(), num1.as_bytes())
        };
        let mut result = vec![0; long.len() + 1];

        for i in 1..=short.len() {
            let z = result[i - 1] + short[short.len() - i] - b'0' + long[long.len() - i] - b'0';
            result[i - 1] = z % 10;
            result[i] = z / 10;
        }

        for i in short.len()..long.len() {
            result[i] += long[long.len() - i - 1] - b'0';
            result[i + 1] = result[i] / 10;
            result[i] %= 10;
        }

        if result[result.len() - 1] == 0 {
            result.remove(result.len() - 1);
        }
        let mut s = String::with_capacity(result.len());
        for &x in result.iter().rev() {
            s.push(char::from(x + b'0'));
        }
        s
    }
}
