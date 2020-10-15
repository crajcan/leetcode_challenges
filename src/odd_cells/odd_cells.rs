pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
    let mut matrix = vec![vec![0; m as usize]; n as usize];

    let mut odds = 0;

    for i in 0..n as usize {
        for j in 0..m as usize {
            for index in indices.iter() {
                if index[0] as usize == i { 
                    matrix[i][j] += 1
                }

                if index[1] as usize == j { 
                    matrix[i][j] += 1
                }
            }

            if matrix[i][j] % 2 == 1 { odds += 1 }
        }
    }

    odds
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_odd_cells() {
      assert_eq!(odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]), 0);
      assert_eq!(odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]), 6);
    }
}
