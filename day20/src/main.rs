use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::ops::RangeInclusive;

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
    lazy_static! {
        static ref EMPTY_LINE_REGEX: Regex = Regex::new(r"\r?\n\r?\n").unwrap();
    }

    let mut input_blocks = EMPTY_LINE_REGEX.split(input);
    let algorithm: Vec<bool> = input_blocks
        .next()
        .unwrap()
        .chars()
        .map(|pixel| pixel == '#')
        .collect();
    let image_input = input_blocks.next().unwrap().trim();
    let mut image: HashSet<(i32, i32)> = image_input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter_map(move |(x, pixel)| {
                    if pixel == '#' {
                        Some((y as i32, x as i32))
                    } else {
                        None
                    }
                })
        })
        .collect();
    let mut y_bounds_of_the_known_world = 0..=image_input.lines().count() as i32 - 1;
    let mut x_bounds_of_the_known_world =
        0..=image_input.lines().next().unwrap().trim().len() as i32 - 1;
    let mut terra_incognita_is_lit = false;

    for _ in 0..2 {
        let mut new_image = HashSet::<(i32, i32)>::new();
        for center_y in expanded(&y_bounds_of_the_known_world, 1) {
            for center_x in expanded(&x_bounds_of_the_known_world, 1) {
                let mut algorithm_index = 0;
                for neighbour_y in center_y - 1..=center_y + 1 {
                    for neighbour_x in center_x - 1..=center_x + 1 {
                        algorithm_index <<= 1;
                        algorithm_index |= if y_bounds_of_the_known_world.contains(&neighbour_y)
                            && x_bounds_of_the_known_world.contains(&neighbour_x)
                        {
                            if image.contains(&(neighbour_y, neighbour_x)) {
                                1
                            } else {
                                0
                            }
                        } else {
                            if terra_incognita_is_lit {
                                1
                            } else {
                                0
                            }
                        };
                    }
                }
                if algorithm[algorithm_index] {
                    new_image.insert((center_y, center_x));
                }
            }
        }
        image = new_image;
        y_bounds_of_the_known_world = expanded(&y_bounds_of_the_known_world, 1);
        x_bounds_of_the_known_world = expanded(&x_bounds_of_the_known_world, 1);
        terra_incognita_is_lit = if terra_incognita_is_lit {
            algorithm[0b111111111]
        } else {
            algorithm[0b000000000]
        };
    }

    println!("Part 1: {}", image.len());

    for _ in 2..50 {
        let mut new_image = HashSet::<(i32, i32)>::new();
        for center_y in expanded(&y_bounds_of_the_known_world, 1) {
            for center_x in expanded(&x_bounds_of_the_known_world, 1) {
                let mut algorithm_index = 0;
                for neighbour_y in center_y - 1..=center_y + 1 {
                    for neighbour_x in center_x - 1..=center_x + 1 {
                        algorithm_index <<= 1;
                        algorithm_index |= if y_bounds_of_the_known_world.contains(&neighbour_y)
                            && x_bounds_of_the_known_world.contains(&neighbour_x)
                        {
                            if image.contains(&(neighbour_y, neighbour_x)) {
                                1
                            } else {
                                0
                            }
                        } else {
                            if terra_incognita_is_lit {
                                1
                            } else {
                                0
                            }
                        };
                    }
                }
                if algorithm[algorithm_index] {
                    new_image.insert((center_y, center_x));
                }
            }
        }
        image = new_image;
        y_bounds_of_the_known_world = expanded(&y_bounds_of_the_known_world, 1);
        x_bounds_of_the_known_world = expanded(&x_bounds_of_the_known_world, 1);
        terra_incognita_is_lit = if terra_incognita_is_lit {
            algorithm[0b111111111]
        } else {
            algorithm[0b000000000]
        };
    }

    println!("Part 2: {}", image.len());
}

fn part2(_input: &str) {}

// fn print_map(map: &HashSet<(i32, i32)>, y_bounds: &RangeInclusive<i32>, x_bounds: &RangeInclusive<i32>, terra_incognita_is_lit: bool) {
//     for y in y_bounds.start() -1..=y_bounds.end()+1 {
//         for x in x_bounds.start() -1..=x_bounds.end()+1 {
//             if y_bounds.contains(&y) && x_bounds.contains(&x) {
//             if map.contains(&(y, x)) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         } else {
//             if terra_incognita_is_lit {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         }
//         println!();
//     }
// }

fn expanded(range: &RangeInclusive<i32>, by: i32) -> RangeInclusive<i32> {
    range.start() - by..=range.end() + by
}
