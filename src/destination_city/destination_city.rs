fn vec_unzip(vec_of_vecs: Vec<Vec<String>>) -> (Vec<String>, Vec<String>) {
    let tuples: Vec<(String, String)> = vec_of_vecs
        .iter()
        .map(|path| (path[0].clone(), path[1].clone()))
        .collect();

    tuples.iter().cloned().unzip()
}

pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let (origins, mut destinations) = vec_unzip(paths);

    origins
        .iter()
        .for_each(|origin| destinations.retain(|destination| destination != origin));

    destinations[0].clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dest_city() {
        assert_eq!(
            dest_city(vec![vec!["A".to_string(), "Z".to_string()]]),
            "Z".to_string()
        );

        assert_eq!(
            dest_city(vec![
                vec!["B".to_string(), "C".to_string()],
                vec!["D".to_string(), "B".to_string()],
                vec!["C".to_string(), "A".to_string()]
            ]),
            "A".to_string()
        );

        assert_eq!(
            dest_city(vec![
                vec!["London".to_string(), "New York".to_string()],
                vec!["New York".to_string(), "Lima".to_string()],
                vec!["Lima".to_string(), "Sao Paulo".to_string()]
            ]),
            "Sao Paulo".to_string()
        );
    }
}
