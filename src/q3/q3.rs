const FILE: &str = "src/q3/input.txt";

#[allow(unused)]
pub mod part_1 {
    use std::collections::HashSet;
    use std::fs::read_to_string;

    use super::FILE;

    pub fn run() {
        match read_to_string(FILE).ok() {
            None => println!("could not read {FILE}"),
            Some(map) => println!("{}", solve(map.as_str())),
        }
    }

    fn solve(map: &str) -> u32 {
        let symbols = find_symbols(map);
        let numbers = find_numbers(map);

        numbers
            .iter()
            .filter(|(n, (row, start, end))| is_adjacdent(&symbols, *n, *row, *start, *end))
            .map(|(n, _)| n)
            .sum()
    }

    fn is_adjacdent(
        symbols: &HashSet<String>,
        n: u32,
        row: usize,
        start: usize,
        end: usize,
    ) -> bool {
        for j in start - 1..=end + 1 {
            // above
            // below
            for i in [row - 1, row + 1] {
                if symbols.contains(&hash(i, j)) {
                    return true;
                }
            }
        }
        // left, right
        [start - 1, end + 1]
            .iter()
            .any(|j| symbols.contains(&hash(row, *j)))
    }

    fn hash(row: usize, col: usize) -> String {
        format!("{row},{col}")
    }

    fn find_symbols(map: &str) -> HashSet<String> {
        map.lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars().enumerate().filter_map(move |(j, chr)| {
                    if chr.is_ascii_digit() || chr == '.' {
                        None
                    } else {
                        Some(hash(i, j))
                    }
                })
            })
            .collect::<HashSet<String>>()
    }

    /**
     * returns: [(number), (row, start, end)]
     */
    fn find_numbers(map: &str) -> Vec<(u32, (usize, usize, usize))> {
        let mut result = Vec::<(u32, (usize, usize, usize))>::new();
        for (i, line) in map.lines().enumerate() {
            // optionally store (start, total, last power of 10), then use pow to calculate
            let mut last: Option<(usize, u32, u32)> = None;
            for (j, chr) in line.chars().enumerate() {
                match chr.to_digit(10) {
                    None => {
                        match last {
                            // end of a number
                            Some((old, total, _)) => result.push((total, (i, old, j))),
                            // continuation of no number
                            None => continue,
                        }
                    }
                    Some(n) => {
                        match last {
                            // start of a new number
                            None => last = Some((j, n, 0)),
                            // continuation of existing number
                            Some((old, total, power)) => {
                                let (totalled, powered) = (total + n.pow(power + 1), power + 1);
                                // last digit means we should add on the number
                                if j == line.chars().count() - 1 {
                                    result.push((totalled, (i, old, j)));
                                }
                                last = Some((old, totalled, powered))
                            }
                        }
                        // end the old number
                    }
                }
            }
            if let Some((old, total, _)) = last {
                result.push((total, (i, old, line.chars().count() - 1)));
            }
        }
        result
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
    fn test_find_numbers_small() {
        let map = ".123..";
        assert_eq!(find_numbers(map), vec![(123, (0, 1, 4))])
    }

    #[test]
    fn test_find_numbers() {
        let map = ".123..
45.%6";

        assert_eq!(
            find_numbers(map),
            vec![(123, (0, 1, 4)), (45, (1, 0, 1)), (6, (1, 4, 5))]
        )
    }
}
