use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').expect("unexpected point format");
        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Path(Vec<Point>);

impl FromStr for Path {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Path(
            s.split(" -> ")
                .map(FromStr::from_str)
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Scan {
    raw: Vec<Tile>,
    width: usize,
    height: usize,
    source: Point,
}

impl Display for Scan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point { x, y };
                let char = match self[&point] {
                    Tile::Air => ".",
                    Tile::Rock => "#",
                    Tile::Sand => "+",
                };
                write!(f, "{}", char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Scan {
    const SOURCE: Point = Point { x: 500, y: 0 };

    fn from_paths(paths: &[Path]) -> Scan {
        // clip left area
        let mut left = Scan::SOURCE.x;
        let mut right = Scan::SOURCE.x;
        const _TOP: usize = 0; // SAND_SOURCE is at 0
        let mut bottom = Scan::SOURCE.y;

        for point in paths.iter().flat_map(|path| &path.0) {
            left = min(left, point.x);
            right = max(right, point.x);
            bottom = max(bottom, point.y);
        }

        let adj_x = left;
        let source = Point {
            x: Scan::SOURCE.x - adj_x,
            y: Scan::SOURCE.y,
        };

        // build scan
        let width = right - left + 1;
        let height = bottom + 1;

        let raw = vec![Tile::Air; width * height];
        let mut scan = Scan {
            raw,
            width,
            height,
            source,
        };

        for path in paths {
            let first = path.0.first().expect("cannot have empty path");
            let first = &Point {
                x: first.x - adj_x,
                y: first.y,
            };
            scan[first] = Tile::Rock;

            for (start, end) in path.0.iter().tuple_windows() {
                let start = Point {
                    x: start.x - adj_x,
                    y: start.y,
                };
                let end = Point {
                    x: end.x - adj_x,
                    y: end.y,
                };

                let mut current = start.to_owned();
                loop {
                    if current.x == end.x {
                        if current.y < end.y {
                            current.y += 1;
                        } else if current.y > end.y {
                            current.y -= 1;
                        } else {
                            break;
                        }
                    } else if current.y == end.y {
                        if current.x < end.x {
                            current.x += 1;
                        } else if current.x > end.x {
                            current.x -= 1;
                        } else {
                            break;
                        }
                    } else {
                        panic!("path must consist of straight lines")
                    }

                    scan[&current] = Tile::Rock;
                }
            }
        }

        scan
    }

    fn try_place_sand(&mut self) -> bool {
        if self[&self.source] != Tile::Air {
            // source is blocked
            return false;
        }

        let mut sand = self.source.to_owned();
        loop {
            let in_abyss_below = sand.y >= self.height - 1;
            let in_abyss_left = sand.x == 0;
            let in_abyss_right = sand.x >= self.width - 1;
            if in_abyss_below || in_abyss_left || in_abyss_right {
                return false;
            }

            let below = Point {
                x: sand.x,
                y: sand.y + 1,
            };
            let below_left = Point {
                x: sand.x - 1,
                y: sand.y + 1,
            };
            let below_right = Point {
                x: sand.x + 1,
                y: sand.y + 1,
            };

            if self[&below] == Tile::Air {
                sand = below;
            } else if self[&below_left] == Tile::Air {
                sand = below_left;
            } else if self[&below_right] == Tile::Air {
                sand = below_right;
            } else {
                self[&sand] = Tile::Sand;
                return true;
            }
        }
    }
}

impl Index<&Point> for Scan {
    type Output = Tile;

    fn index(&self, index: &Point) -> &Self::Output {
        if self.width <= index.x || self.height <= index.y {
            panic!("index {:?} is out of range", index)
        }

        let index = index.x + index.y * self.width;
        &self.raw[index]
    }
}

impl IndexMut<&Point> for Scan {
    fn index_mut(&mut self, index: &Point) -> &mut Self::Output {
        let index = index.x + index.y * self.width;
        &mut self.raw[index]
    }
}

#[aoc_generator(day14)]
fn parse_paths(input: &str) -> Vec<Path> {
    input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .expect("could not parse path")
}

#[aoc(day14, part1)]
fn amount_sand_rests(paths: &[Path]) -> usize {
    let mut scan = Scan::from_paths(&paths);
    // println!("{}", scan);

    let mut count = 0;

    while scan.try_place_sand() {
        // println!("{}", scan);
        count += 1;
    }

    // println!("{}", scan);
    count
}

#[aoc(day14, part2)]
fn amount_sand_rests_until_blocked(input: &[Path]) -> usize {
    // simulate "infinite" floor with a real path
    let mut paths = Vec::with_capacity(input.len() + 1);
    paths.extend(input.to_owned());

    let bottom = paths
        .iter()
        .flat_map(|path| &path.0)
        .map(|point| point.y)
        .max()
        .expect("must have at least one path")
        + 2;
    let bottom_path = Path(vec![
        Point {
            x: Scan::SOURCE.x - bottom,
            y: bottom,
        },
        Point {
            x: Scan::SOURCE.x + bottom,
            y: bottom,
        },
    ]);

    paths.push(bottom_path);

    // then run as normal
    amount_sand_rests(&paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part_one() {
        let paths = parse_paths(INPUT);
        assert_eq!(amount_sand_rests(&paths), 24);
    }

    #[test]
    fn test_part_two() {
        let paths = parse_paths(INPUT);
        assert_eq!(amount_sand_rests_until_blocked(&paths), 93);
    }
}
