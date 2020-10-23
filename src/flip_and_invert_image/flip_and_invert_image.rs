pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    a.iter().fold([].to_vec(), |rows, row| {
        [
            rows,
            vec![row
                .iter()
                .rev()
                .fold([].to_vec(), |elems, elem| [elems, vec![elem ^ 1]].concat())],
        ]
        .concat()
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flip_and_invert_image() {
        assert_eq!(
            flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
        );

        assert_eq!(
            flip_and_invert_image(vec![
                vec![1, 1, 0, 0],
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 0]
            ]),
            vec![
                vec![1, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 0, 1, 0]
            ]
        );
    }
}
