use std::collections::HashSet;

use aoc2022_niels_overkamp::common::{self, AOCResult};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::u64 as u64_parser,
    combinator::all_consuming,
    error::Error,
    multi::separated_list1,
    sequence::{terminated, tuple},
    Finish, Parser,
};

const DAY: &str = "day14";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let point_parser = tuple((terminated(u64_parser, tag(",")), u64_parser));
    let wall_parser = separated_list1(tag(" -> "), point_parser);
    let mut wall_parser = all_consuming(wall_parser);

    let mut map: HashSet<(u64, u64)> = HashSet::new();

    for line in input {
        let result: Result<Vec<(u64, u64)>, Error<&str>> =
            wall_parser.parse(line.as_str()).finish().map(|(_, l)| l);
        let wall = result.map_err(|e| e.to_string())?;
        for line in wall.windows(2) {
            let (p1, p2) = if let &[p1, p2] = line {
                (p1, p2)
            } else {
                panic!("windows returned not 2 length");
            };
            let (hori, iter) = if p1.0 == p2.0 {
                (false, p1.1.min(p2.1)..=p1.1.max(p2.1))
            } else if p1.1 == p2.1 {
                (true, p1.0.min(p2.0)..=p1.0.max(p2.0))
            } else {
                return Err(format!("Found non straight line: {:?} -> {:?}", p1, p2).into());
            };
            println!("{:?} -> {:?} hori:{}", p1, p2, hori);
            for v in iter.into_iter() {
                let p = if hori { (v, p1.1) } else { (p1.0, v) };
                map.insert(p);
            }
        }
    }

    let death_plane = *map.iter().map(|(_, y)| y).max().unwrap();
    println!("plane: {}", death_plane);

    let floor = death_plane + 2;

    let (min_x, max_x) = map.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_x, max_x) = (*min_x, *max_x);

    for y in 0..=death_plane {
        let s: String = (min_x..=max_x)
            .into_iter()
            .map(|x| if map.contains(&(x, y)) { '#' } else { ' ' })
            .collect();
        println!("{}", s);
    }

    let wall_map = map.clone();

    let (mut x, mut y) = (500, 0);
    let mut count = 0;
    let mut count1 = None;
    let count2;
    loop {
        if y > death_plane && count1.is_none() {
            count1 = Some(count);
        }
        match (
            y + 1 == floor,
            map.contains(&(x, y + 1)),
            map.contains(&(x - 1, y + 1)),
            map.contains(&(x + 1, y + 1)),
        ) {
            (false, false, _, _) => (x, y) = (x, y + 1),
            (false, _, false, _) => (x, y) = (x - 1, y + 1),
            (false, _, _, false) => (x, y) = (x + 1, y + 1),
            _ => {
                map.insert((x, y));
                count += 1;
                if (x, y) == (500, 0) {
                    count2 = count;
                    break;
                }
                (x, y) = (500, 0);
            }
        }
    }
    println!("");

    let (min_x, max_x) = map.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_x, max_x) = (*min_x, *max_x);

    for y in 0..=death_plane {
        let s: String = (min_x..=max_x)
            .into_iter()
            .map(|x| {
                if wall_map.contains(&(x, y)) {
                    '#'
                } else if map.contains(&(x, y)) {
                    '.'
                } else {
                    ' '
                }
            })
            .collect();
        println!("{}", s);
    }

    Ok([Some(count1.unwrap().to_string()), Some(count2.to_string())])
}

#[test]
pub fn test_day14() {
    assert!(common::run_test(DAY, &run))
}
