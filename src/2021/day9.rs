use std::{io::BufRead, collections::{HashMap, HashSet}};
use itertools::{enumerate, Itertools};
use aoc2019::utils::read_buffer;

fn main(){
    let mut lava_points: HashMap<(i32, i32), u32> = HashMap::new();
    let mut basins_elements: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in enumerate(read_buffer("input/2021/day9.txt").lines()){
        let line_ = line.unwrap();
        let chars = line_.chars();
        for (x, char) in enumerate(chars){
            lava_points.insert((x as i32, y as i32), char.to_digit(10).unwrap());
        }
    }

    let mut biggest_basins = [0, 0, 0];

    for ((x, y), _val) in lava_points.clone() {
        if basins_elements.contains(&(x, y)) {continue;}

        let mut spent_candidates: HashSet<(i32, i32)> = HashSet::new();
        let mut candidates: Vec<(i32, i32)> = Vec::new();
        candidates.push((x, y));

        let mut curr_basin: HashSet<(i32, i32)> = HashSet::new();

        candidates.sort_by(|a, b| lava_points[b].cmp(&lava_points[a]));
        while candidates.len() > 0 {
            let (x, y) = candidates.pop().unwrap();
            if spent_candidates.contains(&(x, y)){continue;};
            
            let up = (x, y-1);
            let down = (x, y+1);
            let right = (x+1, y);
            let left = (x-1, y);
            
            if &lava_points[&(x, y)] < &9 {
                basins_elements.insert((x, y));
                curr_basin.insert((x, y));

                for pos in [up, down, right, left] {
                    if lava_points.keys().contains(&pos){
                        candidates.push(pos)
                    }
                }
            }
            spent_candidates.insert((x, y));
        }

        //println!("{:?}", curr_basin.iter().map(|x| lava_points[x]).collect::<Vec<u32>>());

        biggest_basins.sort();
        let basin_size = curr_basin.len();
        if basin_size > biggest_basins[0]{
            biggest_basins[0] = basin_size;
        }
    }

    print!("Part2: {:?}", biggest_basins.iter().product::<usize>())

}

fn p1(){
    let mut lava_points: HashMap<(i32, i32), u32> = HashMap::new();
    let mut low_points: Vec<u32> = Vec::new();

    for (y, line) in enumerate(read_buffer("input/2021/day9.txt").lines()){
        let line_ = line.unwrap();
        let chars = line_.chars();
        for (x, char) in enumerate(chars){
            lava_points.insert((x as i32, y as i32), char.to_digit(10).unwrap());
        }
    }

    for ((x, y), val) in lava_points.clone() {
        let up = (x, y-1);
        let down = (x, y+1);
        let right = (x+1, y);
        let left = (x-1, y);

        if &val < &lava_points.get(&up).unwrap_or(&10) &&
            &val < &lava_points.get(&down).unwrap_or(&10) &&
            &val < &lava_points.get(&right).unwrap_or(&10) &&
            &val < &lava_points.get(&left).unwrap_or(&10){
                low_points.push(val.clone());
            }
    }
    
    let sum: u32 = low_points.iter().sum();
    print!("Part1: {}", low_points.len() as u32 + sum);
}