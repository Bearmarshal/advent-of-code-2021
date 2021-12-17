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

    println!("Part 1: {}", decode_version_sum(&mut bit_reader).0);
}

fn part2(input: &str) {
    let mut bit_reader = BitReader::new(input.trim());

    println!("Part 2: {}", decode(&mut bit_reader).0.evaluate());
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

trait Packet {
    fn evaluate(&self) -> u64;
}

struct Literal {
    value: u64,
}

impl Literal {
    const ID: u64 = 4;

    fn from_bits(bit_reader: &mut BitReader) -> (Box<dyn Packet>, u64) {
        let mut value = 0;
        let mut size = 0;
        loop {
            let done = bit_reader.get(1) == 0;
            value = value << 4 | bit_reader.get(4);
            size += 5;
            if done {
                break;
            }
        }
        (Box::new(Literal { value }), size)
    }
}

impl Packet for Literal {
    fn evaluate(&self) -> u64 {
        self.value
    }
}

struct Add {
    subpackets: Vec<Box<dyn Packet>>,
}

impl Add {
    const ID: u64 = 0;

    fn from_bits(bit_reader: &mut BitReader) -> (Box<dyn Packet>, u64) {
        let (subpackets, size) = decode_subpackets(bit_reader);
        (Box::new(Add { subpackets }), size)
    }
}

impl Packet for Add {
    fn evaluate(&self) -> u64 {
        self.subpackets.iter().map(|packet| packet.evaluate()).sum()
    }
}

struct Product {
    subpackets: Vec<Box<dyn Packet>>,
}

impl Product {
    const ID: u64 = 1;

    fn from_bits(bit_reader: &mut BitReader) -> (Box<dyn Packet>, u64) {
        let (subpackets, size) = decode_subpackets(bit_reader);
        (Box::new(Product { subpackets }), size)
    }
}

impl Packet for Product {
    fn evaluate(&self) -> u64 {
        self.subpackets
            .iter()
            .map(|packet| packet.evaluate())
            .product()
    }
}

struct Minimum {
    subpackets: Vec<Box<dyn Packet>>,
}

impl Minimum {
    const ID: u64 = 2;

    fn from_bits(bit_reader: &mut BitReader) -> (Box<dyn Packet>, u64) {
        let (subpackets, size) = decode_subpackets(bit_reader);
        (Box::new(Minimum { subpackets }), size)
    }
}

impl Packet for Minimum {
    fn evaluate(&self) -> u64 {
        self.subpackets
            .iter()
            .map(|packet| packet.evaluate())
            .min()
            .unwrap()
    }
}

struct Maximum {
    subpackets: Vec<Box<dyn Packet>>,
}

impl Maximum {
    const ID: u64 = 3;

    fn from_bits(bit_reader: &mut BitReader) -> (Box<dyn Packet>, u64) {
        let (subpackets, size) = decode_subpackets(bit_reader);
        (Box::new(Maximum { subpackets }), size)
    }
}

impl Packet for Maximum {
    fn evaluate(&self) -> u64 {
        self.subpackets
            .iter()
            .map(|packet| packet.evaluate())
            .max()
            .unwrap()
    }
}

struct GreaterThan {
    first_subpacket: Box<dyn Packet>,
    second_subpacket: Box<dyn Packet>,
}

impl GreaterThan {
    const ID: u64 = 5;

    fn from_bits(bit_reader: &mut BitReader) -> (Box<dyn Packet>, u64) {
        let (subpackets, size) = decode_subpackets(bit_reader);
        let mut subpacket_iterator = subpackets.into_iter();
        (
            Box::new(GreaterThan {
                first_subpacket: subpacket_iterator.next().unwrap(),
                second_subpacket: subpacket_iterator.next().unwrap(),
            }),
            size,
        )
    }
}

impl Packet for GreaterThan {
    fn evaluate(&self) -> u64 {
        if self.first_subpacket.evaluate() > self.second_subpacket.evaluate() {
            1
        } else {
            0
        }
    }
}

struct LesserThan {
    first_subpacket: Box<dyn Packet>,
    second_subpacket: Box<dyn Packet>,
}

impl LesserThan {
    const ID: u64 = 6;

    fn from_bits(bit_reader: &mut BitReader) -> (Box<dyn Packet>, u64) {
        let (subpackets, size) = decode_subpackets(bit_reader);
        let mut subpacket_iterator = subpackets.into_iter();
        (
            Box::new(LesserThan {
                first_subpacket: subpacket_iterator.next().unwrap(),
                second_subpacket: subpacket_iterator.next().unwrap(),
            }),
            size,
        )
    }
}

impl Packet for LesserThan {
    fn evaluate(&self) -> u64 {
        if self.first_subpacket.evaluate() < self.second_subpacket.evaluate() {
            1
        } else {
            0
        }
    }
}

struct EqualTo {
    first_subpacket: Box<dyn Packet>,
    second_subpacket: Box<dyn Packet>,
}

impl EqualTo {
    const ID: u64 = 7;

    fn from_bits(bit_reader: &mut BitReader) -> (Box<dyn Packet>, u64) {
        let (subpackets, size) = decode_subpackets(bit_reader);
        let mut subpacket_iterator = subpackets.into_iter();
        (
            Box::new(EqualTo {
                first_subpacket: subpacket_iterator.next().unwrap(),
                second_subpacket: subpacket_iterator.next().unwrap(),
            }),
            size,
        )
    }
}

impl Packet for EqualTo {
    fn evaluate(&self) -> u64 {
        if self.first_subpacket.evaluate() == self.second_subpacket.evaluate() {
            1
        } else {
            0
        }
    }
}

fn decode(bit_reader: &mut BitReader) -> (Box<dyn Packet>, u64) {
    bit_reader.get(3);
    let mut decoded_packet = match bit_reader.get(3) {
        Add::ID => Add::from_bits(bit_reader),
        Product::ID => Product::from_bits(bit_reader),
        Minimum::ID => Minimum::from_bits(bit_reader),
        Maximum::ID => Maximum::from_bits(bit_reader),
        Literal::ID => Literal::from_bits(bit_reader),
        GreaterThan::ID => GreaterThan::from_bits(bit_reader),
        LesserThan::ID => LesserThan::from_bits(bit_reader),
        EqualTo::ID => EqualTo::from_bits(bit_reader),
        _ => panic!(),
    };

    decoded_packet.1 += 6;
    return decoded_packet;
}

fn decode_subpackets(bit_reader: &mut BitReader) -> (Vec<Box<dyn Packet>>, u64) {
    let mut subpackets = Vec::new();
    let mut subpackets_size = 1;
    match bit_reader.get(1) {
        0 => {
            let mut length = bit_reader.get(15);
            subpackets_size += 15 + length;
            while length > 0 {
                let (subpacket, size) = decode(bit_reader);
                subpackets.push(subpacket);
                length -= size;
            }
        }
        1 => {
            subpackets_size += 11;
            for _ in 0..bit_reader.get(11) {
                let (subpacket, size) = decode(bit_reader);
                subpackets.push(subpacket);
                subpackets_size += size;
            }
        }
        _ => panic!(),
    }
    (subpackets, subpackets_size)
}
