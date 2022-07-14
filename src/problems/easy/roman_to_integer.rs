pub fn roman_to_integer(s: String) -> i32 {
    let mut total: i32 = 0;
    let bytes = s.as_bytes();

    let mut i: usize = 0;
    while i < bytes.len() {
        let phrase = (bytes[i], bytes.get(i + 1));
        println!(
            "phrase: {:?}, {:?}",
            phrase.0 as char,
            *phrase.1.unwrap_or(&0) as char
        );

        let next = match phrase {
            (b'I', Some(b'V')) => {
                i += 2;
                4
            }
            (b'I', Some(b'X')) => {
                i += 2;
                9
            }
            (b'X', Some(b'L')) => {
                i += 2;
                40
            }
            (b'X', Some(b'C')) => {
                i += 2;
                90
            }
            (b'C', Some(b'D')) => {
                i += 2;
                400
            }
            (b'C', Some(b'M')) => {
                i += 2;
                900
            }
            (b'I', _) => {
                i += 1;
                1
            }
            (b'V', _) => {
                i += 1;
                5
            }
            (b'X', _) => {
                i += 1;
                10
            }
            (b'L', _) => {
                i += 1;
                50
            }
            (b'C', _) => {
                i += 1;
                100
            }
            (b'D', _) => {
                i += 1;
                500
            }
            (b'M', _) => {
                i += 1;
                1000
            }
            _ => 0,
        };
        println!("next: {:?}", next);

        total += next;
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_roman_to_integer() {
        assert_eq!(roman_to_integer("III".to_string()), 3);
        assert_eq!(roman_to_integer("IV".to_string()), 4);
        assert_eq!(roman_to_integer("IX".to_string()), 9);
        assert_eq!(roman_to_integer("LVIII".to_string()), 58);
        assert_eq!(roman_to_integer("MCMXCIV".to_string()), 1994);
    }
}
