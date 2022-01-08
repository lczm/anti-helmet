//!
//! Anti Helmet
//! Advent of Code
//! Day 21: Reactor Reboot
//!

use regex::Regex;
use std::fmt;
use std::io::{stdin, BufRead};

mod geometry;
use geometry::*;

/// Defines a reactor that reboots the points in the given cuboid to associated on / off state.
#[derive(Copy, Clone, Debug)]
struct Step {
    cuboid: Cuboid,
    state: bool,
}
impl fmt::Display for Step {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}=>{}", self.cuboid, self.state)
    }
}

fn main() {
    // parse reboot steps from stdin.
    let bound_re = r"-?\d+..-?\d+";
    let step_re = Regex::new(&format!(
        r"(?P<state>on|off) x=(?P<x_bound>{re}),y=(?P<y_bound>{re}),z=(?P<z_bound>{re})",
        re = bound_re
    ))
    .unwrap();

    let steps: Vec<_> = stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| {
            let captures = step_re
                .captures(&line)
                .expect("Unable to parse reactor reboot step.");

            let parse_bound = |bound_str: &str| {
                let (left, right) = bound_str
                    .split_once("..")
                    .map(|(left, right)| {
                        (
                            left.parse::<i64>()
                                .expect("Failed to parse left bound as integer"),
                            right
                                .parse::<i64>()
                                .expect("Failed to parse right bound as integer"),
                        )
                    })
                    .expect("Expected bounds in the format X..Y");
                Bound {
                    begin: left,
                    end: right,
                }
            };

            Step {
                cuboid: Cuboid {
                    x_bound: parse_bound(&captures["x_bound"]),
                    y_bound: parse_bound(&captures["y_bound"]),
                    z_bound: parse_bound(&captures["z_bound"]),
                },
                state: if &captures["state"] == "on" {
                    true
                } else {
                    false
                },
            }
        })
        .collect();

    // compile steps to remove intersections from reboot steps
    // this allows to perform arithmetic on the steps independently.
    let compiled_steps =
        steps[1..]
            .iter()
            .fold(vec![steps[0]], |mut steps: Vec<Step>, &current| {
                // find the correction steps required to account for intersections
                let corrections: Vec<_> = steps.iter().map(|prior| {
                    let intersect = prior.cuboid.intersect(&current.cuboid);
                    // create a correction step to counter over/undercounting due to intersections
                    match (prior.state, current.state) {
                        (true, true) => intersect.map(|intersect| Step {
                            cuboid: intersect,
                            state: false,
                        }),
                        (false, false) => intersect.map(|intersect| Step {
                            cuboid: intersect,
                            state: true,
                        }),
                        (true, false) => intersect.map(|intersect| Step {
                            cuboid: intersect,
                            state: false,
                        }),
                        (false, true) => intersect.map(|intersect| Step {
                            cuboid: intersect,
                            state: true,
                        }),
                    }
                }).flatten()
                    .collect();

                if current.state == true {
                    steps.push(current);
                }
                steps.extend(corrections);
                steps
            });

    // calculate the no. of cubes turned out by compiled reboot steps.
    let n_on: usize = compiled_steps
        .iter()
        .filter(|step| step.state == true)
        .map(|step| step.cuboid.len())
        .sum();
    let n_off: usize = compiled_steps
        .iter()
        .filter(|step| step.state == false)
        .map(|step| step.cuboid.len())
        .sum();
    let n_effective_on = n_on - n_off;
    println!("No. of cubes on: {}", n_effective_on);
}
