use std::{io::BufRead, collections::{HashSet}};
use aoc2019::utils::read_buffer;

fn main(){
    let mut dots: HashSet<(i32, i32)> = HashSet::new();
    let mut dots_size: (i32, i32) = (0, 0);

    for line in read_buffer("input/2021/day13.txt").lines(){
        let line_ = line.unwrap();
        let nums: Vec<_> = line_.split(",").collect();
        let d0 = nums[0].parse().unwrap();
        let d1 = nums[1].parse().unwrap();
        dots.insert((d0, d1));

        if d0 > dots_size.0 { dots_size.0 = d0; }
        if d1 > dots_size.1 { dots_size.1 = d1; }
    }


    //let mock_folds = [(0, 7), (5, 0)];
    let folds = [(655, 0), (0, 447), (327, 0), (0, 223), (163, 0), (0, 111), (81, 0), (0, 55), (40, 0), (0, 27), (0, 13), (0, 6)];
    for point in folds {     
        if point.0 != 0 { 
            for dot in dots.clone() {
                if dot.0 > point.0 {
                    dots.remove(&dot);
                    dots.insert((dots_size.0 - dot.0, dot.1));
                }
            }
            
            dots_size.0 = point.0-1;
        }
        else { 
            for dot in dots.clone() {
                if dot.1 > point.1 {
                    dots.remove(&dot);
                    dots.insert((dot.0, dots_size.1 - dot.1));
                }
            }

            dots_size.1 = point.1-1; 
        }
        if point == (655, 0) { println!("Part1: {}", dots.len()); }

    }
    
    println!("Part2:");
    for j in -1..=dots_size.1+1{
        for i in -2..dots_size.0+2{
            if dots.contains(&(i, j)) { print!("█"); }
            else { print!("."); }
        }
        print!("\n");
    }

}