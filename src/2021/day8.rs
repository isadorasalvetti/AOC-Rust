use std::{io::BufRead, collections::{HashMap, HashSet}};
use aoc2019::utils::read_buffer;
use itertools::{Itertools, enumerate};

fn main(){
    let mut vals: Vec<Vec<String>> = Vec::new();
    let mut obs: Vec<Vec<String>> = Vec::new();

    let input_buffer = read_buffer("input/2021/day8.txt").lines();
    for line in input_buffer{
        let line_str = line.unwrap();
            let parts: Vec<&str> = line_str.split("|").collect();
            let line_vals: Vec<String> = parts[1].split_whitespace().map(|x| x.to_string()).collect();
            vals.push(line_vals);
            let line_obs: Vec<String> = parts[0].split_whitespace().map(|x| x.to_string()).collect();
            obs.push(line_obs);
    }

    // 1, 7, 4, (2, 5, 3), (0, 6, 9), 8
    // "cf", "acf", "bdcf", "acdeg", "abdfg", "acdfg" "abcefg", "abdefg", "abcdfg", "abcdefg"

    let all_chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    let mut val_sum = 0;        
    for i in 0..vals.len(){
        let mut digit_candidates: Vec<Vec<char>> = Vec::new();
        digit_candidates.resize(7, Vec::new());

        obs[i].sort_by(|a, b| a.len().cmp(&b.len()));
        
        // 1
        digit_candidates[2] = obs[i][0].chars().collect();
        digit_candidates[5] = obs[i][0].chars().collect();

        // 7
        digit_candidates[0] = obs[i][1].chars().filter(|x| !digit_candidates[2].contains(x)).collect();

        // 4
        let digits_bd: Vec<char> = obs[i][2].chars().filter(|x| !digit_candidates[2].contains(x)).collect();
        let digits_eg: Vec<char> = all_chars.clone().into_iter().filter(|x| !obs[i][2].chars().contains(x) && !digit_candidates[0].contains(x)).collect();
        digit_candidates[1] = digits_bd.clone();
        digit_candidates[3] = digits_bd;
        digit_candidates[4] = digits_eg.clone();
        digit_candidates[6] = digits_eg;

        // (0, 6, 9)
        let digits_0 = obs[i][6].chars();
        let digits_1 = obs[i][7].chars();
        let digits_2 = obs[i][8].chars();
        let digits_cde: Vec<char> = all_chars.clone().into_iter().filter(|x| (digits_0.clone().contains(x) as usize +
                                                                                    digits_1.clone().contains(x) as usize +
                                                                                    digits_2.clone().contains(x) as usize) == 2
                                                                        ).collect();
        digit_candidates[2] = digits_cde.clone().into_iter().filter(|x| digit_candidates[2].contains(x)).collect();
        digit_candidates[3] = digits_cde.clone().into_iter().filter(|x| digit_candidates[3].contains(x)).collect();
        digit_candidates[4] = digits_cde.clone().into_iter().filter(|x| digit_candidates[4].contains(x)).collect();

        for i in 0..digit_candidates.len(){
            if digit_candidates[i].len() == 1 {
                let sure_digit = digit_candidates[i][0];
                for j in 0..digit_candidates.len() {
                    if i != j { digit_candidates[j].retain(|&x| x != sure_digit); }
                }
            }
        }

        let nums_by_len = [-1, -1, 1, 7, 4, -1, -1, 8];

        // 0 1 2 3 4 5 6
        // a b c d e f g

        // Values
        for (pos, d) in enumerate(&vals[i]){
            let mut num = nums_by_len[d.len()];
            if num < 0 {
                if d.len() == 5 {
                    if d.contains(digit_candidates[2][0]) && d.contains(digit_candidates[5][0]) {
                        num = 3;
                    }
                    else if d.contains(digit_candidates[2][0]){
                        num = 2;
                    }
                    else if d.contains(digit_candidates[5][0]){
                        num = 5;
                    }
                }

                if d.len() == 6 {
                    if !d.contains(digit_candidates[4][0]) {
                        num = 9;
                    }
                    else if !d.contains(digit_candidates[2][0]){
                        num = 6;
                    }
                    else if !d.contains(digit_candidates[3][0]){
                        num = 0;
                    }
                }
            }
            //println!("{}, {}", num, i32::pow(10, 3-(pos as u32)));
            val_sum += num*(i32::pow(10, 3-(pos as u32)))
        }
    }
    println!("{}", val_sum)
}

fn part1() {
    let mut vals: Vec<String> = Vec::new();

    let input_buffer = read_buffer("input/2021/day8.txt").lines();
    for line in input_buffer{
        let line_str = line.unwrap();
            let parts: Vec<&str> = line_str.split("|").collect();
            let mut line_vals: Vec<String> = parts[1].split_whitespace().map(|x| x.to_string()).collect();
            vals.append(&mut line_vals);

    let looking_for: [usize; 4] = [2, 4, 3, 7];
    let res: Vec<&String> = vals.iter().filter(|x| looking_for.contains(&x.len())).collect();

    println!("{:?}", res.len());
    }
}

