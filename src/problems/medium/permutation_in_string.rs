pub fn freqs(bytes: &[u8]) -> Vec<usize> {
    let mut result = vec![0; 26];

    for byte in bytes {
        result[(byte - 97) as usize] += 1;
    }

    result
}

pub fn check_inclusion(s1: String, s2: String) -> bool {
    use std::cmp::Ordering;
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let s1_freqs = freqs(s1_bytes);

    match s1.len().cmp(&s2.len()) {
        Ordering::Equal => s1_freqs == freqs(s2_bytes),
        Ordering::Greater => false,
        Ordering::Less => {
            for start in 0..=s2.len() - s1.len() {
                if s1_freqs == freqs(&s2_bytes[start..start + s1.len()]) {
                    return true;
                }
            }

            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_inclusion() {
        assert_eq!(
            check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );
        assert_eq!(check_inclusion("cbd".to_string(), "abdc".to_string()), true);
        assert_eq!(
            check_inclusion("xbd".to_string(), "abdc".to_string()),
            false
        );
        assert_eq!(
            check_inclusion("bob".to_string(), "bobcat".to_string()),
            true
        );
    }
}
