//!
//! Anti Helmet
//! Advent of Code
//! Day 19: Beacon Scanner
//!

use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};
use std::ops::{Add, Sub};

/// Defines the axes in a 3D space.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32, i32);
impl Point {
    /// Parse a new 3D point from a string in format: 'x,y,z'
    fn parse(pt_str: &str) -> Self {
        let format_err = "Failed to parse malformed 3D point: Expected point in format x,y,z";
        let mut coordinates = pt_str.split(",");
        let mut next_coordinate = move || {
            coordinates
                .next()
                .expect(format_err)
                .parse()
                .expect(format_err)
        };
        Point(next_coordinate(), next_coordinate(), next_coordinate())
    }
    /// Rotates this point around the given axis 90 degress clockwise.
    fn rotate(&self, around: &Axis) -> Self {
        let &Point(x, y, z) = self;
        match around {
            Axis::X => Point(x, z, -y),
            Axis::Y => Point(z, y, -x),
            Axis::Z => Point(y, -x, z),
        }
    }

    /// Rotates this point around the given axis 90 degress anticlockwise.
    fn reverse_rotate(&self, around: &Axis) -> Self {
        let &Point(x, y, z) = self;
        match around {
            Axis::X => Point(x, -z, y),
            Axis::Y => Point(-z, y, x),
            Axis::Z => Point(-y, x, z),
        }
    }

    /// Inverts this point on the the given axis
    fn invert(&self, axis: &Axis) -> Self {
        let &Point(x, y, z) = self;

        match axis {
            Axis::X => Point(-x, y, z),
            Axis::Y => Point(x, -y, z),
            Axis::Z => Point(x, y, -z),
        }
    }

    /// Negate this point such that it points in the opposite direction relative
    /// to the origin.
    fn negate(&self) -> Self {
        let &Point(x, y, z) = self;
        Point(-x, -y, -z)
    }

    /// Computes the manhattan distance between the two points
    fn dist(self, other: Self) -> i32 {
        let Point(x, y, z) = self - other;
        i32::abs(x) + i32::abs(y) + i32::abs(z)
    }
}
impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let Point(lhs_x, lhs_y, lhs_z) = self;
        let Point(rhs_x, rhs_y, rhs_z) = rhs;

        Point(lhs_x - rhs_x, lhs_y - rhs_y, lhs_z - rhs_z)
    }
}
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let Point(lhs_x, lhs_y, lhs_z) = self;
        let Point(rhs_x, rhs_y, rhs_z) = rhs;

        Point(lhs_x + rhs_x, lhs_y + rhs_y, lhs_z + rhs_z)
    }
}

/// Defines a transform that can be applied to points.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Transform {
    /// Defines an orientation transform
    Orient {
        inversion: Option<Axis>,
        rotations: Option<(Axis, u32)>,
    },
    /// Defines a transform that is the unapplies the wrapped transform.
    Inverse(Box<Transform>),
    /// Defines a transform applies all wrapped transforms.
    Stack(Vec<Box<Transform>>),
    /// Identity transform does nothing.
    Identity,
}
impl Transform {
    /// Stack the given transforms together into a single equvilent transform
    fn stack(transforms: &[Transform]) -> Self {
        Transform::Stack(
            transforms
                .into_iter()
                .map(|transform| Box::new(transform.clone()))
                .collect(),
        )
    }

    /// Invert this transform to produce a transform that has the effect of
    /// unapplying the given transform.
    fn invert(&self) -> Self {
        Transform::Inverse(Box::new(self.clone()))
    }

    /// Apply this transform on the given point
    fn apply(&self, pt: &Point) -> Point {
        use Transform::*;
        match self {
            Orient {
                inversion,
                rotations,
            } => {
                let pt = inversion.map_or(*pt, |axis| pt.invert(&axis));
                rotations.map_or(pt, |(axis, n_rotations)| {
                    (0..n_rotations).fold(pt, |pt, _| pt.rotate(&axis))
                })
            }
            Inverse(transform) => transform.unapply(pt),
            Stack(transforms) => transforms.iter().fold(*pt, |pt, t| t.apply(&pt)),
            Identity => *pt,
        }
    }

