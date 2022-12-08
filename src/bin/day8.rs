use std::collections::HashMap;

use aoc2022_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day8";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}


pub fn run(input: &Vec<String>) -> AOCResult {
    let mut map: HashMap<(usize, usize), u32> = HashMap::new();
    for (y, line) in input.into_iter().enumerate() {
        for (x, height) in line.chars().enumerate() {
            map.insert((x,y), height.to_digit(10).ok_or_else(|| format!("Unexpected char {} at {}:{}", height, y, x))?);
        }
    }

    let (max_x, max_y) = *map.keys().max().ok_or("Empty map")?;
    let is_edge = |x: usize, y: usize| x == 0 || y == 0 || x == max_x || y == max_y;


    let mut visible_count = 0;
    let mut max_scenic = 1;
    for (coord, height) in map.iter() {
        let ((x, y), height) = (*coord, *height);
        let mut visible = false;
        let mut scenic = 1;
        for (dx, dy) in [(0,1),(1,0),(0,-1),(-1,0)] {
            let (mut x, mut y) = (x, y);
            let mut viewing_distance = 0;
            loop {
                if is_edge(x, y) {
                    // println!("{:?} ✔️", (x,y));
                    visible = true;
                    break;
                }
                viewing_distance += 1;
                x = ((x as isize) + dx) as usize;
                y = ((y as isize) + dy) as usize;
                let current_height = *map.get(&(x,y)).ok_or_else(|| format!("No map entry at {}:{}", y, x))?;
                // println!("{} <=? {} @ {:?}->{:?}", current_height, height, (dx, dy), (x,y));
                if height <= current_height {
                    // println!("{:?} ✖️", (x,y));
                    break;
                }
            }
            scenic *= viewing_distance;
        }
        if visible {
            visible_count += 1;
        }
        max_scenic = max_scenic.max(scenic);
    }

    Ok([Some(visible_count.to_string()), Some(max_scenic.to_string())])
}

#[test]
pub fn test_day8() {
    assert!(common::run_test(DAY, &run))
}
