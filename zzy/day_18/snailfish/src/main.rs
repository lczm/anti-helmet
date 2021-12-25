//!
//! Anti Helmet
//! Advent of Code
//! Day 18: Snailfish
//!

use std::ops::Add;
use std::io::{BufRead, stdin};

#[cfg(test)]
mod tests;

/// Wraps the result of an explosion reduction operation on a Snailfish number.
struct ExplosionResult {
    number: Number,
    is_reduced: bool,
    add_left: u8,
    add_right: u8,
}

/// Defines a Snailfish number
#[derive(Clone, Debug, PartialEq, Eq)]
enum Number {
    Regular(u8),
    Pair(Box<Number>, Box<Number>),
}
impl Number {
    /// Parse the given string expression as an Snailfish number.
    fn parse(str_expr: &str) -> Self {
        use Number::*;
        // try parsing string as regular no.
        match str_expr.trim().parse::<u8>() {
            Ok(val) => Regular(val),

            // assume that all strings not parsable as regular numbers are to be
            // parsed as snailfish no. pairs.
            Err(_) => {
                // track char positions of pair starting '[' and left / right split ','
                // delimiters in stacks
                let (mut start_pos, mut split_pos) = (Vec::new(), Vec::new());
                let mut pair_num = None;
                for (i, c) in str_expr.chars().enumerate() {
                    match c {
                        '[' => start_pos.push(i),
                        ',' => split_pos.push(i),
                        ']' => {
                            // found start, splitter and ending indexes of the pair.
                            let format_err = "Expected Snailfish pair to be the format [x,y]";
                            let (start_i, split_i, end_i) = (
                                start_pos.pop().expect(format_err),
                                split_pos.pop().expect(format_err),
                                i,
                            );

                            // extract left and right subexpression strings
                            // +1 added to exclude delimiters from subexpression strings
                            let left_str = &str_expr[start_i + 1..split_i];
                            let right_str = &str_expr[split_i + 1..end_i];

                            // recursively parse left and right subexpression strs
                            // and construct snailfish no. pair
                            pair_num = Some(Pair(
                                Box::new(Number::parse(left_str)),
                                Box::new(Number::parse(right_str)),
                            ));
                        }
                        _ => continue,
                    }
                }
                pair_num.expect("Failed to parse Snailfish pair from string.")
            }
        }
    }

    /// Add the given amount to the left most regular number in this Snailfish number.
    fn add_left(self, amount: u8) -> Self {
        use Number::*;
        match self {
            Regular(val) => Regular(val + amount),
            Pair(left, right) => Pair(Box::new(left.add_left(amount)), right),
        }
    }

    /// Add the given amount to the right most regular number in this Snailfish number.
    fn add_right(self, amount: u8) -> Self {
        use Number::*;
        match self {
            Regular(val) => Regular(val + amount),
            Pair(left, right) => Pair(left, Box::new(right.add_right(amount))),
        }
    }

