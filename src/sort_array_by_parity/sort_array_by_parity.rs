use std::cmp::Ordering;

pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
    a.sort_by(|x, y| match (x % 2, y % 2) {
        (0, 1) => Ordering::Less,
        (1, 0) => Ordering::Greater,
        (_, _) => Ordering::Equal,
    });

    a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_array_by_parity() {
        assert_eq!(sort_array_by_parity(vec![]), vec![]);
        assert_eq!(sort_array_by_parity(vec![1]), vec![1]);
        assert_eq!(sort_array_by_parity(vec![2]), vec![2]);
        assert_eq!(sort_array_by_parity(vec![1, 2]), vec![2, 1]);
        assert_eq!(sort_array_by_parity(vec![2, 1]), vec![2, 1]);
        assert_eq!(sort_array_by_parity(vec![3, 1, 2, 4]), vec![2, 4, 3, 1]);
    }
}
