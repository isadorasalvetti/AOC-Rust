use std::{collections::{HashSet}, io::BufRead};

use regex::Regex;
use aoc2019::utils::read_buffer;

fn main(){
    part1n2();
}

struct Coord{
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

fn get_coords(lines: impl Iterator<Item=String>) -> Vec<Coord>{
    let re = Regex::new(r"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)").unwrap();
    let mut coords: Vec<Coord> = Vec::new();
    for line in lines{
        let text = line.as_str();
        let cap = re.captures(text).unwrap();
        let coord = Coord {
            x: cap["x"].parse().unwrap(),
            y: cap["y"].parse().unwrap(), 
            w: cap["w"].parse().unwrap(), 
            h: cap["h"].parse().unwrap(),};
        coords.push(coord);
    }
    return coords;
}

fn part1n2(){
    let mut claimed_squares: HashSet<(i32, i32)> = HashSet::new();
    let mut overlapped_squares: HashSet<(i32, i32)> = HashSet::new();
    let coords = get_coords(read_buffer("input/day3.txt").lines().map(|x| x.unwrap()));

    for c in &coords {
        for i in c.x..c.x+c.w {
            for j in c.y..c.y+c.h {
                let point = (i, j);
                if claimed_squares.contains(&point){ 
                    overlapped_squares.insert(point);
                    continue; 
                }
                claimed_squares.insert(point);
            } 
        }
    }
    println!("Part1: {}", overlapped_squares.len()); // id starts at 1

    for (id, c) in coords.iter().enumerate() {
        let mut is_overlap = false;
        for i in c.x..c.x+c.w {
            for j in c.y..c.y+c.h {
                if overlapped_squares.contains(&(i, j)){
                    is_overlap = true;
                }
            } 
            if is_overlap {break}
        }
        if !is_overlap {
            println!("Part2: {}", id+1); // id starts at 1
            break
        }
    }
}

// fn part1(){
//     let mut contested_spots: HashSet<(i32, i32)> = HashSet::new();
//     let mut claimed_squares: HashSet<(i32, i32)> = HashSet::new();
//     let re = Regex::new(r"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)").unwrap();
//     for line in read_buffer("input/day3.txt").lines(){
//         let line_str = line.unwrap();
//         let caps = re.captures(&line_str.as_str()).unwrap();
        
//         let x:i32 = caps["x"].parse().unwrap();
//         let y:i32 = caps["y"].parse().unwrap();
//         let w:i32 = caps["w"].parse().unwrap();
//         let h:i32 = caps["h"].parse().unwrap();

//         for i in x..x+w {
//             for j in y..y+h {
//                 let mt = (i, j);
//                 if claimed_squares.contains(&mt){
//                     contested_spots.insert(mt);
//                     continue;
//                 }
//                 claimed_squares.insert(mt);
//             } 
//         }
//         //println!("{}: {}, {}, {}, {}", &caps[0], &caps["x"], &caps["y"], &caps["w"], &caps["h"])
//     }
//     println!("Part1: {}", contested_spots.len())
// }


