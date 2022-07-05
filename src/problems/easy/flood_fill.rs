pub fn fill(image: &mut Vec<Vec<i32>>, sr: i32, sc: i32, start_color: i32, color: i32) {
    println!("sr: {}, sc: {}", sr, sc);
    println!("image:\n");

    for row in image.into_iter() {
        println!("{:?}", row);
    }

    if image[sr as usize][sc as usize] == color {
        return;
    }

    if image[sr as usize][sc as usize] != start_color {
        return;
    }

    image[sr as usize][sc as usize] = color;

    let neighbors = vec![(sr - 1, sc), (sr + 1, sc), (sr, sc - 1), (sr, sc + 1)];

    for (x, y) in neighbors {
        if x < 0 || x >= image.len() as i32 || y < 0 || y >= image[0].len() as i32 {
            continue;
        }

        fill(image, x, y, start_color, color)
    }
}

pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let start_color: i32 = image[sr as usize][sc as usize];
    fill(&mut image, sr, sc, start_color, color);

    image
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flood_fill() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let expected = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(flood_fill(image, 1, 1, 2), expected);
    }

    #[test]
    fn test_flood_fill_on_same_color() {
        let image = vec![vec![1, 1, 1], vec![1, 2, 0], vec![1, 0, 1]];
        let expected = vec![vec![1, 1, 1], vec![1, 2, 0], vec![1, 0, 1]];
        assert_eq!(flood_fill(image, 1, 1, 2), expected);
    }

    #[test]
    fn test_flood_fill_next_to_on_same_color() {
        let image = vec![vec![1, 1, 1], vec![1, 2, 0], vec![1, 0, 1]];
        let expected = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(flood_fill(image, 0, 0, 2), expected);
    }
}
