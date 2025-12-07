use super::Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let s_bytes = s.as_bytes();
        if s_bytes[0] == b'0' {
            return 0;
        }

        // dp[i] 表示 s[i:] 的可能情况
        // dp[i] = i + dp[i+1]
        //       = i+(i+1)+dp[i+2]
        let mut dp = vec![0; s_bytes.len() + 1];
        dp[s_bytes.len()] = 1;
        if s_bytes[s_bytes.len() - 1] == b'0' {
            dp[s_bytes.len() - 1] = 0;
        } else {
            dp[s_bytes.len() - 1] = 1;
        }
        for i in (0..s_bytes.len() - 1).rev() {
            if s_bytes[i] == b'0' {
                dp[i] = 0;
                continue;
            }
            // i + dp[i+1]
            dp[i] = dp[i + 1];
            let sub = &s_bytes[i..i + 2];
            // i+(i+1) + dp[i+2]
            if sub[0] == b'1' || (sub[0] == b'2' && b'0' <= sub[1] && sub[1] <= b'6') {
                dp[i] += dp[i + 2]
            }
        }
        dp[0]
    }
}
