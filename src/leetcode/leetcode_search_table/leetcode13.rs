use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let s_bytes = s.as_bytes();
        let mut visited = HashSet::new();
        for i in s_bytes.iter().rev() {
            match i {
                b'I' => {
                    if visited.contains(&b'V') || visited.contains(&b'X') {
                        res -= 1;
                    } else {
                        res += 1;
                    }
                }
                b'V' => res += 5,
                b'X' => {
                    if visited.contains(&b'L') || visited.contains(&b'C') {
                        res -= 10;
                    } else {
                        res += 10;
                    }
                }
                b'L' => res += 50,
                b'C' => {
                    if visited.contains(&b'D') || visited.contains(&b'M') {
                        res -= 100;
                    } else {
                        res += 100;
                    }
                }
                b'D' => res += 500,
                b'M' => res += 1000,
                _ => panic!("unreachable!"),
            }
            visited.insert(i);
        }

        res
    }
}
