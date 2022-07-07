pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::VecDeque;

    let mut visited = vec![vec![false; mat[0].len()]; mat.len()];
    let mut queue = VecDeque::new();

    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            if mat[i][j] == 0 {
                let distance = 0;
                queue.push_back((i, j, distance));
            }
        }
    }

    while let Some((i, j, d)) = queue.pop_front() {
        if visited[i][j] {
            continue;
        }

        visited[i][j] = true;
        mat[i][j] = d;

        if i > 0 {
            queue.push_back((i - 1, j, d + 1));
        }
        if j > 0 {
            queue.push_back((i, j - 1, d + 1));
        }
        if i < mat.len() - 1 {
            queue.push_back((i + 1, j, d + 1));
        }
        if j < mat[0].len() - 1 {
            queue.push_back((i, j + 1, d + 1));
        }
    }

    mat
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_matrix() {
        let mat = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];

        let expected = vec![vec![1, 0, 1], vec![2, 1, 0], vec![2, 1, 0]];

        assert_eq!(update_matrix(mat), expected);
    }

    #[test]
    fn test_update_matrix_2() {
        let mat = vec![
            vec![0, 0, 1, 0, 1, 1, 1, 0, 1, 1],
            vec![1, 1, 1, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 0, 0, 0, 1, 1],
            vec![1, 0, 1, 0, 1, 1, 1, 0, 1, 1],
            vec![0, 0, 1, 1, 1, 0, 1, 1, 1, 1],
            vec![1, 0, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 0, 1, 0, 1, 0, 1],
            vec![0, 1, 0, 0, 0, 1, 0, 0, 1, 1],
            vec![1, 1, 1, 0, 1, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1, 1, 1, 0],
        ];
        let expected = vec![
            vec![0, 0, 1, 0, 1, 2, 1, 0, 1, 2],
            vec![1, 1, 2, 1, 0, 1, 1, 1, 2, 3],
            vec![2, 1, 2, 1, 1, 0, 0, 0, 1, 2],
            vec![1, 0, 1, 0, 1, 1, 1, 0, 1, 2],
            vec![0, 0, 1, 1, 1, 0, 1, 1, 2, 3],
            vec![1, 0, 1, 2, 1, 1, 1, 2, 1, 2],
            vec![1, 1, 1, 1, 0, 1, 0, 1, 0, 1],
            vec![0, 1, 0, 0, 0, 1, 0, 0, 1, 2],
            vec![1, 1, 1, 0, 1, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1, 2, 1, 0],
        ];
        assert_eq!(update_matrix(mat), expected);
    }
}
