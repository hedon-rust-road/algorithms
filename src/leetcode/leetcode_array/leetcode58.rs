use super::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.trim_end();
        let strs = s.split_whitespace();
        strs.last().unwrap_or("").len() as i32
    }
}
