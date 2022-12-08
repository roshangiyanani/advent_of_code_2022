use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::Range;

#[aoc_generator(day4)]
fn parse_ranges(input: &str) -> Vec<(Range<u32>, Range<u32>)> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(',').expect("could not find two ranges");

            let (a_start, a_end) = a.split_once('-').expect("could not find a_start and a_end");
            let a_start = a_start.parse().expect("unable to parse a_start");
            let a_end: u32 = a_end.parse().expect("unable to parse a_end");

            let (b_start, b_end) = b.split_once('-').expect("could not find b_start and b_end");
            let b_start: u32 = b_start.parse().expect("unable to parse b_start");
            let b_end: u32 = b_end.parse().expect("unable to parse b_end");

            // RangeInclusive does not have public fields
            (a_start..a_end + 1, b_start..b_end + 1)
        })
        .collect()
}

#[aoc(day4, part1)]
fn num_full_overlap(ranges: &[(Range<u32>, Range<u32>)]) -> usize {
    ranges
        .iter()
        .filter(|(a, b)| {
            (a.contains(&b.start) && a.contains(&(b.end - 1)))
                || (b.contains(&a.start) && b.contains(&(a.end - 1)))
        })
        .count()
}

#[aoc(day4, part2)]
fn num_any_overlap(ranges: &[(Range<u32>, Range<u32>)]) -> usize {
    ranges
        .iter()
        .filter(|(a, b)| {
            (a.start < b.start && a.end > b.start) || (a.start < b.end && a.end > b.start)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day4::{num_any_overlap, num_full_overlap, parse_ranges};

    const INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_one() {
        let ranges = parse_ranges(INPUT);
        let sum = num_full_overlap(&ranges);
        assert_eq!(sum, 2)
    }

    #[test]
    fn test_part_two() {
        let ranges = parse_ranges(INPUT);
        let sum = num_any_overlap(&ranges);
        assert_eq!(sum, 4)
    }
}
