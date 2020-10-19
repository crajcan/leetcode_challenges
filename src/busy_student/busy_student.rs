pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    (0..start_time.len()).fold(0, |acc, i| {
        if start_time[i] <= query_time && query_time <= end_time[i] {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_busy_student() {
        assert_eq!(busy_student(vec![4], vec![4], 4), 1);
        assert_eq!(busy_student(vec![4], vec![4], 5), 0);
        assert_eq!(busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
    }
}
