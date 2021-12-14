use std::{io::BufRead, collections::{HashMap, HashSet}, path};
use aoc2019::utils::read_buffer;

fn main(){
    let mut cave_connections: HashMap<char, HashSet<char>> = HashMap::new();
    let start_end = HashMap::from([("start", '0'), ("end", '1')]);

    for line in read_buffer("input/2021/day12.txt").lines(){
        let line_ = line.unwrap();
        let caves: Vec<&str> = line_.split("-").collect();

        let cv1;
        let cv2;

        if caves[0].len() > 2 {cv1 = start_end[caves[0]]}
        else {cv1 = caves[0].chars().next().unwrap();}
        if caves[1].len() > 2 {cv2 = start_end[caves[1]]}
        else {cv2 = caves[1].chars().next().unwrap();}

        if !cave_connections.contains_key(&cv1) { cave_connections.insert(cv1, HashSet::new());}
        if !cave_connections.contains_key(&cv2) { cave_connections.insert(cv2, HashSet::new());}

        cave_connections.get_mut(&cv1).unwrap().insert(cv2);
        cave_connections.get_mut(&cv2).unwrap().insert(cv1);
    }

    let mut paths = 0;
    let small_caves_allowed = 1;
    let explored = HashSet::from(['0']);
    let previous_caves = Vec::from(['0']);
    build_path(&'0', &cave_connections, explored, &mut paths, &previous_caves, &small_caves_allowed);

    println!("{}", paths);

}

fn build_path(current_cave: &char, cave_connections: &HashMap<char, HashSet<char>>, explored: HashSet<char>, paths: &mut i32, previous_caves: &Vec<char>, small_caves_allowed: &i32){
    for cave in cave_connections.get(&current_cave).unwrap(){
        let mut previous_caves_ = previous_caves.clone();
        let mut small_caves_allowed_ = small_caves_allowed.clone();
        previous_caves_.push(*cave);

        if explored.contains(cave) { 
            if small_caves_allowed_ > 0 && cave != &'0' { small_caves_allowed_ -= 1; }
            else { continue; }
        }
        let mut explored_ = explored.clone();
        if cave.is_lowercase(){ explored_.insert(*cave); }
        if cave == &'1' { 
            //println!("{:?}", &previous_caves_);
            *paths += 1; 
            continue; 
        }
        build_path(cave, cave_connections, explored_, paths, &previous_caves_, &small_caves_allowed_);
    }
}