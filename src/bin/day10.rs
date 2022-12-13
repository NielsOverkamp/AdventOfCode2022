use std::{str::FromStr, num::ParseIntError};

use aoc2022_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day10";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

enum Instr {
    NoOp, AddX(i64)
}

impl Instr {
    fn run(&self, reg: u64) -> u64 {
        match self {
            Instr::NoOp => reg,
            Instr::AddX(v) => (reg as i64 + v) as u64,
        }
    }

    fn len(&self) -> u64 {
        match self {
            Instr::NoOp => 1,
            Instr::AddX(_) => 2,
        }
    }
}

impl FromStr for Instr {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            Ok(Self::NoOp)
        } else if let Some(s) = s.strip_prefix("addx ") {
            s.parse::<i64>().map(|v| Self::AddX(v))
        } else {
            panic!("Niels did not bother to implement error handling correctly")
        }
    }
}


pub fn run(input: &Vec<String>) -> AOCResult {
    let mut pc: u64 = 0;
    let mut reg: u64 = 1;
    let mut screen: [bool; 40 * 6] = [false;40 * 6];
    let mut strength: u64 = 0;
    for line in input {
        let instr = line.parse::<Instr>()?;
        let new_pc = pc + instr.len();
        let signal_match = (new_pc + 20) / 40;
        if (pc + 20) / 40 < signal_match {
            // println!("l: {}, pc: {}->{}, sm: {}, reg: {}, s: {}", line, pc, new_pc, signal_match, reg, strength);
            strength += reg * (signal_match * 40  - 20);
        }
        for ptr in pc..new_pc {
            let ptr = ptr % 240;
            let x = ptr % 40;
            if x >= (reg.saturating_sub(1)) && x <= (reg + 1) {
                screen[ptr as usize] = true;
            }
        }
        reg = instr.run(reg);
        pc = new_pc;
    }
    let mut test_str = String::with_capacity(240);
    let mut print_str = String::with_capacity(246);
    for (i, v) in screen.into_iter().enumerate() {
        let c = if v {'#'} else {'.'};
        test_str.push(c);
        print_str.push(c);
        if i % 40 == 39 {
            print_str.push('\n');
        }
    }
    println!("{}", print_str);
    Ok([Some(strength.to_string()), Some(test_str)])
}

#[test]
pub fn test_day10() {
    assert!(common::run_test(DAY, &run))
}
