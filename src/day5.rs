use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn parse(input: &str) -> Stacks {
        let lines: Vec<_> = input.lines().collect();

        let (_, count) = lines
            .last()
            .expect("stack does not contain number line")
            .trim_end()
            .rsplit_once(" ")
            .expect("unexpected number line format");

        let num_stacks = count.parse().expect("could not parse number line length");
        let max_height = lines.len() - 1;

        let mut stacks: Vec<_> = (0..num_stacks)
            .map(|_| Vec::with_capacity(max_height))
            .collect();

        for &line in lines.iter().rev().skip(1) {
            let mut line = line;
            for i in 0..num_stacks {
                if line.starts_with("[") {
                    line = line.strip_prefix("[").unwrap();

                    let mut iter = line.chars();
                    let char = iter.next().expect("missing char");
                    stacks[i].push(char);

                    line = &iter.as_str();
                    line = &line.strip_prefix("]").expect("no closing bracket");
                    line = &line.strip_prefix(" ").as_ref().unwrap_or(&line); // if not at end of line
                } else {
                    line = &line
                        .strip_prefix("   ")
                        .expect("unexpected early termination");
                    line = &line.strip_prefix(" ").as_ref().unwrap_or(&line); // if not at end of line
                }
            }
        }

        Stacks(stacks)
    }

    fn apply_single_mover(&mut self, rearrangement: &Rearrangement) {
        for _ in 0..rearrangement.amount {
            let c = self.0[rearrangement.origin as usize]
                .pop()
                .expect("not enough crates in origin stack");
            self.0[rearrangement.destination as usize].push(c);
        }
    }

    fn apply_multiple_mover(&mut self, rearrangement: &Rearrangement, storage: &mut Vec<char>) {
        storage.clear();

        let origin = &mut self.0[rearrangement.origin as usize];
        for _ in 0..rearrangement.amount {
            let c = origin.pop().expect("not enough crates in origin stack");
            storage.push(c);
        }

        let destination = &mut self.0[rearrangement.destination as usize];
        for &c in storage.iter().rev() {
            destination.push(c);
        }
    }

    fn tops(&self) -> String {
        self.0
            .iter()
            .map(|stack| stack.last().expect("no crate in stack"))
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Rearrangement {
    pub amount: u8,
    pub origin: u8,
    pub destination: u8,
}

impl Rearrangement {
    fn parse_line(input: &str) -> Rearrangement {
        let input = input
            .strip_prefix("move ")
            .expect("rearrangement does not start with 'move '");
        let (amount, input) = input
            .split_once(" ")
            .expect("could not find ' ' after amount");
        let amount = amount.parse().expect("could not parse amount");

        let input = input
            .strip_prefix("from ")
            .expect("rearrangement does not contain 'from '");
        let (origin, input) = input
            .split_once(" ")
            .expect("could not find ' ' after origin");
        let origin = origin.parse::<u8>().expect("could not parse origin") - 1u8;

        let destination = input
            .strip_prefix("to ")
            .expect("rearrangement does not contain 'to '");
        let destination = destination
            .parse::<u8>()
            .expect("could not parse destination")
            - 1u8;

        Rearrangement {
            amount,
            origin,
            destination,
        }
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Stacks, Vec<Rearrangement>) {
    let (stacks, rearrangements) = input
        .split_once("\n\n")
        .expect("could not find boundary between stacks and rearrangement");

    let stacks = Stacks::parse(stacks);

    let rearrangements = rearrangements
        .lines()
        .map(Rearrangement::parse_line)
        .collect();

    (stacks, rearrangements)
}

#[aoc(day5, part1)]
pub fn follow_rearrangement_single_mover(input: &(Stacks, Vec<Rearrangement>)) -> String {
    let (stacks, rearrangements) = input;
    let mut stacks = stacks.clone();

    for rearrangement in rearrangements {
        stacks.apply_single_mover(&rearrangement);
    }

    stacks.tops()
}

#[aoc(day5, part2)]
pub fn follow_rearrangement_multiple_mover(input: &(Stacks, Vec<Rearrangement>)) -> String {
    let (stacks, rearrangements) = input;
    let mut stacks = stacks.clone();

    let mut storage = Vec::new();
    for rearrangement in rearrangements {
        stacks.apply_multiple_mover(&rearrangement, &mut storage);
    }

    stacks.tops()
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &str = "\
\x20\x20\x20\x20[D]\x20\x20\x20\x20
[N] [C]\x20\x20\x20\x20
[Z] [M] [P]
 1   2   3\x20

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_one() {
        let parsed = input_generator(input);
        let tops = follow_rearrangement_single_mover(&parsed);
        assert_eq!(tops, "CMZ")
    }

    #[test]
    fn test_part_two() {
        let parsed = input_generator(input);
        let tops = follow_rearrangement_multiple_mover(&parsed);
        assert_eq!(tops, "MCD")
    }
}
