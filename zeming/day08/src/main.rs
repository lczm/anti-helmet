use std::fs;

fn part1() {
    let filename = "in1";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut lines: Vec<String> = content.split("\n")
        .filter(|&s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect();

    let mut counter = 0;

    for l in lines {
        let delimiter_split: Vec<&str> = l
            .split("|")
            .collect();
        let display_split: Vec<&str> = delimiter_split[1]
            .split(" ")
            .collect();
        for d in display_split {
            if d.len() == 2 ||
               d.len() == 3 ||
               d.len() == 4 ||
               d.len() == 7 {
                   counter += 1;
               }
        }
    }
    println!("total times 1/4/7/8/ appeared is: {}", counter);
}

fn part2() {
}

fn main() {
    part1();
    part2();
}
