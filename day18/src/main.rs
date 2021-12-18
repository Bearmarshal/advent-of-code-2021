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
    let mut bit_reader = BitReader::new(input.trim());

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

trait SnailfishNumber {
    fn explode(&mut self, self_box: &mut Box<dyn SnailfishNumber>, recursion_depth: u32) -> Explosion;
    fn value(&self) -> Option<i32>;
    fn add_to_leftmost(&mut self, value: i32);
    fn add_to_rightmost(&mut self, value: i32);
    fn split(&mut self, self_box: &mut Box<dyn SnailfishNumber>) -> Split;
}

struct RegularNumber {
    value: i32,
}

impl RegularNumber {
    fn boxed(value: i32) -> Box<RegularNumber> {
        Box::new(RegularNumber { value })
    }
}

impl SnailfishNumber for RegularNumber {
    fn explode(&mut self, _self_box: &mut Box<dyn SnailfishNumber>, _recursion_depth: u32) -> Explosion {
        Explosion::None
    }

    fn value(&self) -> Option<i32> {
        Some(self.value)
    }

    fn add_to_leftmost(&mut self, value: i32) {
        self.value += value;
    }

    fn add_to_rightmost(&mut self, value: i32) {
        self.value += value;
    }

    fn split(&mut self, self_box: &mut Box<dyn SnailfishNumber>) -> Split {
        if self.value >= 10 {
            *self_box = Pair::boxed(
                RegularNumber::boxed(self.value / 2),
                RegularNumber::boxed((self.value + 1) / 2),
            );
            Split::Happened
        } else {
            Split::None
        }
    }
}

struct Pair {
    left: Box<dyn SnailfishNumber>,
    right: Box<dyn SnailfishNumber>,
}

impl Pair {
    fn boxed(left: Box<dyn SnailfishNumber>, right: Box<dyn SnailfishNumber>) -> Box<Pair> {
        Box::new(Pair { left, right })
    }
}

impl SnailfishNumber for Pair {
    fn explode(&mut self, self_box: &mut Box<dyn SnailfishNumber>, recursion_depth: u32) -> Explosion {
        if recursion_depth >= 4 {
            if let (Some(left_value), Some(right_value)) = (self.left.value(), self.right.value()) {
                *self_box = RegularNumber::boxed(0);
                Explosion::Ongoing(Some(left_value), Some(right_value))
            } else {
                Explosion::None
            }
        } else {
            match self.left.explode(&mut self.left, recursion_depth + 1) {
                Explosion::Ongoing(left_value, right_value) => match right_value {
                    Some(value) => {
                        self.right.add_to_leftmost(value);
                        Explosion::Ongoing(left_value, None)
                    }
                    None => Explosion::Ongoing(left_value, right_value),
                }
                Explosion::None => match self.right.explode(&mut self.right, recursion_depth + 1) {
                    Explosion::Ongoing(left_value, right_value) => match left_value {
                        Some(value) => {
                            self.left.add_to_rightmost(value);
                            Explosion::Ongoing(left_value, None)
                        }
                        None => Explosion::Ongoing(left_value, right_value),
                    }
                    result => result,
                },
            }
        }
    }

    fn value(&self) -> Option<i32> {
        None
    }

    fn add_to_leftmost(&mut self, value: i32) {
        self.left.add_to_leftmost(value);
    }

    fn add_to_rightmost(&mut self, value: i32) {
        self.right.add_to_rightmost(value);
    }

    fn split(&mut self, _self_box: &mut Box<dyn SnailfishNumber>) -> Split {
        match self.left.split(&mut self.left) {
            Split::Happened => Split::Happened,
            Split::None => self.right.split(&mut self.right),
        }
    }
}

// fn decode(bit_reader: &mut BitReader) -> (Box<dyn SnailfishNumber>, u64) {
//     bit_reader.get(3);
//     let mut decoded_packet = match bit_reader.get(3) {
//         Pair::ID => Pair::from_bits(bit_reader),
//         Product::ID => Product::from_bits(bit_reader),
//         Minimum::ID => Minimum::from_bits(bit_reader),
//         Maximum::ID => Maximum::from_bits(bit_reader),
//         RegularNumber::ID => RegularNumber::from_bits(bit_reader),
//         GreaterThan::ID => GreaterThan::from_bits(bit_reader),
//         LesserThan::ID => LesserThan::from_bits(bit_reader),
//         EqualTo::ID => EqualTo::from_bits(bit_reader),
//         _ => panic!(),
//     };

//     decoded_packet.1 += 6;
//     return decoded_packet;
// }

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
