use std::{io::BufRead, collections::HashSet};
use aoc2019::utils::read_buffer;

fn main(){
    part1();
    part2();
}

fn part2(){
    let lines = read_buffer("input/2021/day3.txt").lines();
    let mut candidates_vec: HashSet<Vec<char>> = HashSet::new(); 
    for line in lines{
        let chars: Vec<char> = line.unwrap().chars().collect();
        candidates_vec.insert(chars);
    }

    let a: String = get_code(candidates_vec.clone(), ('1', '0')).iter().collect();
    let b: String = get_code(candidates_vec, ('0', '1')).iter().collect();
    let a_ = i32::from_str_radix(&a.as_str(), 2).unwrap();
    let b_ = i32::from_str_radix(&b.as_str(), 2).unwrap();

    println!("Part 2: {}", a_*b_);

}

fn get_freq(candidates: &HashSet<Vec<char>>) -> [usize; 12]{
    let mut freq = [0; 12];
    for candidate in candidates {
        for (id, char) in candidate.iter().enumerate(){
            if char == &'1' { freq[id] += 1 }
        }
    }
    return freq;
}

fn get_code(candidates_vec_: HashSet<Vec<char>>, code: (char, char)) -> Vec<char>{
    let mut candidates_vec = candidates_vec_.clone();
    let mut freq_array = get_freq(&candidates_vec);
    for pos in 0..12 {
        let to_keep;
        let curr_max = candidates_vec.len() as f32 / 2.0;

        if freq_array[pos] as f32 >= curr_max {to_keep = code.0}  // Equality may be a problem...
        else {to_keep = code.1}
        
        for vec in candidates_vec.clone(){
            if vec[pos] != to_keep {
                candidates_vec.remove(&vec);
            }
        }

        if candidates_vec.len() == 1 { break; }
        freq_array = get_freq(&candidates_vec);

    }

    for candidate in candidates_vec {return candidate;}
    return Vec::new();
}

fn part1(){
    let lines = read_buffer("input/2021/day3.txt").lines();
    let mut freq = [0; 12];
    for line in lines{
        let chars: Vec<char> = line.unwrap().chars().collect();
        for (id, char) in chars.iter().enumerate(){
            if char == &'1' { freq[id] += 1 }
        }
    }
    let most_common: String = freq.iter().map(|x| { if x <= &500 {return '1';} else{return '0'}}).collect();
    let least_common: String = freq.iter().map(|x| { if x >= &500 {return '1';} else{return '0'}}).collect();


    let a = i32::from_str_radix(&most_common.as_str(), 2).unwrap();
    let b = i32::from_str_radix(&least_common.as_str(), 2).unwrap();
    
    println!("Part 1: {}", a*b)
}