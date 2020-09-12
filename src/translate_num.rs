use crate::Solution;

impl Solution {
    pub fn translate_num(num: i32) -> i32 {
        if num < 0 {
            return 0;
        }

        let num = num.to_string();
        let num = num.as_bytes();
        if num.len() < 2 {
            return 1;
        }

        let mut counts = vec![0; num.len()];
        counts[0] = 1;
        counts[1] = match num[0] {
            b'1' => 2,
            b'2' if num[1] <= b'5' => 2,
            _ => 1,
        };

        for i in 2..num.len() {
            counts[i] = match num[i - 1] {
                b'1' => counts[i - 2] + counts[i - 1],
                b'2' if num[i] <= b'5' => counts[i - 2] + counts[i - 1],
                _ => counts[i - 1],
            }
        }
        println!("{:?}", counts);

        counts[num.len() - 1]
    }
}
