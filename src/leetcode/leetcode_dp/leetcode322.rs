use super::Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let amount = amount as usize;
        let mut dp = vec![amount + 1; amount + 1];
        dp[0] = 0;

        for j in 1..=amount {
            for coin in &coins {
                let coin = *coin as usize;
                if coin <= j {
                    dp[j] = dp[j].min(dp[j - coin] + 1)
                }
            }
        }

        if dp[amount] > amount {
            -1
        } else {
            dp[amount] as i32
        }
    }
}
