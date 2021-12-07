use std::fs;

fn part1() {
    let filename = "in1";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut items: Vec<u32> = content.split(",")
        .map(|s| s.trim())
        .map(|s| s.parse::<u32>().expect("parse into u32 error"))
        .collect();

    let days = 80;
    let c_days = 0;
    for _ in c_days..days {
        let mut i = 0;
        let len = items.len();
        while (i != len) {
            let e = items[i];
            if e == 0 {
                items[i] = 6;
                items.push(8);
            } else {
                items[i] -= 1;
            }
            i += 1;
        }
    }

    // for i in items {
    //     println!("{}", i);
    // }
    println!("{}", items.len());
}

fn part2() {
}

fn main() {
    part1();
    part2();
}
