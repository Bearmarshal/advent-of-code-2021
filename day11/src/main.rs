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
    let mut energy_map = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let y_max = energy_map.len();
    let x_max = energy_map[0].len();

    let mut num_flashes = 0;
    for _ in 0..100 {
        let mut to_flash = Vec::<(usize, usize)>::new();
        for y in 0..y_max {
            for x in 0..x_max {
                if energy_map[y][x] > 9 {
                    energy_map[y][x] = 1;
                } else {
                    energy_map[y][x] += 1;
                    if energy_map[y][x] == 10 {
                        to_flash.push((y, x));
                    }
                }
            }
        }

        while let Some((y, x)) = to_flash.pop() {
            num_flashes += 1;
            for adjacent_y in y.checked_sub(1).unwrap_or(0)..=(y + 1).min(y_max - 1) {
                for adjacent_x in x.checked_sub(1).unwrap_or(0)..=(x + 1).min(x_max - 1) {
                    if !(adjacent_y == y && adjacent_x == x) {
                        energy_map[adjacent_y][adjacent_x] += 1;
                        if energy_map[adjacent_y][adjacent_x] == 10 {
                            to_flash.push((adjacent_y, adjacent_x));
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", num_flashes);
}

fn part2(input: &str) {
    let mut energy_map = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let y_max = energy_map.len();
    let x_max = energy_map[0].len();

    let mut step = 0;
    let all_flashes = loop {
        step += 1;
        let mut num_flashes = 0;
        let mut to_flash = Vec::<(usize, usize)>::new();
        for y in 0..y_max {
            for x in 0..x_max {
                if energy_map[y][x] > 9 {
                    energy_map[y][x] = 1;
                } else {
                    energy_map[y][x] += 1;
                    if energy_map[y][x] == 10 {
                        to_flash.push((y, x));
                    }
                }
            }
        }

        while let Some((y, x)) = to_flash.pop() {
            num_flashes += 1;
            for adjacent_y in y.checked_sub(1).unwrap_or(0)..=(y + 1).min(y_max - 1) {
                for adjacent_x in x.checked_sub(1).unwrap_or(0)..=(x + 1).min(x_max - 1) {
                    if !(adjacent_y == y && adjacent_x == x) {
                        energy_map[adjacent_y][adjacent_x] += 1;
                        if energy_map[adjacent_y][adjacent_x] == 10 {
                            to_flash.push((adjacent_y, adjacent_x));
                        }
                    }
                }
            }
        }

        if num_flashes == y_max * x_max {
            break step;
        }
    };

    println!("Part 2: {}", all_flashes);
}
