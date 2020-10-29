pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    grid.iter().fold(0, |sum, row| {
        sum + row.iter().fold(
            0,
            |row_sum, elem| {
                if elem < &0 {
                    row_sum + 1
                } else {
                    row_sum
                }
            },
        )
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_negatives() {
        assert_eq!(count_negatives(vec![vec![-11]]), 1);

        assert_eq!(count_negatives(vec![vec![1, -1], vec![-1, -1],]), 3);

        assert_eq!(count_negatives(vec![vec![3, 2], vec![1, 0],]), 0);

        assert_eq!(
            count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3]
            ]),
            8
        );
    }
}
