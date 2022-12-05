use aoc2022_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day3";

const OFFSET_LOWER_BYTE: u8 = b'a'; // 0x61
const OFFSET_UPPER_BYTE: u8 = b'A'; // 0x41

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut running_sum = 0;
    let mut running_group_sum = 0;
    let mut group_i = 0;
    let mut group_union = !0;
    for line in input {
        let line = line.as_bytes();
        let mut left = 0u64;
        for b in &line[..line.len()/2] {
            let prio = if *b > OFFSET_LOWER_BYTE {
               *b - OFFSET_LOWER_BYTE
            } else {
                *b - OFFSET_UPPER_BYTE + 26
            };
            left |= 1 << (prio as u64);
        }
        let mut right = 0u64;
        for b in &line[line.len()/2..] {
            let prio = if *b > OFFSET_LOWER_BYTE {
               *b - OFFSET_LOWER_BYTE
            } else {
                *b - OFFSET_UPPER_BYTE + 26
            };
            right |= 1 << (prio as u64);
        }

        let duplicate = left & right;
        let mut check = 1;
        let mut prio = None;
        for i in 1..53 {
            if duplicate == check {
                prio = Some(i);
                break;
            }
            check <<= 1;
        }
        let prio = prio.unwrap();
        running_sum += prio;

        let union = left | right;
        group_union &= union;
        println!("{}, {}", union, group_union);
        let mut check = 1;
        if group_i == 2 {
            group_i = 0;
            for i in 1..53 {
                if group_union == check {
                    running_group_sum += i;
                    break;
                }
                check <<= 1;
            }
            group_union = !0;
        } else {
            group_i += 1;
        }

    }
    Ok([Some(running_sum.to_string()), Some(running_group_sum.to_string())])
}

#[test]
pub fn test_day3() {
    assert!(common::run_test(DAY, &run))
}
