use crate::Solution;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        if chars.len() == 0 {
            return 0;
        }

        let chars = chars.as_bytes();
        let mut alpha_num = vec![0; 26];
        for char in chars {
            alpha_num[(char - b'a') as usize] += 1;
        }

        let mut count = 0;
        for word in words.iter() {
            let mut word_alpha_num = vec![0; 26];
            let mut can = true;
            for alpha in word.as_bytes() {
                if word_alpha_num[(alpha - b'a') as usize] == alpha_num[(alpha - b'a') as usize] {
                    can = false;
                    break;
                }
                word_alpha_num[(alpha - b'a') as usize] += 1;
            }
            if can {
                count += word.len();
            }
        }
        count as i32
    }
}
