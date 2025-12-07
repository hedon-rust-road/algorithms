use super::Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m * n <= 0 {
            return m * n;
        }
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 2]; m + 2];
        for i in (1..=m).rev() {
            for j in (1..=n).rev() {
                if i == m && j == n {
                    dp[i][j] = 1;
                    continue;
                }
                dp[i][j] = dp[i][j + 1] + dp[i + 1][j]
            }
        }
        dp[1][1]
    }
}
