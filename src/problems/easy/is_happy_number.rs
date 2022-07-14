use std::collections::HashSet;

pub fn happy_helper(n: i32, mut seen: HashSet<i32>) -> bool {
    let digits = format!("{}", n)
        .as_bytes()
        .iter()
        .map(|byte| (byte - 48) as i32)
        .collect::<Vec<i32>>();

    let sum_of_squares = digits.iter().fold(0, |sum, d| sum + d * d);

    if sum_of_squares == 1 {
        return true;
    }

    if seen.contains(&sum_of_squares) {
        return false;
    } else {
        seen.insert(sum_of_squares);
        return happy_helper(sum_of_squares, seen);
    }
}

pub fn is_happy(n: i32) -> bool {
    happy_helper(n, HashSet::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert_eq!(true, is_happy(1));
        assert_eq!(true, is_happy(19));
        assert_eq!(false, is_happy(4));
        assert_eq!(false, is_happy(20));
        assert_eq!(false, is_happy(16));
        assert_eq!(false, is_happy(26));
    }
}
