//!
//! Anti Helmet
//! Advent of Code
//! Day 21: Reactor Reboot
//! Geometry
//!

use std::fmt;

#[cfg(test)]
mod tests;

/// A bound is defined a range of points from begin to end inclusive.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bound {
    pub begin: i64,
    pub end: i64,
}
impl Bound {
    pub fn new(begin: i64, end: i64) -> Self {
        assert!(begin <= end);
        Self { begin, end }
    }

    /// Whether this bound overlaps the other given bound.
    pub fn overlaps(&self, other: &Self) -> bool {
        (self.begin <= other.begin && self.end >= other.begin)
            || (self.begin > other.begin && other.end >= self.begin)
    }

    /// Compute the intersect between this and the other given bound.
    /// Returns a new bound representing the intersection between the bounds
    /// or None if no intersect exists.
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        if self.overlaps(other) {
            Some(Bound::new(
                i64::max(self.begin, other.begin),
                i64::min(self.end, other.end),
            ))
        } else {
            None
        }
    }

    /// Retrieve the length of of the bound.
    pub fn len(&self) -> usize {
        (self.end - self.begin) as usize + 1
    }
}
impl fmt::Display for Bound {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}..{}", self.begin, self.end)
    }
}

/// Defines a 3D cuboid defined by its bounds on each of the X, Y, Z axes of 3D space.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cuboid {
    pub x_bound: Bound,
    pub y_bound: Bound,
    pub z_bound: Bound,
}
impl Cuboid {
    /// Whether this cuboid overlaps the other given cuboid.
    pub fn overlaps(&self, other: &Self) -> bool {
        [
            (self.x_bound, other.x_bound),
            (self.y_bound, other.y_bound),
            (self.z_bound, other.z_bound),
        ]
        .iter()
        .all(|(self_bound, other_bound)| self_bound.overlaps(other_bound))
    }

    /// Compute the intersection between this cuboid and the given cuboid.
    /// Returns a new cuboid specifying the intersect or None if no intersect exists.
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        self.x_bound
            .intersect(&other.x_bound)
            .map(|x_bound| {
                self.y_bound
                    .intersect(&other.y_bound)
                    .map(|y_bound| {
                        self.z_bound
                            .intersect(&other.z_bound)
                            .map(|z_bound| Cuboid {
                                x_bound,
                                y_bound,
                                z_bound,
                            })
                    })
                    .flatten()
            })
            .flatten()
    }

    pub fn len(&self) -> usize {
        self.x_bound.len() * self.y_bound.len() * self.z_bound.len()
    }
}
impl fmt::Display for Cuboid {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(
            out,
            "x={},y={},z={}",
            self.x_bound, self.y_bound, self.z_bound
        )
    }
}
