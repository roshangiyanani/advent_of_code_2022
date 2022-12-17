use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: Test,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Item {
    worry: usize,
}

impl Item {
    fn apply(&mut self, op: &Operation) {
        self.worry = match op {
            Operation::Add(x) => self.worry + x,
            Operation::Multiply(x) => self.worry * x,
            Operation::Square => self.worry * self.worry,
        }
    }

    fn relieve(&mut self) {
        self.worry /= 3;
    }

    fn relieve_modulo(&mut self, modulo: usize) {
        self.worry %= modulo;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Test {
    divisible_by: usize,
    target_true: usize,
    target_false: usize,
}

impl Test {
    fn target(&self, item: &Item) -> usize {
        if item.worry % self.divisible_by == 0 {
            self.target_true
        } else {
            self.target_false
        }
    }
}

#[aoc_generator(day11)]
fn parse_monkey_notes(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    let mut lines = input.lines();

    loop {
        let monkey_declaration = lines.next().expect("no monkey declaration line");
        assert_eq!(monkey_declaration, format!("Monkey {}:", monkeys.len()));

        let items = lines.next().expect("no starting items line");
        let items = items
            .strip_prefix("  Starting items: ")
            .expect("invalid starting items line format");
        let items = items
            .split(", ")
            .map(|worry| Item {
                worry: worry.parse().expect("could not parse starting item worry"),
            })
            .collect();

        let operation = lines.next().expect("no operation line");
        let operation = operation
            .strip_prefix("  Operation: new = old ")
            .expect("invalid operation line format");
        let (operation, operand) = operation.split_once(' ').expect("invalid operation format");
        let operation = match operation {
            "+" => Operation::Add(operand.parse().expect("could not parse operand")),
            "*" if operand == "old" => Operation::Square,
            "*" => Operation::Multiply(operand.parse().expect("could not parse operand")),
            c => panic!("unexpected operation: '{}'", operation),
        };

        let test = lines.next().expect("no test line");
        let divisible_by = test
            .strip_prefix("  Test: divisible by ")
            .expect("invalid test line");
        let divisible_by = divisible_by.parse().expect("invalid test number format");

        let test_true = lines.next().expect("no test_true line");
        let target_true = test_true
            .strip_prefix("    If true: throw to monkey ")
            .expect("invalid test_true line");
        let target_true = target_true.parse().expect("invalid test_true target");

        let test_false = lines.next().expect("no test_false line");
        let target_false = test_false
            .strip_prefix("    If false: throw to monkey ")
            .expect("invalid test_false line");
        let target_false = target_false.parse().expect("invalid test_false target");

        let monkey = Monkey {
            items,
            operation,
            test: Test {
                divisible_by,
                target_true,
                target_false,
            },
        };

        monkeys.push(monkey);

        match lines.next() {
            Some("") => {} // empty line separates next monkey
            None => break,
            Some(line) => panic!("expected empty line (monkey separator), not '{}'", line),
        }
    }

    monkeys
}

#[aoc(day11, part1)]
fn monkey_business(input: &[Monkey]) -> usize {
    let mut monkeys: Vec<_> = input.iter().cloned().collect();

    let mut num_inspections = vec![0; monkeys.len()];

    const ROUNDS: usize = 20;

    for _round in 0..ROUNDS {
        for i in 0..monkeys.len() {
            assert_ne!(i, monkeys[i].test.target_true);
            assert_ne!(i, monkeys[i].test.target_false);

            num_inspections[i] += monkeys[i].items.len();

            for j in 0..monkeys[i].items.len() {
                let mut item = monkeys[i].items[j];
                item.apply(&monkeys[i].operation);
                item.relieve();

                let target = monkeys[i].test.target(&item);
                monkeys[target].items.push(item);
            }

            monkeys[i].items.clear();
        }
    }

    num_inspections.sort_unstable();
    num_inspections[num_inspections.len() - 2..]
        .iter()
        .product()
}

#[aoc(day11, part2)]
fn monkey_business_no_relief(input: &[Monkey]) -> usize {
    let mut monkeys: Vec<_> = input.iter().cloned().collect();

    let modulo = monkeys.iter().map(|m| m.test.divisible_by).product();

    let mut num_inspections = vec![0; monkeys.len()];

    const ROUNDS: usize = 10_000;

    for _round in 0..ROUNDS {
        for i in 0..monkeys.len() {
            assert_ne!(i, monkeys[i].test.target_true);
            assert_ne!(i, monkeys[i].test.target_false);

            num_inspections[i] += monkeys[i].items.len();

            for j in 0..monkeys[i].items.len() {
                let mut item = monkeys[i].items[j];
                item.apply(&monkeys[i].operation);
                item.relieve_modulo(modulo);

                let target = monkeys[i].test.target(&item);
                monkeys[target].items.push(item);
            }

            monkeys[i].items.clear();
        }
    }

    num_inspections.sort_unstable();
    num_inspections[num_inspections.len() - 2..]
        .iter()
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_monkey_parse() {
        let expected_monkeys = vec![
            Monkey {
                items: vec![Item { worry: 79 }, Item { worry: 98 }],
                operation: Operation::Multiply(19),
                test: Test {
                    divisible_by: 23,
                    target_true: 2,
                    target_false: 3,
                },
            },
            Monkey {
                items: vec![
                    Item { worry: 54 },
                    Item { worry: 65 },
                    Item { worry: 75 },
                    Item { worry: 74 },
                ],
                operation: Operation::Add(6),
                test: Test {
                    divisible_by: 19,
                    target_true: 2,
                    target_false: 0,
                },
            },
            Monkey {
                items: vec![Item { worry: 79 }, Item { worry: 60 }, Item { worry: 97 }],
                operation: Operation::Square,
                test: Test {
                    divisible_by: 13,
                    target_true: 1,
                    target_false: 3,
                },
            },
            Monkey {
                items: vec![Item { worry: 74 }],
                operation: Operation::Add(3),
                test: Test {
                    divisible_by: 17,
                    target_true: 0,
                    target_false: 1,
                },
            },
        ];

        let monkeys = parse_monkey_notes(INPUT);

        assert_eq!(expected_monkeys, monkeys);
    }

    #[test]
    fn test_part_one() {
        let monkeys = parse_monkey_notes(INPUT);
        assert_eq!(monkey_business(&monkeys), 10605)
    }

    #[test]
    fn test_part_two() {
        let monkeys = parse_monkey_notes(INPUT);
        assert_eq!(monkey_business_no_relief(&monkeys), 2713310158)
    }
}
