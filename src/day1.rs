use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    let mut elves = Vec::<Vec<u32>>::new();
    let mut calories = Vec::<u32>::new();

    for line in input.lines() {
        if line.is_empty() {
            elves.push(calories);
            calories = Vec::new();
        } else {
            calories.push(line.parse().expect("could not parse calories"))
        }
    }

    if !calories.is_empty() {
        elves.push(calories);
    }

    elves
}

#[aoc(day1, part1)]
pub fn most_calories_held(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|elf| elf.iter().sum())
        .max()
        .expect("no elves with food")
}

#[aoc(day1, part2)]
pub fn calories_held_by_top_3_elves(input: &[Vec<u32>]) -> u32 {
    let mut sums = input.iter().map(|elf| elf.iter().sum::<u32>());

    let mut min_heap = BinaryHeap::with_capacity(3);
    for _ in 0..3 {
        min_heap.push(Reverse(sums.next().expect("must have >= 3 elves")));
    }

    for sum in sums {
        if sum >= min_heap.peek().unwrap().0 {
            min_heap.pop();
            min_heap.push(Reverse(sum));
        }
    }

    min_heap.iter().map(|r| r.0).sum()
}
