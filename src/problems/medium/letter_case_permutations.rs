pub fn letter_case_permutation(s: String) -> Vec<String> {
    match s.len() {
        0 => vec![],
        1 => {
            let first_char = s.chars().next().unwrap();

            if first_char.is_ascii_alphabetic() {
                vec![s.to_uppercase(), s.to_lowercase()]
            } else {
                vec![s]
            }
        }
        _ => {
            let first_char = s.chars().next().unwrap();

            let possible_firsts = if first_char.is_ascii_alphabetic() {
                if first_char.is_ascii_lowercase() {
                    vec![first_char, first_char.to_ascii_uppercase()]
                } else {
                    vec![first_char, first_char.to_ascii_lowercase()]
                }
            } else {
                vec![first_char]
            };

            let permutations_of_rest = letter_case_permutation(s[1..].to_string());

            let mut result = vec![];
            for possible_first in possible_firsts {
                for permutation_of_rest in &permutations_of_rest {
                    result.push(format!("{}{}", possible_first, permutation_of_rest));
                }
            }

            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_letter_case_permutation() {
        assert_eq!(
            letter_case_permutation("a1b2".to_string()),
            vec![
                "a1b2".to_string(),
                "a1B2".to_string(),
                "A1b2".to_string(),
                "A1B2".to_string()
            ]
        );
    }
}
