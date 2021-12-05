//!
//! Anti Helmet
//! Advent of Code
//! Day 1: Dive!
//!

use std::error::Error;
use std::fmt;
use std::io::stdin;

#[derive(Debug)]
struct ShiftErr {
    direction: String,
}
impl fmt::Display for ShiftErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to shift position in direction: {}",
            self.direction
        )
    }
}
impl Error for ShiftErr {}

struct Position {
    aim: i32,
    depth: i32,
    horizontal: i32,
}
impl Position {
    // Shift to the new position specified by the given direction and magnitude
    // Returns the new position after applying the command
    fn shift(&self, direction: &str, magnitude: i32) -> Result<Self, ShiftErr> {
        match direction {
            "forward" => Ok(Position {
                horizontal: self.horizontal + magnitude,
                depth: self.depth + self.aim * magnitude,
                ..*self
            }),
            "down" => Ok(Position {
                aim: self.aim + magnitude,
                ..*self
            }),
            "up" => Ok(Position {
                aim: self.aim - magnitude,
                ..*self
            }),
            d => Err(ShiftErr {
                direction: d.to_owned(),
            }),
        }
    }
}

fn main() {
    // track current
    let mut pos = Position {
        aim: 0,
        depth: 0,
        horizontal: 0,
    };

    // read command from stdin line by line
    let mut cmd = String::new();
    while let Ok(read_len) = stdin().read_line(&mut cmd) {
        if read_len == 0 {
            break;
        }

        // parse direction & magnitude from command buffer
        let mut cmd_iter = cmd.trim().split(" ");
        let direction = cmd_iter.next();
        let magnitude = cmd_iter.next();
        if let (Some(direction), Some(magnitude)) = (direction, magnitude) {
            let magnitude: i32 = magnitude
                .parse()
                .expect("Failed to parse command magnitude");
            pos = pos
                .shift(direction, magnitude)
                .expect("Failed to shift position");
        } else {
            panic!("Failed to parse command: {}", cmd);
        }

        cmd = String::new();
    }

    println!("Answer: {}", pos.horizontal * pos.depth);
}
