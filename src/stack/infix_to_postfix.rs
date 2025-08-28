use std::collections::HashMap;

use super::par_checker::par_checker;

fn infix_to_postfix(infix: &str) -> Option<String> {
    if !par_checker(infix) {
        return None;
    }

    let mut prec = HashMap::new();
    prec.insert("(", 1);
    prec.insert(")", 1);
    prec.insert("+", 2);
    prec.insert("-", 2);
    prec.insert("*", 3);
    prec.insert("/", 3);

    let mut ops = Vec::new();
    let mut postfix = Vec::new();

    // (A + B) * (C + D) => AB+CD+*

    for token in infix.split_whitespace() {
        // 将数字 0~9 和大写字母 A~Z 入栈
        if ("A" <= token && token <= "Z") || ("0" <= token && token <= "9") {
            postfix.push(token);
        } else if "(" == token {
            // 遇到开始运算符，将运算符入栈
            ops.push(token);
        } else if ")" == token {
            // 遇到结束运算法，将操作数入栈
            let mut top = ops.pop().unwrap();
            while top != "(" {
                postfix.push(top);
                top = ops.pop().unwrap();
            }
        } else {
            // 比较运算法的优先级以决定是否将运算符添加到 postfix 列表中
            while (!ops.is_empty()) && (prec[ops.last().unwrap()] >= prec[token]) {
                postfix.push(ops.pop().unwrap());
            }
            ops.push(token);
        }
    }

    // 将剩下的操作符入栈
    while !ops.is_empty() {
        postfix.push(ops.pop().unwrap());
    }

    // 出栈组成字符串
    let mut postfix_str = "".to_string();
    for c in postfix {
        postfix_str += &c.to_string();
        postfix_str += " ";
    }

    Some(postfix_str)
}

fn post_eval(postfix: &str) -> Option<i32> {
    // A + B
    if postfix.len() < 5 {
        return None;
    }

    let mut ops = vec![];
    for token in postfix.split_whitespace() {
        if "0" <= token && token <= "9" {
            ops.push(token.parse::<i32>().unwrap());
        } else {
            let op2 = ops.pop().unwrap();
            let op1 = ops.pop().unwrap();
            let res = do_calc(token, op1, op2);
            ops.push(res);
        }
    }

    Some(ops.pop().unwrap())
}

fn do_calc(op: &str, op1: i32, op2: i32) -> i32 {
    if "+" == op {
        op1 + op2
    } else if "-" == op {
        op1 - op2
    } else if "*" == op {
        op1 * op2
    } else if "/" == op {
        if 0 == op2 {
            panic!("ZeroDivisionError: Invalid operation!")
        }
        op1 / op2
    } else {
        panic!("OperatorError: Invalid operator: {:?}", op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infix_to_postfix_basic() {
        assert_eq!(infix_to_postfix("A + B"), Some("A B + ".to_string()));
        assert_eq!(infix_to_postfix("A - B"), Some("A B - ".to_string()));
        assert_eq!(infix_to_postfix("A * B"), Some("A B * ".to_string()));
        assert_eq!(infix_to_postfix("A / B"), Some("A B / ".to_string()));
    }

    #[test]
    fn test_infix_to_postfix_with_precedence() {
        assert_eq!(
            infix_to_postfix("A + B * C"),
            Some("A B C * + ".to_string())
        );
        assert_eq!(
            infix_to_postfix("A * B + C"),
            Some("A B * C + ".to_string())
        );
        assert_eq!(
            infix_to_postfix("A / B - C"),
            Some("A B / C - ".to_string())
        );
    }

    #[test]
    fn test_infix_to_postfix_with_parentheses() {
        assert_eq!(
            infix_to_postfix("( A + B ) * C"),
            Some("A B + C * ".to_string())
        );
        assert_eq!(
            infix_to_postfix("A * ( B + C )"),
            Some("A B C + * ".to_string())
        );
        assert_eq!(
            infix_to_postfix("( A + B ) * ( C + D )"),
            Some("A B + C D + * ".to_string())
        );
    }

    #[test]
    fn test_infix_to_postfix_complex() {
        assert_eq!(
            infix_to_postfix("A + B * C - D / E"),
            Some("A B C * + D E / - ".to_string())
        );
        assert_eq!(
            infix_to_postfix("( A + B ) * ( C - D ) / E"),
            Some("A B + C D - * E / ".to_string())
        );
    }

    #[test]
    fn test_infix_to_postfix_with_numbers() {
        assert_eq!(infix_to_postfix("1 + 2"), Some("1 2 + ".to_string()));
        assert_eq!(
            infix_to_postfix("3 * 4 + 5"),
            Some("3 4 * 5 + ".to_string())
        );
        assert_eq!(
            infix_to_postfix("( 1 + 2 ) * 3"),
            Some("1 2 + 3 * ".to_string())
        );
    }

    #[test]
    fn test_infix_to_postfix_single_operand() {
        assert_eq!(infix_to_postfix("A"), Some("A ".to_string()));
        assert_eq!(infix_to_postfix("5"), Some("5 ".to_string()));
    }

    #[test]
    fn test_post_eval_basic() {
        assert_eq!(post_eval("3 4 +"), Some(7));
        assert_eq!(post_eval("10 5 -"), Some(5));
        assert_eq!(post_eval("6 3 *"), Some(18));
        assert_eq!(post_eval("8 2 /"), Some(4));
    }

    #[test]
    fn test_post_eval_complex() {
        assert_eq!(post_eval("3 4 + 2 *"), Some(14));
        assert_eq!(post_eval("15 7 1 1 + - / 3 * 2 1 1 + + -"), Some(5));
        assert_eq!(post_eval("5 1 2 + 4 * + 3 -"), Some(14));
    }

    #[test]
    fn test_post_eval_with_division() {
        assert_eq!(post_eval("9 3 /"), Some(3));
        assert_eq!(post_eval("20 4 /"), Some(5));
        assert_eq!(post_eval("100 10 /"), Some(10));
    }

    #[test]
    #[should_panic]
    fn test_post_eval_division_by_zero() {
        assert_eq!(post_eval("5 0 /"), None);
    }
}
