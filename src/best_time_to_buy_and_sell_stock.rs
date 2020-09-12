use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() { return 0; }
        let (mut result, mut min) = (0, prices[0]);
        for &price in prices.iter().skip(1) {
            if min >= price {
                min = price;
                continue;
            }
            result = result.max(price - min);
        }
        result
    }
}