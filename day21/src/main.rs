use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::successors;

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
        static ref PLAYER_REGEX: Regex =
            Regex::new(r"Player \d+ starting position: (?P<start_pos>\d+)").unwrap();
    }

    let positions: Vec<u32> = PLAYER_REGEX
        .captures_iter(input)
        .map(|player_capture| (player_capture["start_pos"].parse::<u32>().unwrap()))
        .collect();
    let step_lengths = [vec![6u32, 4, 2, 0, 8], vec![5u32, 3, 1, 9, 7]];
    let score_gains = [
        successors(Some((0, positions[0])), |(i, score)| {
            Some(((i + 1) % 5, (score + step_lengths[0][*i] - 1) % 10 + 1))
        })
        .map(|(_, score)| score)
        .skip(1)
        .take(10)
        .collect::<Vec<u32>>(),
        successors(Some((0, positions[1])), |(i, score)| {
            Some(((i + 1) % 5, (score + step_lengths[1][*i] - 1) % 10 + 1))
        })
        .map(|(_, score)| score)
        .skip(1)
        .take(10)
        .collect::<Vec<u32>>(),
    ];
    let winning_player = if score_gains[0].iter().sum::<u32>() > score_gains[1].iter().sum::<u32>()
    {
        0
    } else {
        1
    };
    let whole_10_turns_before_win = 999 / score_gains[winning_player].iter().sum::<u32>();
    let mut rolls_before_win = whole_10_turns_before_win * 60;
    let mut player_score = score_gains
        .iter()
        .map(|score_per_10_turns| {
            whole_10_turns_before_win * score_per_10_turns.iter().sum::<u32>()
        })
        .collect::<Vec<_>>();
    'final10: for step in 0..10 {
        for player in 0..2 {
            player_score[player] += score_gains[player][step];
            rolls_before_win += 3;
            if player_score[player] >= 1000 {
                break 'final10;
            }
        }
    }

    println!(
        "Part 1: {}",
        rolls_before_win * player_score[1 - winning_player]
    );
}

fn part2(input: &str) {
    lazy_static! {
        static ref PLAYER_REGEX: Regex =
            Regex::new(r"Player \d+ starting position: (?P<start_pos>\d+)").unwrap();
    }

    let start_positions: Vec<u32> = PLAYER_REGEX
        .captures_iter(input)
        .map(|player_capture| (player_capture["start_pos"].parse::<u32>().unwrap()))
        .collect();
    let step_weights = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let mut score_weights_after_throws = [
        vec![HashMap::<(u32, u32), u64>::new()],
        vec![HashMap::<(u32, u32), u64>::new()],
    ];
    score_weights_after_throws[0][0].insert((start_positions[0], 0), 1);
    score_weights_after_throws[1][0].insert((start_positions[1], 0), 1);

    let mut finished_after_throw = [
        HashMap::<usize, (u64, u64)>::new(),
        HashMap::<usize, (u64, u64)>::new(),
    ];

    for throw_num in 1.. {
        for player in 0..2 {
            let mut score_weights = HashMap::new();
            for ((position, score), num_universes) in
                score_weights_after_throws[player][throw_num - 1].iter()
            {
                for (step, weight) in step_weights {
                    let new_position = (position + step - 1) % 10 + 1;
                    let new_score = score + new_position;
                    if new_score < 21 {
                        *score_weights.entry((new_position, new_score)).or_insert(0) +=
                            num_universes * weight;
                        finished_after_throw[player]
                            .entry(throw_num)
                            .or_insert((0, 0))
                            .1 += num_universes * weight;
                    } else {
                        finished_after_throw[player]
                            .entry(throw_num)
                            .or_insert((0, 0))
                            .0 += num_universes * weight;
                    }
                }
            }
            score_weights_after_throws[player].push(score_weights);
        }

        if finished_after_throw[0][&throw_num].1 == 0 && finished_after_throw[1][&throw_num].1 == 0
        {
            break;
        }
    }

    let mut winning_universes = [0, 0];
    let mut num_universes_still_ongoing = 1;
    'universe_counting: for throw_num in 1.. {
        for player in 0..2 {
            let (finished, ongoing) = finished_after_throw[player][&throw_num];
            winning_universes[player] += num_universes_still_ongoing * finished;
            num_universes_still_ongoing = ongoing;
            if num_universes_still_ongoing == 0 {
                break 'universe_counting;
            }
        }
    }

    println!(
        "Part 2: {}",
        u64::max(winning_universes[0], winning_universes[1])
    );
}
