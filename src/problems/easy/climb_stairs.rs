pub fn climb_stairs_helper(n: usize, answers: &mut Vec<Option<i32>>) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        x => match answers[x] {
            Some(a) => a,
            None => {
                let answer =
                    climb_stairs_helper(n - 1, answers) + climb_stairs_helper(n - 2, answers);
                answers[x] = Some(answer);
                answer
            }
        },
    }
}

pub fn climb_stairs(n: i32) -> i32 {
    climb_stairs_helper(n as usize, &mut vec![None; (n + 1) as usize])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
        assert_eq!(climb_stairs(5), 8);
        assert_eq!(climb_stairs(6), 13);
        assert_eq!(climb_stairs(7), 21);
        assert_eq!(climb_stairs(8), 34);
        assert_eq!(climb_stairs(9), 55);
        assert_eq!(climb_stairs(10), 89);
        assert_eq!(climb_stairs(11), 144);
        assert_eq!(climb_stairs(12), 233);
        assert_eq!(climb_stairs(13), 377);
        assert_eq!(climb_stairs(14), 610);
        assert_eq!(climb_stairs(15), 987);
        assert_eq!(climb_stairs(16), 1597);
        assert_eq!(climb_stairs(17), 2584);
        assert_eq!(climb_stairs(18), 4181);
        assert_eq!(climb_stairs(19), 6765);
        assert_eq!(climb_stairs(20), 10946);
        assert_eq!(climb_stairs(21), 17711);
        assert_eq!(climb_stairs(22), 28657);
        assert_eq!(climb_stairs(23), 46368);
        assert_eq!(climb_stairs(24), 75025);
        assert_eq!(climb_stairs(25), 121393);
        assert_eq!(climb_stairs(26), 196418);
        assert_eq!(climb_stairs(27), 317811);
        assert_eq!(climb_stairs(28), 514229);
        assert_eq!(climb_stairs(29), 832040);
        assert_eq!(climb_stairs(30), 1346269);
    }
}
