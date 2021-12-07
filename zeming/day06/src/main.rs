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
    let filename = "in1";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut items: Vec<u128> = content.split(",")
        .map(|s| s.trim())
        .map(|s| s.parse::<u128>().expect("parse into u32 error"))
        .collect();

    let mut v: Vec<u128> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    for i in items {
        v[i as usize] += 1;
    }

    for i in 0..256 {
        let popped = v.remove(0);
        v.push(popped);
        v[6] += v[8];
    }

    let mut count:u128 = 0;
    for i in v {
        count += i;
    }

    println!("{}", count);
}

fn main() {
    part1();
    part2();
}
