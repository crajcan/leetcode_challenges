pub fn subtract(a: i32, b:i32) -> i32 {
  a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 2), 3);
    }
}
