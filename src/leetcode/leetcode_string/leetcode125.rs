use super::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut left = 0;
        let mut right = s.len() - 1;
        let s = s.to_lowercase();
        let s_bytes = s.as_bytes();
        while left < right {
            if !s_bytes[left].is_ascii_alphanumeric() {
                left += 1;
            } else if !s_bytes[right].is_ascii_alphanumeric() {
                right -= 1;
            } else if s_bytes[left] != s_bytes[right] {
                return false;
            } else {
                left += 1;
                right -= 1;
            }
        }
        true
    }
}
