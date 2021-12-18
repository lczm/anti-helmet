//!
//! Anti Helmet
//! Advent of Code
//! Day 12: Passage Pathing
//!

use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

/// Represents a cave in a cave system
#[derive(PartialEq, Eq, Hash, Clone)]
enum Cave {
    Start,
    End,
    Small(String),
    Big(String),
}

/// Represents a traversable cave system
struct CaveSystem {
    adjacency_matrix: Vec<Vec<bool>>,
    cave_id_map: HashMap<Cave, u8>,
    id_cave_map: HashMap<u8, Cave>,
}

impl CaveSystem {
    /// Create a new cave system formed from the given list of connections between caves.
    fn new(connections: Vec<(Cave, Cave)>) -> CaveSystem {
        // build mapping between auto assigned cave id and cave instance
        let cave_id_map: HashMap<Cave, u8> = connections
            .iter()
            .flat_map(|(src, dest)| vec![src, dest])
            .collect::<HashSet<_>>()
            .into_iter()
            .enumerate()
            .map(|(i, cave)| (cave.clone(), i as u8))
            .collect();

        // build mapping between cave instance and auto assigned cave id
        let id_cave_map: HashMap<u8, Cave> = cave_id_map
            .keys()
            .map(|cave| (cave_id_map[cave], cave.clone()))
            .collect();

        // compile an adjacency matrix to represent connections
        let n_caves = cave_id_map.len();
        let mut adjacency_matrix = vec![vec![false; n_caves]; n_caves];
        for (src, dest) in connections.iter() {
            let src_id = cave_id_map[&src] as usize;
            let dest_id = cave_id_map[&dest] as usize;

            // mark source and destination caves as adjacency to each other
            // since the connections are undirected,
            // create bidirectional adjacency markings: src -> dest, dest -> src
            adjacency_matrix[src_id][dest_id] = true;
            adjacency_matrix[dest_id][src_id] = true;
        }

        CaveSystem {
            cave_id_map: cave_id_map,
            id_cave_map: id_cave_map,
            adjacency_matrix: adjacency_matrix,
        }
    }

    /// Returns to the caves that are connected to the given cave in this cavef\
    /// system.
    fn connected(&self, cave: &Cave) -> Vec<&Cave> {
        let cave_id = self.cave_id_map[cave];

        self.adjacency_matrix[cave_id as usize]
            .iter()
            .enumerate()
            .filter(|(_, &is_connected)| is_connected)
            .map(|(id, _)| &self.id_cave_map[&(id as u8)])
            .collect()
    }

    /// Perform depth first search on the this cave system to find the no. of paths
    /// between the given begin & end caves.
    fn dfs<'a>(&'a self, begin: &Cave, end: &Cave, visited_small: Vec<&Cave>) -> u32 {
        if begin == end {
            // base case: found target end cave
            1
        } else {
            // recursive case: recursively search for paths from begin to end caves
            let mut total_paths = 0;
            for cave in self.connected(begin) {
                let next_cave = match cave {
                    // big caves can be visited any number of times
                    Cave::Big(_) => Some(cave),
                    // small caves:
                    small_cave @ Cave::Small(_) => match small_cave {
                        // all small caves can be visited at least once
                        _ if !visited_small.contains(&small_cave) => Some(cave),
                        // one small cave can be visited at most twice
                        _ if visited_small.len() == visited_small.iter().collect::<HashSet<_>>().len()
                            => Some(cave),
                        _ => None,
                    },
                    // end cave can be visited next, which will complete the
                    // path and end the recursion.
                    Cave::End => Some(cave),
                    _ => None,
                };
                
                total_paths += next_cave.map_or(0, |cave| {
                    // track small caves visited
                    let mut visited_small = visited_small.clone();
                    if let Cave::Small(_) = cave {
                        visited_small.push(cave);
                    }
                    self.dfs(cave, end, visited_small)
                });
            }

            total_paths
        }
    }
}

fn main() {
    // read the cave system connections from stdin
    let connections: Vec<(Cave, Cave)> = stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read cave system connections from stdin."))
        .map(|line| {
            // closure to wrap caves into cave types
            let wrap_cave = |name: String| -> Cave {
                match name.as_str() {
                    "start" => Cave::Start,
                    "end" => Cave::End,
                    _ if name == name.to_lowercase() => Cave::Small(name),
                    _ => Cave::Big(name),
                }
            };

            if let [src, dest] = line.split("-").collect::<Vec<&str>>().as_slice() {
                (wrap_cave(src.to_string()), wrap_cave(dest.to_string()))
            } else {
                panic!("Expected connectionsa to be in the format: SRC-DEST");
            }
        })
        .collect();
    let system = CaveSystem::new(connections);

    // find paths from start to the end cavews
    println!(
        "No. of distinct paths from start to end: {}",
        system.dfs(&Cave::Start, &Cave::End, vec![])
    )
}
