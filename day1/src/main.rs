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
    let mut lines = input.lines();
    let mut prev_depth: i64 = lines.next().unwrap().trim().parse().unwrap();
    let mut number_of_depth_increases = 0;

    for line in lines {
        let curr_depth: i64 = line.trim().parse().unwrap();
        if curr_depth > prev_depth {
            number_of_depth_increases += 1;
        }
        prev_depth = curr_depth;
    }

    println!("Part 1: {}", number_of_depth_increases);
}

fn part2(input: &str) {
    let lines = input.lines();
    let mut measurments = Vec::<i64>::new();
    let mut number_of_depth_increases = 0;

    for line in lines {
        measurments.push(line.trim().parse().unwrap());
    }

    for i in 3..measurments.len() {
        if measurments[i] > measurments[i - 3] {
            number_of_depth_increases += 1;
        }
    }

    println!("Part 2: {}", number_of_depth_increases);
}
