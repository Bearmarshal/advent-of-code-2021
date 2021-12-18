use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
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
    let lines = input.lines();

    println!("Part 1: {}", "");
}

fn part2(input: &str) {
    let mut bit_reader = BitReader::new(input.trim());

    println!("Part 2: {}", "");
}

#[derive(Debug)]
struct BitReader<'a> {
    hex_chars: Chars<'a>,
    leftover_bits: u64,
    num_leftover_bits: usize,
}

impl BitReader<'_> {
    fn new<'a>(hex_string: &'a str) -> BitReader<'a> {
        BitReader {
            hex_chars: hex_string.chars(),
            leftover_bits: 0,
            num_leftover_bits: 0,
        }
    }

    fn get(&mut self, num_bits: usize) -> u64 {
        while self.num_leftover_bits < num_bits {
            self.leftover_bits <<= 4;
            self.leftover_bits |= self.hex_chars.next().unwrap().to_digit(16).unwrap() as u64;
            self.num_leftover_bits += 4;
        }
        self.num_leftover_bits -= num_bits;
        let value = self.leftover_bits >> self.num_leftover_bits;
        self.leftover_bits &= (1 << self.num_leftover_bits) - 1;
        return value;
    }
}

fn decode_version_sum(bit_reader: &mut BitReader) -> (u64, u64) {
    let mut version_sum = bit_reader.get(3);
    let mut packet_size = 6;
    match bit_reader.get(3) {
        4 => {
            while bit_reader.get(1) != 0 {
                bit_reader.get(4);
                packet_size += 5;
            }
            bit_reader.get(4);
            packet_size += 5;
        }
        _ => match bit_reader.get(1) {
            0 => {
                let mut length = bit_reader.get(15);
                packet_size += 1 + 15 + length;
                while length > 0 {
                    let (sum, size) = decode_version_sum(bit_reader);
                    version_sum += sum;
                    length -= size;
                }
            }
            1 => {
                packet_size += 1 + 11;
                for _ in 0..bit_reader.get(11) {
                    let (sum, size) = decode_version_sum(bit_reader);
                    version_sum += sum;
                    packet_size += size;
                }
            }
            _ => panic!(),
        },
    }
    return (version_sum, packet_size);
}

enum Explosion {
    None,
    Ongoing(Option<i32>, Option<i32>),
}

enum Split {
    None,
    Happened,
}

enum SnailfishNumber {
    RegularNumber(i32),
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

    fn value(&self) -> Option<i32> {
        match self {
            SnailfishNumber::RegularNumber(value) => Some(*value),
            SnailfishNumber::Pair(_, _) => None,
        }
    }

    fn add_to_leftmost(&mut self, value: i32) {
        match self {
            SnailfishNumber::RegularNumber(own_value) => *own_value += value,
            SnailfishNumber::Pair(left, _) => left.add_to_leftmost(value),
        }
    }

    fn add_to_rightmost(&mut self, value: i32) {
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
}

fn decode(snailfish_chars: &mut Chars) -> SnailfishNumber {
    lazy_static! {
        static ref TOKEN_REGEX: Regex =
            Regex::new(r"[\[\],0-9]").unwrap();
    }

    match snailfish_chars.next() {
        Some('[') => todo!(),
        Some(_) => todo!(),
        None => todo!(),
    }
}

// fn decode_subpackets(bit_reader: &mut BitReader) -> (Vec<Box<dyn SnailfishNumber>>, u64) {
//     let mut subpackets = Vec::new();
//     let mut subpackets_size = 1;
//     match bit_reader.get(1) {
//         0 => {
//             let mut length = bit_reader.get(15);
//             subpackets_size += 15 + length;
//             while length > 0 {
//                 let (subpacket, size) = decode(bit_reader);
//                 subpackets.push(subpacket);
//                 length -= size;
//             }
//         }
//         1 => {
//             subpackets_size += 11;
//             for _ in 0..bit_reader.get(11) {
//                 let (subpacket, size) = decode(bit_reader);
//                 subpackets.push(subpacket);
//                 subpackets_size += size;
//             }
//         }
//         _ => panic!(),
//     }
//     (subpackets, subpackets_size)
// }
