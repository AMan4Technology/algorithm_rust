use crate::Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let unit_num = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let unit = vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];

        let mut num = num;

        unit_num.iter().zip(unit.iter()).map(|(&k, &v)| {
            if num < k { return String::new(); }
            let n = num / k;
            num %= k;
            v.repeat(n as usize)
        }).collect()
    }
}
