use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
enum Packet {
    Value(PacketValue),
    List(PacketList),
}

impl Packet {
    fn is_in_order(left: &Packet, right: &Packet) -> Option<bool> {
        use Packet::*;
        match (left, right) {
            (Value(left), Value(right)) => PacketValue::is_in_order(left, right),
            (List(left), List(right)) => PacketList::is_in_order(left, right),
            (Value(left), List(right)) => {
                PacketList::is_in_order(&PacketList(vec![Value(left.to_owned())]), right)
            }
            (List(left), Value(right)) => {
                PacketList::is_in_order(left, &PacketList(vec![Value(right.to_owned())]))
            }
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
struct PacketValue(u8);

impl PacketValue {
    fn parse(s: &mut Peekable<Chars>) -> PacketValue {
        let mut value = s.next().and_then(|c| c.to_digit(10)).expect("not a number") as u8;
        while let Some(c) = s.peek() {
            if let Some(digit) = c.to_digit(10) {
                s.next().unwrap();
                value *= 10;
                value += digit as u8;
            } else {
                break;
            }
        }

        PacketValue(value)
    }

    fn is_in_order(left: &PacketValue, right: &PacketValue) -> Option<bool> {
        if left.0 < right.0 {
            Some(true)
        } else if left.0 == right.0 {
            None
        } else {
            // left > right
            Some(false)
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
struct PacketList(Vec<Packet>);

impl PacketList {
    fn parse(s: &mut Peekable<Chars>) -> PacketList {
        assert_eq!(s.next(), Some('['));

        let mut list = Vec::new();

        loop {
            let &char = s.peek().expect("unexpected end of packet list");
            if char == '[' {
                list.push(Packet::List(PacketList::parse(s)));
                if s.peek() == Some(&',') {
                    s.next();
                }
            } else if char == ']' {
                s.next();
                return PacketList(list);
            } else {
                list.push(Packet::Value(PacketValue::parse(s)));
                if s.peek() == Some(&',') {
                    s.next();
                }
            }
        }
    }

    fn is_in_order(left: &PacketList, right: &PacketList) -> Option<bool> {
        let mut left = left.0.iter();
        let mut right = right.0.iter();

        loop {
            let left = left.next();
            let right = right.next();

            match (left, right) {
                (Some(left), Some(right)) => match Packet::is_in_order(left, right) {
                    Some(result) => return Some(result),
                    None => continue,
                },
                (None, Some(_)) => return Some(true),
                (Some(_), None) => return Some(false),
                (None, None) => return None,
            }
        }
    }
}

#[aoc_generator(day13)]
fn packet_parser(input: &str) -> Vec<Packet> {
    let mut packets = Vec::new();
    let mut input = input.chars().peekable();

    loop {
        let a = PacketList::parse(&mut input);
        assert_eq!(input.next(), Some('\n'));
        packets.push(Packet::List(a));

        let b = PacketList::parse(&mut input);
        packets.push(Packet::List(b));

        if input.peek().is_none() {
            break;
        } else {
            assert_eq!(input.next(), Some('\n'));
            assert_eq!(input.next(), Some('\n'));
        }
    }

    packets
}

#[aoc(day13, part1)]
fn sum_valid_packet_pair_indexes(packets: &[Packet]) -> usize {
    packets
        .iter()
        .tuples::<(_, _)>()
        .enumerate()
        .filter(|(_, (left, right))| {
            Packet::is_in_order(left, right).expect("must know if they are in order")
        })
        .map(|(index, _)| index + 1) // starts at one
        .sum()
}

#[aoc(day13, part2)]
fn decoder_key(packets: &[Packet]) -> usize {
    let div_one = Packet::List(PacketList::parse(&mut "[[2]]".chars().peekable()));
    let div_two = Packet::List(PacketList::parse(&mut "[[6]]".chars().peekable()));

    let mut index_one = 1;
    let mut index_two = 2;

    for packet in packets {
        match Packet::is_in_order(packet, &div_one) {
            Some(true) => {
                index_one += 1;
                index_two += 1
            }
            Some(false) => match Packet::is_in_order(packet, &div_two) {
                Some(true) => {
                    index_two += 1;
                }
                Some(false) => {}
                None => panic!("must have ordering"),
            },
            None => panic!("must have ordering"),
        }
    }

    index_one * index_two
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part_one() {
        let packet_pairs = packet_parser(INPUT);
        let sum_valid_indexes = sum_valid_packet_pair_indexes(&packet_pairs);
        assert_eq!(sum_valid_indexes, 13)
    }

    #[test]
    fn test_part_two() {
        let packet_pairs = packet_parser(INPUT);
        let key = decoder_key(&packet_pairs);
        assert_eq!(key, 140)
    }
}
