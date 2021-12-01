use std::fs;

fn part1() {
    let filename = "in1";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split_content = content.split("\n")
                        .filter(|&x| !x.is_empty());

    let mut items: Vec<i32> = Vec::new();
    for s in split_content {
        items.push(s.to_string().parse::<i32>().unwrap());
    }

    let mut counter = 0;

    for i in 1..items.len() {
        let i_lower = i - 1;
        if items[i] > items[i_lower] {
            counter += 1;
        }
    }
    println!("Part1: Total measurements larger than previous: {}", counter);
}

fn part2() {
    let filename = "in1";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split_content = content.split("\n")
                        .filter(|&x| !x.is_empty());

    let mut items: Vec<i32> = Vec::new();
    for s in split_content {
        items.push(s.to_string().parse::<i32>().unwrap());
    }

    let mut counter = 0;

    for i in 1..items.len() - 2 {
        let i_lower = i - 1;
        let a = items[i-1] + items[i] + items[i+1];
        let b = items[i] + items[i+1] + items[i+2];
        if b > a {
            counter += 1;
        }
    }
    println!("Part2: Total measurements larger than previous: {}", counter);
}

fn main() {
    part1();
    part2();
}
