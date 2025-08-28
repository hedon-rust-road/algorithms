pub fn par_checker(par: &str) -> bool {
    let mut stack = vec![];

    for c in par.chars() {
        match c {
            '[' | '(' | '{' => stack.push(c),
            ']' | ')' | '}' => {
                if !stack.pop().map_or(false, |open| pair_matches(open, c)) {
                    return false;
                }
            }
            _ => continue,
        }
    }

    stack.is_empty()
}

fn pair_matches(open: char, close: char) -> bool {
    let opens = "([{";
    let closes = ")]}";
    return opens.find(open) == closes.find(close);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_par_checker_balanced() {
        assert!(par_checker("()"));
        assert!(par_checker("([])"));
        assert!(par_checker("{[()]}"));
        assert!(par_checker("([{}])"));
        assert!(par_checker(""));
        assert!(par_checker("(((())))"));
    }

    #[test]
    fn test_par_checker_unbalanced() {
        assert!(!par_checker("("));
        assert!(!par_checker(")"));
        assert!(!par_checker("([)]"));
        assert!(!par_checker("{[(])}"));
        assert!(!par_checker("((())"));
        assert!(!par_checker("(()))"));
        assert!(!par_checker("([)"));
    }

    #[test]
    fn test_par_checker_with_non_bracket_chars() {
        assert!(par_checker("a(b[c]{d}e)f"));
        assert!(!par_checker("a(b[c{d}e)f"));
        assert!(par_checker("abc"));
    }
}
