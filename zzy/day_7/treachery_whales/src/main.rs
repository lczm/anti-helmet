///!
///! Anti-Helmet
///! Advent of Code
///! Day 7: The Treachery of Whales
///!

use std::io::{stdin, Read};

fn main() {
  // read crab submarine positions from stdin
  let mut input = String::new();
  stdin().read_to_string(&mut input).expect("Failed to read crab submarine positions.");
  let positions: Vec<_> = input.trim_end()
    .split(",")
    .map(|s| s.parse::<i32>().expect("Failed to parse crab submarine position as u32."))
    .collect();

  // find position search bonds by finding min/max crab positions
  let &min_pos = positions.iter().min().unwrap();
  let &max_pos = positions.iter().max().unwrap();

  // brute force and evalulate fuel consumption for each possible target position
  let min_fuel = (min_pos..=max_pos).map(|target_pos| {
    // compute fuel required to move the target position
    positions.iter()
      .map(|pos| {
        let steps = i32::abs(target_pos - pos);
        // taking n steps to position n costs fuel: 1 + 2 + ... + n
        // use the summation formula to compute fuel cost: (n*(n+1)) / 2
        steps * (steps + 1) / 2
      })
      .sum::<i32>()
  })
  .min()
  .unwrap();

  println!("Min fuel required: {}", min_fuel);
}