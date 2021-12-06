//!
//! Anti Helmet
//! Advent of Code
//! Day 6: Lanternfish
//!

use std::collections::HashMap;
use std::io::{stdin, Read};

/// Defines the delay in days  before a a lanternfish can reproduces
const REPRODUCE_DELAY_DAYS: u8 = 6;

/// Models a lanternfish that reproduces on a 7 day cycle.
#[derive(PartialEq, Eq, Hash)]
enum Fish {
    /// Incubating state tracks no. of days left before the fish reproduces.
    Incubating(u8),
    Reproducing,
}

impl Fish {
    /// Advance the lanternfish model by one day.
    /// Returns the next iteration of the current fish, and the new fish spawned,
    /// if any together as a vector.
    fn simulate(&self) -> Vec<Fish> {
        use Fish::*;
        match self {
            Incubating(n_days) if *n_days <= 1 => vec![Reproducing],
            Incubating(n_days) => vec![Incubating(n_days - 1)],
            Reproducing => vec![
                // reproduced: reset incubation cycle counter
                Incubating(REPRODUCE_DELAY_DAYS),
                // new fish: spend 2 more days incubating before reproducing.
                Incubating(REPRODUCE_DELAY_DAYS + 2),
            ],
        }
    }
}

/// Count the no. of fishes in each unique fish state.
/// Returns a hashmap with Fish state as key and value as count.
fn count_fishes<T: Iterator<Item = Fish>>(fishes: T) -> HashMap<Fish, usize> {
    let mut fish_counts = HashMap::new();

    for fish in fishes {
        let count = fish_counts.entry(fish).or_insert(0);
        *count += 1
    }
    fish_counts
}

fn main() {
    // read fish states from stdin
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read fish states form std in");
    let fishes: Vec<_> = input
        .trim_end()
        .split(",")
        .map(|s| s.parse::<u8>().expect("Failed to parse as unsigned int"))
        .map(|n_days| Fish::Incubating(n_days))
        .collect();

    // simulate 256 days of lanternfish reproduction
    let mut fish_counts = count_fishes(fishes.into_iter());
    for _ in 1..=256 {
        // compute counts grouped by kind of fish for current day
        let mut new_counts = HashMap::new();
        for (fish_kind, n_fishes) in fish_counts {
            for new_fish_kind in fish_kind.simulate() {
                // update count for new fish types in counts map
                let count = new_counts.entry(new_fish_kind).or_default();
                *count += n_fishes;
            }
        }
        fish_counts = new_counts;
    }

    // compute total number of fishes
    println!(
        "no. of lanternfish: {}",
        fish_counts.values().map(|&v| v).sum::<usize>()
    );
}
