pub fn add_island(
    grid: &mut Vec<Vec<i32>>,
    islands: &mut Vec<Vec<(usize, usize)>>,
    i: usize,
    j: usize,
) {
    if grid[i][j] == 1 {
        grid[i][j] = 3;
        let island = islands.last_mut().unwrap();
        island.push((i, j));

        let (a, b) = (i as i32, j as i32);
        let neighbors = vec![(a - 1, b), (a + 1, b), (a, b - 1), (a, b + 1)];

        for (x, y) in neighbors {
            if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[0].len() as i32 {
                continue;
            }

            add_island(grid, islands, x as usize, y as usize);
        }
    } else {
        ()
    }
}

pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut islands = vec![];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                islands.push(vec![]);
                add_island(&mut grid, &mut islands, i as usize, j as usize);
            }
        }
    }

    islands.iter().fold(0, |max, island| {
        if max < island.len() {
            island.len()
        } else {
            max
        }
    }) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_area_of_island() {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(max_area_of_island(grid), 6);

        let grid = vec![
            vec![0, 0, 1, 0, 0],
            vec![1, 0, 1, 0, 0],
            vec![0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1],
        ];
        assert_eq!(max_area_of_island(grid), 2);
    }
}
