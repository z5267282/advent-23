use std::fs::read_to_string;

pub fn solve() -> Option<u32> {
    let contents = read_to_string("src/q1/input.txt").ok()?;
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
    Some(result)
}
