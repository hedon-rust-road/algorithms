use core::str;

use super::Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digit_to_letters = [
            " ", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];
        if digits.is_empty() {
            return vec![];
        }
        let mut res = vec![];
        helper(&digits, &digit_to_letters, &mut res, "");
        res
    }
}

fn helper(digits: &str, dight_to_letters: &[&str; 10], res: &mut Vec<String>, value: &str) {
    if value.len() == digits.len() {
        res.push(value.to_string());
        return;
    }

    let c = digits.as_bytes()[value.len()];
    let letters = dight_to_letters[(c - b'0') as usize];
    for letter in letters.chars() {
        let mut value = value.to_string();
        value.push(letter);
        helper(digits, dight_to_letters, res, &value);
    }
}
