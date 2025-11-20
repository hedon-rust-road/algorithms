use super::Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for token in tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let op1 = stack.pop().unwrap();
                    let op2 = stack.pop().unwrap();
                    match token.as_str() {
                        "+" => stack.push(op2 + op1),
                        "-" => stack.push(op2 - op1),
                        "*" => stack.push(op2 * op1),
                        "/" => stack.push(op2 / op1),
                        _ => panic!("unreachable!"),
                    }
                }
                _ => stack.push(token.parse::<i32>().unwrap()),
            }
        }
        stack[0]
    }
}
