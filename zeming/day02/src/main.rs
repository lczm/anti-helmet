use std::fs;

fn part1() {
    let filename = "in1";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let split_content = content.split("\n")
        .filter(|&x| !x.is_empty());

    let mut items: Vec<String> = Vec::new();
    for s in split_content {
        items.push(s.to_string());
    }

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for s in items {
        if s.contains("forward") {
            let i: i32 = s.chars()
                .skip("forward".len() + 1)
                .collect::<String>()
                .parse()
                .unwrap();
            horizontal += i;
        } else if s.contains("down") {
            let i: i32 = s.chars()
                .skip("down".len() + 1)
                .collect::<String>()
                .parse()
                .unwrap();
            depth += i;
        } else if s.contains("up") {
            let i: i32 = s.chars()
                .skip("up".len() + 1)
                .collect::<String>()
                .parse()
                .unwrap();
            depth -= i;
        }
    }

    println!("{}, {}, {}", horizontal, depth, horizontal * depth);
}

fn part2() {
    let filename = "in1";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let split_content = content.split("\n")
        .filter(|&x| !x.is_empty());

    let mut items: Vec<String> = Vec::new();
    for s in split_content {
        items.push(s.to_string());
    }

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for s in items {
        if s.contains("forward") {
            let i: i32 = s.chars()
                .skip("forward".len() + 1)
                .collect::<String>()
                .parse()
                .unwrap();
            horizontal += i;
            depth += aim * i;
        } else if s.contains("down") {
            let i: i32 = s.chars()
                .skip("down".len() + 1)
                .collect::<String>()
                .parse()
                .unwrap();
            aim += i;
        } else if s.contains("up") {
            let i: i32 = s.chars()
                .skip("up".len() + 1)
                .collect::<String>()
                .parse()
                .unwrap();
            aim -= i;
        }
    }

    println!("{}", horizontal * depth);
}

fn main() {
    part1();
    part2();
}
