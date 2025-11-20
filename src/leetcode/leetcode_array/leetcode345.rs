pub fn reverse_vowels(s: String) -> String {
    let mut bs = s.into_bytes();
    if bs.len() == 0 {
        return String::from_utf8_lossy(&bs).into_owned();
    }
    let mut left: usize = 0;
    let mut right: usize = bs.len() - 1;

    while left < right {
        if !is_vowel(bs[left]) {
            left += 1;
        } else if !is_vowel(bs[right]) {
            right -= 1;
        } else {
            bs.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    String::from_utf8_lossy(&bs).into_owned()
}

fn is_vowel(s: u8) -> bool {
    match s as char {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}
