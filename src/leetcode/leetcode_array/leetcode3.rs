use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let bs = s.as_bytes();
    let mut seen = HashSet::new();
    let mut left = 0;
    let mut max_len = 0;

    for (right, char) in bs.iter().enumerate() {
        while seen.contains(char) {
            seen.remove(&bs[left]);
            left += 1;
        }
        seen.insert(char);
        max_len = max_len.max(right - left + 1);
    }

    max_len as i32
}
