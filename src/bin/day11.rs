use std::fmt::Display;

use aoc2022_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day11";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

type Num = u64;
type Id = usize;

enum Operation {
    Square,
    Add(Num),
    Mul(Num),
}

impl Operation {
    fn apply(&self, num1: Num) -> Num {
        match self {
            Operation::Square => num1 * num1,
            Operation::Add(num2) => num1 + num2,
            Operation::Mul(num2) => num1 * num2,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Square => write!(f,"^2"),
            Operation::Add(num) => write!(f, "+{}", num),
            Operation::Mul(num) => write!(f, "*{}", num),
        }
    }
}

struct Monkey {
    oper: Operation,
    div_test: Num,
    if_true: Id,
    if_false: Id,
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut items_vec: Vec<(Id, Num)> = vec![];
    let mut monkeys: Vec<Monkey> = vec![];
    for (i, monkey) in input.chunks(7).enumerate() {
        if let [_, items, oper, div_test, if_true, if_false, ..] = monkey {
            let new_items = items
                .strip_prefix("  Starting items: ")
                .ok_or_else(|| format!("Failed to parse monkey at items: {}\n{:?}", items, monkey))
                .and_then(|items| {
                    items
                        .split(", ")
                        .map(|item| {
                            item.parse::<Num>()
                                .map(|n| (i, n))
                                .map_err(|e| e.to_string())
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?;
            items_vec.extend(new_items);

            monkeys.push(Monkey {
                oper: oper
                    .strip_prefix("  Operation: new = old ")
                    .ok_or_else(|| {
                        format!("Failed to parse monkey at oper: {}\n{:?}", oper, monkey)
                    })
                    .and_then(|oper| {
                        if oper.starts_with("* old") {
                            Ok(Operation::Square)
                        } else if let Some(num) = oper.strip_prefix("* ") {
                            num.parse::<Num>()
                                .map(|op| Operation::Mul(op))
                                .map_err(|e| e.to_string())
                        } else if let Some(num) = oper.strip_prefix("+ ") {
                            num.parse::<Num>()
                                .map(|op| Operation::Add(op))
                                .map_err(|e| e.to_string())
                        } else {
                            Err(format!(
                                "Failed to parse monkey at oper: {}\n{:?}",
                                oper, monkey
                            ))
                        }
                    })?,
                div_test: div_test
                    .strip_prefix("  Test: divisible by ")
                    .ok_or_else(|| {
                        format!(
                            "Failed to parse monkey at div_test: {}\n{:?}",
                            div_test, monkey
                        )
                    })
                    .and_then(|div_test| div_test.parse::<Num>().map_err(|e| e.to_string()))?,
                if_true: if_true
                    .strip_prefix("    If true: throw to monkey ")
                    .ok_or_else(|| {
                        format!(
                            "Failed to parse monkey at if_true: {}\n{:?}",
                            if_true, monkey
                        )
                    })
                    .and_then(|if_true| if_true.parse::<Id>().map_err(|e| e.to_string()))?,
                if_false: if_false
                    .strip_prefix("    If false: throw to monkey ")
                    .ok_or_else(|| {
                        format!(
                            "Failed to parse monkey at if_false: {}\n{:?}",
                            if_false, monkey
                        )
                    })
                    .and_then(|if_false| if_false.parse::<Id>().map_err(|e| e.to_string()))?,
            })
        } else {
            return Err(format!("Failted to parse monkey {:?}", monkey).into());
        }
    }
    let mut inspection_counts: Vec<usize> = vec![0; monkeys.len()];
    for (mut monkey_id, mut item) in items_vec.clone().into_iter() {
        let mut i = 0;
        while i < 20 {
            let monkey = &monkeys[monkey_id];
            item = monkey.oper.apply(item) / 3;
            inspection_counts[monkey_id] += 1;
            let new_monkey_id = if item % monkey.div_test == 0 {
                monkey.if_true
            } else {
                monkey.if_false
            };
            if new_monkey_id < monkey_id {
                i += 1;
            }
            monkey_id = new_monkey_id;
        }
    }
    inspection_counts.sort();
    let inspection_count1 =
        (inspection_counts[monkeys.len() - 1] * inspection_counts[monkeys.len() - 2]).to_string();

    let world = monkeys.iter().map(|m| m.div_test).reduce(|d1, d2| d1*d2).unwrap_or(1);

    let mut inspection_counts: Vec<usize> = vec![0; monkeys.len()];
    for (mut monkey_id, mut item) in items_vec.into_iter() {
        let mut i = 0;
        while i < 10000 {
            let monkey = &monkeys[monkey_id];
            item = monkey.oper.apply(item) % world;
            inspection_counts[monkey_id] += 1;
            let new_monkey_id = if item % monkey.div_test == 0 {
                monkey.if_true
            } else {
                monkey.if_false
            };
            if new_monkey_id < monkey_id {
                i += 1;
            }
            monkey_id = new_monkey_id;
        }
    }
    inspection_counts.sort();
    let inspection_count2 =
        (inspection_counts[monkeys.len() - 1] * inspection_counts[monkeys.len() - 2]).to_string();

    Ok([Some(inspection_count1), Some(inspection_count2)])
}

#[test]
pub fn test_day11() {
    assert!(common::run_test(DAY, &run))
}
