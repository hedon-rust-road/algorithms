use super::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for r in s.chars() {
            match r {
                '[' | '(' | '{' => {
                    stack.push(r);
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => panic!("unreachable!"),
            }
        }
        stack.len() == 0
    }
}
