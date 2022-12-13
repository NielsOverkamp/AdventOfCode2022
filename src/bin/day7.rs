use aoc2022_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day7";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}


pub fn run(input: &Vec<String>) -> AOCResult {
    let mut stack: Vec<u64> = vec![];
    let mut wd = 0;
    let mut small_sum = 0;
    let mut dir_sizes: Vec<u64> = vec![];
    for line in input {
        if line.starts_with("$ cd /") || line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        } else if line.starts_with("$ cd ..") {
            if wd <= 100000 {
                small_sum += wd;
            }
            dir_sizes.push(wd);
            wd += stack.pop().ok_or("tried to cd .. at empty stack")?;
        } else if line.starts_with("$ cd ") {
            // Assume that we enter ever dir only once
            stack.push(wd);
            wd = 0;
        } else {
            let (size, _) = line.split_once(" ").ok_or_else(|| format!("Expected {{nr}} {{name}}, got {}", line))?;
            wd += size.parse::<u64>()?;
        }
    }
    let root_size;
    loop {
        if wd <= 100000 {
            small_sum += wd;
        }
        dir_sizes.push(wd);
        if let Some(dir) = stack.pop() {
            wd += dir;
        } else {
            root_size = wd;
            break;
        }
    }

    let excess = 30000000 - (70000000 - root_size);

    let mut min = u64::MAX;
    for dir in dir_sizes {
        if dir > excess {
            min = min.min(dir);
        }
    }

    Ok([Some(small_sum.to_string()), Some(min.to_string())])
}

#[test]
pub fn test_day7() {
    assert!(common::run_test(DAY, &run))
}
