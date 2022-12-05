use std::{ops::Range, num::ParseIntError, str::FromStr};

use aoc2022_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day4";

struct RangePair {
    range1: Range<u64>,
    range2: Range<u64>
}

impl RangePair {
    fn self_contains(&self) -> bool {
        (self.range1.start >= self.range2.start && self.range1.end <= self.range2.end) ||
            (self.range2.start >= self.range1.start && self.range2.end <= self.range1.end)
    }

    fn self_overlaps(&self) -> bool {
        self.self_contains() || self.range2.contains(&self.range1.start) || self.range2.contains(&(self.range1.end - 1))
    }

}

impl FromStr for RangePair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ((r1_start, r1_end), (r2_start, r2_end)) = s.split_once(",")
                                                        .and_then(|(r1, r2)| r1.split_once("-").zip(r2.split_once("-")))
                                                        .unwrap();
        let r1_start = r1_start.parse::<u64>()?;
        let r1_end = r1_end.parse::<u64>()?;
        let r2_start = r2_start.parse::<u64>()?;
        let r2_end = r2_end.parse::<u64>()?;

        Ok(RangePair {
            range1: Range { start: r1_start, end: r1_end + 1 },
            range2: Range { start: r2_start, end: r2_end + 1 }
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let input = input.iter()
                     .map(String::as_str)
                     .map(str::parse::<RangePair>)
                     .map(Result::unwrap);
    let self_contains_count = input
        .clone()
        .map(|rp| rp.self_contains())
        .filter(|b| *b)
        .count();
    let self_overlaps_count = input
        .map(|rp| rp.self_overlaps())
        .filter(|b| *b)
        .count();
    Ok([Some(self_contains_count.to_string()), Some(self_overlaps_count.to_string())])
}

#[test]
pub fn test_day4() {
    assert!(common::run_test(DAY, &run))
}
