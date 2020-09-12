use crate::Solution;

impl Solution {
    pub fn min_increment_for_unique(a: Vec<i32>) -> i32 {
        if a.len() < 2 {
            return 0;
        }

        let mut counts = vec![0; 50000];
        for &x in a.iter() {
            counts[x as usize] += 1;
        }

        let mut count = 0;
        for i in 0..49999 as usize {
            if counts[i] < 2 {
                continue;
            }
            counts[i + 1] += counts[i] - 1;
            count += counts[i] - 1;
        }
        count
    }
}
