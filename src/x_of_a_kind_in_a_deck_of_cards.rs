use crate::Solution;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut counts = vec![0; 10000];
        for &value in deck.iter() {
            counts[value as usize] += 1;
        }

        let mut min = -1;
        for &v in counts.iter() {
            if v == 0 {
                continue;
            }
            if v < 2 {
                return false;
            }
            if min == -1 || min == v {
                min = v;
                continue;
            }
            let divisor = Self::greatest_common_divisor(min, v);
            if divisor == 1 {
                return false;
            }
            min = divisor;
        }
        true
    }
}
