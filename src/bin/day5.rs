use aoc2022_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day5";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut stacks: Vec<Vec<u8>> = vec![];
    let mut input_iter = input.into_iter();
    let mut operations: Vec<(usize, usize, usize)> = vec![];
    'line: loop {
        let line = input_iter.next();
        if line.is_none() {
            break
        }
        let line = line.unwrap();
        for (container, i) in line.as_bytes().chunks(4).zip(0..) {
            if i >= stacks.len() {
                stacks.push(vec![]);
            }
            match container {
                &[b' ', b' ', b' ', b' '] => (),
                &[b'[', c,    b']'] | &[b'[', c,    b']', b' '] => stacks[i].insert(0, c),
                &[_, b'1', _, _]          => break 'line,
                _ => eprintln!("Unknown container {:?}", container)
            }
        }
    }

    input_iter.next();

    for line in input_iter {
        let (amount, from, to) = line.strip_prefix("move ")
            .and_then(|s| s.split_once(" from "))
            .and_then(|(amount, s)| s.split_once(" to ")
                      .map(|(from, to)| (amount, from, to)))
            .ok_or_else(|| format!("Could not parse line {}", line))?;

        let (amount, from, to) = (amount.parse::<usize>()?, from.parse::<usize>()? - 1, to.parse::<usize>()? - 1);
        operations.push((amount, from, to))
    }

    let mut stacks1 = stacks.clone();

    for (amount, from, to) in operations.iter() {
        let from_stack = stacks1.get_mut(*from).ok_or_else(|| format!("From stack does not exist in operation {:?}", (amount, from, to)))?;

        let picked_up: Vec<_> = from_stack.drain((from_stack.len() - amount)..).collect();

        let to_stack = stacks1.get_mut(*to).ok_or_else(|| format!("To stack does not exist in operation {:?}", (amount, from, to)))?;
        to_stack.extend(picked_up.iter().rev());
    }
    let message1: String = stacks1.into_iter().map(|mut s| s.pop().unwrap() as char).collect();

    let mut stacks2 = stacks;

    for (amount, from, to) in operations.iter() {
        let from_stack = stacks2.get_mut(*from).ok_or_else(|| format!("From stack does not exist in operation {:?}", (amount, from, to)))?;

        let picked_up: Vec<_> = from_stack.drain((from_stack.len() - amount)..).collect();

        let to_stack = stacks2.get_mut(*to).ok_or_else(|| format!("To stack does not exist in operation {:?}", (amount, from, to)))?;
        to_stack.extend(picked_up);
    }

    let message2: String = stacks2.into_iter().map(|mut s| s.pop().unwrap() as char).collect();

    Ok([Some(message1), Some(message2)])
}

#[test]
pub fn test_day5() {
    assert!(common::run_test(DAY, &run))
}
