use aoc2022_niels_overkamp::common::{self, AOCResult};
use nom::{
    bytes::complete::tag,
    character::{complete::i128 as i128_parser, streaming::char},
    combinator::all_consuming,
    error::Error,
    sequence::{preceded, separated_pair},
    Finish, Parser,
};
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

const DAY: &str = "day15";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

type Num = i128;
type Point = (Num, Num);
type Line = (Point, Point);

fn intersection(((x1, y1), (x2, y2)): Line, ((x3, y3), (x4, y4)): Line) -> Option<Point> {
    let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    let tn = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
    let un = (x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2);
    let b = if d > 0 {
        tn > d || un > d || tn < 0 || un < 0
    } else {
        tn < d || un < d || tn > 0 || un > 0
    };
    if b || d == 0 {
        return None;
    }
    let temp = tn * (x2 - x1);
    if temp % d != 0 {
        return None;
    }

    Some((x1 + tn * (x2 - x1) / d, y1 + tn * (y2 - y1) / d))
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut line_parser = {
        let v_parser = |c| preceded(char(c).and(char('=')), i128_parser);
        let point_parser = || separated_pair(v_parser('x'), tag(", "), v_parser('y'));
        let sensor_parser = preceded(tag("Sensor at "), point_parser());
        let beacon_parser = preceded(tag("closest beacon is at "), point_parser());
        let line_parser = separated_pair(sensor_parser, tag(": "), beacon_parser);
        all_consuming(line_parser)
    };

    let check_height = input.last().unwrap().parse::<Num>()?;

    let check_size = check_height * 2;
    let check_area = 0..=check_size;
    let mut l_lines = HashSet::new();
    let mut r_lines = HashSet::new();
    let mut oor_intersections: HashMap<Point, usize> = HashMap::new();
    let mut sensors = vec![];

    let mut check_line = HashSet::new();
    let mut check_line_beacons = HashSet::new();

    let filter_add = |i: Option<Point>, i_map: &mut HashMap<Point, usize>| {
        i.filter(|i| check_area.contains(&i.0) && check_area.contains(&i.1))
            .map(|i| *i_map.entry(i).or_default() += 1)
    };

    for line in &input[..input.len() - 1] {
        let (sensor, beacon) = line_parser(line.as_str())
            .finish()
            .map(|(_, l)| l)
            .map_err(|e: Error<&str>| e.to_string())?;

        let range = sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1);
        let dist = check_height.abs_diff(sensor.1);

        let range = range as Num;
        let dist = dist as Num;

        // Part 2

        sensors.push((sensor, range));

        let corners = [
            (sensor.0 + range + 1, sensor.1),
            (sensor.0, sensor.1 + range + 1),
            (sensor.0 - range - 1, sensor.1),
            (sensor.0, sensor.1 - range - 1),
        ];

        let r_line1 = (corners[0], corners[1]);
        let l_line1 = (corners[1], corners[2]);
        let r_line2 = (corners[2], corners[3]);
        let l_line2 = (corners[3], corners[0]);

        for l_line in l_lines.iter() {
            filter_add(intersection(*l_line, r_line1), &mut oor_intersections);
            filter_add(intersection(*l_line, r_line2), &mut oor_intersections);
        }
        for r_line in r_lines.iter() {
            filter_add(intersection(*r_line, l_line1), &mut oor_intersections);
            filter_add(intersection(*r_line, l_line2), &mut oor_intersections);
        }

        l_lines.extend(&[l_line1, l_line2]);
        r_lines.extend(&[r_line1, r_line2]);

        // Part 1
        let radius = (range - dist).max(-1);
        check_line.extend(
            Range {
                start: (sensor.0 - radius),
                end: (sensor.0 + radius) + 1,
            }
            .into_iter(),
        );

        if beacon.1 == check_height {
            check_line_beacons.insert(beacon.0);
        }
    }

    let possible_locations = oor_intersections.iter().filter(|(_, n)| **n >= 4);
    println!("l={:?}", possible_locations.clone().map(|(i, _)| i).collect::<Vec<_>>().as_slice());
    let mut location = None;
    for (pos, _) in possible_locations {
        let mut in_range = false;
        for (sensor, range) in sensors.iter() {
            if (sensor.0.abs_diff(pos.0) + sensor.1.abs_diff(pos.1)) as Num <= *range {
                in_range = true;
                break;
            }
        }
        if !in_range {
            location = Some(pos);
            break;
        }
    }

    let location = location.ok_or("No suitable location found")?;
    let tuning_frequency = location.0 * check_size + location.1;

    let count = check_line.difference(&check_line_beacons).count();

    Ok([Some(count.to_string()), Some(tuning_frequency.to_string())])
}

#[test]
pub fn test_day15() {
    assert!(common::run_test(DAY, &run))
}
