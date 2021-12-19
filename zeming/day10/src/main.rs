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
    let score = HashMap::from([
        ("(", 3),
        (")", 3),
        ("[", 57),
        ("]", 57),
        ("{", 1197),
        ("}", 1197),
        ("<", 25137),
        (">", 25137)
    ]);

    let mut sum = 0;
    for line in s.iter() {
        let mut semi: i32 = 0;
        let mut square: i32 = 0;
        let mut curly: i32 = 0;
        let mut arrow: i32 = 0;

        let mut corrupt_semi: i32 = 0;
        let mut corrupt_square: i32 = 0;
        let mut corrupt_curly: i32 = 0;
        let mut corrupt_arrow: i32 = 0;
        let mut corrupt = HashMap::from([
            ('(', 0),
            ('[', 0),
            ('{', 0),
            ('<', 0)
        ]);

        let mut stack: VecDeque<char> = VecDeque::new();

        for c in line.chars() {
            match c {
                '(' => {
                    semi += 1;
                    stack.push_front('(');
                }
                '[' => {
                    square += 1;
                    stack.push_front('[');
                }
                '{' => {
                    curly += 1;
                    stack.push_front('{');
                }
                '<' => {
                    arrow += 1;
                    stack.push_front('<');
                }
                ')' => {
                    if semi > 0 && *stack.front().unwrap() == '(' {
                        semi -= 1;
                        stack.pop_front();
                    } else {
                        *corrupt.get_mut(&'(').unwrap() += 1;
                        // println!("got : {:?}", &*stack.front().unwrap());
                        // println!("got : {:?}", &*stack.back().unwrap());
                        break;
                    }
                }
                ']' => {
                    if square > 0 && *stack.front().unwrap() == '[' {
                        square -= 1;
                        stack.pop_front();
                    } else {
                        *corrupt.get_mut(&'[').unwrap() += 1;
                        // println!("got : {:?}", &*stack.front().unwrap());
                        // println!("got : {:?}", &*stack.back().unwrap());
                        break;
                    }
                }
                '}' => {
                    if curly > 0 && *stack.front().unwrap() == '{' {
                        curly -= 1;
                        stack.pop_front();
                    } else {
                        *corrupt.get_mut(&'{').unwrap() += 1;
                        // println!("got : {:?}", &*stack.front().unwrap());
                        // println!("got : {:?}", &*stack.back().unwrap());
                        break;
                    }
                }
                '>' => {
                    if arrow > 0 && *stack.front().unwrap() == '<' {
                        arrow -= 1;
                        stack.pop_front();
                    } else {
                        *corrupt.get_mut(&'<').unwrap() += 1;
                        // println!("got : {:?}", &*stack.front().unwrap());
                        // println!("got : {:?}", &*stack.back().unwrap());
                        break;
                    }
                }
                _ => {
                    panic!("die");
                }
            }
        }

        for (k, v) in &corrupt {
            if *v != 0 {
                // println!("{}", score.get(&k.to_string() as &str).unwrap() * v);
                sum += score.get(&k.to_string() as &str).unwrap() * v;
            }
        }
    }
    println!("part 1 sum: {}", sum);
}

fn part2() {
}

fn main() {
    part1();
    part2();
}
