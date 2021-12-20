//!
//! Anti Helmet
//! Advent of Code
//! Day 14: Extended Polymerization
//!

use regex::Regex;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::{stdin, BufRead};

/// Convert the given slice of two elements into a 2-element pair tuple
/// Returns the pair tuple derived from the given slice.
fn to_pair<T: Copy>(elements: &[T]) -> (T, T) {
    if elements.len() != 2 {
        panic!("Expected to be given a slice of exactly 2 elements");
    }

    let mut iter = elements.iter();
    (*iter.next().unwrap(), *iter.next().unwrap())
}

/// Count the elements produces by the given iterator
fn count<T: Hash + Eq>(elements: impl Iterator<Item = T>) -> HashMap<T, i64> {
    let mut counts = HashMap::new();
    for element in elements {
        let cnt = counts.entry(element).or_insert(0);
        *cnt += 1;
    }
    counts
}

fn main() {
    // read input lines from stdiin
    let lines: Vec<_> = stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read polymer sequence & transforms from stdin"))
        .collect();
    if lines.len() <= 0 {
        panic!("Unable to read polymer sequence & transforms from empty input");
    }

    // parse polymer transforms from read lines i the format XY -> Z.
    // represent transforms as a map from (X, Y) to Z
    let transform_re = Regex::new("(?P<pair>[A-Z][A-Z]) -> (?P<insert>[A-Z])").unwrap();
    let transforms: HashMap<_, _> = lines[2..]
        .iter()
        .map(|line| {
            let caps = transform_re
                .captures(line)
                .expect("Expected polymer transforms in the format: XZ -> Y");

            (
                to_pair(&caps["pair"].chars().collect::<Vec<_>>()),
                caps["insert"].chars().next().unwrap(),
            )
        })
        .collect();

    // read inital polymer sequence & calculate element pair counts in inital polymer sequence
    let inital_seq: Vec<_> = lines[0].chars().collect();
    let mut pair_counts = count(inital_seq.windows(2).map(to_pair));

    // apply polymer transforms over polymer sequence
    for step in 1..=40 {
        // compute changes to perform the polymer based on the transforms
        let changeset: Vec<_> = pair_counts
            .iter()
            .flat_map(|(&pair, &count)| {
                transforms.get(&pair).map(|&insert| {
                    // form new pairs from inserting character in between pair AB,
                    // inserting character C will replace pair AB with new pairs AC, CB
                    let (left, right) = pair;
                    let count = count as i64;

                    vec![
                        (pair, -count),
                        ((left, insert), count),
                        ((insert, right), count),
                    ]
                })
            })
            .flatten()
            .collect();

        // apply changeset to pair counts
        for (pair, offset) in changeset.into_iter() {
            let count = pair_counts.entry(pair).or_insert(0);
            *count += offset;
        }
        println!("step: {}", step);
    }

    // calculate element counts in sequence
    let mut element_counts = HashMap::new();
    for ((left, _), count) in pair_counts.into_iter().filter(|(_, c)| *c > 1) {
        let left_count = element_counts.entry(left).or_insert(0);
        *left_count += count;
    }
    // last element is not accounted for as we only cunted then left side of pair
    let last_count = element_counts
        .entry(inital_seq[inital_seq.len() - 1])
        .or_insert(0);
    *last_count += 1;

    let (max_count, min_count) = element_counts
        .values()
        .max()
        .zip(element_counts.values().min())
        .unwrap();
    println!("Max-min quantity difference: {}", max_count - min_count);
}
