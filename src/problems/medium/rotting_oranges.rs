pub fn rot(grid: &mut Vec<Vec<i32>>, next_gen: &mut Vec<Vec<i32>>, freshes: &mut i32) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            match grid[i][j] {
                0 => next_gen[i][j] = 0,
                1 => {
                    *freshes += 1;

                    let mut neighbors = vec![];

                    if i > 0 {
                        neighbors.push(grid[i - 1][j]);
                    }
                    if j > 0 {
                        neighbors.push(grid[i][j - 1]);
                    }
                    if i < grid.len() - 1 {
                        neighbors.push(grid[i + 1][j]);
                    }
                    if j < grid[0].len() - 1 {
                        neighbors.push(grid[i][j + 1]);
                    }

                    next_gen[i][j] = 1;
                    for neighbor in neighbors {
                        if neighbor == 2 {
                            next_gen[i][j] = 2;
                        }
                    }
                }
                2 => next_gen[i][j] = 2,
                _ => panic!("bad state"),
            }
        }
    }
}

pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut next_gen = vec![vec![0; grid[0].len()]; grid.len()];

    let mut count = 0;

    loop {
        let mut freshes = 0;

        rot(&mut grid, &mut next_gen, &mut freshes);

        if freshes == 0 {
            break; //we're done, everything is rotten
        } else {
            //we saw freshes but couldn't change them
            if next_gen == grid {
                return -1;
            }

            //tick
            count += 1;

            std::mem::swap(&mut grid, &mut next_gen);
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_oranges_rotting_0() {
        let grid = vec![vec![1, 2]];
        assert_eq!(oranges_rotting(grid), 1);
    }
    #[test]
    fn test_oranges_rotting_1() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(oranges_rotting(grid), 4);
    }

    #[test]
    fn test_oranges_rotting_2() {
        let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
        assert_eq!(oranges_rotting(grid), -1);
    }

    #[test]
    fn test_oranges_rotting_3() {
        let grid = vec![vec![0, 2], vec![1, 1]];
        assert_eq!(oranges_rotting(grid), 2);
    }

    #[test]
    fn test_oranges_rotting_4() {
        let grid = vec![vec![0, 1, 2, 2], vec![1, 1, 1, 2], vec![0, 1, 2, 1]];
        assert_eq!(oranges_rotting(grid), 3);
    }
}
