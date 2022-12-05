use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day3)]
pub fn build_rucksacks(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect()
}

fn duplicate_item(a: &str, b: &str) -> char {
    let a: HashSet<_> = HashSet::from_iter(a.chars());
    b.chars()
        .find(|b| a.contains(b))
        .expect("no incorrectly packed item")
}

fn badge(a: &str, b: &str, c: &str) -> char {
    let a: HashSet<_> = HashSet::from_iter(a.chars());
    let b: HashSet<_> = HashSet::from_iter(b.chars());
    c.chars()
        .find(|c| a.contains(c) && b.contains(c))
        .expect("no common (badge) item")
}

fn item_type(item: char) -> Result<u32, String> {
    match item {
        'a'..='z' => Ok(item as u32 - 'a' as u32 + 1),
        'A'..='Z' => Ok(item as u32 - 'A' as u32 + 27),
        _ => Err(format!("invalid item type '{}'", item)),
    }
}

#[aoc(day3, part1)]
pub fn duplicate_item_type_priority_sum(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| duplicate_item(a, b))
        .map(|item| item_type(item).unwrap())
        .sum()
}

#[aoc(day3, part2)]
pub fn badge_item_type_priority_sum(input: &[String]) -> u32 {
    input
        .chunks(3)
        .map(|chunk| {
            if let [a, b, c] = &chunk[..] {
                badge(a, b, c)
            } else {
                panic!("elves must be split into groups of three")
            }
        })
        .map(|item| item_type(item).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_one() {
        let rucksacks = build_rucksacks(INPUT);
        let sum = duplicate_item_type_priority_sum(&rucksacks);
        assert_eq!(sum, 157);
    }

    #[test]
    fn test_part_two() {
        let rucksacks = build_rucksacks(INPUT);
        let sum = badge_item_type_priority_sum(&rucksacks);
        assert_eq!(sum, 70);
    }
}
