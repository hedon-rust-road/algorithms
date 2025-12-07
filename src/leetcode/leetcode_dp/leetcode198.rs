use super::Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut dp = vec![0; nums.len() + 1];
        dp[nums.len() - 1] = nums[nums.len() - 1];

        for i in (0..nums.len() - 1).rev() {
            dp[i] = dp[i + 1];
            if nums[i] + dp[i + 2] > dp[i + 1] {
                dp[i] = nums[i] + dp[i + 2]
            }
        }
        dp[0]
    }
}
