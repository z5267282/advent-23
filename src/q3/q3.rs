pub mod part_1 {
    use std::collections::HashSet;

    fn find_symbols(map: &str) -> HashSet<String> {
        map.lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars().enumerate().filter_map(move |(j, chr)| {
                    if chr.is_ascii_digit() || chr == '.' {
                        None
                    } else {
                        Some(format!("{i},{j}"))
                    }
                })
            })
            .collect::<HashSet<String>>()
    }

    fn find_numbers(map: &str) -> Vec<(usize, (usize, usize))> {
        map.lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.match_indices(|chr: char| chr.is_ascii_digit())
                    .map(move |(j, item)| (i, (j, j + item.chars().count())))
            })
            .collect::<Vec<(usize, (usize, usize))>>()
    }

    #[test]
    fn test_find_symbols() {
        let map = ".%.
..$";
        assert_eq!(
            find_symbols(map),
            HashSet::from(["0,1".to_string(), "1,2".to_string()])
        );
    }

    #[test]
    fn test_find_numbers() {
        let map = ".123..
45.%6";

        assert_eq!(
            find_numbers(map),
            vec![(0, (1, 4)), (1, (0, 2)), (1, (4, 5))]
        )
    }
}
