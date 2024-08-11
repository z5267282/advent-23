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

    /**
     *
     * -----
     * -l  r
     * -----
     */
    fn is_adjacdent(
        symbols: &HashSet<String>,
        n: u32,
        row: usize,
        start: usize,
        end: usize,
    ) -> bool {
        // we must be very careful to not create a negative number from a usize

        // above
        // check cols start ... end to avoid overflow
        if row > 0 {
            for j in start..=end {
                if symbols.contains(&hash(row - 1, j)) {
                    return true;
                }
            }
        }
        // below
        for j in start..=end {
            if symbols.contains(&hash(row + 1, j)) {
                return true;
            }
        }

        // left column
        if start > 0 {
            if row > 0 {
                if symbols.contains(&hash(row - 1, start - 1)) {
                    return true;
                }
            }

            if symbols.contains(&hash(row, start - 1))
                || symbols.contains(&hash(row + 1, start - 1))
            {
                return true;
            }
        }

        // right column
        if row > 0 {
            if symbols.contains(&hash(row - 1, start + 1)) {
                return true;
            }
        }

        if symbols.contains(&hash(row, start + 1)) || symbols.contains(&hash(row + 1, start + 1)) {
            return true;
        }

        false
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
            // optionally store (start, digits in order)
            // the digits should be a u32 slice, but not sure how to unpack and concat these
            // reverse digits, then calculate powers in bases
            let mut last: Option<(usize, Vec<u32>)> = None;
            for (j, chr) in line.chars().enumerate() {
                match chr.to_digit(10) {
                    None => {
                        match last {
                            // end of a number
                            Some((old, digits)) => {
                                result.push((base_10_number(&digits), (i, old, j)));
                                last = None;
                            }
                            // continuation of no number
                            None => continue,
                        }
                    }
                    Some(n) => {
                        match last {
                            // start of a new number
                            None => last = Some((j, vec![n])),
                            // continuation of existing number
                            Some((old, mut digits)) => {
                                digits.push(n);
                                last = Some((old, digits));
                            }
                        }
                        // last digit means we should add on the number
                        if j == line.chars().count() - 1 {
                            if let Some((old, ref digits)) = last {
                                result.push((base_10_number(digits), (i, old, j + 1)));
                            }
                        }
                    }
                }
            }
        }
        result
    }

    fn base_10_number(digits: &[u32]) -> u32 {
        digits
            .iter()
            .rev()
            .enumerate()
            // cheat and use as
            .map(|(i, digit)| digit * u32::pow(10, i as u32))
            .sum()
    }

    #[test]
    fn test_base_10() {
        assert_eq!(base_10_number(&[1, 2, 3]), 123)
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
            // TODO: fix indices
            vec![(123, (0, 1, 4)), (45, (1, 0, 2)), (6, (1, 4, 5))]
        )
    }
}
