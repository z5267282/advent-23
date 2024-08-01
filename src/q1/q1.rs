use std::fs::read_to_string;

const FILE_PATH: &str = "src/q1/one.txt";

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
    use std::cmp::{max, min};
    use std::collections::HashMap;

    use super::open_file;

    pub fn run() {
        match open_file() {
            Some(contents) => println!("{}", solve(contents)),
            None => println!("could not open file"),
        }
    }

    fn solve(contents: String) -> i32 {
        let words = gen_word_mapping();
        let mut result = 0;
        // first occuring index for the value
        for line in contents.lines() {
            (0..=100).for_each(|i| {
                let mut left = HashMap::<i32, usize>::new();
                // last occuring
                let mut right = HashMap::<i32, usize>::new();
                if let Some(word) = words.get(&i) {
                    if let Some(idx) = line.find(word) {
                        left.entry(i)
                            .and_modify(|pos| *pos = min(*pos, idx))
                            .or_insert(idx);
                    }
                }
                if let Some(word) = words.get(&i) {
                    if let Some(idx) = line.rfind(word) {
                        right
                            .entry(i)
                            .and_modify(|pos| *pos = max(*pos, idx))
                            .or_insert(idx);
                    }
                }
                let word = i.to_string();
                if let Some(idx) = line.find(&word) {
                    left.entry(i)
                        .and_modify(|pos| *pos = min(*pos, idx))
                        .or_insert(idx);
                }
                if let Some(idx) = line.rfind(&word) {
                    right
                        .entry(i)
                        .and_modify(|pos| *pos = max(*pos, idx))
                        .or_insert(idx);
                }
                let mut first = left
                    .iter()
                    .map(|(i, idx)| (*i, *idx))
                    .collect::<Vec<(i32, usize)>>();
                first.sort_by(|(_, b), (_, d)| b.cmp(d));
                let f = first.first().unwrap_or(&(0, 0));
                let (fi, _) = f;
                result += *fi;

                let mut last = right
                    .iter()
                    .map(|(i, idx)| (*i, *idx))
                    .collect::<Vec<(i32, usize)>>();
                last.sort_by(|(a, _), (c, _)| c.cmp(a));
                let l = first.first().unwrap_or(&(0, 0));
                let (li, _) = l;
                result += *li;
            })
        }
        result
    }

    fn gen_word_mapping() -> HashMap<i32, String> {
        let mut j = 1;
        let mut words = HashMap::<i32, String>::new();
        [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .for_each(|word| {
            words.insert(j, word.to_string());
            j += 1;
        });
        words
    }
}
