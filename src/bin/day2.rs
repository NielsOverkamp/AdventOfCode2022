use aoc2022_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day2";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let score1: u64 = input.iter().map(|line| match line.as_str() {
            "A X" => 3 + 1,
            "A Y" => 6 + 2,
            "A Z" => 0 + 3,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 6 + 1,
            "C Y" => 0 + 2,
        "C Z" => 3 + 3,
        _ => 0
        }).sum();

    let score2: u64 = input.iter().map(|line| match line.as_str() {
            "A X" => 0 + 3,
            "A Y" => 3 + 1,
            "A Z" => 6 + 2,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 0 + 2,
            "C Y" => 3 + 3,
        "C Z" => 6 + 1,
        _ => 0
        }).sum();


    Ok([Some(score1.to_string()), Some(score2.to_string())])
}

#[test]
pub fn test_day2() {
    assert!(common::run_test(DAY, &run))
}
