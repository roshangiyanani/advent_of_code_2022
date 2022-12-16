use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instruction {
    Noop,
    AddX(i64),
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(instruction: &str) -> Result<Self, Self::Error> {
        if instruction == "noop" {
            Ok(Instruction::Noop)
        } else if let Some(("addx", v)) = instruction.split_once(" ") {
            let v = v
                .parse()
                .map_err(|_| format!("could not parse V: '{}'", v))?;
            Ok(Instruction::AddX(v))
        } else {
            Err(format!("illegal instruction '{}'", instruction))
        }
    }
}

struct CpuSimulator<'a, I>
where
    I: Iterator<Item = &'a Instruction>,
{
    x: i64,
    instructions: I,
    pending_add_x: Option<i64>,
}

impl<'a, I> CpuSimulator<'a, I>
where
    I: Iterator<Item = &'a Instruction>,
{
    fn new(iter: I) -> CpuSimulator<'a, I> {
        CpuSimulator {
            x: 1,
            instructions: iter,
            pending_add_x: None,
        }
    }
}

impl<'a, I> Iterator for CpuSimulator<'a, I>
where
    I: Iterator<Item = &'a Instruction>,
{
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x;
        if let Some(v) = self.pending_add_x {
            self.x += v;
            self.pending_add_x = None;

            Some(x)
        } else if let Some(&instruction) = self.instructions.next() {
            match instruction {
                Instruction::AddX(v) => self.pending_add_x = Some(v),
                Instruction::Noop => {}
            };

            Some(x)
        } else {
            None
        }
    }
}

fn interesting<I, T>(mut iter: I) -> [T; 6]
where
    I: Iterator<Item = T>,
{
    [
        iter.nth(20 - 1).expect("could not find 20th item"),
        iter.nth(60 - 20 - 1).expect("could not find 60th item"),
        iter.nth(100 - 60 - 1).expect("could not find 100th item"),
        iter.nth(140 - 100 - 1).expect("could not find 140th item"),
        iter.nth(180 - 140 - 1).expect("could not find 180th item"),
        iter.nth(220 - 180 - 1).expect("could not find 220th item"),
    ]
}

#[aoc_generator(day10)]
pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(Instruction::try_from)
        .collect::<Result<_, _>>()
        .expect("unable to parse instructions")
}

#[aoc(day10, part1)]
pub fn sum_interesting_signal_strengths(instructions: &[Instruction]) -> i64 {
    let signal_strengths = CpuSimulator::new(instructions.iter())
        .enumerate()
        .map(|(i, x)| (i + 1) as i64 * x);

    interesting(signal_strengths).iter().sum()
}

#[aoc(day10, part2)]
pub fn render_sprites(instructions: &[Instruction]) -> String {
    const DISPLAY_HEIGHT: usize = 6;
    const DISPLAY_WIDTH: usize = 40;

    let mut simulator = CpuSimulator::new(instructions.iter());

    let mut display = String::with_capacity((DISPLAY_WIDTH + 1) * DISPLAY_HEIGHT + 1);
    display.push('\n');

    for _ in 0..DISPLAY_HEIGHT {
        for cursor in 0..DISPLAY_WIDTH {
            let sprite = simulator.next().expect("not enough instructions") as usize;

            let char = if cursor >= sprite - 1 && cursor <= sprite + 1 {
                '#'
            } else {
                '.'
            };
            display.push(char);
        }
        display.push('\n');
    }

    display
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_SMALL: &str = "\
noop
addx 3
addx -5";

    const INPUT_LARGE: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_simulator_small() {
        let instructions = parse_instructions(INPUT_SMALL);
        let simulator = CpuSimulator::new(instructions.iter());
        let x_states: Vec<_> = simulator.collect();

        assert_eq!(x_states, [1, 1, 1, 4, 4]);
    }

    #[test]
    fn test_simulator_large() {
        let instructions = parse_instructions(INPUT_LARGE);

        let simulator = CpuSimulator::new(instructions.iter());
        let x_states = interesting(simulator);
        assert_eq!(x_states, [21, 19, 18, 21, 16, 18]);

        assert_eq!(sum_interesting_signal_strengths(&instructions), 13140);
    }

    #[test]
    fn test_render() {
        const EXPECTED_RENDER: &str = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        let instructions = parse_instructions(INPUT_LARGE);
        let render = render_sprites(&instructions);

        assert_eq!(EXPECTED_RENDER, render);
    }
}
