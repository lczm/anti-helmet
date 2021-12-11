use std::io::{self, BufRead};

fn main() {
    println!("Hello, world!");
    let lines = read_lines();
    let nums = lines.iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let count1 = count_inc(nums.iter());
    let count2 = count_gt(nums.as_slice());
    println!("{}", count1);
    println!("{}", count2);
}

fn read_lines() -> Vec<String> {
    io::stdin().lock().lines()
        .map(|line| line.expect(""))
        .collect::<Vec<_>>()
}

fn count_inc<T>(mut xs: T) -> i32
    where T: Iterator,
          T::Item: PartialOrd
{
    xs.next().map_or(
        0, |first| xs.fold(
            (0, first), |(count, prev), next| (
                if next > prev { count + 1 } else { count }, next)).0)
}

fn count_gt<T>(xs: &[T]) -> i32
    where T: PartialOrd
{
    xs.windows(2).map(|ab| if ab[0] < ab[1] {1} else {0}).sum()
}
