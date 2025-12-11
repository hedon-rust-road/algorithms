use super::Solution;

impl Solution {
    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut min = prices[0];

        for i in 1..prices.len() {
            if prices[i] > min {
                res += prices[i] - min;
            }
            min = prices[i];
        }
        res
    }
}
