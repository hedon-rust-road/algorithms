use super::Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // dp[i] 表示 s[:i] 是否能被 word_dict 组成
        // 截取的时候，从尾巴截取
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for i in 1..=s.len() {
            let sub = &s[..i];
            for word in &word_dict {
                if sub.ends_with(word) {
                    dp[i] = dp[i] || dp[i - word.len()];
                }
            }
        }
        dp[s.len()]
    }
}
