//!
//! Anti Helmet
//! Advent of Code
//! Day 13: Transparent Origami
//!

use regex::Regex;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

/// Defines a dot on the Origami Paper represented as 2D (x, y) points.
/// Where the x-axis advances the dot to the right and the y-axis advances the dot downwards.
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Dot {
    x: u16,
    y: u16,
}

/// Defines a fold instruction to be applied to the dots on the origami paper
enum Fold {
    X(u16),
    Y(u16),
}

/// Apply the fold instruction to the given origami dot.
/// Returns then dot after folding on origami paper.
fn fold(dot: Dot, instruction: &Fold) -> Dot {
    match *instruction {
        Fold::X(location) if dot.x <= location => dot,
        Fold::X(location) => Dot {
            // compute the negative offset: x coord - location
            x: location - (dot.x - location),
            ..dot
        },
        Fold::Y(location) if dot.y <= location => dot,
        Fold::Y(location) => Dot {
            // compute the negative offset: y coord - location
            y: location - (dot.y - location),
            ..dot
        },
    }
}

/// Render & and Display the given origami dots a 2D text space.
/// Returns the dots rendered as a text string.
fn display(dots: &[Dot]) -> String {
    if dots.len() < 1 {
        panic!("Expected to be given at least one dot to display");
    }
    // compute bounds of display
    let min_x = dots.iter()
        .map(|dot| dot.x)
        .min().unwrap();
    let max_x = dots.iter()
        .map(|dot| dot.x)
        .max().unwrap();
    let min_y = dots.iter()
        .map(|dot| dot.y)
        .min().unwrap();
    let max_y = dots.iter()
        .map(|dot| dot.y)
        .max().unwrap();
    
    // draw dots on display buffer.
    let row_len = (max_x-min_x+1) as usize;
    let n_rows = (max_y-min_y+1) as usize;
    let mut buffer = vec![vec!["."; row_len]; n_rows];
    for &Dot{x, y} in dots {
        buffer[(y-min_y) as usize ][(x-min_x) as usize] = "#"
    }

    let render: String = buffer.into_iter()
        .map(|row| format!("{}\n", row.as_slice().join("")))
        .collect();
    render
}


fn main() {
    // parse origami paper dots & folding instructions from stdin
    let lines: Vec<_> = stdin()
        .lock()
        .lines()
        .map(|rst| rst.expect("Failed to to read input lines from STDIN"))
        .collect();

    let input_segs: Vec<_> = lines.split(|line| line == "").collect();
    if input_segs.len() != 2 {
        panic!("Expected 2 input segments separated by a empty line");
    }

    let dots: HashSet<Dot> = input_segs[0]
        .into_iter()
        .map(|line| {
            let mut dot_splits = line.split(",");
            let (x, y) = dot_splits
                .next()
                .zip(dot_splits.next())
                .map(|(x, y)| {
                    (
                        x.parse().expect("Failed to parse integer"),
                        y.parse().expect("Failed to parse integer"),
                    )
                })
                .expect("Failed to parse origami dots: expected dots in X,Y format");
            Dot { x, y }
        })
        .collect();

    let instruction_regex = Regex::new(r"fold along (?P<axis>[xy])=(?P<value>\d+)").unwrap();
    let instructions: Vec<_> = input_segs[1]
        .into_iter()
        .map(|line| {
            let caps = instruction_regex
                .captures(line)
                .expect("Failed to parse folding instructions");
            let value = caps["value"].parse().expect("Failed to parse integer");
            match &caps["axis"] {
                "x" => Fold::X(value),
                "y" => Fold::Y(value),
                _ => panic!("Unsupported instruction axis: Expected 'y' or 'x'"),
            }
        })
        .collect();

    // apply folding instructions to dots
    let folded_dots: Vec<_> = dots
        .into_iter()
        .map(|inital_dot| {
            instructions
                .iter()
                .fold(inital_dot, fold)
        })
        // remove overlapping dots by finding unique folded dots.
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    
    print!("{}", display(folded_dots.as_slice()));
}
