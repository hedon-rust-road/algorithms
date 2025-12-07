use super::Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        // pos - neg = target
        // pos + neg = sum
        // 2pos = target + sum
        // pos = (target + sum) / 2
        // 在 nums 里面找到一组数和为 (target + sum) / 2 的情况，转为 0-1 背包
        let sum = nums.iter().sum();
        if target.abs() > sum || (target + sum) % 2 != 0 {
            return 0;
        }

        let bag_size = ((target + sum) / 2) as usize;
        let mut dp = vec![0; bag_size + 1];
        dp[0] = 1;

        for num in nums {
            for i in (num as usize..=bag_size).rev() {
                dp[i] += dp[i - num as usize];
            }
        }

        dp[bag_size]
    }
}
