use super::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut min = prices[0];

        for i in 1..prices.len() {
            if prices[i] - min > max {
                max = prices[i] - min;
            }
            if prices[i] < min {
                min = prices[i];
            }
        }

        max
    }
}
