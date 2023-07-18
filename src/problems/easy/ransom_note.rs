pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    use std::collections::HashMap;

    let mut letters = HashMap::new();
      
    for c in magazine.chars() {
        *letters.entry(c).or_insert(0) += 1;
    } 

    for c in ransom_note.chars() {
        match letters.get(&c) {
            None => return false,
            Some(count) => {
                if count == &0 {
                    return false;
                }

                // letters.entry(c).and_modify(|count| *count -= 1);
                letters.insert(c, count - 1);
            }
        }
    }

    true
}

#[cfg(test)]
mod test {
    #[test]
    fn test_can_construct() {
        assert_eq!(super::can_construct("a".to_string(), "b".to_string()), false);
        assert_eq!(super::can_construct("aa".to_string(), "ab".to_string()), false);
        assert_eq!(super::can_construct("aa".to_string(), "aab".to_string()), true);
    }
}