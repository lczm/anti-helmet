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
        .map(|pair| Command { 
            label: pair[0].to_string(),
            n: pair[1].to_string().parse().unwrap() })
        .collect::<Vec<_>>()
}

struct Command {
    label: String,
    n: i32
}

fn part1(commands: &Vec<Command>) -> i32 {
    fn sum_over(commands: &Vec<Command>, label: &str) -> i32 {
        commands.iter()
            .filter(|cmd| cmd.label == label)
            .map(|cmd| cmd.n)
            .sum()
    }
    let fwd = sum_over(commands, "forward");
    let up = sum_over(commands, "up");
    let down = sum_over(commands, "down");
    println!("{}, {}, {}", fwd, up, down);
    fwd * (down - up)
}
