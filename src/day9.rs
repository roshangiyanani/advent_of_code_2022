use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::{max, min};
use std::ops::Range;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Move {
    direction: Direction,
    steps: u8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(c: &str) -> Result<Self, Self::Error> {
        use Direction::*;

        match c {
            "L" => Ok(Left),
            "R" => Ok(Right),
            "U" => Ok(Up),
            "D" => Ok(Down),
            _ => Err(format!("invalid direction character: '{}'", c)),
        }
    }
}

#[aoc_generator(day9)]
pub fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|input| {
            let (direction, steps) = input.split_once(' ').expect("unexpected move format");
            let direction = direction.try_into().expect("unable to parse direction");
            let steps = steps.parse().expect("unable to parse amount");
            Move { direction, steps }
        })
        .collect()
}

fn get_ranges(moves: &[Move]) -> (Range<i32>, Range<i32>) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut x_range = 0..1;
    let mut y_range = 0..1;

    for mv in moves {
        match mv.direction {
            Direction::Left => {
                x += mv.steps as i32;
                x_range.end = max(x_range.end, x + 1);
            }
            Direction::Right => {
                x -= mv.steps as i32;
                x_range.start = min(x_range.start, x);
            }
            Direction::Up => {
                y += mv.steps as i32;
                y_range.end = max(y_range.end, y + 1);
            }
            Direction::Down => {
                y -= mv.steps as i32;
                y_range.start = min(y_range.start, y);
            }
        };
    }

    (x_range, y_range)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn move_towards(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        };
    }

    fn diff(&self, tail: &Coordinate) -> (i8, i8) {
        let diff_x = (self.x as isize - tail.x as isize) as i8;
        let diff_y = (self.y as isize - tail.y as isize) as i8;
        (diff_x, diff_y)
    }

    fn follow(&mut self, diff_x: i8, diff_y: i8) {
        match (diff_x, diff_y) {
            // on top of each other
            (0, 0) => (),
            // already adjacent
            (0, 1) | (0, -1) | (1, 0) | (-1, 0) | (1, 1) | (1, -1) | (-1, -1) | (-1, 1) => (),
            // one directional
            (0, 2) => self.y += 1,
            (0, -2) => self.y -= 1,
            (2, 0) => self.x += 1,
            (-2, 0) => self.x -= 1,
            // diagonal
            (1, 2) | (2, 1) | (2, 2) => {
                self.x += 1;
                self.y += 1;
            }
            (1, -2) | (2, -1) | (2, -2) => {
                self.x += 1;
                self.y -= 1;
            }
            (-1, -2) | (-2, -1) | (-2, -2) => {
                self.x -= 1;
                self.y -= 1;
            }
            (-1, 2) | (-2, 1) | (-2, 2) => {
                self.x -= 1;
                self.y += 1;
            }
            _ => panic!("unexpected position diff ({}, {})", diff_x, diff_y),
        };
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Positions {
    head: Coordinate,
    tail: Coordinate,
}

pub fn tail_visited_positions<const CHAIN_LENGTH: usize>(moves: &[Move]) -> usize {
    let (x_range, y_range) = get_ranges(moves);

    let x_len = x_range.len();
    let y_len = y_range.len();

    let mut visited = vec![false; x_len * y_len];

    let start = Coordinate {
        x: -x_range.start as usize,
        y: -y_range.start as usize,
    };
    visited[start.x + start.y * y_len] = true;

    moves.iter().fold([start; CHAIN_LENGTH], |mut chain, mv| {
        for _ in 0..mv.steps {
            chain[0].move_towards(mv.direction);

            for i in 1..CHAIN_LENGTH {
                let (diff_x, diff_y) = chain[i - 1].diff(&chain[i]);
                chain[i].follow(diff_x, diff_y);
            }

            let tail = chain[CHAIN_LENGTH - 1];
            visited[tail.x + tail.y * y_len] = true;
        }

        chain
    });

    visited.iter().filter(|v| **v).count()
}

#[aoc(day9, part1)]
pub fn tail_visits_chain_two(moves: &[Move]) -> usize {
    tail_visited_positions::<2>(moves)
}

#[aoc(day9, part2)]
pub fn tail_visits_chain_ten(moves: &[Move]) -> usize {
    tail_visited_positions::<10>(moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT_2: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1_small() {
        let moves = parse_moves(INPUT_1);
        let num_visited = tail_visits_chain_two(&moves);

        assert_eq!(num_visited, 13)
    }

    #[test]
    fn test_part2_small() {
        let moves = parse_moves(INPUT_1);
        let num_visited = tail_visits_chain_ten(&moves);

        assert_eq!(num_visited, 1)
    }

    #[test]
    fn test_part2_large() {
        let moves = parse_moves(INPUT_2);
        let num_visited = tail_visits_chain_ten(&moves);

        assert_eq!(num_visited, 36)
    }
}
