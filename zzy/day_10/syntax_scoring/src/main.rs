//!
//! Anti Helmet
//! Advent of Code
//! Day 10: Syntax Scoring
//!

use std::collections::VecDeque;
use std::io::{stdin, BufRead};

/// Defines the syntax errors that can occur while lintiing
enum SyntaxError {
    /// Corruption caused by finding an unexpected character
    Corruption(char),
    /// Incomplete syntax caused missing closing chunk characters
    Incomplete(Vec<char>),
}

/// Lint the given line for syntax errors
/// Returns true if the line passes linting, otherwise returns the first syntax
/// error that causes linting to fail.
fn lint(line: &str) -> Result<bool, SyntaxError> {
    // define closures mapping between opening and closing chunk chars
    let map_closing = |opening: char| match opening {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Unknown chunk character"),
    };

    // chunk stack tracking open chunks
    let mut open_chunks = VecDeque::new();
    for c in line.chars() {
        match c {
            // open new chunk by pushing to stack
            '(' | '[' | '{' | '<' => {
                open_chunks.push_back(map_closing(c));
            }
            // check chunk closing character is aligns with currently open chunks
            ')' | ']' | '}' | '>' => {
                match open_chunks.pop_back() {
                    // data corruption: unexpected character
                    Some(expected) if c != expected => return Err(SyntaxError::Corruption(c)),
                    _ => continue,
                }
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }

    // check for unclosed chunks
    if open_chunks.is_empty() {
        Ok(true)
    } else {
        // unwind stack of unclosed chunks to obtain expected characters
        Err(SyntaxError::Incomplete(
            open_chunks.into_iter().rev().collect(),
        ))
    }
}

fn main() {
    // read & lint lines from stdin
    let errors: Vec<_> = stdin()
        .lock()
        .lines()
        .map(|line| lint(&line.expect("Failed to read lines from stdin")))
        // filter out non incomplete errors
        .flat_map(|rst| match rst {
            Err(err @ SyntaxError::Incomplete(_)) => Some(err),
            _ => None,
        })
        .collect();

    // tabulate completion string scores
    let mut scores: Vec<u64> = errors
        .into_iter()
        .map(|err| {
            let expected_chars = match err {
                SyntaxError::Incomplete(expected) => expected,
                _ => panic!("Unexpected syntax error type"),
            };

            let score = expected_chars
                .into_iter()
                .map(|c| match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("Unexpected character scoring completion strings"),
                })
                .fold(0, |left, right| left * 5 + right);

            score
        })
        .collect();

    // find middle score
    scores.sort_unstable();
    let middle_score = scores[scores.len() / 2];

    println!("Total syntax error score: {}", middle_score);
}
