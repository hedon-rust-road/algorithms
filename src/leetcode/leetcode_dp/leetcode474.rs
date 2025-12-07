use super::Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for str in strs {
            let (zero, one) = count(&str);
            for i in (zero..=m).rev() {
                for j in (one..=n).rev() {
                    dp[i][j] = dp[i][j].max(dp[i - zero][j - one] + 1);
                }
            }
        }
        dp[m][n]
    }
}

fn count(str: &str) -> (usize, usize) {
    let mut zero = 0;
    for c in str.as_bytes() {
        if c == &b'0' {
            zero += 1;
        }
    }
    (zero, str.len() - zero)
}
