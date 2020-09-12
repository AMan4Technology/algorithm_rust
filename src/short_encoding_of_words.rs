use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        if words.is_empty() {
            return 0;
        }
        if words.len() == 1 {
            return (words[0].len() + 1) as i32;
        }

        let mut words = words;
        words.sort_by(|a, b| {
            let (len_a, len_b) = (a.len(), b.len());
            for i in 0..len_a.min(len_b) {
                let (a, b) = (&a[len_a - 1 - i..len_a - i], &b[len_b - 1 - i..len_b - i]);
                if a > b {
                    return Ordering::Greater;
                } else if a < b {
                    return Ordering::Less;
                }
            }
            len_a.cmp(&len_b)
        });

        let mut count = 0;
        for i in 0..words.len() - 1 {
            if words[i + 1].ends_with(&words[i]) {
                continue;
            }
            count += words[i].len() + 1;
        }
        (count + words[words.len() - 1].len() + 1) as i32
    }
}
