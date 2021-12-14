use std::io::{self, BufRead};

fn main() {
    println!("Hello, world!");
    let lines = read_lines();
    let commands = parse_commands(lines);
    println!("Part 1: {:?}", part1(&commands));
}

fn read_lines() -> Vec<String> {
    io::stdin()
        .lock().lines()
        .map(|line| line.expect("Error"))
        .collect::<Vec<_>>()
}

fn parse_commands(lines: Vec<String>) -> Vec<Command> {
    lines.iter()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|pair| (pair[0], pair[1].parse::<i32>().unwrap()))
        .map(|(label, n)| match label {
            "forward" => Command::Forward(n),
            "up" => Command::Up(n),
            "down" => Command::Down(n),
            _ => panic!()
        })
        .collect::<Vec<_>>()
}

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32)
}

fn part1(commands: &Vec<Command>) -> i32 {
    let mut fwd = 0;
    let mut depth = 0;
    for command in commands {
        match command {
            Command::Forward(n) => fwd += n,
            Command::Up(n) => depth -= n,
            Command::Down(n) => depth += n
        }  
    }
    fwd * depth
}
