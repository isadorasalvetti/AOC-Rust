use std::{io::BufRead, collections::HashSet};
use std::cmp::{max, min};
use itertools::enumerate;
use aoc2019::utils::read_buffer;

fn main(){
    let mut pipes: Vec<[(i32, i32); 2]> = Vec::new();

    let input = read_buffer("input/2021/day5.txt").lines();
    for line in input{
        let line_ = line.unwrap();
        let points: Vec<&str> = line_.split(" -> ").collect();
        let mut pipe = [(0, 0); 2];
        for (id, point) in enumerate(points){
            let nums: Vec<i32> = point.split(",").map(|f|f.parse().unwrap()).collect();
            pipe[id] = (nums[0], nums[1]);
        }
        pipes.push(pipe)
    }

    let mut overlaps: HashSet<(i32, i32)> = HashSet::new();
    let mut spots: HashSet<(i32, i32)> = HashSet::new();

    for pipe in &pipes {
        let hor_range = min(pipe[0].1, pipe[1].1)..=max(pipe[0].1, pipe[1].1);
        let vert_range = min(pipe[0].0, pipe[1].0)..=max(pipe[0].0, pipe[1].0);

        //println!("{:?}", pipe);

        if pipe[0].0 == pipe[1].0 {
            for i in hor_range.clone() {
                if !spots.insert((pipe[0].0, i)){
                    overlaps.insert((pipe[0].0, i));
                }
            }
        }
        else if pipe[0].1 == pipe[1].1 {
            for i in vert_range.clone() {
                if !spots.insert((i, pipe[0].1)){
                    overlaps.insert((i, pipe[0].1));
                }
            }
        }
        else {
            let diff_i = pipe[1].0 - pipe[0].0;
            let steps = i32::abs(diff_i);

            let step_i = diff_i/steps;
            let step_j = (pipe[1].1 - pipe[0].1)/steps;
            
            for i in 0..=steps{
                let point = (pipe[0].0+step_i*i, pipe[0].1+step_j*i);
                //println!("{:?}", &point);
                if !spots.insert(point){
                    overlaps.insert(point);
                }
            } 
        }
    }

    //println!("{:?}", &overlaps);
    println!("Part2: {:?}", &overlaps.len());

}
