//!
//! Anti Helmet
//! Advent of Code
//! Day 5: Hydrothermal Venture
//!

use regex::Regex;
use std::collections::HashMap;
use std::io::{stdin, Read};

/// Represents a point / vector on 2D x, y plane
#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

/// Represents a line segments on 2D plane
struct Line {
    begin: Point,
    end: Point,
}
impl Line {
    fn iter(&self) -> PointIterator {
        let delta = Point {
            x: (self.end.x - self.begin.x).signum(),
            y: (self.end.y - self.begin.y).signum(),
        };
        PointIterator {
            current: self.begin,
            delta: delta,
            end: Point {
                x: self.end.x + delta.x,
                y: self.end.y + delta.y,
            },
        }
    }
}

/// Iterates over all points on a line
struct PointIterator {
    current: Point,
    delta: Point,
    end: Point,
}
impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let pt = Some(self.current);
            self.current.x += self.delta.x;
            self.current.y += self.delta.y;
            pt
        }
    }
}

fn main() {
    // read lines from stdin
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read lines from stdin");

    // parse lines using regex
    let line_re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let lines: Vec<_> = input
        .trim_end()
        .split("\n")
        .map(|line| {
            let captures = line_re.captures(line).expect("Failed to parse line");

            let err_msg = "Failed to parse point as int";
            (
                Point {
                    x: captures[1].parse().expect(err_msg),
                    y: captures[2].parse().expect(err_msg),
                },
                Point {
                    x: captures[3].parse().expect(err_msg),
                    y: captures[4].parse().expect(err_msg),
                },
            )
        })
        .map(|(begin, end)| Line {
            begin: begin,
            end: end,
        })
        .collect();

    // walk each line and mark each point it covers inthe intersect_counts map
    let mut intersect_counts = HashMap::new();
    for line in lines {
        for pt in line.iter() {
            let count = intersect_counts.entry(pt).or_insert(0);
            *count += 1;
        }
    }
    let n_intersect_pts = intersect_counts
        .values()
        .filter(|&&count| count >= 2)
        .count();
    println!("No. of points 2 or more intersections: {}", n_intersect_pts)
}
