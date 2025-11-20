use std::collections::HashMap;

pub fn word_pattern(pattern: String, s: String) -> bool {
    let items: Vec<&str> = s.split_whitespace().collect();
    if pattern.len() != items.len() {
        return false;
    }

    let mut p_to_s_mapping: HashMap<char, &str> = HashMap::new();
    let mut s_to_p_mapping: HashMap<&str, char> = HashMap::new();

    for (char, item) in pattern.chars().zip(items) {
        if let Some(&exist) = p_to_s_mapping.get(&char) {
            if exist != item {
                return false;
            }
        } else {
            p_to_s_mapping.insert(char, item);
        }

        if let Some(&exist) = s_to_p_mapping.get(item) {
            if exist != char {
                return false;
            }
        } else {
            s_to_p_mapping.insert(item, char);
        }
    }
    true
}
