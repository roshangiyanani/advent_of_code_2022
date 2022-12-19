use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{izip, Itertools};
use std::iter;

/// creates visibility vector where edges are (trivially) known to be visible
fn init_visibility(width: usize, height: usize) -> Vec<bool> {
    let mut visible = Vec::with_capacity(width * height);

    // first row is visible
    visible.extend(iter::repeat(true).take(width));

    for _ in 2..height {
        // left column is visible
        visible.push(true);

        // center is not (yet) known to be visible
        visible.extend(iter::repeat(false).take(width - 2));

        // right column is visible
        visible.push(true);
    }

    // last row is visible
    visible.extend(iter::repeat(true).take(width));

    visible
}

fn calculate_visible_by_row<'a, I, J, K, L>(forest: I, visible: K)
where
    I: Iterator<Item = J>,
    J: Iterator<Item = &'a u8>,
    K: Iterator<Item = L>,
    L: Iterator<Item = &'a mut bool>,
{
    for (mut forest, mut visible) in forest.into_iter().zip_eq(visible) {
        assert_eq!(true, *visible.next().expect("cannot have 0 width row"));
        let mut tallest_in_row = *forest.next().expect("cannot have 0 width row");

        for (&height, visible) in forest.zip_eq(visible) {
            if height > tallest_in_row {
                tallest_in_row = height;
                *visible = true;
            }
        }
    }
}

fn calculate_visible_by_column<'a, I, J, K, L>(mut forest: I, mut visible: K)
where
    I: Iterator<Item = J>,
    J: Iterator<Item = &'a u8>,
    K: Iterator<Item = L>,
    L: Iterator<Item = &'a mut bool>,
{
    let mut tallest_in_column: Vec<u8> = forest
        .next()
        .expect("must have at least one row")
        .cloned()
        .collect();
    assert!(visible
        .next()
        .expect("must have at least one row")
        .all(|&mut v| v));

    for (forest, visible) in forest.into_iter().zip(visible) {
        for (tallest_in_column, &height, visible) in
            izip!(tallest_in_column.iter_mut(), forest, visible)
        {
            if height > *tallest_in_column {
                *tallest_in_column = height;
                *visible = true;
            }
        }
    }
}

#[aoc_generator(day8)]
pub fn build_forest(input: &str) -> (Vec<u8>, usize) {
    let width = input
        .lines()
        .next()
        .expect("input must have at least one line")
        .len();
    let mut array = Vec::<u8>::with_capacity(input.len());

    for line in input.lines() {
        array.extend(
            line.chars()
                .map(|c| c.to_digit(10).expect("invalid tree height") as u8),
        );
    }

    (array, width)
}

fn _print_visible(visibility: &[bool], width: usize) {
    for chunk in visibility.chunks_exact(width) {
        for v in chunk {
            print!("{}", if *v { "T" } else { "F" });
        }
        println!();
    }
    println!();
}

fn calculate_scenic_score_in_direction<'a, I>(height: u8, iter: I) -> usize
where
    I: Iterator<Item = &'a u8>,
{
    let mut count = 0;

    for &h in iter {
        count += 1;
        if h >= height {
            break;
        }
    }

    count
}

fn calculate_scenic_score(forest: &[u8], width: usize, i: usize, j: usize) -> usize {
    let height = forest[i + j * width];

    let left =
        calculate_scenic_score_in_direction(height, forest[j * width..i + j * width].iter().rev());
    let right = calculate_scenic_score_in_direction(
        height,
        forest[i + j * width + 1..(j + 1) * width].iter(),
    );
    let top = calculate_scenic_score_in_direction(
        height,
        forest[..=i + j * width].iter().rev().step_by(width).skip(1),
    );
    let bottom = calculate_scenic_score_in_direction(
        height,
        forest[i + j * width..].iter().step_by(width).skip(1),
    );

    // println!("left: {}, right: {}, top: {}, bottom: {}", left, right, top, bottom);
    left * right * top * bottom
}

#[aoc(day8, part1)]
pub fn visible_trees(input: &(Vec<u8>, usize)) -> usize {
    let (forest, width) = input;
    // println!("{:?}", forest);

    let mut visible = init_visibility(*width, forest.len() / width);
    // print_visible(&visible, *width);

    // left
    calculate_visible_by_row(
        forest.chunks_exact(*width).map(IntoIterator::into_iter),
        visible
            .chunks_exact_mut(*width)
            .map(IntoIterator::into_iter),
    );
    // print_visible(&visible, *width);

    // right
    calculate_visible_by_row(
        forest.chunks_exact(*width).map(|i| i.iter().rev()),
        visible.chunks_exact_mut(*width).map(|i| i.iter_mut().rev()),
    );
    // print_visible(&visible, *width);

    // top
    calculate_visible_by_column(
        forest.chunks_exact(*width).map(IntoIterator::into_iter),
        visible
            .chunks_exact_mut(*width)
            .map(IntoIterator::into_iter),
    );
    // print_visible(&visible, *width);

    // bottom
    calculate_visible_by_column(
        forest.rchunks_exact(*width).map(IntoIterator::into_iter),
        visible
            .rchunks_exact_mut(*width)
            .map(IntoIterator::into_iter),
    );
    // print_visible(&visible, *width);

    visible.iter().filter(|&v| *v).count()
}

#[aoc(day8, part2)]
pub fn max_scenic_score(input: &(Vec<u8>, usize)) -> usize {
    let (forest, width) = input;
    let height = forest.len() / width;

    let tree_indexes = (0..*width).flat_map(|i| (0..height).map(move |j| (i, j)));
    tree_indexes
        .map(|(i, j)| calculate_scenic_score(forest, *width, i, j))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn test_part_one() {
        let input = build_forest(INPUT);
        let visible = visible_trees(&input);
        assert_eq!(visible, 21)
    }

    #[test]
    fn test_scenic_score() {
        let (forest, width) = build_forest(INPUT);

        assert_eq!(calculate_scenic_score(&forest, width, 2, 1), 4, "(2, 1)");
        assert_eq!(calculate_scenic_score(&forest, width, 2, 3), 8, "(2, 3)");
    }

    #[test]
    fn test_max_scenic_score() {
        let input = build_forest(INPUT);

        assert_eq!(max_scenic_score(&input), 8)
    }
}
