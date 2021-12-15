use std::fs;
use std::collections::HashMap;

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
    let filename = "in1";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut lines: Vec<String> = content.split("\n")
        .filter(|&s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect();

    let mut sum = 0;

    // length to number mappings are:
    // len 2 ==> 1
    // len 3 ==> 7
    // len 4 ==> 4
    // len 5 ==> (2 | 3 | 5)
    // len 6 ==> (0 | 6 | 9)
    // len 7 ==> 8

    for l in lines {
        let mut delimiter_split: Vec<&str> = l
            .split("|")
            .collect();
        let mut guess_split: Vec<&str> = delimiter_split[0]
            .trim()
            .split(" ")
            .collect();
        let mut display_split: Vec<&str> = delimiter_split[1]
            .trim()
            .split(" ")
            .collect();

        // TODO : get 2/3/4/7 at the start of the list
        // this will sort by the length of the strings
        // this should sort by the lengths and arrange them in
        // 2, 3, 4, 7 order
        // total order should be
        // 2, 3, 4, 7, 5{2, 3, 5}, 6{0, 6, 9} where in {} is random
        
        // for s in guess_split.iter() {
        //     println!("{}", s);
        // }
        
        guess_split.sort_by(|a, b| a.len().cmp(&b.len()));
        // println!("---");

        let temp = guess_split[guess_split.len() - 1].clone();
        let lenn = guess_split.len() - 1;
        guess_split[lenn] = guess_split[3];
        guess_split[3] = temp;

        for s in guess_split.iter() {
            println!("{}", s);
        }

        let mut map: HashMap<i32, String> = HashMap::new();
        let mut index_0 = 123;
        let mut index_1 = 123;
        let mut index_2 = 123;
        let mut index_3 = 123;
        let mut index_4 = 123;
        let mut index_5 = 123;
        let mut index_6 = 123;
        let mut index_7 = 123;
        let mut index_8 = 123;
        let mut index_9 = 123;
        for i in 0..guess_split.len() {
            match guess_split[i].len() {
                2 => {
                    let mut c: Vec<char> = guess_split[i].chars().collect();
                    c.sort();
                    let cs: String = c.into_iter().collect();
                    map.insert(2, cs);
                    index_1 = i;
                    // println!("@@@ 2 appeared");
                }
                3 => {
                    let mut c: Vec<char> = guess_split[i].chars().collect();
                    c.sort();
                    let cs: String = c.into_iter().collect();
                    map.insert(3, cs);
                    index_7 = i;
                    // println!("@@@ 3 appeared");
                }
                4 => {
                    let mut c: Vec<char> = guess_split[i].chars().collect();
                    c.sort();
                    let cs: String = c.into_iter().collect();
                    map.insert(4, cs);
                    index_4 = i;
                    // println!("@@@ 4 appeared");
                }
                7 => {
                    let mut c: Vec<char> = guess_split[i].chars().collect();
                    c.sort();
                    let cs: String = c.into_iter().collect();
                    map.insert(8, cs);
                    index_8 = i;
                    // println!("@@@ 7 appeared");
                }
                // Need to mainly figure out 5 and 6
                5 => {
                    // 3 is the only one which will have "all" the letters of 1
                    let e: Vec<char> = guess_split[index_1].chars().collect();
                    let f: Vec<char> = guess_split[i].chars().collect();
                    let mut c = 0;
                    for j in 0..f.len() {
                        for k in 0..e.len() {
                            if f[j] == e[k] {
                                c += 1;
                            }
                        }
                    }
                    if c == e.len() {
                        index_3 = i;
                    }

                    // Compare against 4, 2 will be the only number that has
                    // 2 out of the 4 sticks of 4
                    c = 0;
                    let e2: Vec<char> = guess_split[index_4].chars().collect();
                    for j in 0..f.len() {
                        for k in 0..e2.len() {
                            if f[j] == e2[k] {
                                c += 1;
                            }
                        }
                    }
                    if c == 2 {
                        index_2 = i;
                    }
                }
                6 => {
                    // 9 will have all of '4'
                    let e: Vec<char> = guess_split[index_4].chars().collect();
                    let f: Vec<char> = guess_split[i].chars().collect();
                    let mut c = 0;
                    for j in 0..f.len() {
                        for k in 0..e.len() {
                            if f[j] == e[k] {
                                c += 1;
                            }
                        }
                    }
                    if c == e.len() {
                        index_9 = i;
                    }

                    // Compare against 1, 6 will be the only number that has
                    // only 1 out of the 2 sticks of 1
                    c = 0;
                    let e2: Vec<char> = guess_split[index_1].chars().collect();
                    for j in 0..f.len() {
                        for k in 0..e2.len() {
                            if f[j] == e2[k] {
                                c += 1;
                            }
                        }
                    }
                    if c == 1 {
                        index_6 = i;
                    }
                }
                _ => {
                    println!("die");
                    println!("{}", guess_split[i]);
                    panic!("die");
                }
            }
        }

        // remainders:
        // 5 (len-5, alone)
        // 0 (len-6, alone)
        for i in 0..guess_split.len() {
            if guess_split[i].len() == 5 {
                if i == index_2 || i == index_3 {
                    continue;
                } else {
                    index_5 = i;
                }
            }
            if guess_split[i].len() == 6 {
                if i == index_6 || i == index_9 {
                    continue;
                } else {
                    index_0 = i;
                }
            }
        }

        // println!("{}", index_0);
        // println!("{}", index_1);
        // println!("{}", index_2);
        // println!("{}", index_3);
        // println!("{}", index_4);
        // println!("{}", index_5);
        // println!("{}", index_6);
        // println!("{}", index_7);
        // println!("{}", index_8);
        // println!("{}", index_9);

        let mut display_str = String::new();
        for i in 0..display_split.len() {
            match display_split[i].len() {
                2 => {
                    display_str.push('1');
                }
                3 => {
                    display_str.push('7');
                }
                4 => {
                    display_str.push('4');
                }
                7 => {
                    display_str.push('8');
                }
                5 => {
                    let mut a: Vec<char> = display_split[i].chars().collect();
                    a.sort();
                    // index 2, 3, 5
                    let mut b: Vec<char> = guess_split[index_2].chars().collect();
                    b.sort();
                    let mut c: Vec<char> = guess_split[index_3].chars().collect();
                    c.sort();
                    let mut d: Vec<char> = guess_split[index_5].chars().collect();
                    d.sort();
                    if a == b {
                        display_str.push('2');
                    } else if a == c {
                        display_str.push('3');
                    } else if a == d {
                        display_str.push('5');
                    }
                }
                6 => {
                    let mut a: Vec<char> = display_split[i].chars().collect();
                    a.sort();
                    // index 0, 6, 9
                    let mut b: Vec<char> = guess_split[index_0].chars().collect();
                    b.sort();
                    let mut c: Vec<char> = guess_split[index_6].chars().collect();
                    c.sort();
                    let mut d: Vec<char> = guess_split[index_9].chars().collect();
                    d.sort();
                    if a == b {
                        display_str.push('0');
                    } else if a == c {
                        display_str.push('6');
                    } else if a == d {
                        display_str.push('9');
                    }
                }
                _ => {
                    panic!("die");
                }
            }
        }
        let display_int = display_str.parse::<i32>().unwrap();
        println!("display: {}", display_int);
        sum += display_int;
    }

    println!("sum: {}", sum);
}

fn main() {
    part1();
    part2();
}
