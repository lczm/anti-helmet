use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let lines = lines.map(|line| line.expect("Cannot read line"));
    for line in lines {
        println!("{}", line);
    }
}
