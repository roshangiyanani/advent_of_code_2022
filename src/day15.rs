use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::{max, Ordering};
use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("x=")
            .ok_or_else(|| format!("missing 'x=' in '{}'", s))?;
        let (x, s) = s
            .split_once(',')
            .ok_or_else(|| format!("missing ',' in '{}'", s))?;
        let x = x.parse().map_err(|e: ParseIntError| e.to_string())?;

        let y = s
            .strip_prefix(" y=")
            .ok_or_else(|| format!("missing ' y=' in '{}'", s))?;
        let y = y.parse().map_err(|e: ParseIntError| e.to_string())?;

        Ok(Point { x, y })
    }
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Sensor(Point);

impl Sensor {
    fn x_range_for_y_in_manhattan_range(
        &self,
        manhattan_distance: u64,
        y: i64,
    ) -> Option<Range<i64>> {
        let abs_diff = self.0.y.abs_diff(y);
        if abs_diff > manhattan_distance {
            None
        } else {
            let width = (manhattan_distance - abs_diff) as i64;
            Some(self.0.x - width..self.0.x + width + 1)
        }
    }
}

fn order_range(a: &Range<i64>, b: &Range<i64>) -> Ordering {
    match Ord::cmp(&a.start, &b.start) {
        Ordering::Equal => Ord::cmp(&a.end, &b.end),
        o => o,
    }
}

fn collapse_ordered_range<T: PartialOrd + Ord>(
    a: Range<T>,
    b: Range<T>,
) -> Result<Range<T>, (Range<T>, Range<T>)> {
    assert!(a.start <= a.end);
    assert!(b.start <= b.end);
    assert!(a.start <= b.start, "ranges must be sorted");

    if a.end < b.start {
        Err((a, b))
    } else {
        Ok(a.start..max(a.end, b.end))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Beacon(Point);

#[aoc_generator(day15)]
fn parse_sensor_report(input: &str) -> Vec<(Sensor, Beacon)> {
    input
        .lines()
        .map(|line| {
            let line = line.strip_prefix("Sensor at ").expect("unexpected format");
            let (sensor, beacon) = line
                .split_once(": closest beacon is at ")
                .expect("unexpected format");
            let sensor = Sensor(sensor.parse().expect("could not parse sensor location"));
            let beacon = Beacon(beacon.parse().expect("could not parse beacon location"));
            (sensor, beacon)
        })
        .collect()
}

fn known_empty_positions<const Y: i64>(input: &[(Sensor, Beacon)]) -> i64 {
    let num_in_range: i64 = input
        .iter()
        .map(|(sensor, beacon)| {
            let md = sensor.0.manhattan_distance(&beacon.0);
            let range = sensor.x_range_for_y_in_manhattan_range(md, Y);
            range
        })
        .filter_map(|range| range)
        .sorted_unstable_by(order_range)
        .coalesce(collapse_ordered_range)
        .map(|range| range.end - range.start)
        .sum();

    let num_beacons = input
        .iter()
        .filter_map(|(_, beacon)| {
            if beacon.0.y == Y {
                Some(beacon.0.x)
            } else {
                None
            }
        })
        .unique()
        .count() as i64;
    num_in_range - num_beacons
}

#[aoc(day15, part1)]
fn known_empty_positions_2_000_000(input: &[(Sensor, Beacon)]) -> i64 {
    const Y: i64 = 2_000_000;
    known_empty_positions::<Y>(input)
}

fn find_distress_signal<const SEARCH_SPACE: i64>(input: &[(Sensor, Beacon)]) -> Option<Point> {
    for y in 0..=SEARCH_SPACE {
        let mut collapsed_range = input
            .iter()
            .map(|(sensor, beacon)| {
                let md = sensor.0.manhattan_distance(&beacon.0);
                let range = sensor.x_range_for_y_in_manhattan_range(md, y);
                range
            })
            .filter_map(|range| range)
            .sorted_unstable_by(order_range)
            .coalesce(collapse_ordered_range)
            .skip_while(|range| range.end <= 0);

        let first_range = collapsed_range.next().expect("no range in row");
        assert!(first_range.start <= 0);
        if first_range.end <= SEARCH_SPACE {
            let x = first_range.end;
            return Some(Point { x, y });
        }
    }

    None
}

#[aoc(day15, part2)]
fn tuning_frequency(input: &[(Sensor, Beacon)]) -> i64 {
    const SIZE: i64 = 4_000_000;

    let point = find_distress_signal::<SIZE>(input).expect("no distress signal found");
    point.x * SIZE + point.y
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part_one() {
        let input = parse_sensor_report(INPUT);
        let count = known_empty_positions::<10>(&input);
        assert_eq!(count, 26)
    }

    #[test]
    fn test_part_two() {
        let input = parse_sensor_report(INPUT);
        let distress_signal = find_distress_signal::<20>(&input);
        assert_eq!(distress_signal, Some(Point { x: 14, y: 11 }))
    }
}
