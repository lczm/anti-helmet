use std::fs;
use std::collections::HashMap;
use std::cmp::{min, max};

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

fn part1() {
    let filename = "in1";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split_content = content.split("\n")
        .filter(|&x| !x.is_empty());

    let mut items: Vec<(Point, Point)> = Vec::new();
    for s in split_content {
        let th: Vec<&str> = s.split("->")
            .map(|s| s.trim())
            .collect();
        let a: Vec<u32> = th[0].split(",")
            .map(|s| s.parse::<u32>().expect("parse into u32 error"))
            .collect();
        let b: Vec<u32> = th[1].split(",")
            .map(|s| s.parse::<u32>().expect("parse into u32 error"))
            .collect();
        items.push((
            Point {x:a[0], y:a[1]},
            Point {x:b[0], y:b[1]},));
    }

    let mut grid: HashMap<Point, u32> = HashMap::new();
    for i in items.iter() {
        let a = i.0;
        let b = i.1;
        if a.x == b.x { // same x-axis
            for y in min(a.y, b.y)..max(a.y, b.y) + 1 {
                if !grid.contains_key(&Point{x:a.x, y:y}) {
                    grid.insert(Point{x:a.x, y:y}, 1);
                } else {
                    *grid.get_mut(&Point{x:a.x, y:y}).unwrap() += 1;
                }
            }
        } else if a.y == b.y { // same y-axis
            for x in min(a.x, b.x)..max(a.x, b.x) + 1 {
                if !grid.contains_key(&Point{x:x, y:a.y}) {
                    grid.insert(Point{x:x, y:a.y}, 1);
                } else {
                    *grid.get_mut(&Point{x:x, y:a.y}).unwrap() += 1;
                }
            }
        }
    }

    let mut count = 0;
    for (p, i) in &grid {
        // println!("{:?}, {}", p, i);
        if i >= &2 {
            count += 1;
        }
    }

    println!("points with two overlaps: {}", count);
}

fn part2() {
    let filename = "in1";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split_content = content.split("\n")
        .filter(|&x| !x.is_empty());

    let mut items: Vec<(Point, Point)> = Vec::new();
    for s in split_content {
        let th: Vec<&str> = s.split("->")
            .map(|s| s.trim())
            .collect();
        let a: Vec<u32> = th[0].split(",")
            .map(|s| s.parse::<u32>().expect("parse into u32 error"))
            .collect();
        let b: Vec<u32> = th[1].split(",")
            .map(|s| s.parse::<u32>().expect("parse into u32 error"))
            .collect();
        items.push((
            Point {x:a[0], y:a[1]},
            Point {x:b[0], y:b[1]},));
    }

    let mut test = 0;
    let mut grid: HashMap<Point, u32> = HashMap::new();
    for i in items.iter() {
        let a = i.0;
        let b = i.1;
        if a.x == b.x { // same x-axis
            for y in min(a.y, b.y)..max(a.y, b.y) + 1 {
                if !grid.contains_key(&Point{x:a.x, y:y}) {
                    grid.insert(Point{x:a.x, y:y}, 1);
                } else {
                    *grid.get_mut(&Point{x:a.x, y:y}).unwrap() += 1;
                }
            }
        } else if a.y == b.y { // same y-axis
            for x in min(a.x, b.x)..max(a.x, b.x) + 1 {
                if !grid.contains_key(&Point{x:x, y:a.y}) {
                    grid.insert(Point{x:x, y:a.y}, 1);
                } else {
                    *grid.get_mut(&Point{x:x, y:a.y}).unwrap() += 1;
                }
            }
        } else if max(a.x, b.x) - min(a.x, b.x) == max(a.y, b.y) - min(a.y, b.y) {
            // diagonals
            let diff = max(a.x, b.x) - min(a.x, b.x);
            // println!("diff: {}, {:?}, {:?}", diff, a, b);
            if (a.x < b.x) && (a.y < b.y) {
                for i in 0..diff + 1 {
                    let x = a.x + i;
                    let y = a.y + i;
                    if !grid.contains_key(&Point{x:x, y:y}) {
                        grid.insert(Point{x:x, y:y}, 1);
                    } else {
                        *grid.get_mut(&Point{x:x, y:y}).unwrap() += 1;
                    }
                }
                test += 1;
            } else if (a.x < b.x) && (a.y > b.y) {
                for i in 0..diff + 1 {
                    let x = a.x + i;
                    let y = a.y - i;
                    if !grid.contains_key(&Point{x:x, y:y}) {
                        grid.insert(Point{x:x, y:y}, 1);
                    } else {
                        *grid.get_mut(&Point{x:x, y:y}).unwrap() += 1;
                    }
                }
                test += 1;
            } else if (a.x > b.x) && (a.y > b.y) {
                for i in 0..diff + 1 {
                    let x = a.x - i;
                    let y = a.y - i;
                    if !grid.contains_key(&Point{x:x, y:y}) {
                        grid.insert(Point{x:x, y:y}, 1);
                    } else {
                        *grid.get_mut(&Point{x:x, y:y}).unwrap() += 1;
                    }
                }
                test += 1;
            } else if (a.x > b.x) && (a.y < b.y) {
                for i in 0..diff + 1 {
                    let x = a.x - i;
                    let y = a.y + i;
                    if !grid.contains_key(&Point{x:x, y:y}) {
                        grid.insert(Point{x:x, y:y}, 1);
                    } else {
                        *grid.get_mut(&Point{x:x, y:y}).unwrap() += 1;
                    }
                }
                test += 1;
            }
        }
    }

    let mut count = 0;
    for (p, i) in &grid {
        // println!("{:?}, {}", p, i);
        if i >= &2 {
            count += 1;
        }
    }

    println!("points with two overlaps: {}", count);
}

fn main() {
    part1();
    part2();
}
