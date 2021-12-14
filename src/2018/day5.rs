use std::io::BufRead;
use aoc2019::utils::read_buffer;

fn main(){
    let lines = read_buffer("input/2018/day5mock.txt").lines();
    for line in lines{
        let mut chars: Vec<char> = line.unwrap().chars().collect();
        chars = remove_instabilities(chars);
        println!("Part1: {:?}", &chars.len());

        let alphabet = (b'a'..=b'z')           // Start as u8
        .map(|c| c as char)            // Convert all to chars
        .filter(|c| c.is_alphabetic()); // Filter only alphabetic chars

        let mut results: [usize; 26] = [0; 26];
        for (id, char) in alphabet.enumerate() {
            let filtered_chars = chars.to_owned().into_iter().filter(|c| *c != char && *c != char.to_uppercase().next().unwrap()).collect();
            results[id] = remove_instabilities(filtered_chars).len();
        }

        //println!("{:?}", results);
        println!("Part2: {:?}", results.iter().min());

    }
}

fn remove_instabilities(mut chars: Vec<char>) -> Vec<char>{
    loop {
        let mut skip = false;
        let mut remove_ids: Vec<usize> = Vec::new();
        for (id, char) in chars.iter().enumerate(){
            if id+1 >= chars.len() { break; }
            if skip { skip = false; continue;}
            if char.is_uppercase(){
                if chars[id+1] == char.to_lowercase().next().unwrap(){
                    remove_ids.push(id);
                    remove_ids.push(id+1);
                    skip = true;
                }
            }
            else if char.is_lowercase(){
                if chars[id+1] == char.to_uppercase().next().unwrap(){
                    remove_ids.push(id);
                    remove_ids.push(id+1);
                    skip = true;
                }
            }
        }
        
        if remove_ids.len() == 0 { break; }
        
        //println!("Removing {:?} characters.", remove_ids.len());
        remove_ids.reverse();

        for to_remove in &remove_ids{
            chars.remove(*to_remove);
        }
    }
    return chars;
}