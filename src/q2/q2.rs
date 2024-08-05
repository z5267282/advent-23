const FILE: &str = "src/q2/input.txt";

pub mod part_1 {
    use std::cmp::max;
    use std::collections::HashMap;
    use std::fs::read_to_string;

    use super::FILE;

    pub fn run() {
        match read_to_string(FILE).ok() {
            None => println!("could not open {FILE}"),
            Some(contents) => println!("{}", solve(contents)),
        };
    }

    fn solve(contents: String) -> u32 {
        let mut result = 0;
        for line in contents.lines() {
            if let Some((game, simulations)) = line.split_once(": ") {
                if valid_game(simulations) {
                    result += parse_game_id(game);
                }
            }
        }
        result
    }

    fn parse_game_id(game: &str) -> u32 {
        match game.split_once(" ") {
            None => 0,
            Some((_, number)) => match number.parse::<u32>().ok() {
                Some(id) => id,
                None => 0,
            },
        }
    }

    /**
     * Take the max of each of red, green and blue
     * Limits:
     *  R: 12
     *  G: 13
     *  B: 14
     */
    fn valid_game(simulations: &str) -> bool {
        let mut colours = HashMap::<&str, i32>::from([("red", 0), ("green", 0), ("blue", 0)]);
        for round in simulations.split("; ") {
            for combo in round.split(", ") {
                if let Some((number, colour)) = combo.split_once(" ") {
                    if let Some(parsed) = number.parse::<i32>().ok() {
                        colours.entry(colour).and_modify(|e| *e = max(*e, parsed));
                    }
                }
            }
        }
        let limits = HashMap::<&str, i32>::from([("red", 12), ("green", 13), ("blue", 14)]);
        colours
            .iter()
            .all(|(colour, amount)| match limits.get(colour) {
                Some(limit) => amount <= limit,
                None => false,
            })
    }
}
