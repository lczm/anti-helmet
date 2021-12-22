//!
//! Anti Helmet
//! Advent of Code
//! Day 15: Chitons
//!

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::io::{stdin, BufRead};
use std::ops::Index;

/// Key trait defines a type that produces a hashable key
trait Key<K: Hash + Copy + Eq> {
    fn key(&self) -> &K;
}

/// MaxMap combines the O(1) lookup of a hashmap with the O(1) tracking
/// of the O(1) lookup of the maximum value characteristic of a binary heap.
struct MaxMap<K: Hash + Copy + Eq, V: Copy + Ord + Key<K>> {
    map: HashMap<K, V>,
    max: BinaryHeap<V>,
}
impl<K: Hash + Copy + Eq, V: Copy + Ord + Key<K>> MaxMap<K, V> {
    fn new() -> MaxMap<K, V> {
        MaxMap {
            map: HashMap::new(),
            max: BinaryHeap::new(),
        }
    }

    /// Push the given value onto the MaxMap
    fn push(&mut self, value: V) {
        self.map.insert(*value.key(), value);
        self.max.push(value);
    }

    /// Pop the maximum value of the MaxMap in constant time.
    /// Returns None If there no values stored in the MaxMap.
    fn pop(&mut self) -> Option<V> {
        self.max.pop()
    }

    fn len(&self) -> usize {
        self.max.len()
    }
}
impl<K: Hash + Copy + Eq, V: Copy + Ord + Key<K>> Index<&K> for MaxMap<K, V> {
    type Output = V;

    fn index(&self, key: &K) -> &V {
        &self.map[key]
    }
}

/// Represents a position in the Cave
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

/// Defines Cave with varying chiton risk levels at different positions.
struct Cave {
    risk_sample: Vec<Vec<u8>>,
}
impl Cave {
    fn new(risk_sample: Vec<Vec<u8>>) -> Self {
        assert!(risk_sample.len() > 0 && risk_sample[0].len() > 0);
        Self { risk_sample }
    }

    /// Returns the 2D length bounds of this Cave.
    fn bounds(&self) -> (usize, usize) {
        // actual cave is 5-times larger than risk sample
        (self.risk_sample[0].len() * 5, self.risk_sample.len() * 5)
    }

    /// Compute & Return the chiton risk of the given cave position
    fn risk(&self, &Position { x, y }: &Position) -> u8 {
        // check that we are given position within bounds
        let (len_x, len_y) = self.bounds();
        if x >= len_x || y >= len_y {
            panic!("Cannot derive risk of position that is out of bounds of the cave.");
        }

        let (len_sample_x, len_sample_y) = (self.risk_sample[0].len(), self.risk_sample.len());
        // find the risk sample that the position corresponds to
        let (sample_x, sample_y) = (x % len_sample_x, y % len_sample_y);
        let sample_risk = self.risk_sample[sample_y][sample_x] as usize;

        // increment risk sample depending on far position is from the risk sample
        let increment = (x / len_sample_x) + (y / len_sample_y);
        // -1 / +1 required as modulus only produces values between 0-8 when
        // we want values from 1-9
        ((sample_risk - 1 + increment) % 9 + 1) as u8
    }

    /// Generate a returns a list of cave Positions that are connected to the given
    /// cave position.
    fn connected(&self, &Position { x, y }: &Position) -> Vec<Position> {
        let (len_x, len_y) = self.bounds();

        // offset to apply to x / y axis to generate connected cave positions
        // only directly vertical / horiztonal cave positions are connected.
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .flat_map(|(offset_x, offset_y)| {
                isize::checked_add(x as isize, *offset_x)
                    .zip(isize::checked_add(y as isize, *offset_y))
                    // filter out out of bounds positions
                    .filter(|(x, y)| {
                        *x >= 0 && *y >= 0 && *x < len_x as isize && *y < len_y as isize
                    })
                    .map(|(x, y)| Position {
                        x: x as usize,
                        y: y as usize,
                    })
            })
            .collect::<Vec<_>>()
    }
}

/// Pairs a cave position with the minimum chiton risk currently derived
/// incured from traveling to the Cave Position from some starting position.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct TravelRisk {
    position: Position,
    risk: u32,
}
impl Ord for TravelRisk {
    fn cmp(&self, other: &Self) -> Ordering {
        // travel risk with a lower risk should be ranked higher
        self.risk.cmp(&other.risk).reverse()
    }
}
impl PartialOrd for TravelRisk {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Key<Position> for TravelRisk {
    fn key(&self) -> &Position {
        &self.position
    }
}

/// Find the risk of the safest path from given being position to end position.
/// The safest path is defined as a path which incures the lowest chiton risk
/// Looks up chiton risks of each position in the givenn cave
/// Tracks the minimum risk computed so far in the given min_risks heap.
fn find_safest(begin: Position, end: Position, cave: &Cave) -> u32 {
    // initialize travel risk MaxMap
    let (len_x, len_y) = cave.bounds();
    let mut travel_risks = MaxMap::new();
    for x in 0..=len_x {
        for y in 0..=len_y {
            // skip begin position
            let position = Position { x, y };
            travel_risks.push(TravelRisk {
                risk: if position == begin {
                    // starting position's risk is never counted
                    0
                } else {
                    u32::MAX
                },
                position: position,
            });
        }
    }

    // apply dijkstra's algorithm to resolve the path that incurs the lowest risk
    let mut current: Position = begin;
    while current != end && travel_risks.len() > 0 {
        for other in cave.connected(&current) {
            // find the risk to traverse to 'other' position via current 'begin' position
            let via_risk = travel_risks[&current].risk + cave.risk(&other) as u32;

            // relax min risk of traversing to 'other' position if safer via
            // current 'begin' position
            if via_risk < travel_risks[&other].risk {
                let shorter_risk = TravelRisk {
                    position: other,
                    risk: via_risk,
                };
                travel_risks.push(shorter_risk);
            }
        }

        // explore cave position with least risk next
        let min_risk = travel_risks.pop().unwrap();
        current = min_risk.position;
    }

    if travel_risks.len() <= 0 {
        panic!("No path exists between begin and end positions");
    }

    travel_risks[&end].risk
}

fn main() {
    // read with chiton risk levels sample from stdin.
    let risk_sample: Vec<Vec<_>> = stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read cave with chiton risk levels from stdin"))
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("Failed to parse cave chiton risk level as digit")
                        as u8
                })
                .collect()
        })
        .collect();
    if risk_sample.len() <= 0 || risk_sample[0].len() <= 0 {
        panic!("Expected to read at least once risk level.");
    }
    let cave = Cave::new(risk_sample);

    // find the risk of the safest route from top left of the cave tom the bottom right
    let (len_x, len_y) = cave.bounds();
    let top_left = Position { x: 0, y: 0 };
    let bottom_right = Position {
        x: len_x - 1,
        y: len_y - 1,
    };

    let min_risk = find_safest(top_left, bottom_right, &cave);
    println!("Risk of the safest route: {}", min_risk);
}
