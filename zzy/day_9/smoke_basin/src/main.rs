use std::cell::RefCell;
use std::collections::HashSet;
///!
///! Anti-Helmet
///! Advent of Code
///! Day 9: Smoke Basin
///!
use std::io::{stdin, Read};
use std::rc::Rc;

/// Represent a (x, y) point in 2D space
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point(usize, usize);

/// Extract the surrounding points from the given point in the given 2D heights.
/// Surrounding are defined as the that on the top, bottom, left, right of the given
/// point. If a surrounding point is not defined for the given point in heights
/// ie due to point existing on the boundary of heights, the point will be obmitted
/// in returned points.
fn surroundings(heights: &[&[u8]], Point(x, y): Point) -> Vec<Point> {
    // extract dimensions of heights
    let max_y = heights.len();
    let max_x = heights[0].len();

    // check that given point exists within heights
    assert!(x < max_x && y < max_y);

    // collect a vector of defined surrounding points
    [
        if (x as isize) - 1 < 0 {
            None
        } else {
            Some(Point(x - 1, y))
        },
        if x + 1 >= max_x {
            None
        } else {
            Some(Point(x + 1, y))
        },
        if (y as isize) - 1 < 0 {
            None
        } else {
            Some(Point(x, y - 1))
        },
        if y + 1 >= max_y {
            None
        } else {
            Some(Point(x, y + 1))
        },
    ]
    .iter()
    // flatten to remove the wrapping option & discard undefined points
    .flatten()
    .map(|&pt| pt)
    .collect()
}

/// Measure the size of the basin specified by is lowest point in given 2D heights.
/// Avoids exploring any points that are specified as already explored in explored set.
/// Returns the no. of points that make up the basin
fn measure_basin(
    heights: &[&[u8]],
    lowest_pt: Point,
    explored: Rc<RefCell<HashSet<Point>>>,
) -> u32 {
    // collect basin points in the immediate surrounding of the given point
    let Point(lowest_x, lowest_y) = lowest_pt;
    let basin_pts = surroundings(heights, lowest_pt)
        .into_iter()
        .filter(|pt| !explored.borrow().contains(pt))
        // surrounding basin points < height 9 and must be 1 height higher than lowest point
        .filter(|&Point(other_x, other_y)| {
            let basin_height = heights[other_y][other_x];
            basin_height < 9 && basin_height > heights[lowest_y][lowest_x]
        });

    // mark lowest point as explored
    explored.borrow_mut().insert(lowest_pt);

    // recursively calculate total no. of basin points
    basin_pts
        .map(|pt| measure_basin(heights, pt, explored.clone()))
        // +1 to account for lowest point
        .sum::<u32>()
        + 1
}

/// Check if the given point is considered a low point in the given 2D heights.
fn is_low(heights: &[&[u8]], point: Point) -> bool {
    let Point(x, y) = point;
    // compare current with all surrounding points to check if its lower than all.
    surroundings(heights, point)
        .into_iter()
        .all(|Point(other_x, other_y)| heights[y][x] < heights[other_y][other_x])
}

fn main() {
    // read smoke flow heights from stdin
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read smoke flow height map into string");
    // heights is indexed in y, x order.
    let heights: Vec<_> = input
        .trim_end()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("Failed to parse height in smoke flow height map")
                        as u8
                })
                .collect::<Vec<_>>()
        })
        .collect();
    // reconstruct heights as a 2D slice
    let heights: &Vec<&[u8]> = &heights.iter().map(|v| v.as_slice()).collect();
    assert!(heights.len() >= 1); // check that heights is 2D

    // generate all possible points on 2D space defined by heights
    let max_y = heights.len();
    let max_x = heights[0].len();
    let all_points = (0..max_y).flat_map(|y| (0..max_x).map(|x| (x, y)).collect::<Vec<_>>());

    // find low points in smoke flow
    let low_pts = all_points
        .map(|(x, y)| Point(x, y))
        .filter(|&pt| is_low(&heights, pt));

    // collect basin sizes form lowest points
    let mut basin_sizes: Vec<u32> = low_pts
        .map(|low_pt| measure_basin(&heights, low_pt, Rc::new(RefCell::new(HashSet::new()))))
        .collect();

    // select top 3 basin sizes
    basin_sizes.sort_unstable();
    basin_sizes.reverse();

    println!(
        "Risk sum: {}",
        basin_sizes[..3].iter().fold(1, |left, &right| left * right)
    );
}
