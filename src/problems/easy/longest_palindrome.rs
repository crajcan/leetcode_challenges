pub fn longest_palindrome(s: String) -> i32 {
    use std::collections::HashMap;
    let freqs = s.chars().fold(HashMap::new(), |mut freqs, c| {
        *freqs.entry(c).or_insert(0) += 1;

        freqs
    });

    let length = freqs.iter().fold(0, |l, (_letter, freq)| {
        if freq % 2 == 1 {
            l + freq - 1
        } else {
            l + freq
        }
    });

    if length < s.len() as i32 {
        return length + 1;
    }

    length
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_longest_palindrome() {
        assert_eq!(longest_palindrome("babad".to_string()), 3);
        assert_eq!(longest_palindrome("cbbd".to_string()), 2);
        assert_eq!(longest_palindrome("abccccdd".to_string()), 3);
    }
}
