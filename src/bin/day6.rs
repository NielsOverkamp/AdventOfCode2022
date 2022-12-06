use std::collections::HashMap;

use aoc2022_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day6";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

#[inline]
fn all_unique(c1: u8, c2: u8, c3: u8, c4: u8) -> bool {
    c1 != c2 && c1 != c3 && c1 != c4 && c2 != c3 && c2 != c4 && c3 != c4
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let s = input.get(0).ok_or("Empty imput")?;
    let mut buf: [u8; 4] = [0,0,0,0];
    let mut buf_i = 0;
    let mut res = None;
    for (c_i, c) in s.bytes().enumerate() {
        if c_i >= 3 && all_unique(c, buf[(buf_i + 1) % 4], buf[(buf_i + 2) % 4], buf[(buf_i + 3) % 4]) {
            res = Some(c_i + 1);
            break
        }
        buf[buf_i] = c;
        buf_i += 1;
        buf_i %= 4;
    }
    let res1 = res.ok_or("No marker found")?;

    let mut hashbuf: HashMap<u8, usize> = HashMap::new();
    let mut res = None;
    for (c_i, c) in s.bytes().enumerate() {
        *hashbuf.entry(c).or_default() += 1;
        if c_i >= 13 {
            if hashbuf.values().all(|v| *v <= 1) {
                res = Some(c_i + 1);
                break;
            }
            let remove_key = s.as_bytes().get(c_i - 13).unwrap();
            *hashbuf.get_mut(remove_key).unwrap() -= 1;
        }
    }

    let res2 = res.ok_or("No message marker found")?;

    Ok([Some(res1.to_string()), Some(res2.to_string())])
}

#[test]
pub fn test_day6() {
    assert!(common::run_test(DAY, &run))
}
