use std::{io::BufRead, collections::{HashSet}};
use aoc2019::utils::read_buffer;

const DIM: (usize, usize) = (138, 136);
//const DIM: (usize, usize) = (9, 8);


fn main(){
    let input = read_buffer("input/2021/day25.txt").lines();
    let mut bottom_cucumbers = HashSet::<(usize, usize)>::new();
    let mut right_cucumbers = HashSet::<(usize, usize)>::new();

    for (l, line) in input.enumerate() {
        let line_str = line.unwrap();
        for (c, char) in line_str.chars().enumerate() {
            match char {
                'v' => bottom_cucumbers.insert((c, l)),
                '>' => right_cucumbers.insert((c, l)),
                '.' => continue,
                _ => panic!(),
            };
        }
    }

    for y in 0..=DIM.1{
        for x in 0..=DIM.0{
            if bottom_cucumbers.contains(&(x, y)){
                print!("v");
            }
            else if right_cucumbers.contains(&(x, y)){
                print!(">");
            }
            else{
                print!(".");
            }        
        }
        println!("");
    }

    let mut counter = 0;

    loop {
        let mut stopped = true;
        let mut new_bottom_cucumbers = HashSet::<(usize, usize)>::new();
        let mut new_right_cucumbers = HashSet::<(usize, usize)>::new();
        for (x, y) in &right_cucumbers {
            let next;
            if x == &DIM.0 { next = (0, *y); }
            else { next = (x+1, *y); }
            if right_cucumbers.contains(&next) || bottom_cucumbers.contains(&next) {
                new_right_cucumbers.insert((*x, *y));
            }
            else {
                stopped = false;
                new_right_cucumbers.insert(next);
            }
        }
        right_cucumbers = new_right_cucumbers;

        for (x, y) in &bottom_cucumbers {
            let next;
            if y == &DIM.1 { next = (*x, 0); }
            else { next = (*x, y+1); }
            if right_cucumbers.contains(&next) || bottom_cucumbers.contains(&next) {
                new_bottom_cucumbers.insert((*x, *y));
            }
            else {
                stopped = false;
                new_bottom_cucumbers.insert(next);
            }
        }
        bottom_cucumbers = new_bottom_cucumbers;
        if stopped {
            println!("P1: {}", &counter + 1);
            break;
        }
        counter += 1;
    }

}