use std::collections::HashSet;

use aoc2022_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day9";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

const R: [i64;4]= [1,0,0,1];
const U: [i64;4]= [0,-1,1,0];
const L: [i64;4]= [-1,0,0,-1];
const D: [i64;4]= [0,1,-1,0];


#[inline]
fn rotate(vec: [i64; 2], dir: [i64;4]) -> [i64; 2] {
    [vec[0]*dir[0] + vec[1]*dir[1], vec[0]*dir[2] + vec[1]*dir[3]]
}

#[inline]
fn subtract(vec1: [i64; 2], vec2: [i64; 2]) -> [i64; 2] {
    [vec1[0]-vec2[0], vec1[1]-vec2[1]]
}

#[inline]
fn add(vec1: [i64; 2], vec2: [i64; 2]) -> [i64; 2] {
    [vec1[0]+vec2[0], vec1[1]+vec2[1]]
}

const LENGTH: usize = 10;

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut visited2: HashSet<[i64;2]> = HashSet::new();
    let mut visited10: HashSet<[i64;2]> = HashSet::new();
    let mut rope = [[0i64,0i64];LENGTH];
    visited2.insert([0,0]);
    visited10.insert([0,0]);
    for line in input {
        let (dir, count) = line.split_once(" ").ok_or("Line with no space encountered")?;
        let count = count.parse::<i64>()?;
        let (dir, r_dir) = match dir {
            "R" => Ok((R, R)),
            "U" => Ok((U, D)),
            "L" => Ok((L, L)),
            "D" => Ok((D, U)),
            c => Err(format!("Unknown direction {}", c)),
        }?;
        for _ in 0..count {
            rope[0] = add(rope[0], rotate([1, 0], r_dir));
            for i in 0..LENGTH-1 {
                let head = &rope[i];
                let next = &rope[i+1];

                let dif = subtract(*head, *next);
                let next_delta = match dif {
                    [ 2,  0]                        => [ 1,  0],
                    [-2,  0]                        => [-1,  0],
                    [ 0,  2]                        => [ 0,  1],
                    [ 0, -2]                        => [ 0, -1],
                    [ 1,  2] | [ 2,  2] | [ 2,  1]  => [ 1,  1],
                    [ 2, -1] | [ 2, -2] | [ 1, -2]  => [ 1, -1],
                    [-1, -2] | [-2, -2] | [-2, -1]  => [-1, -1],
                    [-2,  1] | [-2,  2] | [-1,  2]  => [-1,  1],
                    [_, _] => [0, 0],
                };
                let next = add(*next, next_delta);
                match i {
                    0 => {visited2.insert(next);},
                    8 => {visited10.insert(next);},
                    _ => ()
                }
                // println!("{}: N: {:?}->{:?}, H: {:?} dif: {:?}, td: {:?}", line, rope[i+1], next, head, dif, next_delta);
                rope[i+1] = next;
            }
            // println!("R: {:?}", rope);
        }
    }

    Ok([Some(visited2.len().to_string()), Some(visited10.len().to_string())])
}

#[test]
pub fn test_day9() {
    assert!(common::run_test(DAY, &run))
}
