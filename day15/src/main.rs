use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
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
    let risk_map = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let y_max = risk_map.len();
    let x_max = risk_map[0].len();

    let mut path_costs = HashMap::<(usize, usize), u32>::new();
    let mut heap = BinaryHeap::<PartialPath>::new();

    heap.push(PartialPath {
        position: (0, 0),
        path_cost: (0),
        goal: (y_max - 1, x_max - 1),
    });
    path_costs.insert((0, 0), 0);
    let cost = loop {
        let PartialPath {
            position,
            path_cost,
            goal,
        } = heap.pop().unwrap();

        if position == goal {
            break path_cost;
        }
        if path_cost > path_costs[&position] {
            continue;
        }
        let (y, x) = position;
        for adjacent_y in y.saturating_sub(1)..=(y + 1).min(y_max - 1) {
            for adjacent_x in x.saturating_sub(1)..=(x + 1).min(x_max - 1) {
                if !(adjacent_y == y) ^ (adjacent_x == x) {
                    continue;
                }
                let next_cost = path_cost + risk_map[adjacent_y][adjacent_x];
                let next_position = (adjacent_y, adjacent_x);
                if path_costs.contains_key(&next_position) && next_cost >= path_costs[&next_position] {
                    continue;
                }
                path_costs.insert(next_position, next_cost);
                heap.push(PartialPath {
                    position: next_position,
                    path_cost: next_cost,
                    goal,
                })
            }
        }
    };

    println!("Part 1: {}", cost);
}

fn part2(input: &str) {
    let risk_map = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let original_y_max = risk_map.len();
    let original_x_max = risk_map[0].len();
    let y_max = 5 * original_y_max;
    let x_max = 5 * original_x_max;

    let mut path_costs = HashMap::<(usize, usize), u32>::new();
    let mut heap = BinaryHeap::<PartialPath>::new();

    heap.push(PartialPath {
        position: (0, 0),
        path_cost: (0),
        goal: (y_max - 1, x_max - 1),
    });
    path_costs.insert((0, 0), 0);
    let cost = loop {
        let PartialPath {
            position,
            path_cost,
            goal,
        } = heap.pop().unwrap();

        if position == goal {
            break path_cost;
        }
        if path_cost > path_costs[&position] {
            continue;
        }
        let (y, x) = position;
        for adjacent_y in y.saturating_sub(1)..=(y + 1).min(y_max - 1) {
            for adjacent_x in x.saturating_sub(1)..=(x + 1).min(x_max - 1) {
                if !(adjacent_y == y) ^ (adjacent_x == x) {
                    continue;
                }
                let next_cost = path_cost + (risk_map[adjacent_y % original_y_max][adjacent_x % original_x_max] + (adjacent_y / original_y_max) as u32 + (adjacent_x / original_x_max) as u32 - 1) % 9 + 1;
                let next_position = (adjacent_y, adjacent_x);
                if path_costs.contains_key(&next_position) && next_cost >= path_costs[&next_position] {
                    continue;
                }
                path_costs.insert(next_position, next_cost);
                heap.push(PartialPath {
                    position: next_position,
                    path_cost: next_cost,
                    goal,
                })
            }
        }
    };

    println!("Part 2: {}", cost);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct PartialPath {
    position: (usize, usize),
    path_cost: u32,
    goal: (usize, usize),
}

impl Ord for PartialPath {
    fn cmp(&self, other: &Self) -> Ordering {
        u32::cmp(
            &(other.path_cost + heuristic(&other.position, &other.goal)),
            &(&self.path_cost + heuristic(&self.position, &self.goal)),
        )
    }
}

impl PartialOrd for PartialPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(start: &(usize, usize), goal: &(usize, usize)) -> u32 {
    let dy = if start.1 > goal.1 {
        start.1 - goal.1
    } else {
        goal.1 - start.1
    };
    let dx = if start.0 > goal.0 {
        start.0 - goal.0
    } else {
        goal.0 - start.0
    };
    usize::max(dy, dx) as u32
}
