use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::prelude::*;

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
    let mut individual_alignments: VecDeque<_> = input
        .trim()
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    individual_alignments.make_contiguous().sort();
    let mut alignments = VecDeque::<(i32, i32)>::new();
    for alignment in individual_alignments {
        match alignments.iter_mut().last() {
            Some((a, number)) if *a == alignment => *number += 1,
            _ => alignments.push_back((alignment, 1)),
        }
    }

    let mut consumed_fuel = 0;
    while alignments.len() > 1 {
        let (_, number_lowest) = alignments.iter().next().unwrap();
        let (_, number_highest) = alignments.iter().last().unwrap();
        if number_lowest < number_highest {
            let (lowest_alignment, number_lowest) = alignments.pop_front().unwrap();
            let (next_lowest_alignment, number_next_lowest) = alignments.iter_mut().next().unwrap();
            *number_next_lowest += number_lowest;
            consumed_fuel += number_lowest * (*next_lowest_alignment - lowest_alignment);
        } else {
            let (highest_alignment, number_highest) = alignments.pop_back().unwrap();
            let (next_highest_alignment, number_next_highest) =
                alignments.iter_mut().last().unwrap();
            *number_next_highest += number_highest;
            consumed_fuel += number_highest * (highest_alignment - *next_highest_alignment);
        }
    }

    println!("Part 1: {}", consumed_fuel);
}

fn part2(input: &str) {
    let mut individual_alignments: VecDeque<_> = input
        .trim()
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    individual_alignments.make_contiguous().sort();
    let mut alignments = VecDeque::<(i32, i32)>::new();
    for alignment in individual_alignments {
        match alignments.iter_mut().last() {
            Some((a, number)) if *a == alignment => *number += 1,
            _ => alignments.push_back((alignment, 1)),
        }
    }

    let mut best_consumption = calculate_consumption_for_alignment(&alignments, 0);
    for alignment in alignments.iter().next().unwrap().0..=alignments.iter().last().unwrap().0 {
        let consumption = calculate_consumption_for_alignment(&alignments, alignment);
        if consumption < best_consumption {
            best_consumption = consumption;
        }
    }

    println!("Part 2: {}", best_consumption);
}

fn calculate_consumption_for_alignment(
    alignments: &VecDeque<(i32, i32)>,
    target_alignment: i32,
) -> i32 {
    let mut total_consumption = 0;
    for (alignment, number) in alignments {
        let alignment_diff = (target_alignment - alignment).abs();
        total_consumption += number * alignment_diff * (alignment_diff + 1) / 2;
    }
    total_consumption
}
