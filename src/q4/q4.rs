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
    use std::collections::HashMap;

    // card number -> number of matches
    // then need to know how many times a card appears
    // this is iteratively based on number of matches
    // use queue to process this

    // matches: card -> no
    pub fn solve(matches: HashMap<u32, u32>) -> u32 {
        let mut copies = HashMap::<u32, u32>::new();

        todo!()
    }
}
