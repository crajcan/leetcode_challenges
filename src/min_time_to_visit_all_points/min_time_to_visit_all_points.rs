use std::cmp;

pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
     let mut distance = 0;

     for i in 0..points.len() - 1 {
         distance += cmp::max((points[i][0] - points[i+1][0]).abs(), (points[i][1] - points[i+1][1]).abs())
     }

     distance
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_time_to_visit_all_points() {
        assert_eq!(min_time_to_visit_all_points(vec![vec![3, 2]]), 0);
        assert_eq!(min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]), 5);
        assert_eq!(min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]), 7);
    }
}
