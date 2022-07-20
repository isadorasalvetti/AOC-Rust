
use std::{io::BufRead, collections::{HashSet, HashMap}};

use aoc2019::utils::read_buffer;

fn main(){
    // Test input
    //Mine
    let algorithm_ = String::from("#######..#.##.##...##.#.#..###..##....######.#.#..#..######.#.#..#####..##.##...#..##........#.#.#...##..##.#####..####.#####..####.#.##.#.#.#.##...##.##.#....###..#...###.#.##..##....##.##.#####..#...#..#....##..##.......##.##....###...#.##...######.##.#######.#.#.#.##.#.#..##.##...##.#.##.#####.#####.###.#....###..###.##.....###..#.##.########..#.#..####..#.###...##...##....##.#.#####..#...##.#..###...##......#.....#.##....##.###..#####..##.###....#..##..##.##.#######.#.##.##.#.####..###.###..#.####..##..");
    
    //Small test
    //let algorithm_=  String::from("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#");
    
    //Big test - 5326
    //let algorithm_ = String::from("#.#.#.#.#......#.#.#.#.##..#.##.##..#..##...#.#.#.#...##.##.##.###....#..#...#.#..###.#...#..##.#.###..#..####.###...#.#.#..##..##.##..##..###..#....#.#....#####.#...###...#.#....###...#..##.##..#..#.##..###..#.##.###..#.####...#.##.....#.###...#.##.##.#.#######...#.###..##..##..#.#.#.#####...#....#.....##.#.#...##.######....#..#......#.#.#.#.##...######.#.#####..#####..#.#.#.#.###.#.#....#..##..#..#.#.#..##....##..#.#.......##...#..####.####.#.#..#.###..#...#......###...#...#.##.#.####..#.#....###.####..#.");

    let algorithm: Vec<char> = algorithm_.chars().collect();
    
    let mut bound: (i32, i32) = (-1, 101);
    
    let mut input_chars = HashMap::<(i32, i32), char>::new();
    for (j, line_) in read_buffer("input/2021/day20.txt").lines().enumerate() {
        let line = line_.unwrap();
        for (i, char_) in line.chars().enumerate(){
            input_chars.insert((i as i32, j as i32), char_);
        }
    }

    let mut default = '.';
    let mut last_default = '.';
    for _ in 0..50{
        bound = (bound.0-1, bound.1+1);
        let mut last_input_chars = input_chars.clone();
        for j in bound.0..bound.1{
            for i in bound.0..bound.1{
                let key = &(i, j);
                let mut num = ['0'; 9];
                for j in -1..=1{
                    for i in -1..=1{
                        let ind = (i+1 + (j+1)*3) as usize;
                        match last_input_chars.get(&(key.0+i, key.1+j)).unwrap_or(&default) {
                            '#' => num[ind] = '1',
                            '.' => num[ind] = '0',
                            _ => panic!()
                        };
                    }
                }
                let bin_num = usize::from_str_radix(&num.iter().collect::<String>(), 2).unwrap();
                *input_chars.entry(*key).or_insert(default) = algorithm[bin_num];
            }
        }
        match default {
            '.' => default = algorithm[0],
            '#' => default = '.',
            _ => panic!()
        }
    }

    let mut count = 0;
        for j in bound.0+1..bound.1-1{
            for i in bound.0+1..bound.1-1{
                if input_chars.contains_key(&(i, j)) && input_chars.get(&(i, j)).unwrap() == &'#' {count += 1;}
                print!("{}", input_chars[&(i, j)])
            }
            print!("\n")
        }
        println!("Part2: {}", count);





}