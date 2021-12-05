//! 
//! Anti Helmet 
//! Advent of Code
//! Day 3: Binary Diagnostic
//!

use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{stdin, Read};

/// Search for the rating the given numbers using the given bit_criteria.
/// Bit criteria func decides the filter bit given 1 / 0 bit counts.
/// Successfully narrows down numbers using the filter bit selected by the bit_criteria.
/// Returns the last remaining number as the rating.
type Digits = [char];
fn find_rating<'a, F>(pos: usize, nums: &[&'a Digits], bit_criteria: F) -> &'a Digits
where
    F: Fn(usize, usize) -> char,
{
    if pos > nums[0].len() {
        panic!("Out of bounds: Position exceeded length of digits");
    }

    match nums.len() {
        // bad case: no nums given
        0 => panic!("Expected at least one number to be remaining as rating"),
        // base case: only one number: should be the rating
        1 => nums[0],
        _ => {
            // collect digit at pos of every number & calculate no. of 1 and 0 digits.
            let pos_digits: Vec<char> = nums.iter().map(|num| num[pos]).collect();
            let n_ones = pos_digits.iter().filter(|&&c| c == '1').count();
            let n_zeros = pos_digits.len() - n_ones;

            // compute filter bit with bit_criteria func
            let filter_bit = bit_criteria(n_ones, n_zeros);

            // search for numbers that satisfy filter bit
            let satisfy_nums: Vec<_> = nums
                .iter()
                .filter(|num| num[pos] == filter_bit)
                // remove the redundant nested reference via ref deconstruction
                .map(|&num| num)
                .collect();

            find_rating(pos + 1, &satisfy_nums, bit_criteria)
        }
    }
}

fn main() {
    // read input into 2D of digits representing numbers
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input into string.");

    let nums: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let nums_ref: Vec<&[char]> = nums.iter().map(|num| -> &[char] { &num }).collect();
    // find oxygen generator rating
    let o2_rating_str: String = find_rating(0, &nums_ref, |n_ones, n_zeros| {
        // return filter bit for o2 gen rating
        match n_ones.cmp(&n_zeros) {
            Equal | Greater => '1',
            Less => '0',
        }
    })
    .iter()
    .collect();
    let o2_rating = u32::from_str_radix(&o2_rating_str, 2)
        .expect("Failed to parse O2 rating from binary string");

    let co2_rating_str: String = find_rating(0, &nums_ref, |n_ones, n_zeros| {
        // return filter bit for co2 scrubber rating
        match n_ones.cmp(&n_zeros) {
            Less => '1',
            Equal | Greater => '0',
        }
    })
    .iter()
    .collect();
    let co2_rating = u32::from_str_radix(&co2_rating_str, 2)
        .expect("Failed to parse CO2 rating from binary string");

    println!("life support rating: {}", o2_rating * co2_rating);
}
