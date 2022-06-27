pub fn number_of_steps(num: i32) -> i32 {
    match num {
        0 => 0,
        _ => match num % 2 {
            0 => 1 + number_of_steps(num / 2),
            _ => 1 + number_of_steps(num - 1),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_of_steps() {
        assert_eq!(number_of_steps(1), 1);
        assert_eq!(number_of_steps(2), 2);
        assert_eq!(number_of_steps(3), 3);
        assert_eq!(number_of_steps(4), 3);
        assert_eq!(number_of_steps(8), 4);
        assert_eq!(number_of_steps(14), 6);
    }
}
