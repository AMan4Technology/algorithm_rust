use crate::Solution;

impl Solution {
    pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
        if a.is_empty() {
            return 0;
        }

        let mut count = 0;
        let mut sums = vec![0; k as usize];
        let mut important = Vec::new();
        for i in 0..a.len() {
            let curr = a[i] % k;
            if curr == 0 {
                if sums[0] == 0 {
                    important.push(0);
                }
                sums[0] += 1;
                count += sums[0];
                continue;
            }

            let curr = if curr < 0 { curr + k } else { curr } as usize;

            let mut new_sums = vec![0; k as usize];
            let mut new_important = Vec::with_capacity(important.len());
            new_sums[curr] = 1;
            new_important.push(curr);
            for &x in important.iter() {
                let new_sum = (x + curr) % k as usize;
                if new_sum == 0 {
                    count += sums[x];
                }
                new_sums[new_sum] += sums[x];
                if new_sum != curr {
                    new_important.push(new_sum);
                }
            }
            sums = new_sums;
            important = new_important;
        }
        count
    }
}
