use std::{io::BufRead, collections::HashSet};

use aoc2019::utils::read_buffer;
use itertools::enumerate;

const CALLS: [i32; 100] = [
    12, 28, 0, 63, 26, 38, 64, 17, 74, 67, 51, 44, 77, 32, 6, 10, 52, 47, 61, 46, 50, 29, 15, 1,
    39, 37, 13, 66, 45, 8, 68, 96, 53, 40, 76, 72, 21, 93, 16, 83, 62, 48, 11, 9, 20, 36, 91, 19,
    5, 42, 99, 84, 4, 95, 92, 89, 7, 71, 34, 35, 55, 22, 59, 18, 49, 14, 54, 85, 82, 58, 24, 73,
    31, 97, 69, 43, 65, 27, 81, 56, 87, 70, 33, 88, 60, 2, 75, 90, 57, 94, 23, 30, 78, 80, 41, 3,
    98, 25, 79, 86,
];

fn main() {
    let input = read_buffer("input/2021/day4.txt");
    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut current_board: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let line_ = line.unwrap();
        if line_.len() == 0 {
            boards.push(current_board);
            current_board = Vec::new();
            continue;
        }
        let row: Vec<i32> = line_.split(" ").map(|x| x.parse().unwrap()).collect();
        current_board.push(row);
    }

    let mut winners: HashSet<(usize, usize)> = HashSet::new();

    for (board_num, rows) in enumerate(&boards) {
        let mut cols: Vec<Vec<i32>> = Vec::new();
        let mut col: Vec<i32> = Vec::new();
        for x in 0..5 {
            for y in 0..5 {
                col.push(rows[y][x]);
            }
            cols.push(col);
            col = Vec::new();    
        }

        let mut win_conds = rows.clone(); win_conds.append(&mut cols);
        let mut calls_tracker = [0; 10]; 

        'popo: for (win, call) in enumerate(CALLS){
            for (i, win_cond) in enumerate(&win_conds) {
                if win_cond.contains(&call){
                    calls_tracker[i] += 1;
                }

                if calls_tracker[i] > 4 {
                    winners.insert((win, board_num));
                    break 'popo;
                }
            }
        }    
    }

    // Change to min for part 1:
    let winning_board = winners.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap();
    let _winning_board = boards[winning_board.1].clone();
    let mut unmarked_nums: HashSet<&i32> = _winning_board.iter().flatten().collect();

    for num in CALLS.to_vec().split_at(winning_board.0).0{
        unmarked_nums.remove(num);
    }
    unmarked_nums.remove(&CALLS[winning_board.0]);
    println!("{:?}, {:?}", _winning_board, unmarked_nums);

    let sum: i32 = unmarked_nums.into_iter().sum();
    println!("Part2: {}", sum*CALLS[winning_board.0])


}