    /// Reverse the transformation performed on the given point.
    fn unapply(&self, pt: &Point) -> Point {
        use Transform::*;
        match self {
            Orient {
                inversion,
                rotations,
            } => {
                let pt = rotations.map_or(*pt, |(axis, n_rotations)| {
                    (0..n_rotations).fold(*pt, |pt, _| pt.reverse_rotate(&axis))
                });
                inversion.map_or(pt, |axis| pt.invert(&axis))
            }
            Inverse(transform) => transform.apply(pt),
            Stack(transforms) => transforms.iter().rev().fold(*pt, |pt, t| t.unapply(&pt)),
            Identity => *pt,
        }
    }
}

/// Defines a Scanner that detects beacons relative to its own position and orientation.
/// The scanner is able to detect beacons up to 1000 units away on x, y, z axis.
#[derive(Clone)]
struct Scanner {
    beacon_pts: Vec<Point>,
}
impl Scanner {
    /// Apply the given closure f over the beacon_pts in this scanner.
    fn map<F: Fn(&Point) -> Point>(&self, f: F) -> Self {
        Self {
            beacon_pts: self.beacon_pts.iter().map(f).collect(),
        }
    }

    /// Attempt to pinpoint the other scanner relative to the this scanner.
    /// Attempts to infer by matching beacons positions between this scanner
    /// and the other scanner. Expects the other scanner to be in the same orientation
    /// as this scanner.
    fn pinpoint(&self, other: &Scanner) -> Option<Point> {
        // compute deltas between beacon points of the two scanners
        // and count no. of times each delta occurs.
        let mut delta_counts = HashMap::new();
        for &our_pt in self.beacon_pts.iter() {
            for &their_pt in other.beacon_pts.iter() {
                let delta = our_pt - their_pt;
                let count = delta_counts.entry(delta).or_insert(0);
                *count += 1;

                // the delta with more or equal to 12 occurances determines other scanner's
                // relative position as it signifies 12 identical beacons matched by
                // both scanners.
                if *count >= 12 {
                    return Some(delta);
                }
            }
        }

        None
    }
}

/// Resolve an absoute offset & transform from src scanner to dest scanner using the given
/// scanner map.  Source and destination scanners are specified by their ids assigned in the
/// scanner map.  Returns the resolved absolute offset & transform or none if it can be resolved.
fn resolve(
    src_id: usize,
    dest_id: usize,
    visited_ids: &HashSet<usize>,
    scanner_map: &HashMap<(usize, usize), (Point, Transform)>,
) -> Option<(Point, Transform)> {
    // base case: scanner_map already has entry
    if scanner_map.contains_key(&(src_id, dest_id)) {
        Some(scanner_map[&(src_id, dest_id)].clone())
    } else {
        // find the offset / transforms of the scanners connected to the source scanner.
        let connected: Vec<_> = scanner_map
            .iter()
            .filter(|((s, connected_id), _)| *s == src_id && !visited_ids.contains(connected_id))
            .map(|((_, connected_id), (offset, transform))| (connected_id, (offset, transform)))
            .collect();

        if connected.len() <= 0 {
            // dead end: not possible to resolve absolute offset / transform
            None
        } else {
            // attempt to resolve via connected scanner
            connected
                .into_iter()
                .flat_map(
                    |(&connected_id, (&connected_offset, connected_transform))| {
                        let mut visited_ids = visited_ids.clone();
                        visited_ids.insert(connected_id);
                        // recusively resolve to destination via connected scanner
                        resolve(connected_id, dest_id, &visited_ids, scanner_map).map(
                            |(dest_offset, dest_transform)| {
                                (
                                    connected_offset + connected_transform.unapply(&dest_offset),
                                    // build transform by stacking connected & dest transforms
                                    Transform::stack(&[
                                        connected_transform.clone(),
                                        dest_transform,
                                    ]),
                                )
                            },
                        )
                    },
                )
                .next()
        }
    }
}

