use std::{io::BufRead, collections::{HashMap}};
use aoc2019::utils::read_buffer;

fn main(){
    let points_per_conpletion_char = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let points_per_char = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut line_points: Vec<i64> = Vec::new();
    let mut points = 0;

    'popo: for line in read_buffer("input/2021/day10.txt").lines(){
        let mut expected_chars: Vec<char> = Vec::new();
        let line_ = line.unwrap();
        let chars = line_.chars();

        for char_ in chars{
            if char_ == '(' {expected_chars.push(')');}
            else if char_ == '{' {expected_chars.push('}');}
            else if char_ == '<' {expected_chars.push('>');}
            else if char_ == '[' {expected_chars.push(']');}
            else if char_ != expected_chars.pop().unwrap() {
                points += points_per_char[&char_];
                continue 'popo;
            }
        }

        let line_point:i64 = expected_chars.iter().rev().fold(0, |acc, b| acc*5 + points_per_conpletion_char[b]);
        line_points.push(line_point);
    }

    line_points.sort();

    println!("Part 1: {}", points);
    println!("Part 2: {}", line_points[line_points.len()/2]);

}
