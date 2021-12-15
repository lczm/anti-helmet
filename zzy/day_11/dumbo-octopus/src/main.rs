//!
//! Anti Helmet
//! Advent of Code
//! Day 11: Dumbo Octopus
//!

use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};
use std::io::{stdin, BufRead};

/// Represents a point on a 2D plane.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point(usize, usize);

/// Represents the energy states of all octopuses at a point in time.
struct OctopusState {
    /// Energy level of each octopus indexed (y, x)
    energy_levels: Vec<Vec<u8>>,
}
impl OctopusState {
    fn new(energy_levels: Vec<Vec<u8>>) -> OctopusState {
        if energy_levels.len() <= 0 {
            panic!("Expected to be given 2D vector of energy levels, given only 1D.")
        }
        OctopusState { energy_levels }
    }

    /// Retrieves the energy level of the octopus at the given point
    fn at(&self, Point(x, y): Point) -> u8 {
        self.energy_levels[y][x]
    }

    /// Length of the y axis of the stored energy levels
    fn y_len(&self) -> usize {
        self.energy_levels.len()
    }

    /// Length of the x axis of the stored energy levels
    fn x_len(&self) -> usize {
        self.energy_levels[0].len()
    }

    /// Derive the defined points surrounding the given points
    /// Surrounding points are 1 euclidean distance away from the given point.
    /// Defined points are within the bounds of the dimensions of the stored energy levels.
    /// Returns a vector of defined, surrounding points relative to given point.
    fn surrounding(&self, Point(x, y): Point) -> Vec<Point> {
        // generate offsets for surrounding point
        let offset: [isize; 3] = [-1, 0, 1];
        offset
            .iter()
            .flat_map(|&x_offset| {
                offset
                    .iter()
                    .flat_map(|&y_offset| {
                        let (new_x, x_overflow) = isize::overflowing_add(x as isize, x_offset);
                        let (new_y, y_overflow) = isize::overflowing_add(y as isize, y_offset);

                        // check generated points are within bounds
                        if x_overflow || y_overflow {
                            None
                        } else {
                            let (new_x, new_y) = (new_x as usize, new_y as usize);

                            if new_x >= self.x_len() || new_y >= self.y_len() {
                                None
                            }
                            // identical to given point, exclude.
                            else if new_x == x && new_y == y {
                                None
                            } else {
                                Some(Point(new_x, new_y))
                            }
                        }
                    })
                    .collect::<Vec<Point>>()
            })
            .collect()
    }

    /// Compiles the points of the octopuses that are elligible for flashing
    fn flashing_pts(&self) -> HashSet<Point> {
        (0..self.y_len())
            .flat_map(|y| (0..self.x_len()).map(move |x| Point(x, y)))
            .filter(|&pt| self.at(pt) > 9)
            .collect()
    }

    /// Simulate a single octopuses' flashing step to derive the next state
    /// Returns the next octopuses state and the no. of flashes that occured.
    fn next(mut self) -> (OctopusState, u32) {
        // before resolving any flashes increament energy levels by 1
        for y in 0..self.y_len() {
            for x in 0..self.x_len() {
                self.energy_levels[y][x] += 1;
            }
        }

        // resolve all flashing octopuses,
        let mut flashed_pts = HashSet::new();
        loop {
            let flashing_pts: HashSet<_> = self
                .flashing_pts()
                .difference(&flashed_pts)
                .map(|&pt| pt)
                .collect();
            if flashing_pts.len() == 0 {
                break;
            }

            // propagating energy increases to surrounding octopuses
            for &flashed_pt in &flashing_pts {
                for pt in self.surrounding(flashed_pt) {
                    if !flashed_pts.contains(&pt) {
                        let Point(x, y) = pt;
                        self.energy_levels[y][x] += 1;
                    }
                }
            }

            // mark flashing octopuses as flashed.
            flashed_pts = flashed_pts.union(&flashing_pts).map(|&pt| pt).collect();
        }

        // reset the flashed octopuses' stored energy levels
        for &Point(x, y) in &flashed_pts {
            self.energy_levels[y][x] = 0;
        }

        (self, flashed_pts.len() as u32)
    }

    /// Check if all octopuses are synchronized in their energy levels
    fn sync(&self) -> bool {
        if self.x_len() * self.y_len() <= 1 {
            // no elements to compare to, in sync by default.
            true
        } else {
            let target = self.energy_levels[0][0];
            self.energy_levels
                .iter()
                .flat_map(|col| col.iter())
                .all(|&level| level == target)
        }
    }
}
impl Display for OctopusState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // dump contents of octopus energy levels
        let lines: Vec<String> = self
            .energy_levels
            .iter()
            .map(|col| col.iter().map(|c| format!("{}", c)).collect::<String>())
            .collect();

        write!(f, "{}", lines.as_slice().join("\n"))
    }
}

fn main() {
    // read initial state of octopus energy levels from stdin
    let initial_state = OctopusState::new(
        stdin()
            .lock()
            .lines()
            .map(|line| {
                line.expect("Failed to read energy levels from stdin")
                    .chars()
                    .map(|c| {
                        c.to_digit(10)
                            .expect("Failed to parse energy levels as digit")
                            as u8
                    })
                    .collect()
            })
            .collect(),
    );

    // simulate octopus flashing until octopuses' energy levels will be in sync
    let mut state = initial_state;
    let mut n_steps = 0;
    while !state.sync() {
        let (next_state, _) = state.next();
        state = next_state;
        n_steps += 1;
    }

    println!("Octopuses will be sync on step {}", n_steps);
}
