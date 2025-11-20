pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut count = [0; 26];

    for (&s_byte, &t_byte) in s.as_bytes().iter().zip(t.as_bytes()) {
        count[(s_byte - b'a') as usize] += 1;
        count[(t_byte - b'a') as usize] -= 1;
    }

    count.iter().all(|&c| c == 0)
}
