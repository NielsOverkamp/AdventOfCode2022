use aoc2022_niels_overkamp::common::{self, AOCResult};
use std::result::Result::*;

const DAY: &str = "day1";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut calories = vec![0];
    for calorie in input.iter() {
        if let Ok(calorie) = calorie.parse::<u64>() {
            *calories.last_mut().unwrap() += calorie;
        } else {
            calories.push(0);
        }
    }

    calories.sort_by(|a,b| a.cmp(b).reverse());

    Ok([Some(calories[0].to_string()), Some(calories[0..3].iter().sum::<u64>().to_string())])
}

#[test]
pub fn test_day1() {
    assert!(common::run_test(DAY, &run))
}
