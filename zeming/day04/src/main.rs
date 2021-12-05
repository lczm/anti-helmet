use std::fs;

fn part1() {
    let filename = "in1";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split_content = content.split("\n")
        .filter(|&x| !x.is_empty());

    let mut items: Vec<String> = Vec::new();
    for s in split_content {
        items.push(s.to_string());
    }

    let order: Vec<u32> = items[0].split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let og = 1;
    let mut i = og;
    let mut boards: Vec<Vec<Vec<u32>>> = Vec::new();
    let mut boards_tracker: Vec<Vec<Vec<bool>>> = Vec::new();
    while i < items.len() - 1 - og {
        // by right this should be filtered out already
        if items[i].trim().is_empty() {
            i += 1;
            continue;
        }

        let mut board: Vec<Vec<u32>> = Vec::new();
        let mut board_tracker: Vec<Vec<bool>> = Vec::new();
        for _ in 0..5 {
            let b: Vec<u32> = items[i].split_whitespace()
                .map(|s| s.parse().expect("parse error into u32"))
                .collect();
            board.push(b);
            board_tracker.push(vec![false, false, false, false, false]);
            i += 1;
        }
        boards.push(board);
        boards_tracker.push(board_tracker);
    }

    let mut called = 0;
    let mut unmarked_sum = 0;
    let mut test = 0;
    'outer: for o in order {
        for i in 0..boards.len() {
            for j in 0..boards[i].len() {
                for k in 0..boards[i][j].len() {
                    if boards[i][j][k] == o {
                        test += 1;
                        boards_tracker[i][j][k] = true;
                        // can start checking for bingos
                        if test >= 5 {
                            // for every board
                            for z in 0..boards.len() {
                                // check cols
                                for x in 0..5 {
                                    if boards_tracker[z][x][0] && 
                                       boards_tracker[z][x][1] &&
                                       boards_tracker[z][x][2] &&
                                       boards_tracker[z][x][3] &&
                                       boards_tracker[z][x][4] {
                                           // println!("!!!{}", o);
                                           for a in 0..5 {
                                               for b in 0..5 {
                                                   if boards_tracker[z][a][b] == false {
                                                       unmarked_sum += boards[z][a][b];
                                                   }
                                               }
                                           }
                                           called = o;
                                           break 'outer;
                                       }
                                }
                                // check rows
                                for y in 0..5 {
                                    if boards_tracker[z][0][y] && 
                                       boards_tracker[z][1][y] &&
                                       boards_tracker[z][2][y] &&
                                       boards_tracker[z][3][y] &&
                                       boards_tracker[z][4][y] {
                                           // println!("@@@{}", o);
                                           for a in 0..5 {
                                               for b in 0..5 {
                                                   if boards_tracker[z][a][b] == false {
                                                       unmarked_sum += boards[z][a][b];
                                                   }
                                               }
                                           }
                                           called = o;
                                           break 'outer;
                                       }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let score: i64 = ((called as u64 * unmarked_sum as u64)).try_into().unwrap();
    println!("{}", score);
}

fn part2() {
    let filename = "in1";
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split_content = content.split("\n")
        .filter(|&x| !x.is_empty());

    let mut items: Vec<String> = Vec::new();
    for s in split_content {
        items.push(s.to_string());
    }

    let order: Vec<u32> = items[0].split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let og = 1;
    let mut i = og;
    let mut boards: Vec<Vec<Vec<u32>>> = Vec::new();
    let mut boards_tracker: Vec<Vec<Vec<bool>>> = Vec::new();
    while i < items.len() - 1 - og {
        // by right this should be filtered out already
        if items[i].trim().is_empty() {
            i += 1;
            continue;
        }

        let mut board: Vec<Vec<u32>> = Vec::new();
        let mut board_tracker: Vec<Vec<bool>> = Vec::new();
        for _ in 0..5 {
            let b: Vec<u32> = items[i].split_whitespace()
                .map(|s| s.parse().expect("parse error into u32"))
                .collect();
            board.push(b);
            board_tracker.push(vec![false, false, false, false, false]);
            i += 1;
        }
        boards.push(board);
        boards_tracker.push(board_tracker);
    }

    let mut called = 0;
    let mut unmarked_sum = 0;
    let mut test = 0;
    let mut boards_won = 0;
    let mut board_no = 0;
    let mut won_boards: Vec<usize> = Vec::new();
    'outer: for o in order {
        for i in 0..boards.len() {
            for j in 0..boards[i].len() {
                for k in 0..boards[i][j].len() {
                    if boards[i][j][k] == o {
                        test += 1;
                        boards_tracker[i][j][k] = true;
                        // can start checking for bingos
                        if test >= 5 {
                            // for every board
                            'inner1: for z in 0..boards.len() {
                                // check cols
                                'outer2: for x in 0..5 {
                                    if boards_tracker[z][x][0] && 
                                       boards_tracker[z][x][1] &&
                                       boards_tracker[z][x][2] &&
                                       boards_tracker[z][x][3] &&
                                       boards_tracker[z][x][4] ||
                                       boards_tracker[z][0][x] && 
                                       boards_tracker[z][1][x] &&
                                       boards_tracker[z][2][x] &&
                                       boards_tracker[z][3][x] &&
                                       boards_tracker[z][4][x] {
                                           for &w in won_boards.iter() {
                                               if w == z {
                                                   continue 'inner1;
                                               }
                                           }

                                           called = o;
                                           boards_won += 1;
                                           board_no = z;
                                           // println!("board_no: {}", board_no);
                                           // println!("item: {}", boards[i][j][k]);
                                           won_boards.push(board_no);

                                           if won_boards.len() == boards.len() {
                                               for a in 0..5 {
                                                   for b in 0..5 {
                                                       if boards_tracker[z][a][b] == false {
                                                           unmarked_sum += boards[z][a][b];
                                                       }
                                                   }
                                               }
                                               break 'outer;
                                           }
                                       }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let score: u32 = called * unmarked_sum;
    // println!("{}, {}", called, unmarked_sum);
    println!("{}", score);
}

fn main() {
    part1();
    part2();
}
