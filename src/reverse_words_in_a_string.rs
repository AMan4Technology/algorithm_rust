use crate::Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut words = Vec::new();
        let mut s = s.chars();
        let mut word = String::new();
        while let Some(c) = s.next() {
            if c != ' ' {
                word.push(c);
                continue;
            }
            if word.len() != 0 {
                words.push(word);
                word = String::new();
            }
        }
        if word.len() != 0 {
            words.push(word);
        }
        words.reverse();
        words.join(" ")
    }
}
