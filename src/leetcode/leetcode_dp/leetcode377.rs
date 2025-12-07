use super::Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![0; target + 1];
        dp[0] = 1; // 凑成 0 有 1 种方法

        // 完全背包有 2 种循环顺序
        // 1. 求组合数（不考虑顺序）：先遍历物品，再遍历背包
        // 2. 求排列数（考虑顺序）：先遍历背包，再遍历物品
        for j in 1..=target {
            for &num in &nums {
                let num = num as usize;
                if num <= j {
                    dp[j] += dp[j - num];
                }
            }
        }

        dp[target]
    }
}
