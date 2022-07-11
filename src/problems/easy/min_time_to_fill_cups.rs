pub fn cups(amount: &[i32]) -> i32 {
    match (amount[0], amount[1], amount[2]) {
        (0, 0, 0) => 0,
        (_, 0, 0) => 1 + cups(&[amount[0] - 1, 0, 0]),
        (0, _, 0) => 1 + cups(&[0, amount[1] - 1, 0]),
        (0, 0, _) => 1 + cups(&[0, 0, amount[2] - 1]),
        (_, _, 0) => 1 + cups(&[amount[0] - 1, amount[1] - 1, 0]),
        (_, 0, _) => 1 + cups(&[amount[0] - 1, 0, amount[2] - 1]),
        (x, y, z) => {
            let min = amount.iter().min().unwrap();

            if x == *min {
                1 + cups(&[x, y - 1, z - 1])
            } else if y == *min {
                1 + cups(&[x - 1, y, z - 1])
            } else {
                1 + cups(&[x - 1, y - 1, z])
            }
        }
    }
}

pub fn fill_cups(amount: Vec<i32>) -> i32 {
    cups(&amount[..])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cups() {
        assert_eq!(fill_cups(vec![1, 4, 2]), 4);
        assert_eq!(fill_cups(vec![5, 4, 4]), 7);
        assert_eq!(fill_cups(vec![5, 0, 0]), 5);
    }
}
