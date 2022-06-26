pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | '}' | ']' => match stack.pop() {
                Some(x) => {
                    if x != c {
                        return false;
                    }
                }
                None => return false,
            },
            _ => (),
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_parentheses() {
        assert_eq!(is_valid("()".to_string()), true);
        assert_eq!(is_valid("()[]{}".to_string()), true);
        assert_eq!(is_valid("(]".to_string()), false);
        assert_eq!(is_valid("([)]".to_string()), false);
        assert_eq!(is_valid("{[]}".to_string()), true);
    }
}
