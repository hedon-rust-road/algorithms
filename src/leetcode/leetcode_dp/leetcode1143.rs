use super::Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let m = text1.len();
        let n = text2.len();
        if m == 0 || n == 0 {
            return 0;
        }
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();

        for i in 0..m {
            for j in 0..n {
                if text1[i] == text2[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
                }
            }
        }

        dp[m][n]
    }
}
