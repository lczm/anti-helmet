use std::fs;

// not needed...
fn length(n: i32, base: i32) -> i32 {
    let mut pow = base;
    let mut count = 1;
    while n >= pow {
        count += 1;
        pow *= 10;
    }
    return count;
}

fn convert(n: String) -> u32 {
    let base: u32 = 2;
    let mut pow = 0;
    let mut total = 0;
    for i in 0..n.len() {
        let r = n.len() - 1 - i;
        if n.as_bytes()[r] == b'1' {
            total += base.pow(pow);
        }
        pow += 1;
    }
    return total;
}

fn part1() {
    let filename = "in1";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split_content = content.split("\n")
        .filter(|&x| !x.is_empty());

    let mut items: Vec<Vec<u8>> = Vec::new();
    for s in split_content {
        items.push(s.to_string().into_bytes());
    }

    let length = items[0].len();
    let height = items.len();

    println!("{} {}", length, height);

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for i in 0..length {
        let mut zeros = 0;
        let mut ones = 0;
        for j in 0..height {
            if items[j][i] == 48 { // 48 == 0
                zeros += 1;
            } else if items[j][i] == 49 { // 49 == 1
                ones += 1;
            }
        }
        if zeros > ones {
            gamma_rate.push_str("0");
            epsilon_rate.push_str("1");
        } else {
            gamma_rate.push_str("1");
            epsilon_rate.push_str("0");
        }
    }

    println!("epsilon_rate, {}", epsilon_rate);
    println!("gamma_rate, {}", gamma_rate);

    let c_epsilon_rate = convert(epsilon_rate);
    let c_gamma_rate = convert(gamma_rate);

    println!("c_epsilon_rate, {}", c_epsilon_rate);
    println!("c_gamma_rate, {}", c_gamma_rate);

    let power_consumption = c_gamma_rate * c_epsilon_rate;

    println!("power_consumption, {}", power_consumption);
}

fn part2() {
}

fn main() {
    part1();
    part2();
}
