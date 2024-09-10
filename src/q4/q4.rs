#[allow(unused)]
pub mod part_1 {
    const FILE: &str = "src/q4/input.txt";

    use std::collections::HashSet;
    use std::fs;

    pub fn run() {
        match fs::read_to_string(FILE) {
            Err(e) => println!("error reading file : {e}"),
            Ok(contents) => {
                let answer: u32 = contents.lines().map(|line| solve(line)).sum();
                println!("answer : {answer}");
            }
        }
    }

    pub fn solve(card: &str) -> u32 {
        match card.split_once(" | ") {
            None => 0,
            Some((left, right)) => match left.split_once(": ") {
                None => 0,
                Some((_, raw_need)) => {
                    let need = raw_need
                        .split(" ")
                        // .filter_map(|raw| match raw.parse::<u32>() {
                        .filter_map(|raw| raw.parse::<u32>().ok())
                        .collect::<HashSet<u32>>();
                    // probably a smart way to do this with iterator chains
                    // first card wort0h 2^0 (so start on -1)
                    // can't start on -1 due to u32, so just divide final answer by 2
                    let mut common = 0;
                    for h in right.split_ascii_whitespace() {
                        match h.parse::<u32>().ok() {
                            None => continue,
                            Some(n) => {
                                if need.contains(&n) {
                                    common += 1;
                                }
                            }
                        }
                    }
                    2u32.pow(common) / 2
                }
            },
        }
    }
}

#[allow(unused)]
pub mod part_2 {
    const FILE: &str = "src/q4/input.txt";

    use std::collections::{HashMap, HashSet};
    use std::fs;

    /**
     * card 1 : won 2, 3, 4, 5
     * card 2 : won
     */

    // card number -> number of matches
    // then need to know how many times a card appears
    // this is iteratively based on number of matches
    // use queue to process this

    pub fn run() {
        match fs::read_to_string(FILE) {
            Err(e) => println!("error reading file : {e}"),
            Ok(contents) => {
                let mut cards = HashMap::new();
                contents
                    .lines()
                    .for_each(|line| solve(line, &mut cards));
                let answer: i32 = cards
                    .values()
                    .sum();
                dbg!(&cards);
                println!("answer : {answer}");
            }
        }
    }

    fn solve(line: &str, cards: &mut HashMap<i32, i32>) {
        if let Some((card, rest)) = line.split_once(": ") {
            if let Some((left, right)) = rest.split_once(" | ") {
                let need = left
                    .split_ascii_whitespace()
                    .map(&str::parse::<i32>)
                    .map(|i| i.unwrap())
                    .collect::<HashSet<i32>>();
                let have = right
                    .split_ascii_whitespace()
                    .map(&str::parse::<i32>)
                    .map(|i| i.unwrap())
                    .fold(0, |acc, curr|
                        if need.contains(&curr) {
                            acc + 1
                    } else {
                        acc
                    });
                if let Some((_, n)) = card.split_once(' ') {
                    let id = n
                        .trim_start()
                        .trim_end()
                        .parse::<i32>()
                        .unwrap();

                    // win current card
                    cards
                        .entry(id)
                        .and_modify(|v| *v += 1)
                        .or_insert(1);

                    // winning the original and then the copies
                    for copy in (id + 1)..=(id + have) {
                        let curr = cards.get(&id).unwrap().clone();
                        cards
                            .entry(copy)
                            .and_modify(|v| *v += curr)
                            .or_insert(1);
                    }
                }
            }
        }
    }
}
