use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut results = vec![];
        if digits.is_empty() { return results; }

        let mut map = HashMap::new();
        map.insert(b'2', vec!["a", "b", "c"]);
        map.insert(b'3', vec!["d", "e", "f"]);
        map.insert(b'4', vec!["g", "h", "i"]);
        map.insert(b'5', vec!["j", "k", "l"]);
        map.insert(b'6', vec!["m", "n", "o"]);
        map.insert(b'7', vec!["p", "q", "r", "s"]);
        map.insert(b'8', vec!["t", "u", "v"]);
        map.insert(b'9', vec!["w", "x", "y", "z"]);

        fn deal(prefix: String, digits: &[u8], i: usize, length: usize, results: &mut Vec<String>, map: &HashMap<u8, Vec<&str>>) {
            if i == length {
                results.push(prefix);
                return;
            }
            for x in map.get(&digits[i]).unwrap().iter() {
                deal(prefix.clone() + x, digits, i + 1, length, results, map);
            }
        }

        deal(String::new(), digits.as_bytes(), 0, digits.len(), &mut results, &map);
        results
    }
}