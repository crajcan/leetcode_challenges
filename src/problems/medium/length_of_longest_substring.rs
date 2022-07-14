pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;

    let chars = s.as_bytes();
    let mut last_positions: HashMap<u8, usize> = HashMap::new();
    let mut substring: &[u8] = &[];
    let mut last_start = 0;

    chars.iter().enumerate().fold(0, |max, (i, c)| {
        let last_position = last_positions.get(c);

        // we havent seen this char, or haven't seen it in this substring
        if last_position.is_none() || *last_position.unwrap() < last_start {
            // record that we saw it
            last_positions.insert(*c, i);

            // add it to the current substring
            substring = &chars[last_start..i + 1];

            // update the running max
            if substring.len() > max {
                substring.len()
            } else {
                max
            }
        } else {
            // the substring already has this char
            // move the substring window past the last occurance
            last_start = last_positions.get(c).unwrap() + 1;
            substring = &chars[last_start..i + 1];

            // record that we saw this char
            last_positions.insert(*c, i);

            max
        }
    }) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("tmmabct".to_string()), 5);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
