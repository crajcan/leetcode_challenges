pub fn title_to_number(column_title: String) -> i32 {
    let mut bytes = column_title.chars().rev().collect::<Vec<char>>();
    let mut total = 0;

    while !bytes.is_empty() {
        let byte = bytes.pop().unwrap();

        total *= 26;
        total += (byte as i32) - 64;
    }

    total as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_title_to_number() {
        assert_eq!(title_to_number("A".to_string()), 1);
        assert_eq!(title_to_number("AB".to_string()), 28);
        assert_eq!(title_to_number("ZY".to_string()), 701);
    }
}
