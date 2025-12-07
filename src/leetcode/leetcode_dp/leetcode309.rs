use super::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let n = prices.len();

        // 三个状态
        let mut hold = -prices[0]; // 第一天买入
        let mut sold = 0; // 第一天不可能卖出
        let mut rest = 0; // 第一天不操作

        for i in 1..n {
            let prev_hold = hold;
            let prev_sold = sold;
            let prev_rest = rest;

            // 更新第 i 天状态

            // i 是 hold，要么之前就是 hold，要么就是静默后当天买入
            hold = prev_hold.max(prev_rest - prices[i]);

            // i 是 sold，必须是当天卖出
            sold = prev_hold + prices[i];

            // i 是 rest，要么就是 rest，要么就是 sold 进入 rest
            rest = prev_rest.max(prev_sold);
        }

        // 最后一天，要么卖出，要么静默
        sold.max(rest)
    }
}
