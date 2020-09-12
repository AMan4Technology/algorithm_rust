use crate::Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let start = s.find('(');
        if start.is_none() || s.len() - start.unwrap() < 2 {
            return 0;
        }

        let s = &s.as_bytes()[start.unwrap()..];
        let mut stack = Vec::with_capacity(s.len());
        let mut counts = vec![0; s.len() + 1];
        let mut max = 0;
        let mut right = 0;

        for i in 0..s.len() {
            match s[i] {
                b'(' => {
                    stack.push(i);
                    right = 0;
                }
                b')' => {
                    if let Some(index) = stack.pop() {
                        counts[index] += 2;
                    } else if right == 0 {
                        right = i;
                        max = max.max(counts.iter().fold(0, |sum, count| sum + count));
                        counts = vec![0; s.len() + 1];
                    }
                }
                _ => {}
            }
        }

        if stack.len() == 0 {
            return max.max(counts.iter().fold(0, |sum, count| sum + count));
        }

        stack.push(counts.len());
        let mut start = 0;
        for &end in stack.iter() {
            let mut count = 0;
            for i in start..end {
                count += counts[i];
            }
            max = max.max(count);
            start = end + 1;
        }
        max
    }
}
