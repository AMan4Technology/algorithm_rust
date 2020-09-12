use crate::Solution;

impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        if a.len() < 3 { return false; }
        let mut sum: i32 = a.iter().sum();
        if sum % 3 != 0 { return false; }

        sum /= 3;

        let mut sum_i = 0;
        let mut split = 0;
        for i in 0..a.len() - 2 {
            sum_i += a[i];
            if sum_i == sum {
                split = i;
                break;
            }
        }

        if sum_i != sum { return false; }

        sum_i = 0;
        for i in (split + 2..a.len()).rev() {
            sum_i += a[i];
            if sum_i == sum {
                return true;
            }
        }

        false
    }
}