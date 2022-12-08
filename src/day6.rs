use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> String {
    input.to_owned()
}

#[aoc(day6, part1)]
pub fn start_of_packet_detector(input: &str) -> usize {
    let mut a = input.chars();
    let mut b = input.chars().skip(1);
    let mut c = input.chars().skip(2);
    let mut d = input.chars().skip(3);

    for current in 4..=input.len() {
        let a = a.next().unwrap();
        let b = b.next().unwrap();
        let c = c.next().unwrap();
        let d = d.next().unwrap();

        if a != b && a != c && a != d && b != c && b != d && c != d {
            return current;
        }
    }

    panic!("no packet start")
}

#[aoc(day6, part2)]
pub fn start_of_message_detector(input: &str) -> usize {
    let mut remove = input.chars();
    let mut add = input.chars();

    let mut map = HashMap::<char, u8>::with_capacity(14);

    for _ in 0..13 {
        let c = add.next().unwrap();
        map.entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    for current in 14..=input.len() {
        {
            let c = add.next().unwrap();
            map.entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        if map.len() == 14 {
            return current;
        }

        {
            let c = remove.next().unwrap();
            match map.entry(c) {
                Entry::Occupied(mut e) => {
                    match e.get_mut() {
                        0 => panic!("value cannot be 0"),
                        1 => {
                            e.remove_entry();
                        }
                        v => *v -= 1,
                    };
                }
                Entry::Vacant(_) => panic!("key is expected"),
            }
        }
    }

    panic!("no message start")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_ONE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT_TWO: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT_THREE: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT_FOUR: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT_FIVE: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part_one() {
        assert_eq!(start_of_packet_detector(INPUT_ONE), 7, "one");
        assert_eq!(start_of_packet_detector(INPUT_TWO), 5, "two");
        assert_eq!(start_of_packet_detector(INPUT_THREE), 6, "three");
        assert_eq!(start_of_packet_detector(INPUT_FOUR), 10, "four");
        assert_eq!(start_of_packet_detector(INPUT_FIVE), 11, "five");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(start_of_message_detector(INPUT_ONE), 19, "one");
        assert_eq!(start_of_message_detector(INPUT_TWO), 23, "two");
        assert_eq!(start_of_message_detector(INPUT_THREE), 23, "three");
        assert_eq!(start_of_message_detector(INPUT_FOUR), 29, "four");
        assert_eq!(start_of_message_detector(INPUT_FIVE), 26, "five");
    }
}
