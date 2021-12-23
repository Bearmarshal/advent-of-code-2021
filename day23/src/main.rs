use crate::PossibleMove::*;
use lazy_static::lazy_static;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::{env, vec};

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
    let playboard: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim_end().chars().collect())
        .collect();
    let mut wait_spots: HashMap<usize, Option<char>> = HashMap::new();
    let mut rooms: HashMap<char, (usize, Vec<char>)> = HashMap::new();
    let mut unknown_room_position = 'A'..='D';
    let mut room_ownership = HashMap::new();
    for y in 0..playboard.len() {
        for x in 0..playboard[y].len() {
            match playboard[y][x] {
                '.' if playboard[y + 1][x] == '#' => {
                    wait_spots.insert(x, None);
                }
                amphipod if ('A'..='D').contains(&amphipod) => {
                    let room_owner = *room_ownership
                        .entry(x)
                        .or_insert_with(|| unknown_room_position.next().unwrap());
                    rooms
                        .entry(room_owner)
                        .and_modify(|(_, occupants)| occupants.push(amphipod))
                        .or_insert((x, vec![amphipod]));
                }
                _ => (),
            }
        }
    }
    for (_, (_, occupants)) in rooms.iter_mut() {
        occupants.reverse();
    }
    let room_depth = rooms[&'A'].1.len();

    println!(
        "Part 1: {}",
        find_cheapest_solution(&wait_spots, &rooms, room_depth)
            .unwrap()
            .0
    );
}

fn part2(input: &str) {
    let mut folded_bit = Some("  #D#C#B#A#\n  #D#B#A#C#");
    let mut playboard_lines = vec![];
    for line in input.lines() {
        playboard_lines.push(line);
        if line.contains('A') || line.contains('C') || line.contains('B') || line.contains('D') {
            for extra_line in folded_bit
                .into_iter()
                .flat_map(|extra_input| extra_input.lines())
            {
                playboard_lines.push(extra_line);
            }
            folded_bit = None;
        }
    }

    let playboard: Vec<Vec<char>> = playboard_lines
        .into_iter()
        .map(|line| line.trim_end().chars().collect())
        .collect();
    let mut wait_spots: HashMap<usize, Option<char>> = HashMap::new();
    let mut rooms: HashMap<char, (usize, Vec<char>)> = HashMap::new();
    let mut unknown_room_position = 'A'..='D';
    let mut room_ownership = HashMap::new();
    for y in 0..playboard.len() {
        for x in 0..playboard[y].len() {
            match playboard[y][x] {
                '.' if playboard[y + 1][x] == '#' => {
                    wait_spots.insert(x, None);
                }
                amphipod if ('A'..='D').contains(&amphipod) => {
                    let room_owner = *room_ownership
                        .entry(x)
                        .or_insert_with(|| unknown_room_position.next().unwrap());
                    rooms
                        .entry(room_owner)
                        .and_modify(|(_, occupants)| occupants.push(amphipod))
                        .or_insert((x, vec![amphipod]));
                }
                _ => (),
            }
        }
    }
    for (_, (_, occupants)) in rooms.iter_mut() {
        occupants.reverse();
    }
    let room_depth = rooms[&'A'].1.len();

    println!(
        "Part 2: {}",
        find_cheapest_solution(&wait_spots, &rooms, room_depth)
            .unwrap()
            .0
    );
}

