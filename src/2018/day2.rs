use std::{collections::{HashMap, HashSet}, io::BufRead};
use itertools::zip;

use aoc2019::utils::read_buffer;


fn main(){
    part2();
    part1();
}

fn part2(){
    let input: Vec<String> = read_buffer("input/day2.txt").lines().map(|x| x.unwrap()).collect();
    for id1 in 0..input.len() {
        for id2 in id1+1..input.len(){
            let error_position = compare_ids(input[id1].as_str(), input[id2].as_str());
            if !(error_position == usize::MAX) {
                let mut matching_id = input[id1].to_owned();
                matching_id.remove(error_position);
                println!("P1: {}", matching_id)
            }
        }
    }       
}

fn compare_ids(id1: &str, id2: &str) -> usize {
    let mut error_position = usize::MAX;
    for (index, (char1, char2)) in zip(id1.chars(), id2.chars()).enumerate(){
        if !(char1 == char2) {
            if error_position != usize::MAX {return usize::MAX}
            error_position = index; 
        }
    }
    return error_position
}

fn part1(){
    let mut twos = 0;
    let mut threes = 0;
    let input = read_buffer("input/day2.txt").lines();
    for line in input {
        let values: HashSet<i32> = count_chars(line.unwrap().trim()).into_values().collect();
        if values.contains(&2){ twos += 1 }
        if values.contains(&3){ threes += 1 }
    }

    println!("P1: {}", twos*threes)
}

fn count_chars(sequence: &str) -> HashMap<char, i32>{
    let mut counter = HashMap::new();
    for char in sequence.chars(){
        *counter.entry(char).or_insert(0) += 1;
    }
    return counter;
}