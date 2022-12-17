use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

pub struct Heightmap {
    heights: Vec<u8>,
    num_cols: usize,
    start: (usize, usize),
    end: (usize, usize),
}

#[aoc_generator(day12)]
pub fn parse_heightmap(input: &str) -> Heightmap {
    let num_cols = input
        .lines()
        .next()
        .expect("must have at least one row")
        .len();

    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;

    let mut heights = Vec::with_capacity(input.len());
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            match char {
                'a'..='z' => {
                    heights.push(char as u8 - 'a' as u8 + 1);
                }
                'S' => {
                    assert_eq!(start, None, "multiple 'S' chars");
                    start = Some((row, col));
                    heights.push(1) // S has height 'a'
                }
                'E' => {
                    assert_eq!(end, None, "multiple 'E' chars");
                    end = Some((row, col));
                    heights.push(26) // E has height 'z'
                }
                _ => panic!("unexpected character '{}'", char),
            }
        }
    }

    Heightmap {
        heights,
        num_cols,
        start: start.expect("no 'S' char found"),
        end: end.expect("no 'E' char found"),
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn build_bfs_route_to_start(
    heights: &Vec<u8>,
    num_rows: usize,
    num_cols: usize,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut parents: Vec<Option<Direction>> = vec![None; heights.len()];
    let mut queue = VecDeque::from([end]);

    let index = |row: usize, col: usize| -> usize { row * num_cols + col };

    while let Some((row, col)) = queue.pop_front() {
        if start == (row, col) {
            break;
        }

        let height = heights[index(row, col)];

        if col > 0 {
            let left = index(row, col - 1);
            if parents[left] == None && height <= heights[left] + 1 {
                parents[left] = Some(Direction::Right);
                queue.push_back((row, col - 1));
            }
        }

        if col + 1 < num_cols {
            let right = index(row, col + 1);
            if parents[right] == None && height <= heights[right] + 1 {
                parents[right] = Some(Direction::Left);
                queue.push_back((row, col + 1));
            }
        }

        if row > 0 {
            let up = index(row - 1, col);
            if parents[up] == None && height <= heights[up] + 1 {
                parents[up] = Some(Direction::Down);
                queue.push_back((row - 1, col));
            }
        }

        if row + 1 < num_rows {
            let down = index(row + 1, col);
            if parents[down] == None && height <= heights[down] + 1 {
                parents[down] = Some(Direction::Up);
                queue.push_back((row + 1, col));
            }
        }
    }

    build_route(num_rows, num_cols, start, end, &parents)
}

fn build_bfs_route_to_a(
    heights: &Vec<u8>,
    num_rows: usize,
    num_cols: usize,
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut parents: Vec<Option<Direction>> = vec![None; heights.len()];
    let mut queue = VecDeque::from([end]);
    let mut start: Option<(usize, usize)> = None;

    let index = |row: usize, col: usize| -> usize { row * num_cols + col };

    while let Some((row, col)) = queue.pop_front() {
        let height = heights[index(row, col)];
        if height == 1 {
            start = Some((row, col));
            break;
        }

        if col > 0 {
            let left = index(row, col - 1);
            if parents[left] == None && height <= heights[left] + 1 {
                parents[left] = Some(Direction::Right);
                queue.push_back((row, col - 1));
            }
        }

        if col + 1 < num_cols {
            let right = index(row, col + 1);
            if parents[right] == None && height <= heights[right] + 1 {
                parents[right] = Some(Direction::Left);
                queue.push_back((row, col + 1));
            }
        }

        if row > 0 {
            let up = index(row - 1, col);
            if parents[up] == None && height <= heights[up] + 1 {
                parents[up] = Some(Direction::Down);
                queue.push_back((row - 1, col));
            }
        }

        if row + 1 < num_rows {
            let down = index(row + 1, col);
            if parents[down] == None && height <= heights[down] + 1 {
                parents[down] = Some(Direction::Up);
                queue.push_back((row + 1, col));
            }
        }
    }

    let start = start.expect("did not find path to any square with height 'a'");
    build_route(num_rows, num_cols, start, end, &parents)
}

fn build_route(
    num_rows: usize,
    num_cols: usize,
    start: (usize, usize),
    end: (usize, usize),
    parents: &[Option<Direction>],
) -> Vec<(usize, usize)> {
    let index = |row: usize, col: usize| -> usize { row * num_cols + col };

    let mut route = vec![start];
    let mut current = start;
    while current != end {
        let (row, col) = current;
        current = match parents[index(row, col)] {
            None => panic!("did not find path to end"),
            Some(Direction::Left) => {
                assert!(col > 0);
                (row, col - 1)
            }
            Some(Direction::Right) => {
                assert!(col + 1 < num_cols);
                (row, col + 1)
            }
            Some(Direction::Up) => {
                assert!(row > 0);
                (row - 1, col)
            }
            Some(Direction::Down) => {
                assert!(row + 1 < num_rows);
                (row + 1, col)
            }
        };
        route.push(current);
    }
    route
}

#[aoc(day12, part1)]
pub fn len_shortest_path_from_start(heightmap: &Heightmap) -> usize {
    let route = build_bfs_route_to_start(
        &heightmap.heights,
        &heightmap.heights.len() / heightmap.num_cols,
        heightmap.num_cols,
        heightmap.start,
        heightmap.end,
    );
    route.len() - 1 // num steps, not num nodes
}

#[aoc(day12, part2)]
pub fn len_shortest_path_from_any_a(heightmap: &Heightmap) -> usize {
    let route = build_bfs_route_to_a(
        &heightmap.heights,
        &heightmap.heights.len() / heightmap.num_cols,
        heightmap.num_cols,
        heightmap.end,
    );
    route.len() - 1 // num steps, not num nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part_one() {
        let heightmap = parse_heightmap(INPUT);
        assert_eq!(len_shortest_path_from_start(&heightmap), 31);
    }

    #[test]
    fn test_part_two() {
        let heightmap = parse_heightmap(INPUT);
        assert_eq!(len_shortest_path_from_any_a(&heightmap), 29);
    }
}
