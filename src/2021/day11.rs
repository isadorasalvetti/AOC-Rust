use std::{io::BufRead, collections::{HashMap, HashSet}};
use itertools::enumerate;
use aoc2019::utils::read_buffer;

fn main(){
    let mut flashes = 0;
    let mut octopus: HashMap<(i32, i32), u32> = HashMap::new();

    for (y, line) in enumerate(read_buffer("input/2021/day11.txt").lines()){
        let line_ = line.unwrap();
        for (x, char) in enumerate(line_.chars()){
            octopus.insert((x as i32, y as i32), char.to_digit(10).unwrap());
        }
    }

    for _i in 0..1000 {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut candidates: Vec<(i32, i32)> = octopus.clone().into_keys().collect();

        while candidates.len() > 0{
            let (x, y) = candidates.pop().unwrap();
            if visited.contains(&(x, y)) || !octopus.contains_key(&(x, y)) { continue; }

            let dumbo = octopus.get_mut(&(x, y)).unwrap();

            if dumbo == &9 {
                flashes += 1;
                *dumbo = 0;

                candidates.push((x, y));
                candidates.push((x+1, y));
                candidates.push((x+1, y-1));
                candidates.push((x+1, y+1));
                candidates.push((x, y+1));
                candidates.push((x-1, y));
                candidates.push((x-1, y-1));
                candidates.push((x-1, y+1));
                candidates.push((x, y-1));

                visited.insert((x, y));
            }
            else {
                *dumbo += 1;
            }
        }
        if _i == 99 {println!("Part1: {}", flashes);}
        if octopus.values().all(|x| x == &0) {println!("Part2: {}", _i + 1); break;}
    }
    
        
}