//!
//! Anti Helmet
//! Advent of Code
//! Day 20: Trench Map
//!

use std::collections::HashMap;
use std::io::{stdin, BufRead};

/// Simulate a multiverse Dirac Dice Game. Each roll of the dice splits the universe
/// into 3, where the dice rolls 1, 2 or 3 respectively. Counts the no. of unvierses.
/// where each player wins.  Returns the no. of universe where player 1 or 2 wins.
fn simulate_die(
    positions: (u8, u8),
    scores: (u8, u8),
    player_turn: u8,
    dice_map: &HashMap<u8, u8>,
) -> (u64, u64) {
    assert!(player_turn == 1 || player_turn == 2);

    match scores {
        // player 1 wins
        (win_score, _) if win_score >= 21 => (1, 0),
        // player 2 wins
        (_, win_score) if win_score >= 21 => (0, 1),
        _ => {
            // simulate all rolls of the dice using precomputed dirac dice map

            // by listing out all possiblities of 3 dirac dice roll we can calculate
            // the min and max sum to be between 3 and 9. Use the precomputed
            // dice map to compute universe branching factor.
            (3..=9)
                .map(|n_moves| (n_moves, dice_map[&n_moves]))
                .map(|(n_moves, n_branches)| {
                    // update player position and scores for the new universes
                    let (mut player_1_score, mut player_2_score) = scores;
                    let (mut player_1_position, mut player_2_position) = positions;
                    if player_turn == 1 {
                        player_1_position = (player_1_position + n_moves) % 10;
                        player_1_score += player_1_position + 1;
                    } else {
                        player_2_position = (player_2_position + n_moves) % 10;
                        player_2_score += player_2_position + 1;
                    }

                    // recursively continue game simulation, swapping to the other player's turn.
                    let (player_1_wins, player_2_wins) = simulate_die(
                        (player_1_position, player_2_position),
                        (player_1_score, player_2_score),
                        if player_turn == 1 { 2 } else { 1 },
                        dice_map,
                    );
                    // multiply the current no. universes where each player wins by branch factor
                    (
                        n_branches as u64 * player_1_wins,
                        n_branches as u64 * player_2_wins,
                    )
                })
                .reduce(
                    |(sum_player_1, sum_player_2), (player_1_wins, player_2_wins)| {
                        (sum_player_1 + player_1_wins, sum_player_2 + player_2_wins)
                    },
                )
                .unwrap()
        }
    }
}

fn main() {
    // parse inital player positions from stdin
    let parse_err = "Failed to parse inital player position from input";
    let positions: Vec<_> = stdin()
        .lock()
        .lines()
        .map(|line| {
            line.expect(parse_err)
                .split(":")
                .last()
                .expect(parse_err)
                .trim()
                .parse::<u8>()
                // -1 as positions vec reprsents postions 1 to 10 with values starting from 0.
                .expect(parse_err)
                - 1
        })
        .collect();
    assert_eq!(positions.len(), 2);

    // precompute dirac dice to no of universe branches mapping for 3 rolls
    let mut dice_map = HashMap::new();
    for first_roll in 1..=3 {
        for second_roll in 1..=3 {
            for third_roll in 1..=3 {
                let roll_sum = first_roll + second_roll + third_roll;
                let n_universes = dice_map.entry(roll_sum).or_insert(0);
                *n_universes += 1;
            }
        }
    }

    // simulate dice game starting with player 1
    let (player_1_wins, player_2_wins) =
        simulate_die((positions[0], positions[1]), (0, 0), 1, &dice_map);
    println!("Max wins: {}", u64::max(player_1_wins, player_2_wins));
}
