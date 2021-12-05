//!
//! Anti Helmet
//! Advent of Code
//! Day 4: Giant Squid
//!

use std::cell::{Ref, RefCell};
use std::io::{stdin, Read};

/// Defines a 5 by 5 bingo board
struct Board {
    nums: [[u32; 5]; 5],
    marks: [[bool; 5]; 5],
}

impl Board {
    /// Construct a bingo board given 25 nums
    fn new(nums: &[u32]) -> Self {
        if nums.len() != 25 {
            panic!("Expected 25 numbers per bingo board constructed.");
        }

        let mut board = Board {
            nums: [[0; 5]; 5],
            marks: [[false; 5]; 5],
        };

        for (i, &num) in nums.iter().enumerate() {
            let (row, col) = (i / 5, i % 5);
            board.nums[row][col] = num;
        }

        board
    }

    /// Marks the given number on this bingo board
    fn mark(&mut self, num: u32) {
        for r in 0..5 {
            for c in 0..5 {
                if self.nums[r][c] == num {
                    self.marks[r][c] = true;
                }
            }
        }
    }

    /// Check if the given bingo board has satisfied the winning condition of all
    /// elements in a row being marked or all elements in a column being marked.
    /// Returns true if the winning condition is satisfied, false otherwise.
    fn wins(&self) -> bool {
        // check for wins by row
        for row in 0..5 {
            if self.marks[row].iter().all(|m| *m) {
                return true;
            }
        }
        // check for wins by column
        for col in 0..5 {
            if self
                .marks
                .iter()
                .map(|&col_marks| col_marks[col])
                .all(|m| m)
            {
                return true;
            }
        }
        // no win
        false
    }

    /// Compute & Returns the score attained given by this board if it wins.
    /// Uses the sum of all umarked numbers on the board & the last drawn number.
    fn score(&self, last_drawn: u32) -> u32 {
        let unmarked_sum: u32 = (self.nums.iter().flatten())
            .zip(self.marks.iter().flatten())
            .filter(|(_, &mark)| !mark)
            .map(|(num, _)| num)
            .sum();

        unmarked_sum * last_drawn
    }
}

/// Represents a bingo game win
struct Win<'a> {
    board: Ref<'a, Board>,
    n_marks: u32,
    last_drawn: u32,
}

fn main() {
    // read drawn numbers from stdin
    let mut drawn_nums = String::new();
    stdin()
        .read_line(&mut drawn_nums)
        .expect("Failed to read drawn numbers");
    let drawn_nums: Vec<u32> = drawn_nums
        .split(",")
        .map(|s| s.trim().parse().expect("Failed to parse drawn number"))
        .collect();

    // read all bingo board's numbers from stin
    let mut board_nums = String::new();
    stdin()
        .read_to_string(&mut board_nums)
        .expect("Failed to read board numbers");
    let board_nums: Vec<u32> = board_nums
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse board number"))
        .collect();

    // parse bingo board's nums into boards
    const N_NUMS_BOARD: usize = 5 * 5; // no. of numbers in each board
    let boards: Vec<RefCell<Board>> = (0..board_nums.len())
        .step_by(N_NUMS_BOARD)
        .map(|begin| RefCell::new(Board::new(&board_nums[begin..begin + N_NUMS_BOARD])))
        .collect();

    // play bingo simulation using boards
    let Win {
        board, last_drawn, ..
    } = boards
        .iter()
        // flat map will collapse all None options, leaving us with wins only.
        .flat_map(|board| {
            for (i, &num) in drawn_nums.iter().enumerate() {
                // mark board with drawn numbers
                // block required to contain board mutable reference scope.
                {
                    let mut board = board.borrow_mut();
                    board.mark(num);
                }

                // bingo board wins: record down no. of plays to win & last drawn num.
                if board.borrow().wins() {
                    return Some(Win {
                        board: board.borrow(),
                        n_marks: i as u32 + 1,
                        last_drawn: num,
                    });
                }
            }
            None
        })
        // find the bingo board win that take the most marks to win
        .max_by_key(|win| win.n_marks)
        .expect("Expected at least one winning board.");
    // score all winning boards to find last winning board
    println!("Last winning board score: {}", board.score(last_drawn));
}
