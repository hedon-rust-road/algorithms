use super::Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let s = s.trim();

        let mut strs: Vec<&str> = s.split(" ").filter(|&s| s != "").collect();
        let mut left = 0;
        let mut right = strs.len() - 1;

        while left < right {
            strs.swap(left, right);
            left += 1;
            right -= 1;
        }

        strs.join(" ").to_string()
    }
}
