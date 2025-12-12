use super::Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let symbols = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];

        let mut res = "".to_string();

        for i in 0..values.len() {
            while num >= values[i] {
                num -= values[i];
                res.push_str(symbols[i]);
            }
        }

        res
    }
}