fn main() {
    // parse the scanner reports written to stdin
    let scanners: Vec<_> = stdin()
        .lock()
        .lines()
        .map(|rst| rst.expect("Failed to read scanner report lines from std"))
        .collect::<Vec<_>>()
        // scanner reports are delimited by a empty line.
        .split(|line| line == "")
        .map(|report_lines| {
            Scanner {
                // skip scanner header in report lines
                beacon_pts: report_lines[1..]
                    .into_iter()
                    .map(|line| Point::parse(line))
                    .collect(),
            }
        })
        .collect();

    // permutate all possible scanner orientation transforms
    // permutate possible "front" axis on which the scanner faces forward.
    let transforms: Vec<_> = [Axis::X, Axis::Y, Axis::Z]
        .iter()
        .flat_map(|&front_axis|
            // permutate whether to invert the front axis such that it faces backward.
            [Some(front_axis), None].iter().flat_map(|&invert_axis|
                // permutate no. of rotations to perform around front axis
                (0..4).map(|n_rotations|
                    Transform::Orient {
                        inversion: invert_axis,
                        rotations: Some((front_axis, n_rotations)),
                    }
                ).collect::<Vec<_>>()
            ).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // precompute orientation transformed scanners
    let transformed_scanners: HashMap<_, _> = scanners
        .iter()
        .enumerate()
        .map(|(scanner_id, scanner)| {
            (
                scanner_id,
                transforms
                    .iter()
                    .map(|transform| (transform, scanner.map(|pt| transform.apply(pt))))
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    // search for scanners that are able to pinpoint each other via mapping identical beacons
    let mut scanner_map = HashMap::new();
    for scanner_id in 0..scanners.len() {
        for (scanner_transform, scanner) in transformed_scanners[&scanner_id].iter() {
            for other_id in scanner_id + 1..scanners.len() {
                for (other_transform, other) in transformed_scanners[&other_id].iter() {
                    // skip matching transforms of the scanner and already mapped scanners
                    if !scanner_map.contains_key(&(scanner_id, other_id)) {
                        scanner.pinpoint(other).map(|offset| {
                            // offset is computed relative to the scanner's transform.
                            // unapply to remove the transform taint from offset
                            // let offset = scanner_transform.unapply(&offset);
                            // compile overall transform from scanner to other scanner
                            let transform = Transform::stack(&[
                                (*scanner_transform).clone(),
                                other_transform.invert(),
                            ]);

                            // record relative offset & transform of scanner with other scanner
                            // in scanner map
                            scanner_map.insert(
                                (other_id, scanner_id),
                                (
                                    other_transform.unapply(&offset).negate(),
                                    transform.invert(),
                                ),
                            );
                            scanner_map.insert(
                                (scanner_id, other_id),
                                (scanner_transform.unapply(&offset), transform),
                            );
                        });
                    }
                }
            }
        }
    }

    // use first scanner as "origin" scanner: its coordinates are considered canon.
    let origin_id = 0;

    // resolve all other scanners relative to origin scanner
    let resolved_map: Vec<_> = (0..scanners.len())
        .map(|scanner_id| {
            if scanner_id == origin_id {
                // origin scanner does not require an offset or transform
                (origin_id, (Point(0, 0, 0), Transform::Identity))
            } else {
                (
                    scanner_id,
                    resolve(origin_id, scanner_id, &HashSet::new(), &scanner_map)
                        .expect("Failed to resolve scanner relative to origin scanner"),
                )
            }
        })
        .collect();

    let unique_beacons: HashSet<_> = resolved_map
        .iter()
        .flat_map(|(scanner_id, (offset, transform))|
            // map beacon_pts to origin scanner 3D space by applying inverse transform
            // and adding the offset of the beacon's scanner
            scanners[*scanner_id].map(|pt|
                    transform.unapply(pt) + *offset
                ).beacon_pts)
        .collect();
    println!("No. of unique beacons: {}", unique_beacons.len());

    let max_dist = resolved_map
        .iter()
        .flat_map(|(_, (offset_left, _))| {
            resolved_map
                .iter()
                .map(|(_, (offset_right, _))| offset_left.dist(*offset_right))
                .collect::<Vec<_>>()
        })
        .max()
        .expect("Expected at least one distance to be computed");
    println!("Maximum distance between two scanners: {}", max_dist);
}
