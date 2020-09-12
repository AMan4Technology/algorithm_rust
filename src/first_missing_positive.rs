use crate::Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut range = Vec::new();
        'num: for &num in nums.iter() {
            if num <= 0 {
                continue;
            }

            if range.is_empty() {
                range.push((num, num));
                continue;
            }

            if range[0].0 <= num && num <= range[0].1 {
                continue;
            }

            if num == range[0].0 - 1 {
                range[0].0 = num;
                continue;
            } else if num < range[0].0 {
                range.insert(0, (num, num));
                continue;
            }

            let len = range.len();
            for i in 1..len {
                if range[i].0 <= num && num <= range[i].1 {
                    continue 'num;
                }

                if num == range[i - 1].1 + 1 {
                    range[i - 1].1 = num;
                    continue 'num;
                }

                if num == range[i].0 - 1 {
                    range[i].0 = num;
                    continue 'num;
                }

                if range[i - 1].1 < num && num < range[i].0 {
                    range.insert(i, (num, num));
                    continue 'num;
                }
            }

            if num == range[len - 1].1 + 1 {
                range[len - 1].1 = num;
            } else if num > range[len - 1].1 {
                range.push((num, num));
            }
        }

        if range.is_empty() || range[0].0 > 1 {
            return 1;
        }

        for i in 1..range.len() {
            if range[i - 1].1 != range[i].0 - 1 {
                return range[i - 1].1 + 1;
            }
        }
        range[range.len() - 1].1 + 1
    }
}
