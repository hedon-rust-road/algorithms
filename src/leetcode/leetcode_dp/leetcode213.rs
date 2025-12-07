use super::Solution;

impl Solution {
    pub fn rob_2(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            _ => {
                // 要 nums[0]
                let a = rob_1(&nums[0..nums.len() - 1]);
                // 不要 nums[1]
                let b = rob_1(&nums[1..]);
                a.max(b)
            }
        }
    }
}

fn rob_1(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut dp = vec![0; nums.len() + 1];
    dp[nums.len() - 1] = nums[nums.len() - 1];

    for i in (0..nums.len() - 1).rev() {
        dp[i] = dp[i + 1];
        if nums[i] + dp[i + 2] > dp[i + 1] {
            dp[i] = nums[i] + dp[i + 2];
        }
    }
    dp[0]
}
