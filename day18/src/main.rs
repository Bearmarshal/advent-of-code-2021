use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;
use std::str::Chars;

fn main() -> std::io::Result<()> {
    let mut input_file = File::open(match env::args().skip(1).next() {
        Some(filename) => filename,
        None => "input.txt".to_string(),
    })?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    part1(&input);
    part2(&input);
    Ok(())
}

fn part1(input: &str) {
    let magnitude = input
        .lines()
        .map(|line| decode(&mut line.trim().chars()))
        .reduce(SnailfishNumber::add)
        .unwrap()
        .magnitude();

    println!("Part 1: {}", magnitude);
}

fn part2(input: &str) {
    let snailfish_numbers = input
        .lines()
        .map(|line| decode(&mut line.trim().chars()))
        .collect::<Vec<_>>();
    let mut magnitude_max = 0;
    for (i, first_number) in snailfish_numbers.iter().enumerate() {
        for (j, second_number) in snailfish_numbers.iter().enumerate() {
            if i == j {
                continue;
            }
            magnitude_max = *[
                magnitude_max,
                (first_number.clone() + second_number.clone()).magnitude(),
                (second_number.clone() + first_number.clone()).magnitude(),
            ]
            .iter()
            .max()
            .unwrap();
        }
    }
    println!("Part 2: {}", magnitude_max);
}

enum Explosion {
    None,
    Ongoing(Option<u32>, Option<u32>),
}

enum Split {
    None,
    Happened,
}

#[derive(Clone)]
enum SnailfishNumber {
    RegularNumber(u32),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

impl SnailfishNumber {
    fn explode(&mut self, recursion_depth: u32) -> Explosion {
        match self {
            SnailfishNumber::RegularNumber(_) => Explosion::None,
            SnailfishNumber::Pair(left, right) if recursion_depth >= 4 => {
                if let (Some(left_value), Some(right_value)) = (left.value(), right.value()) {
                    *self = SnailfishNumber::RegularNumber(0);
                    Explosion::Ongoing(Some(left_value), Some(right_value))
                } else {
                    Explosion::None
                }
            }
            SnailfishNumber::Pair(left, right) => match left.explode(recursion_depth + 1) {
                Explosion::Ongoing(left_value, right_value) => match right_value {
                    Some(value) => {
                        right.add_to_leftmost(value);
                        Explosion::Ongoing(left_value, None)
                    }
                    None => Explosion::Ongoing(left_value, right_value),
                },
                Explosion::None => match right.explode(recursion_depth + 1) {
                    Explosion::Ongoing(left_value, right_value) => match left_value {
                        Some(value) => {
                            left.add_to_rightmost(value);
                            Explosion::Ongoing(None, right_value)
                        }
                        None => Explosion::Ongoing(left_value, right_value),
                    },
                    result => result,
                },
            },
        }
    }

    fn value(&self) -> Option<u32> {
        match self {
            SnailfishNumber::RegularNumber(value) => Some(*value),
            SnailfishNumber::Pair(_, _) => None,
        }
    }

    fn add_to_leftmost(&mut self, value: u32) {
        match self {
            SnailfishNumber::RegularNumber(own_value) => *own_value += value,
            SnailfishNumber::Pair(left, _) => left.add_to_leftmost(value),
        }
    }

    fn add_to_rightmost(&mut self, value: u32) {
        match self {
            SnailfishNumber::RegularNumber(own_value) => *own_value += value,
            SnailfishNumber::Pair(_, right) => right.add_to_rightmost(value),
        }
    }

    fn split(&mut self) -> Split {
        match self {
            SnailfishNumber::RegularNumber(value) if *value >= 10 => {
                *self = SnailfishNumber::Pair(
                    Box::new(SnailfishNumber::RegularNumber(*value / 2)),
                    Box::new(SnailfishNumber::RegularNumber((*value + 1) / 2)),
                );
                Split::Happened
            }
            SnailfishNumber::RegularNumber(_) => Split::None,
            SnailfishNumber::Pair(left, right) => match left.split() {
                Split::Happened => Split::Happened,
                Split::None => right.split(),
            },
        }
    }

    fn magnitude(self) -> u32 {
        match self {
            SnailfishNumber::RegularNumber(value) => value,
            SnailfishNumber::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut output = SnailfishNumber::Pair(Box::new(self), Box::new(rhs));
        loop {
            if let Explosion::Ongoing(_, _) = output.explode(0) {
                continue;
            } else if let Split::Happened = output.split() {
                continue;
            } else {
                break output;
            }
        }
    }
}

fn decode(snailfish_chars: &mut Chars) -> SnailfishNumber {
    match snailfish_chars.next() {
        Some('[') => {
            let first = decode(snailfish_chars);
            if snailfish_chars.next() != Some(',') {
                panic!()
            }
            let second = decode(snailfish_chars);
            if snailfish_chars.next() != Some(']') {
                panic!()
            }
            SnailfishNumber::Pair(Box::new(first), Box::new(second))
        }
        Some(digit_char) => SnailfishNumber::RegularNumber(digit_char.to_digit(10).unwrap()),
        None => panic!(),
    }
}
