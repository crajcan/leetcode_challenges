pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    match num_rows {
        1 => vec![vec![1]],
        2 => vec![vec![1], vec![1, 1]],
        _ => {
            let mut new_row = vec![];

            let mut above_rows = generate(num_rows - 1);
            let above_row = above_rows.last().unwrap();

            new_row.push(1);

            for i in 0..above_row.len() - 1 {
                new_row.push(above_row[i] + above_row[i + 1]);
            }

            new_row.push(1);

            above_rows.append(&mut vec![new_row]);

            above_rows
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pascals_triangle() {
        assert_eq!(generate(1), vec![vec![1]]);
        assert_eq!(generate(2), vec![vec![1], vec![1, 1]]);
        assert_eq!(generate(3), vec![vec![1], vec![1, 1], vec![1, 2, 1]]);
        assert_eq!(
            generate(4),
            vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]]
        );
        assert_eq!(
            generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
