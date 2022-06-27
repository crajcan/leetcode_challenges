pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;

    for i in 0..mat.len() {
        for j in 0..mat.len() {
            if i == j || i + j == mat.len() - 1 {
                count += mat[i][j];
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_diagonal_sum() {
        assert_eq!(diagonal_sum(vec![vec![5]]), 5);
        assert_eq!(
            diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            25
        );
        assert_eq!(
            diagonal_sum(vec![
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1]
            ]),
            8
        );
    }
}
