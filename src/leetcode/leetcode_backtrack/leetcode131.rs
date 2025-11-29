use super::Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        if s.is_empty() {
            return vec![];
        }

        let n = s.len();

        // dp[i][j] 表示 s[i..=j] 是否是回文
        let mut dp = vec![vec![false; n]; n];

        // 判断所有字段是否是回文，缓存起来
        let s_bytes = s.as_bytes();
        for right in 0..n {
            for left in 0..=right {
                if s_bytes[left] == s_bytes[right] && (right - left <= 2 || dp[left + 1][right - 1])
                {
                    dp[left][right] = true
                }
            }
        }

        // 回溯处理
        let mut res = vec![];
        let mut cur = vec![];
        backtrack(&s, 0, &dp, &mut cur, &mut res);
        res
    }
}

fn backtrack(
    s: &str,
    start: usize,
    dp: &Vec<Vec<bool>>,
    current: &mut Vec<String>,
    res: &mut Vec<Vec<String>>,
) {
    if start == s.len() {
        if current.len() > 0 {
            res.push(current.clone());
        }
        return;
    }

    for end in start..s.len() {
        if dp[start][end] {
            current.push(s[start..=end].to_string());
            backtrack(s, end + 1, dp, current, res);
            current.pop();
        }
    }
}
