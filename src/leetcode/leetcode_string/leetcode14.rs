use std::usize;

use super::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let mut min_len = usize::MAX;
        let strs_bytes: Vec<&[u8]> = strs
            .iter()
            .map(|s| {
                let s_bytes = s.as_bytes();
                min_len = min_len.min(s_bytes.len());
                s_bytes
            })
            .collect();

        let mut count = 0;
        while count < min_len {
            let cur = strs_bytes[0][count];
            for i in 1..strs_bytes.len() {
                if strs_bytes[i][count] != cur {
                    return strs[0][..count].to_string();
                }
            }
            count += 1;
        }

        strs[0][..count].to_string()
    }
}
