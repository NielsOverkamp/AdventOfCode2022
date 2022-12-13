use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc2022_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day12";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

enum Dirs {
    Left,
    Up,
    Right,
    Down,
}

impl Dirs {
    fn apply(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Dirs::Left => (x.wrapping_sub(1), y),
            Dirs::Up => (x, y + 1),
            Dirs::Right => (x + 1, y),
            Dirs::Down => (x, y.wrapping_sub(1)),
        }
    }
}

const DIRECTIONS: [Dirs; 4] = [Dirs::Left, Dirs::Up, Dirs::Right, Dirs::Down];

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut map: HashMap<(usize, usize), u8> = HashMap::new();
    let mut start: Option<(usize, usize)> = None;
    let mut goal: Option<(usize, usize)> = None;
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            map.insert(
                (x, y),
                match c {
                    b'S' => 0,
                    b'E' => 25,
                    c => c - b'a',
                },
            );
            match c {
                b'S' => start = Some((x, y)),
                b'E' => goal = Some((x, y)),
                _ => (),
            }
        }
    }
    let start = start.ok_or("No start found")?;
    let goal = goal.ok_or("No end found")?;

    // A* Search
    let h = |(x, y)| goal.0.abs_diff(x) + goal.1.abs_diff(y);

    let mut frontier = BinaryHeap::from([Reverse((h(start), start))]);
    let mut visited = HashSet::from([start]);
    let mut g_scores = HashMap::from([(start, 0usize)]);
    let mut f_scores = HashMap::from([(start, h(start))]);
    let mut length = None;

    while let Some(Reverse((_, pos))) = frontier.pop() {
        if pos == goal {
            length = g_scores.get(&goal);
            break;
        }
        let height = *map.get(&pos).unwrap();
        let g_score = *g_scores.get(&pos).unwrap();
        for dir in DIRECTIONS {
            let neighbour = dir.apply(pos);
            if let Some(_) = map.get(&neighbour).filter(|hn| **hn <= height + 1) {
                if *g_scores.get(&neighbour).unwrap_or(&usize::MAX) > g_score + 1 {
                    g_scores.insert(neighbour, g_score + 1);
                    let f_score = g_score + 1 + h(neighbour);
                    f_scores.insert(neighbour, f_score);
                    if !visited.contains(&neighbour) {
                        frontier.push(Reverse((f_score, neighbour)));
                        visited.insert(neighbour);
                    }
                }
            }
        }
    }

    let length = length.ok_or("No path to goal found")?;

    let mut frontier = BinaryHeap::from([Reverse((0, goal))]);
    let mut visited = HashSet::from([goal]);
    let mut shortest_path = usize::MAX;

    while let Some(Reverse((length, pos))) = frontier.pop() {
        if *map.get(&pos).unwrap() == 0 {
            shortest_path = shortest_path.min(length);
        }
        let height = *map.get(&pos).unwrap();
        for neighbour in DIRECTIONS
            .iter()
            .map(|d| d.apply(pos))
            .filter(|pos| !visited.contains(pos))
            .filter(|pos| map.get(pos).map(|h| (*h + 1) >= height).unwrap_or(false))
            .collect::<Vec<_>>()
        {
            frontier.push(Reverse((length + 1, neighbour)));
            visited.insert(neighbour);
        }
    }

    Ok([Some(length.to_string()), Some(shortest_path.to_string())])
}

#[test]
pub fn test_day12() {
    assert!(common::run_test(DAY, &run))
}
