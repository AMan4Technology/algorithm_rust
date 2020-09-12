use crate::Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        if digits.is_empty() { return vec![]; }
        let (mut result, length) = (vec![1], digits.len());
        for i in 0..length {
            let val = digits[length - 1 - i] + result[i];
            result[i] = val % 10;
            result.push(val / 10);
        }
        if result[result.len() - 1] == 0 { result.pop(); }
        result.reverse();
        result
    }
}