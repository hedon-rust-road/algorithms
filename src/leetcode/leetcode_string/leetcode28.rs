use super::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.is_empty() || haystack.len() < needle.len() {
            return -1;
        }

        let h_bytes = haystack.as_bytes();
        let n_bytes = needle.as_bytes();

        for i in 0..=h_bytes.len() - n_bytes.len() {
            if h_bytes[i] != n_bytes[0] {
                continue;
            }
            let mut ok = true;
            for j in 1..n_bytes.len() {
                if h_bytes[j + i] != n_bytes[j] {
                    ok = false;
                    break;
                }
            }
            if ok {
                return i as i32;
            }
        }

        -1
    }
}