fn find_cheapest_solution(
    wait_spots: &HashMap<usize, Option<char>>,
    rooms: &HashMap<char, (usize, Vec<char>)>,
    room_depth: usize,
) -> Option<(usize, Vec<PossibleMove>)> {
    lazy_static! {
        static ref MOVE_COSTS: HashMap<char, usize> =
            [('A', 1), ('B', 10), ('C', 100), ('D', 1000)]
                .into_iter()
                .collect::<HashMap<_, _>>();
    }

    let possible_moves = enumerate_possible_moves(&wait_spots, &rooms, room_depth);
    if possible_moves.is_empty() && wait_spots.values().all(Option::is_none) {
        return Some((0, vec![]));
    }

    let mut cheapest_solution = None;
    for possible_move in possible_moves {
        let mut cost;
        let mut new_wait_spots = wait_spots.clone();
        let mut new_rooms = rooms.clone();
        match possible_move {
            RoomToRoom {
                amphipod,
                room,
                steps,
            } => {
                new_rooms.entry(room).and_modify(|(_, occupants)| {
                    occupants.pop();
                });
                new_rooms.entry(amphipod).and_modify(|(_, occupants)| {
                    occupants.push(amphipod);
                });
                cost = steps * MOVE_COSTS[&amphipod];
            }
            CorridorToRoom {
                amphipod,
                wait_spot,
                steps,
            } => {
                new_wait_spots.insert(wait_spot, None);
                new_rooms.entry(amphipod).and_modify(|(_, occupants)| {
                    occupants.push(amphipod);
                });
                cost = steps * MOVE_COSTS[&amphipod];
            }
            RoomToCorridor {
                amphipod,
                room,
                wait_spot,
                steps,
            } => {
                new_wait_spots.insert(wait_spot, Some(amphipod));
                new_rooms.entry(room).and_modify(|(_, occupants)| {
                    occupants.pop();
                });
                cost = steps * MOVE_COSTS[&amphipod];
            }
        }

        if let Some((cheapest_subsolution, mut moves)) =
            find_cheapest_solution(&new_wait_spots, &new_rooms, room_depth)
        {
            cost += cheapest_subsolution;
            moves.push(possible_move);
            match possible_move {
                RoomToCorridor { .. } => (),
                _ => return Some((cost, moves)),
            }
            match cheapest_solution {
                Some((lowest_cost, _)) if lowest_cost < cost => continue,
                _ => cheapest_solution = Some((cost, moves)),
            }
        }
    }
    cheapest_solution
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PossibleMove {
    RoomToRoom {
        amphipod: char,
        room: char,
        steps: usize,
    },
    CorridorToRoom {
        amphipod: char,
        wait_spot: usize,
        steps: usize,
    },
    RoomToCorridor {
        amphipod: char,
        room: char,
        wait_spot: usize,
        steps: usize,
    },
}

fn enumerate_possible_moves(
    wait_spots: &HashMap<usize, Option<char>>,
    rooms: &HashMap<char, (usize, Vec<char>)>,
    room_depth: usize,
) -> Vec<PossibleMove> {
    let leftmost_wait_spot = *wait_spots.keys().min().unwrap();
    let rightmost_wait_spot = *wait_spots.keys().max().unwrap();
    let mut possible_moves = vec![];
    for (&room, (position, occupants)) in rooms.iter() {
        if !occupants.is_empty() && !occupants.iter().all(|occupant| *occupant == room) {
            let amphipod = *occupants.last().unwrap();
            let (destination_position, destination_occupation) = &rooms[&amphipod];
            for x in (leftmost_wait_spot..*position).rev() {
                match wait_spots.get(&x) {
                    Some(Some(_)) => break,
                    Some(None) => possible_moves.push(RoomToCorridor {
                        amphipod,
                        room,
                        wait_spot: x,
                        steps: position - x + room_depth + 1 - occupants.len(),
                    }),
                    None => (),
                }
                if x == *destination_position
                    && (destination_occupation.is_empty()
                        || destination_occupation == &vec![amphipod])
                {
                    possible_moves.push(RoomToRoom {
                        amphipod,
                        room,
                        steps: position - x + room_depth + 1 - occupants.len() + room_depth
                            - destination_occupation.len(),
                    })
                }
            }
            for x in *position..=rightmost_wait_spot {
                match wait_spots.get(&x) {
                    Some(Some(_)) => break,
                    Some(None) => possible_moves.push(RoomToCorridor {
                        amphipod,
                        room,
                        wait_spot: x,
                        steps: x - position + room_depth + 1 - occupants.len(),
                    }),
                    None => (),
                }
                if x == *destination_position
                    && (destination_occupation.is_empty()
                        || destination_occupation == &vec![amphipod])
                {
                    possible_moves.push(RoomToRoom {
                        amphipod,
                        room,
                        steps: x - position + room_depth + 1 - occupants.len() + room_depth
                            - destination_occupation.len(),
                    })
                }
            }
        } else {
            for x in (leftmost_wait_spot..*position).rev() {
                match wait_spots.get(&x) {
                    Some(Some(amphipod)) => {
                        if *amphipod == room {
                            possible_moves.push(CorridorToRoom {
                                amphipod: *amphipod,
                                wait_spot: x,
                                steps: position - x + room_depth - occupants.len(),
                            })
                        }
                        break;
                    }
                    _ => (),
                }
            }
            for x in *position..=rightmost_wait_spot {
                match wait_spots.get(&x) {
                    Some(Some(amphipod)) => {
                        if *amphipod == room {
                            possible_moves.push(CorridorToRoom {
                                amphipod: *amphipod,
                                wait_spot: x,
                                steps: x - position + room_depth - occupants.len(),
                            })
                        }
                        break;
                    }
                    _ => (),
                }
            }
        }
    }
    possible_moves.sort();
    possible_moves
}
