pub fn reverse_string(s: &mut Vec<char>) {
    if s.len() == 0 {
        return;
    }
    let mut left: usize = 0;
    let mut right: usize = s.len() - 1;

    while left < right {
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
}