    /// Performs an explosion reduction on the this Snailfish number.
    /// Call explode with a depth of 0.
    /// Explodes a single Snailfish no. pair with a depth 4 or deeper.
    /// Returns the explosion reduction result.
    fn explode(self, depth: u32) -> ExplosionResult {
        use Number::*;

        match self {
            Pair(left, right) => match (*left, *right) {
                (Regular(left), Regular(right)) if depth >= 4 => {
                    // pair satisfies criteria for explosion: replace pair with
                    // a regular 0, return left & right values for adding to
                    // regular numbers in parent snailfish numbers.
                    ExplosionResult {
                        number: Regular(0),
                        is_reduced: true,
                        add_left: left,
                        add_right: right,
                    }
                }
                (Regular(left), right) => {
                    // found a regular no. as the left subexpression of the pair.
                    // explode the right subexpression add the left expoded value to the regular no.
                    let ExplosionResult {
                        number,
                        is_reduced,
                        add_left,
                        add_right,
                    } = right.explode(depth + 1);
                    ExplosionResult {
                        number: Pair(Box::new(Regular(left + add_left)), Box::new(number)),
                        is_reduced: is_reduced,
                        // carry on the add_right value to the previous recursion
                        // as it has yet to be added the a right regular no.
                        add_left: 0,
                        add_right: add_right,
                    }
                }
                (left, Regular(right)) => {
                    // found a regular no. as the right subexpression of the pair.
                    // explode the right subexpression add the right expoded value to the regular no.
                    let ExplosionResult {
                        number,
                        is_reduced,
                        add_left,
                        add_right,
                    } = left.explode(depth + 1);
                    ExplosionResult {
                        number: Pair(Box::new(number), Box::new(Regular(right + add_right))),
                        is_reduced: is_reduced,
                        // carry on the add_left value to the previous recursion
                        // as it has yet to be added the a left regular no.
                        add_left: add_left,
                        add_right: 0,
                    }
                }
                (left @ Pair(_, _), right @ Pair(_, _)) => {
                    // recursively explode left and right nested subexpression of pair.
                    let left_result = left.explode(depth + 1);
                    // shortcircuit exploding the right subexpr if left expr has already exploded.
                    let right_result = if !left_result.is_reduced {
                        right.explode(depth + 1)
                    } else {
                        ExplosionResult {
                            number: right,
                            is_reduced: false,
                            add_left: 0,
                            add_right: 0,
                        }
                    };

                    // add carry values into exploded subexpression.
                    let right_num = right_result.number.add_left(left_result.add_right);
                    let left_num = left_result.number.add_right(right_result.add_left);

                    ExplosionResult {
                        number: Pair(Box::new(left_num), Box::new(right_num)),
                        is_reduced: left_result.is_reduced || right_result.is_reduced,
                        add_left: left_result.add_left,
                        add_right: right_result.add_right,
                    }
                }
            },
            Regular(_) => ExplosionResult {
                number: self,
                is_reduced: false,
                add_left: 0,
                add_right: 0,
            },
        }
    }

    /// Perform a split reduction operation on this snailfish number.
    /// Returns the reduced snailfish number a boolean signifying if a
    /// split reduction actually occurred.
    fn split(self) -> (Self, bool) {
        use Number::*;
        match self {
            Regular(val) if val >= 10 => (
                // split the value into:
                // on the left: original / 2 rounded down.
                // on the left: original / 2 rounded up.
                Pair(
                    Box::new(Regular(val / 2)),
                    Box::new(Regular(val / 2 + if val % 2 == 0 { 0 } else { 1 })),
                ),
                true,
            ),
            Pair(left, right) => {
                // split the left / right subexpression of the pair.
                let (left_split, is_reduced) = left.split();
                // shortcircuit splitting the right subexpr if the left subexpr has been split.
                let (right_split, is_reduced) = if !is_reduced {
                    right.split()
                } else {
                    (*right, true)
                };

                (
                    Pair(Box::new(left_split), Box::new(right_split)),
                    is_reduced,
                )
            }
            _ => (self, false),
        }
    }

    /// Reduces this Snailfish number by performing explosions and splits reduction
    /// until no longer applicable.
    fn reduce(self) -> Self {
        let (mut can_explode, mut can_split) = (true, true);
        let mut num = self;

        while can_explode || can_split {
            // perform explosion reduction
            let result = num.explode(0);
            num = result.number;
            can_explode = result.is_reduced;
            if result.is_reduced {
                // perform only one reduction per loop.
                continue;
            } 

            // perform split reduction
            let (split_num, is_reduced) = num.split();
            num = split_num;
            can_split = is_reduced;
        }

        num
    }
    
    /// Compute & return the magnitude of this Snailfish number
    fn magnitude(&self) -> u32 {
        use Number::*;
        match self {
            Regular(val) => *val as u32,
            Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}
impl Add for Number {
    type Output = Self;

    /// Adds two Snailfish numbers togther and reuturns the reduced result.
    fn add(self, other: Self) -> Self {
        Number::Pair(
            Box::new(self),
            Box::new(other)
        ).reduce()
    }
}

fn main() {
    // read snailfish number assignment from stdin
    let nums: Vec<_> = stdin().lock().lines()
        .map(|line| line.expect("Failed to read Snailfish number lines from stdin"))
        .map(|line| Number::parse(&line))
        .collect();

    // find max magnitude achievable when adding any two snailfish numbers
    let mut max_magnitude = 0;
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            max_magnitude = u32::max(
                max_magnitude, (nums[i].clone() + nums[j].clone()).magnitude()
            );
            // since snailfish sum is not associative, sum with args swapped.
            max_magnitude = u32::max(
                max_magnitude, (nums[j].clone() + nums[i].clone()).magnitude()
            );
        }
    }

    println!("Max magnitude of sum of any two numbers: {}", max_magnitude);
}
