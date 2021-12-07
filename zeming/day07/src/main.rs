use std::fs;

fn part1() {
    let filename = "in1";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut items: Vec<i64> = content.split(",")
        .map(|s| s.trim())
        .map(|s| s.parse::<i64>().expect("parse into i64 error"))
        .collect();

    items.sort();
    let median = items.len() / 2;
    let v = items[median];
    let mut moves = 0;
    for i in items {
        moves += (i - v).abs();
    }
    println!("{}", moves);
}

fn part2() {
    let filename = "in1";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut items: Vec<i128> = content.split(",")
        .map(|s| s.trim())
        .map(|s| s.parse::<i128>().expect("parse into i64 error"))
        .collect();

    let mut moves = Vec::new();
    for i in items.iter() {
        let mut s = 0;
        for j in items.iter() {
            let diff = (j - i).abs();
            s += (1..diff + 1).sum::<i128>()
        }
        moves.push(s);
    }

    println!("{}", moves.iter().min().unwrap());
}

fn main() {
    part1();
    part2();
}
