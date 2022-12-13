use std::{cmp::Ordering, ops::Index};

use aoc2022_niels_overkamp::common::{self, AOCResult};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as char_parser, u64 as u64_parser},
    combinator::{all_consuming, value},
    error::Error,
    multi::many0,
    sequence::terminated,
    Finish, Parser,
};

const DAY: &str = "day13";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

enum SymbolsOrdering {
    Equal,
    Less,
    Greater,
    RaiseLeftNum(u64),
    RaiseRightNum(u64),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Symbols {
    LBracket,
    RBracket,
    Num(u64),
}

#[derive(PartialEq, Eq, Clone)]
struct SymbolList {
    l: Vec<Symbols>,
}

impl Symbols {
    fn lex<'a>(input: &'a str) -> Result<SymbolList, Error<&'a str>> {
        let l_bracket = value(Self::LBracket, char_parser('['));
        let r_bracket = value(Self::RBracket, char_parser(']'));
        let num = u64_parser.map(|n| Self::Num(n));
        let symbol = alt((l_bracket, r_bracket, num));
        let comma0 = tag(",").or(tag(""));
        let symbol_comma0 = terminated(symbol, comma0);
        let list = many0(symbol_comma0).map(|l| l);

        all_consuming(list)(input)
            .finish()
            .map(|(_, l)| SymbolList { l })
    }

    fn cmp(&self, other: &Self) -> SymbolsOrdering {
        match (self, other) {
            (Symbols::LBracket, Symbols::LBracket) | (Symbols::RBracket, Symbols::RBracket) => {
                SymbolsOrdering::Equal
            }
            (Symbols::LBracket, Symbols::Num(n)) => SymbolsOrdering::RaiseRightNum(*n),
            (Symbols::Num(n), Symbols::LBracket) => SymbolsOrdering::RaiseLeftNum(*n),
            (Symbols::RBracket, _) => SymbolsOrdering::Less,
            (_, Symbols::RBracket) => SymbolsOrdering::Greater,
            (Symbols::Num(nl), Symbols::Num(nr)) => {
                if nl < nr {
                    SymbolsOrdering::Less
                } else if nl > nr {
                    SymbolsOrdering::Greater
                } else {
                    SymbolsOrdering::Equal
                }
            }
        }
    }
}

impl PartialOrd for SymbolList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut left = self.l.iter();
        let mut right = other.l.iter();

        let mut front_log_left: Vec<Symbols> = vec![];
        let mut front_log_right: Vec<Symbols> = vec![];

        while let Some((l, r)) = {
            let l = front_log_left.pop().or_else(|| left.next().map(|s| *s));
            let r = front_log_right.pop().or_else(|| right.next().map(|s| *s));
            l.zip(r)
        } {
            match l.cmp(&r) {
                SymbolsOrdering::Equal => (),
                SymbolsOrdering::Less => {
                    return Some(Ordering::Less);
                }
                SymbolsOrdering::Greater => {
                    return Some(Ordering::Greater);
                }
                SymbolsOrdering::RaiseLeftNum(n) => {
                    front_log_left.extend([Symbols::RBracket, Symbols::Num(n)]);
                }
                SymbolsOrdering::RaiseRightNum(n) => {
                    front_log_right.extend([Symbols::RBracket, Symbols::Num(n)]);
                }
            };
        }
        if front_log_left.len() == 0
            && front_log_right.len() == 0
            && left.next().is_none()
            && right.next().is_none()
        {
            return Some(Ordering::Equal);
        } else {
            return None;
        }
    }
}

pub fn run<'a>(input: &'a Vec<String>) -> AOCResult {
    let mut count = 0;
    let marker1 = SymbolList { l: vec![Symbols::LBracket, Symbols::LBracket, Symbols::Num(2), Symbols::RBracket, Symbols::RBracket] };
    let marker2 = SymbolList { l: vec![Symbols::LBracket, Symbols::LBracket, Symbols::Num(6), Symbols::RBracket, Symbols::RBracket] };
    let mut packets: Vec<SymbolList> = vec![marker1.clone(), marker2.clone()];
    for (i, pair) in input.chunks(3).enumerate() {
        if let [left, right, ..] = pair {
            let left = Symbols::lex(left).map_err(|e| e.to_string())?;
            let right = Symbols::lex(right).map_err(|e| e.to_string())?;

            match left.partial_cmp(&right) {
                Some(Ordering::Less) => {
                    count += i + 1;
                }
                None => panic!("Found indentical inputs: {:?}", pair),
                _ => (),
            }

            packets.push(left);
            packets.push(right);
        }
    }

    packets.sort_by(|a, b| a.partial_cmp(b).expect("invalid comparison"));

    let key = packets.into_iter()
           .enumerate()
           .fold(1, |b, (i, p)| {
               if p == marker1 || p == marker2 {
                   b * (i + 1)
               } else {
                   b
               }
           });

    Ok([Some(count.to_string()), Some(key.to_string())])
}

#[test]
pub fn test_day13() {
    assert!(common::run_test(DAY, &run))
}
