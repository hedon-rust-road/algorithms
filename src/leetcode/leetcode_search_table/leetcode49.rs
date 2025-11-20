use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut m: HashMap<[i32; 26], Vec<String>> = HashMap::new();
        for str in strs {
            let t = get_type(&str);
            m.entry(t).or_insert(vec![]).push(str);
        }
        m.into_values().collect()
    }
}

fn get_type(str: &String) -> [i32; 26] {
    let mut result = [0; 26];
    for &c in str.as_bytes() {
        result[(c - b'a') as usize] += 1;
    }
    result
}
