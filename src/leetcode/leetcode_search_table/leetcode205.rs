use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_to_t = HashMap::new();
    let mut t_to_s = HashMap::new();
    s.chars().zip(t.chars()).all(|(s_c, t_c)| {
        let s_match = s_to_t.insert(s_c, t_c).map_or(true, |old| old == t_c);
        let t_match = t_to_s.insert(t_c, s_c).map_or(true, |old| old == s_c);
        s_match && t_match
    })
}
