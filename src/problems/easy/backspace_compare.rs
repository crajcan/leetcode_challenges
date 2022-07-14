pub fn apply_backspaces(s: &[u8]) -> Vec<u8> {
    s.iter().fold(vec![], |mut new_s, c| {
        if *c != '#' as u8 {
            new_s.push(*c);
        } else {
            new_s.pop();
        }

        new_s
    })
}

pub fn backspace_compare(s: String, t: String) -> bool {
    apply_backspaces(s.as_bytes()) == apply_backspaces(t.as_bytes())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_backspace_compare() {
        assert_eq!(
            backspace_compare("ab#c".to_string(), "ad#c".to_string()),
            true
        );
        assert_eq!(
            backspace_compare("ab##".to_string(), "c#d#".to_string()),
            true
        );
        assert_eq!(
            backspace_compare("a##c".to_string(), "#a#c".to_string()),
            true
        );
        assert_eq!(backspace_compare("a#c".to_string(), "b".to_string()), false);
    }
}
