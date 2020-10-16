
pub fn max_69_number(num: i32) -> i32 {
    let mut seen_6 = false; 

    num.to_string().chars().fold(String::new(), |mut res, c| {
       if !seen_6 && c == '6' {
         res.push('9');
         seen_6 = true;
       } else {
         res.push(c);
       }

       res
    }).parse().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_69_number() {
        assert_eq!(max_69_number(9999), 9999);
        assert_eq!(max_69_number(9969), 9999);
        assert_eq!(max_69_number(9669), 9969);
    }
}
