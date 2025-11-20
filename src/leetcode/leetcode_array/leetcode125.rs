pub fn is_palindrome(s: String) -> bool {
    let s = s.as_bytes();
    let mut left: usize = 0;
    let mut right: usize = s.len() - 1;

    while left < right {
        if !s[left].is_ascii_alphanumeric() {
            left += 1;
        } else if !s[right].is_ascii_alphanumeric() {
            right -= 1;
        } else if s[left].to_ascii_lowercase() == s[right].to_ascii_lowercase() {
            left += 1;
            right -= 1;
        } else {
            return false;
        }
    }

    true
}
