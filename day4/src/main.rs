use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
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
    let board_regex = Regex::new(r"(?:( *\d+){5}(?:\r?\n|$)){5}").unwrap();
    let bingo_numbers = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(str::trim)
        .map(str::parse::<i32>)
        .flatten();

    let mut bingo_boards: Vec<BingoBoard> = board_regex
        .find_iter(input)
        .map(|board_match| BingoBoard::new(board_match.as_str()))
        .collect();

    let mut winning_score = -1;
    'numbers_loop: for called_number in bingo_numbers {
        for board in bingo_boards.iter_mut() {
            if board.mark_and_check(called_number) {
                winning_score = board.calc_score() * called_number;
                break 'numbers_loop;
            }
        }
    }

    println!("Part 1: {}", winning_score);
}

fn part2(input: &str) {
    let board_regex = Regex::new(r"(?:( *\d+){5}(?:\r?\n|$)){5}").unwrap();
    let bingo_numbers = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(str::trim)
        .map(str::parse::<i32>)
        .flatten();

    let mut bingo_boards: Vec<BingoBoard> = board_regex
        .find_iter(input)
        .map(|board_match| BingoBoard::new(board_match.as_str()))
        .collect();

    let mut last_winning_score = -1;
    for called_number in bingo_numbers {
        if bingo_boards.len() > 1 {
            let winning_boards = bingo_boards
                .iter_mut()
                .map(|board| board.mark_and_check(called_number))
                .collect::<Vec<bool>>();
            let mut winning_boards_iter = winning_boards.iter();
            bingo_boards.retain(|_| !*winning_boards_iter.next().unwrap());
        } else if bingo_boards[0].mark_and_check(called_number) {
            last_winning_score = bingo_boards[0].calc_score() * called_number;
            break;
        }
    }

    println!("Part 2: {}", last_winning_score);
}

#[derive(Debug)]
struct BingoBoard {
    rows: Vec<Vec<(i32, bool)>>,
}

impl BingoBoard {
    pub fn new(board_repr: &str) -> BingoBoard {
        lazy_static! {
            static ref NUMBER_REGEX: Regex = Regex::new(r"\d+").unwrap();
        }

        BingoBoard {
            rows: board_repr
                .lines()
                .map(|line| {
                    NUMBER_REGEX
                        .find_iter(line)
                        .map(|number_match| (number_match.as_str().parse::<i32>().unwrap(), false))
                        .collect()
                })
                .collect(),
        }
    }

    pub fn mark_and_check(&mut self, called_number: i32) -> bool {
        for row in self.rows.iter_mut() {
            for (x, (number, marked)) in row.iter_mut().enumerate() {
                if *number == called_number {
                    *marked = true;
                    return row.iter().all(|(_, marked)| *marked)
                        || self.rows.iter().all(|row| row[x].1);
                }
            }
        }
        return false;
    }

    pub fn calc_score(&self) -> i32 {
        self.rows
            .iter()
            .flatten()
            .filter(|(_, is_checked)| !*is_checked)
            .map(|(number, _)| number)
            .fold(0, |a, b| a + b)
    }
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        for row in self.rows.iter() {
            for (number, marked) in row {
                let marking = if *marked { ("(", ")") } else { (" ", " ") };
                write!(f, "|{}{:>2}{}", marking.0, number, marking.1)?;
            }
            writeln!(f, "|")?;
        }
        Ok(())
    }
}
