//!
//! Anti Helmet
//! Advent of Code
//! Day 17: Trick Shot
//!

use regex::Regex;
use std::io::{stdin, Read};

/// Defines a target area
struct Area {
    x_begin: i32,
    x_end: i32,
    y_begin: i32,
    y_end: i32,
}
impl Area {
    fn new(x_begin: i32, x_end: i32, y_begin: i32, y_end: i32) -> Self {
        // check area bounds define a valid area
        if x_begin > x_end && y_begin > y_end {
            panic!("Given area bounds that do not define a valid area");
        }
        Self {
            x_begin,
            x_end,
            y_begin,
            y_end,
        }
    }

    /// Check if the given point is within this Area
    fn within(&self, (x, y): (i32, i32)) -> bool {
        self.x_begin <= x && x <= self.x_end && self.y_begin <= y && y <= self.y_end
    }
}

/// Defines a launchable probe.
struct Probe {
    // x, y coordinates describing the position of the probe
    x: i32,
    y: i32,

    // x, y velocity describing the current movement of the probe
    velocity_x: i32,
    velocity_y: i32,

    // keep track the highest y position attained by the porbe
    max_y: i32,
}
impl Probe {
    /// Create a new probe with the given starting x, y velocity starting at
    /// 0, 0 launhing position.
    fn new(velocity_x: i32, velocity_y: i32) -> Self {
        Probe {
            x: 0,
            y: 0,
            max_y: 0,
            velocity_x: velocity_x,
            velocity_y: velocity_y,
        }
    }

    /// Advance the probe to its next position and velocity.
    fn next(self) -> Self {
        let new_y = self.y + self.velocity_y;
        Probe {
            x: self.x + self.velocity_x,
            y: new_y,
            max_y: i32::max(new_y, self.max_y),
            velocity_x: self.velocity_x - self.velocity_x.signum(),
            velocity_y: self.velocity_y - 1,
        }
    }

    /// Returns the current position of the probe.
    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Determine if its possible for the probe to reaching the given target area.
    /// Note that this does not guarantee that the probe will reach the target area,
    /// only that it is still possible.
    fn is_reachable(&self, area: &Area) -> bool {
        if self.velocity_x >= 0 {
            self.x <= area.x_end && self.y >= area.y_begin
        } else {
            area.x_begin <= self.x && self.y >= area.y_begin
        }
    }
}

/// Simulate the given probe to determine if it will area the given target area
/// and its maximum y coordinate the probe reached during it flight.
fn simulate(mut probe: Probe, area: &Area) -> (bool, i32) {
    while probe.is_reachable(area) {
        // check if the probe reached the target area
        if area.within(probe.position()) {
            return (true, probe.max_y);
        }

        // advance the probe to the next step
        probe = probe.next();
    }
    (false, probe.max_y)
}

fn main() {
    // parse target area from stdin using regex
    let mut target_str = String::new();
    stdin()
        .read_to_string(&mut target_str)
        .expect("Failed to read target area from STDIN");

    let range_re = |axis| {
        format!(
            r"{axis}=(?P<{axis}_begin>-?\d+)..(?P<{axis}_end>-?\d+)",
            axis = axis
        )
    };
    let target_re = Regex::new(&format!(
        r"target area: {}, {}",
        range_re("x"),
        range_re("y")
    ))
    .unwrap();

    let captures = target_re
        .captures(target_str.trim_end())
        .expect("Failed to parse target area expression");

    let parse_bound = |name| {
        captures
            .name(name)
            .unwrap()
            .as_str()
            .parse()
            .expect("Failed to parse area coordinate bounds as integer")
    };
    let target_area = Area::new(
        parse_bound("x_begin"),
        parse_bound("x_end"),
        parse_bound("y_begin"),
        parse_bound("y_end"),
    );

    // intuitively, the fastest x velocity we can go without missing the target area is
    // x_end as it will reach the right end of the target in 1 step
    let max_velocity_x = target_area.x_end;
    
    // since we know that target area is below the launch area,
    // the min y velocity we can go without missing the target area is y_end which will take
    // us to the bottom of the target area in 1 step
    let min_velocity_y = target_area.y_begin;

    // since we know that the target area is below the launch area, we know that
    // the probe will pass by y = 0 again where its y velocity will be negative initial velocity.
    //
    // after passing by y = 0 the lowest possible velocity we can get is min_velocity_y
    // which will take us to the bottom to the target in 1 step after passing y = 0.
    //
    // hence the max velocity is abs(min_velocity_y)
    let max_velocity_y = i32::abs(min_velocity_y);

    // send probes with varying velocities to the target area
    let mut max_y = i32::MIN;
    let mut n_reached = 0;
    for velocity_x in 1..=max_velocity_x {
        for velocity_y in min_velocity_y..=max_velocity_y {
            let (reached_target, max_probe_y) = simulate(Probe::new(velocity_x, velocity_y), &target_area);
            if reached_target {
                max_y = i32::max(max_probe_y, max_y);
                n_reached += 1;
            }
        }
    }

    println!("Max y reachable with probe: {}", max_y);
    println!("No. of  distict velocity reachable: {}", n_reached);
}
