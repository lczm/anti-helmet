use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
///!
///! Anti-Helmet
///! Advent of Code
///! Day 8: Seven Segment Search
///!
use std::io::{stdin, Read};
use std::iter::FromIterator;

type Pattern = HashSet<char, RandomState>;

/// Finds & Returns the segments in left that does not exists in the right segment
fn diff_seg(left: &Pattern, right: &Pattern) -> HashSet<char> {
    left.difference(right).map(|&c| c).collect()
}

/// Checks whether the given signal pattern contains all segments in find_segs
/// Returns true if all segments in find_segs are found in pattern, false otherwise.
fn contains_segs(pattern: &Pattern, find_segs: &Pattern) -> bool {
    diff_seg(find_segs, pattern).len() == 0
}

/// Attempt to infer the digit represented by the given signal pattern.
/// Uses a given infered map containing already infered digits to support inference.
/// Returns the infered digit or none if there is insufficient context to infer the digit.
fn infer(sig_pattern: &Pattern, infered: &HashMap<char, Pattern>) -> Option<char> {
    match sig_pattern.len() {
        // infer obvious digits identifiable by sig_pattern length
        2 => Some('1'),
        3 => Some('7'),
        4 => Some('4'),
        7 => Some('8'),
        6 => {
            // infer '6' / '9' / '4'  based on whether signal pattern contains
            // specific segments
            infered
                .get(&'1')
                .zip(infered.get(&'4'))
                .map(|(one, four)| match sig_pattern {
                    pat if contains_segs(pat, four) => '9',
                    pat if contains_segs(pat, one) => '0',
                    _ => '6',
                })
        }
        5 => {
            // compute 'e' segment segment by diffing '8' with '9'
            let e_seg = infered
                .get(&'8')
                .zip(infered.get(&'9'))
                .map(|(eight, nine)| diff_seg(eight, nine));

            e_seg.zip(infered.get(&'1')).map(|(e_seg, one)| {
                // infer '2' / '3' / '5' based on whether signal patterns
                // contain specific segments
                match sig_pattern {
                    pat if contains_segs(pat, one) => '3',
                    pat if contains_segs(pat, &e_seg) => '2',
                    _ => '5',
                }
            })
        }
        _ => panic!("Got an sig pattern with an unexpected no. of segments"),
    }
}

fn main() {
    // read signal pattern & output digits from stdin
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read signal pattern & output digits from STDIN");
    let input_cases = input.trim_end().split("\n").map(|line| {
        line.split_at(
            line.find("|")
                .expect("Expected signal pattern & output digits to be delimited by '|'"),
        )
    });

    // infer signal patterns and decode the output number in each test case
    let mut output_sum: u32 = 0;
    for (sig_patterns, output_digits) in input_cases {
        let sig_patterns = sig_patterns
            .split_whitespace()
            .map(|sig_pattern| HashSet::from_iter(sig_pattern.chars()));

        // infer digit represented by the signal pattern over 3 passes into inference map
        let mut infered: HashMap<char, Pattern> = HashMap::new();
        for _pass in 1..=3 {
            infered = sig_patterns
                .clone()
                .flat_map(|pattern| {
                    let digit = infer(&pattern, &infered);
                    digit.zip(Some(pattern))
                })
                .collect();
        }
        // reverse infered map to form mapping from signal pattern to digit
        let pattern_digit_map: Vec<_> = infered
            .into_iter()
            .map(|(digit, pattern)| (pattern, digit))
            .collect();

        // use pattern digit map to parse output number
        let output_number: u32 = output_digits
            .split_whitespace()
            // skip token as it will be the delimiter '|'
            .skip(1)
            .map(|digit_pattern| HashSet::from_iter(digit_pattern.chars()))
            .map(|digit_pattern| {
                // find digit that matches digit pattern
                pattern_digit_map
                    .iter()
                    .filter(|(pattern, _)| digit_pattern == *pattern)
                    .map(|(_, digit)| digit)
                    .next()
                    .expect("Encounted unknown signal pattern when parsing output digits")
            })
            .collect::<String>()
            .parse()
            .expect("Failed to parse output number as integer");
        output_sum += output_number;
    }

    println!("Sum of all output values: {}", output_sum);
}
