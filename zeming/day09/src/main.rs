use std::fs;
use std::collections::{HashMap, VecDeque};

fn read_file(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    return content.split("\n")
            .filter(|&s| !s.is_empty())
            .map(|s| s.trim().to_string())
            .collect();
}

fn part1() {
    let s = read_file("in1");
    let mut collection: Vec<Vec<i32>> = Vec::new();

    for line in s {
        let temp = line.split("");
        let mut col: Vec<i32> = Vec::new();
        for t in temp {
            if t != "" {
                col.push(t.parse::<i32>().unwrap());
            }
        }
        collection.push(col);
    }

    let min_x: isize = 0;
    let max_x: isize = (collection.len() - 1).try_into().unwrap();
    let min_y: isize = 0;
    let max_y: isize = (collection[0].len() - 1).try_into().unwrap();

    let mut sum = 0;

    // get_bound(0, 0, min_x, max_x, min_y, max_y);
    for x in 0..collection.len() {
        for y in 0..collection[x].len() {
            let s_x = if x as isize - 1 < min_x { x } else { x - 1 };
            let s_y = if y as isize - 1 < min_y { y } else { y - 1 };
            let e_x = if x as isize + 1 > max_x { x } else { x + 1 };
            let e_y = if y as isize + 1 > max_y { y } else { y + 1 };
            // println!("---");
            let mut smaller_than_region: bool = true;
            'percell: 
            for ix in s_x..e_x + 1 {
                for iy in s_y..e_y + 1 {
                    // if ix == x && iy == y {
                    //     continue;
                    // }
                    if collection[ix][iy] < collection[x][y] {
                        smaller_than_region = false;
                        break 'percell;
                    }
                }
            }
            if smaller_than_region {
                // println!("smaller than region, {}, {} {}", collection[x][y], x, y);
                sum += collection[x][y] + 1;
            }
            // println!("{} {} {} {}", s_x, e_x, s_y, e_x);
            // println!("---");
        }
    }
    println!("total sum is : {}", sum);
}

fn part2() {
    let s = read_file("in1");
    let mut collection: Vec<Vec<i32>> = Vec::new();

    for line in s {
        let temp = line.split("");
        let mut col: Vec<i32> = Vec::new();
        for t in temp {
            if t != "" {
                col.push(t.parse::<i32>().unwrap());
            }
        }
        collection.push(col);
    }

    for c in collection.iter() {
        println!("{:?}", c);
    }

    let min_x: isize = 0;
    let max_x: isize = (collection.len() - 1).try_into().unwrap();
    let min_y: isize = 0;
    let max_y: isize = (collection[0].len() - 1).try_into().unwrap();

    let mut basins: Vec<usize> = Vec::new();
    let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
    let mut found: HashMap<(usize, usize), bool> = HashMap::new();
    for x in 0..collection.len() {
        for y in 0..collection[x].len() {
            let s_x = if x as isize - 1 < min_x { x } else { x - 1 };
            let s_y = if y as isize - 1 < min_y { y } else { y - 1 };
            let e_x = if x as isize + 1 > max_x { x } else { x + 1 };
            let e_y = if y as isize + 1 > max_y { y } else { y + 1 };
            // println!("---");
            let mut smaller_than_region: bool = true;
            'percell: 
            for ix in s_x..e_x + 1 {
                for iy in s_y..e_y + 1 {
                    if collection[ix][iy] < collection[x][y] {
                        smaller_than_region = false;
                        break 'percell;
                    }
                }
            }

            if smaller_than_region {
                stack.push_back((x, y));
            } else {
                continue;
            }

            let mut basin_size = 1;
            found.insert((x, y), true);
            while stack.len() != 0 {
                let (fx, fy) = *stack.front().unwrap();
                println!("{}, {}, {}", fx, fy, collection[fx][fy]);
                let s_x = if fx as isize - 1 < min_x { fx } else { fx - 1 };
                let s_y = if fy as isize - 1 < min_y { fy } else { fy - 1 };
                let e_x = if fx as isize + 1 > max_x { fx } else { fx + 1 };
                let e_y = if fy as isize + 1 > max_y { fy } else { fy + 1 };
                if collection[fx][s_y] != 9 && !found.contains_key(&(fx, s_y)) {
                    stack.push_back((fx, s_y));
                    found.insert((fx, s_y), true);
                    basin_size += 1;
                }
                if collection[fx][e_y] != 9 && !found.contains_key(&(fx, e_y)) {
                    stack.push_back((fx, e_y));
                    found.insert((fx, e_y), true);
                    basin_size += 1;
                }
                if collection[s_x][fy] != 9 && !found.contains_key(&(s_x, fy)) {
                    stack.push_back((s_x, fy));
                    found.insert((s_x, fy), true);
                    basin_size += 1;
                }
                if collection[e_x][fy] != 9 && !found.contains_key(&(e_x, fy)) {
                    stack.push_back((e_x, fy));
                    found.insert((e_x, fy), true);
                    basin_size += 1;
                }
                // for x in stack {
                //     println!("{:?}", x);
                // }
                // panic!("stop");
                stack.pop_front();
            }
            println!("@@@ basin size :{}", basin_size);
            basins.push(basin_size);
            found.clear();
            // stack.clear();
        }
    }

    basins.sort();
    basins.reverse();

    for b in basins.iter() {
        println!("{}", b);
    }

    let size = basins[0] * basins[1] * basins[2];
    println!("multiplied sizes: {}", size);
}

fn main() {
    part1();
    part2();
}
