pub fn convert_to_title(mut column_number: i32) -> String {
    if column_number == 1 {
        return "A".to_string();
    }

    let mut chars: Vec<u8> = vec![];

    while column_number > 0 {
        column_number -= 1;
        let letter_number = column_number % 26;

        chars.push((letter_number + 65) as u8);

        column_number /= 26
    }

    let chars_reversed: Vec<u8> = chars.into_iter().rev().collect();
    String::from_utf8_lossy(&chars_reversed).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_to_title() {
        assert_eq!(convert_to_title(1), "A");
        assert_eq!(convert_to_title(28), "AB");
        assert_eq!(convert_to_title(701), "ZY");
        assert_eq!(convert_to_title(52), "AZ");
        assert_eq!(convert_to_title(26), "Z");
        assert_eq!(convert_to_title(27), "AA");
        assert_eq!(convert_to_title(28), "AB");
        assert_eq!(convert_to_title(701), "ZY");
        assert_eq!(convert_to_title(52), "AZ");
        assert_eq!(convert_to_title(26), "Z");
        assert_eq!(convert_to_title(27), "AA");
        assert_eq!(convert_to_title(28), "AB");
        assert_eq!(convert_to_title(701), "ZY");
        assert_eq!(convert_to_title(52), "AZ");
        assert_eq!(convert_to_title(26), "Z");
        assert_eq!(convert_to_title(27), "AA");
        assert_eq!(convert_to_title(28), "AB");
        assert_eq!(convert_to_title(701), "ZY");
        assert_eq!(convert_to_title(52), "AZ");
        assert_eq!(convert_to_title(26), "Z");
        assert_eq!(convert_to_title(27), "AA");
        assert_eq!(convert_to_title(28), "AB");
        assert_eq!(convert_to_title(701), "ZY");
        assert_eq!(convert_to_title(52), "AZ");
        assert_eq!(convert_to_title(26), "Z");
    }
}
