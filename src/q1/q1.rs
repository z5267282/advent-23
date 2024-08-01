use std::fs::read_to_string;

const FILE_PATH: &str = "src/q1/small.txt";

fn open_file() -> Option<String> {
    read_to_string(FILE_PATH).ok()
}

#[allow(unused)]
pub mod part_1 {
    use super::open_file;

    pub fn run() {
        if let Some(contents) = open_file() {
            println!("{}", solve(contents))
        }
    }

    fn solve(contents: String) -> u32 {
        let mut result: u32 = 0;
        for line in contents.lines() {
            let numbers = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();
            let first = numbers.get(0).unwrap_or(&0);
            let last = numbers.get(numbers.len() - 1).unwrap_or(&0);
            result += first * 10 + last;
        }
        result
    }
}

#[allow(unused)]
pub mod part_2 {
    use std::cmp::{max, max_by_key, min, min_by_key};
    use std::collections::HashMap;

    use super::open_file;

    pub fn run() {
        match open_file() {
            Some(contents) => println!("{}", solve(contents)),
            None => println!("could not open file"),
        }
    }

    /*
    find the first and last occurences of all numbers
    string should be considered as numeric counterpart except 0

    */
    fn solve(contents: String) -> i32 {
        let words = HashMap::<i32, &str>::from([
            (1, "one"),
            (2, "two"),
            (3, "three"),
            (4, "four"),
            (5, "five"),
            (6, "six"),
            (7, "seven"),
            (8, "eight"),
            (9, "nine"),
        ]);
        let mut result = 0;
        for line in contents.lines() {
            let (mut left, mut right) =
                (HashMap::<i32, usize>::new(), HashMap::<i32, usize>::new());
            // handle 0 case
            left_find(0, line, "0", &mut left);
            right_find(0, line, "0", &mut right);
            for (number, word) in words.iter() {
                // word
                left_find(*number, line, word, &mut left);
                right_find(*number, line, word, &mut right);

                // number
                let stringified_number = number.to_string();
                left_find(*number, line, &stringified_number, &mut left);
                right_find(*number, line, &stringified_number, &mut right);
            }

            let (tens, _) = left
                .iter()
                .fold(None, |old, (a, b)| match old {
                    None => Some((a, b)),
                    Some((c, d)) => Some(min_by_key((c, d), (a, b), |(_, idx)| *idx)),
                })
                .unwrap_or((&0, &0));
            let (ones, _) = right
                .iter()
                .fold(None, |old, (a, b)| match old {
                    None => Some((a, b)),
                    Some((c, d)) => Some(max_by_key((c, d), (a, b), |(_, idx)| *idx)),
                })
                .unwrap_or((&0, &0));
            result += 10 * tens + ones;
        }
        result
    }

    fn left_find(number: i32, line: &str, word: &str, left: &mut HashMap<i32, usize>) {
        if let Some(idx) = line.find(word) {
            left.entry(number)
                .and_modify(|pos| *pos = min(*pos, idx))
                .or_insert(idx);
        }
    }

    fn right_find(number: i32, line: &str, word: &str, right: &mut HashMap<i32, usize>) {
        if let Some(idx) = line.rfind(word) {
            right
                .entry(number)
                .and_modify(|pos| *pos = max(*pos, idx))
                .or_insert(idx);
        }
    }
}
