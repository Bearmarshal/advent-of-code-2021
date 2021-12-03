use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    part1(&input);
    part2(&input);
    Ok(())
}

fn part1(input: &str) {
    let lines = input.lines();
    let mut rate_bits = Vec::<(u32, u32)>::new();

    for line in lines {
        if rate_bits.len() == 0 {
            rate_bits.resize(line.trim().len(), (0, 0));
        }

        for (bit_index, digit) in line.trim().chars().enumerate() {
            match digit {
                '0' => rate_bits[bit_index].0 += 1,
                '1' => rate_bits[bit_index].1 += 1,
                _ => panic!(),
            };
        }
    }

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for (num_zeros, num_ones) in rate_bits {
        gamma_rate <<= 1;
        epsilon_rate <<= 1;
        if num_ones > num_zeros {
            gamma_rate += 1
        } else {
            epsilon_rate += 1
        }
    }

    println!("Part 1: {}", gamma_rate * epsilon_rate);
}

fn part2(input: &str) {
    let lines: Vec<&str> = input.lines().map(str::trim).collect();
    let line_digits: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(2).unwrap())
                .collect()
        })
        .collect();
    let mut o2_candidates = line_digits.clone();
    let mut co2_candidates = line_digits.clone();
    let num_bits = lines[0].len();

    let mut o2_generator_rate = 0;
    let mut co2_scrubber_rate = 0;

    for bit_index in 0..num_bits {
        if o2_candidates.len() > 1 {
            let most_common_o2_bit = get_most_common_bit(&o2_candidates, bit_index);
            o2_candidates.retain(|bit_vector| bit_vector[bit_index] == most_common_o2_bit);
        }

        if co2_candidates.len() > 1 {
            let most_common_co2_bit = get_most_common_bit(&co2_candidates, bit_index);
            co2_candidates.retain(|bit_vector| bit_vector[bit_index] != most_common_co2_bit);
        }
    }

    assert_eq!(o2_candidates.len(), 1);
    assert_eq!(co2_candidates.len(), 1);

    for bit in &o2_candidates[0] {
        o2_generator_rate <<= 1;
        o2_generator_rate += bit;
    }

    for bit in &co2_candidates[0] {
        co2_scrubber_rate <<= 1;
        co2_scrubber_rate += bit;
    }

    println!("Part 2: {}", o2_generator_rate * co2_scrubber_rate);
}

fn get_most_common_bit(bit_vectors: &Vec<Vec<u32>>, bit_index: usize) -> u32 {
    let mut num_zeros = 0;
    let mut num_ones = 0;
    for bits in bit_vectors {
        match bits[bit_index] {
            0 => num_zeros += 1,
            1 => num_ones += 1,
            _ => panic!(),
        };
    }
    if num_zeros > num_ones {
        0
    } else {
        1
    }
}
