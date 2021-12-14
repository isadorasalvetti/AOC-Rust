use std::io::BufRead;

use aoc2019::utils::read_buffer;

fn main(){
    part1();
    part2();
}

fn part1(){
    let input = read_buffer("input/2021/day2.txt").lines();
    let mut forward = 0;
    let mut down = 0;

    for line in input {
        let line_str = line.unwrap();
        let split: Vec<&str> = line_str.split(" ").collect();
        match split[0] {
            "forward" => forward += split[1].parse::<i32>().unwrap(),
            "down" => down += split[1].parse::<i32>().unwrap(),
            "up" => down -= split[1].parse::<i32>().unwrap(),
            _ => panic!()
        }
    }

    println!("Part1: {}", forward*down)
}

fn part2(){
    let input = read_buffer("input/2021/day2.txt").lines();
    let mut forward = 0;
    let mut down = 0;
    let mut aim = 0;

    for line in input {
        let line_str = line.unwrap();
        let split: Vec<&str> = line_str.split(" ").collect();
        match split[0] {
            "forward" => {
                let forward_val = split[1].parse::<i32>().unwrap();
                forward += forward_val;
                down += forward_val * aim;
            },
            "down" => aim += split[1].parse::<i32>().unwrap(),
            "up" => aim -= split[1].parse::<i32>().unwrap(),
            _ => panic!()
        }
    }

    println!("Part2: {}", forward*down)
}