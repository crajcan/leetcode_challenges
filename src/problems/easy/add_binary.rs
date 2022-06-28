pub fn add_binary(a: String, b: String) -> String {
    let a_int = u128::from_str_radix(&a, 2).expect("Not a binary number!");
    let b_int = u128::from_str_radix(&b, 2).expect("Not a binary number!");

    format!("{:b}", a_int + b_int)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_binary() {
        assert_eq!(add_binary("11".to_string(), "1".to_string()), "100");
        assert_eq!(add_binary("1010".to_string(), "1011".to_string()), "10101");
        assert_eq!(add_binary("111".to_string(), "1".to_string()), "1000");
    }
}
