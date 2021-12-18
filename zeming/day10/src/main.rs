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
    let s = read_file("in3");
    let score = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);

    for line in s.iter() {
        let mut semi: i32 = 0;
        let mut square: i32 = 0;
        let mut curly: i32 = 0;
        let mut arrow: i32 = 0;

        let mut corrupt_semi: i32 = 0;
        let mut corrupt_square: i32 = 0;
        let mut corrupt_curly: i32 = 0;
        let mut corrupt_arrow: i32 = 0;

        let mut stack: VecDeque<char> = VecDeque::new();

        for c in line.chars() {
            match c {
                '(' => {
                    semi += 1;
                    stack.push_back('(');
                }
                '[' => {
                    square += 1;
                    stack.push_back('[');
                }
                '{' => {
                    curly += 1;
                    stack.push_back('{');
                }
                '<' => {
                    arrow += 1;
                    stack.push_back('<');
                }
                ')' => {
                    if semi > 0 && *stack.front().unwrap() == '(' {
                        semi -= 1;
                    } else {
                        corrupt_semi += 1;
                        break;
                    }
                }
                ']' => {
                    if square > 0 && *stack.front().unwrap() == '[' {
                        square -= 1;
                    } else {
                        corrupt_square += 1;
                        break;
                    }
                }
                '}' => {
                    if curly > 0 && *stack.front().unwrap() == '{' {
                        curly -= 1;
                    } else {
                        corrupt_curly += 1;
                        break;
                    }
                }
                '>' => {
                    if arrow > 0 && *stack.front().unwrap() == '<' {
                        arrow -= 1;
                    } else {
                        corrupt_arrow += 1;
                        break;
                    }
                }
                _ => {
                    panic!("die");
                }
            }
        }

        println!("{} {} {} {}", semi, square, curly, arrow);
        println!("{} {} {} {}", corrupt_semi, corrupt_square, corrupt_curly, corrupt_arrow);
        println!("---");
    }
}

fn part2() {
}

fn main() {
    part1();
    part2();
}
