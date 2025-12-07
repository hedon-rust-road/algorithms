use super::Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_iter = s.bytes();
        let mut s_byte = s_iter.next();

        for t_byte in t.bytes() {
            if Some(t_byte) == s_byte {
                s_byte = s_iter.next();
            }
        }
        s_byte.is_none()
    }
}
