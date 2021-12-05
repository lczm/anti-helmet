use std::fs;

// not needed...
fn _length(n: i32, base: i32) -> i32 {
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

    // println!("p1: epsilon_rate, {}", epsilon_rate);
    // println!("p1: gamma_rate, {}", gamma_rate);

    let c_epsilon_rate = convert(epsilon_rate);
    let c_gamma_rate = convert(gamma_rate);

    println!("p1: c_epsilon_rate, {}", c_epsilon_rate);
    println!("p1: c_gamma_rate, {}", c_gamma_rate);

    let power_consumption = c_gamma_rate * c_epsilon_rate;

    println!("p1: power_consumption, {}", power_consumption);
}

fn part2() {
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

    let mut oxygen_h_idx: Vec<usize> = (0..height).collect();
    let mut carbon_h_idx: Vec<usize> = (0..height).collect();

    let mut oxygen_r_idx: Vec<usize> = Vec::new();
    let mut carbon_r_idx: Vec<usize> = Vec::new();

    let mut oxygen_zeros = 0;
    let mut oxygen_ones = 0;
    let mut oxygen_zeros_idx: Vec<usize> = Vec::new();
    let mut oxygen_ones_idx: Vec<usize> = Vec::new();

    let mut carbon_zeros = 0;
    let mut carbon_ones = 0;
    let mut carbon_zeros_idx: Vec<usize> = Vec::new();
    let mut carbon_ones_idx: Vec<usize> = Vec::new();

    for i in 0..length {
        for &j in oxygen_h_idx.iter() {
            if items[j][i] == 48 {
                oxygen_zeros += 1;
                oxygen_zeros_idx.push(j);
            } else if items[j][i] == 49 {
                oxygen_ones += 1;
                oxygen_ones_idx.push(j);
            }
        }
        for &j in carbon_h_idx.iter() {
            if items[j][i] == 48 {
                carbon_zeros += 1;
                carbon_zeros_idx.push(j);
            } else if items[j][i] == 49 {
                carbon_ones += 1;
                carbon_ones_idx.push(j);
            }
        }

        carbon_ones_idx.sort();
        carbon_ones_idx.reverse();
        carbon_zeros_idx.sort();
        carbon_zeros_idx.reverse();

        oxygen_ones_idx.sort();
        oxygen_ones_idx.reverse();
        oxygen_zeros_idx.sort();
        oxygen_zeros_idx.reverse();

        // for oxygen remove ones
        // for carbon remove zeros
        if oxygen_zeros > oxygen_ones {
            if oxygen_h_idx.len() > 1 {
                for &e in oxygen_ones_idx.iter() {
                    for k in 0..oxygen_h_idx.len() {
                        if e == oxygen_h_idx[k] {
                            oxygen_r_idx.push(k);
                        }
                    }
                }
                for &e in oxygen_r_idx.iter() {
                    oxygen_h_idx.remove(e);
                }
            }
        } else if oxygen_ones >= oxygen_zeros { // for oxygen remove zeros, carbon remove ones
            if oxygen_h_idx.len() > 1 {
                for &e in oxygen_zeros_idx.iter() {
                    for k in 0..oxygen_h_idx.len() {
                        if e == oxygen_h_idx[k] {
                            oxygen_r_idx.push(k);
                        }
                    }
                }
                for &e in oxygen_r_idx.iter() {
                    oxygen_h_idx.remove(e);
                }
            }
        }

        if carbon_zeros > carbon_ones {
            if carbon_h_idx.len() > 1 {
                for &e in carbon_zeros_idx.iter() {
                    for k in 0..carbon_h_idx.len() {
                        if e == carbon_h_idx[k] {
                            carbon_r_idx.push(k);
                        }
                    }
                }
                for &e in carbon_r_idx.iter() {
                    carbon_h_idx.remove(e);
                }
            }
        } else if carbon_ones >= carbon_zeros {
            if carbon_h_idx.len() > 1 {
                for &e in carbon_ones_idx.iter() {
                    for k in 0..carbon_h_idx.len() {
                        if e == carbon_h_idx[k] {
                            carbon_r_idx.push(k);
                        }
                    }
                }
                for &e in carbon_r_idx.iter() {
                    carbon_h_idx.remove(e);
                }
            }
        }

        // reset all 
        carbon_zeros = 0;
        carbon_ones = 0;
        oxygen_zeros = 0;
        oxygen_ones = 0;
        oxygen_zeros_idx.clear();
        oxygen_ones_idx.clear();
        carbon_zeros_idx.clear();
        carbon_ones_idx.clear();
        oxygen_r_idx.clear();
        carbon_r_idx.clear();
    }

    let mut oxygen_rating: String = String::new();
    let mut carbon_rating: String = String::new();

    for &b in items[oxygen_h_idx[0]].iter() {
        if b == 49 {
            oxygen_rating.push_str("1");
        } else {
            oxygen_rating.push_str("0");
        }
    }

    for &b in items[carbon_h_idx[0]].iter() {
        if b == 49 {
            carbon_rating.push_str("1");
        } else {
            carbon_rating.push_str("0");
        }
    }

    println!("p2: oxygen_rating, {}", oxygen_rating);
    println!("p2: carbon_rating, {}", carbon_rating);
    let c_oxygen_rating = convert(oxygen_rating);
    let c_carbon_rating = convert(carbon_rating);
    println!("p2: oxygen_rating, {}", c_oxygen_rating);
    println!("p2: carbon_rating, {}", c_carbon_rating);
    println!("p2: life support rating, {}", c_oxygen_rating * c_carbon_rating);
}

fn main() {
    part1();
    part2();
}
