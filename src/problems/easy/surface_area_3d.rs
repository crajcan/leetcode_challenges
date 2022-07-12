pub fn surface_area(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut height = grid
        .iter()
        .fold(0, |max, row| std::cmp::max(max, *row.iter().max().unwrap()));
    println!("height: {}", height);

    let mut floor = 0;
    let mut walls = 0;
    let mut cieling = 0;

    for level in 0..height {
        for i in 0..grid.len() {
            vec![1, 1, 1]
                .iter()
                .zip(0..)
                .for_each(|c| println!("c: {:?}", c));
            vec![1, 0, 1]
                .iter()
                .zip(0..)
                .for_each(|c| println!("c: {:?}", c));
            vec![1, 1, 1]
                .iter()
                .zip(0..)
                .for_each(|c| println!("c: {:?}", c));

            for j in 0..grid[0].len() {
                println!("walls: {}", walls);
                println!("i, j : {},{}", i, j);
                //  measure floor
                if level == 0 && grid[i][j] != 0 {
                    floor = floor + 1;
                }

                if grid[i][j] > 0 {
                    walls += 4;
                    // top
                    if i > 0 {
                        println!("    checking top neighbor");
                        if grid[i - 1][j] != 0 {
                            println!("        found top neighbor");
                            walls -= 1;
                        };
                    }
                    // right
                    if j < grid[0].len() - 1 {
                        println!("    checking right neighbor");
                        if grid[i][j + 1] != 0 {
                            println!("        found right neighbor");
                            walls -= 1;
                        };
                    }

                    // bottom
                    if i < grid.len() - 1 {
                        if grid[i + 1][j] != 0 {
                            println!("    checking below neighbor");
                            println!("        found below neighbor");
                            walls -= 1;
                        };
                    }
                    // left
                    if j > 0 {
                        println!("    checking left neighbor");
                        if grid[i][j - 1] != 0 {
                            println!("        found left neighbor");
                            walls -= 1;
                        };
                    }
                }

                if grid[i][j] == 1 {
                    cieling += 1;
                }
            }
        }

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] > 0 {
                    grid[i][j] -= 1;
                }
            }
        }
    }

    println!("floor: {}", floor);
    println!("walls: {}", walls);
    println!("cieling: {}", cieling);

    floor + walls + cieling
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_surface_area() {
        assert_eq!(surface_area(vec![vec![2]]), 10);
        assert_eq!(surface_area(vec![vec![1, 2], vec![3, 4]]), 34);
        assert_eq!(surface_area(vec![vec![1, 0], vec![0, 2]]), 16);

        assert_eq!(
            surface_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            32
        );
    }
}
