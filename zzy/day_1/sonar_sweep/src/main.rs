//!
//! Anti Helmet
//! Advent of Code
//! Day 1: Sonar Sweep
//!

use std::io::stdin;

fn main() {
    // Parses all measurements from stdin until encountering an EOF.
    let mut measures: Vec<u16> = Vec::new();
    let mut line = String::new();
    while let Ok(len_read) = stdin().read_line(&mut line) {
        if len_read == 0 {
            // reached EOF: end of input
            break;
        }

        // parse measure from line
        let measure = line
            .trim_end()
            .parse()
            .expect("Failed to parse measurement from line");
        measures.push(measure);

        // clear string read buffer
        line = String::new();
    }

    // compute measurement moving sums consisting of 3 element windows
    let moving_sums: Vec<u16> = measures
        .windows(3)
        .map(|triple| triple.iter().sum())
        .collect();

    // compute no. of times current measure increases compared to previous measurement
    let n_increasing: u16 = moving_sums
        .windows(2)
        .map(|sums| match &sums {
            &[prev, current] if current > prev => 1,
            _ => 0,
        })
        .sum();

    println!(
        "There are {} measurements that are larger than the previous measurement.",
        n_increasing
    );
}
