pub fn remove_outer_parentheses(s: String) -> String {
    let mut result = String::new();
    let mut context = 0;

    for c in s.chars() {
        if context != 0 && (context != 1 || c != ')') {
            result.push(c);
        }
        if c == '(' {
            context += 1;
        } else {
            context -= 1;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_outer_parentheses() {
        assert_eq!(remove_outer_parentheses("()()".to_string()), "".to_string());
        assert_eq!(
            remove_outer_parentheses("(()())(())".to_string()),
            "()()()".to_string()
        );
        assert_eq!(
            remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())".to_string()
        );
    }
}
