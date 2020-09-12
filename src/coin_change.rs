use crate::Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut num = vec![0; amount + 1];
        for i in 1..=amount {
            num[i] = amount + 1;
            for value in coins.iter() {
                if *value as usize > i { continue; };
                num[i] = num[i].min(num[i - *value as usize] + 1);
            }
        }
        if num[amount] > amount { -1 } else { num[amount] as i32 }
    }
}