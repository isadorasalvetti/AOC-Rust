use std::{io::BufRead, collections::{HashMap, HashSet, BinaryHeap}, ops::Rem};
use std::cmp::Ordering;
use aoc2019::utils::read_buffer;

const starting_pos: (i32, i32) = (0, 0);
const target_pos_p1: (i32, i32) = (100*1-1, 100*1-1);
const target_pos_p2: (i32, i32) = (100*5-1, 100*5-1);

#[derive(Copy, Clone, Eq, PartialEq)]
struct Cost {
    cost: u32,
    pos: (i32, i32),
}
impl Ord for Cost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}
impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main(){
    let mut risk: HashMap<(i32, i32), u32> = HashMap::new();

    for (y, line) in read_buffer("input/2021/day15.txt").lines().enumerate() {
        for (x, char) in line.unwrap().chars().enumerate(){
            for y_ in 0..5 {
                for x_ in 0..5 {
                    let mut val = (char.to_digit(10).unwrap() + x_ + y_).rem(9);
                    if val == 0 { val = 9 };
                    risk.insert((x as i32 + 100*x_ as i32, y as i32 + 100*y_ as i32), val);        
                }
            }
        }
    }

    let mut explored = HashSet::<(i32, i32)>::new();
    let mut sums = BinaryHeap::<Cost>::new();
    let mut next ;
    sums.push(Cost {pos: starting_pos, cost: 0} );

    'bob: loop {
        next = sums.pop().unwrap();
        if explored.contains(&next.pos){ continue; }

        //let tmp: Vec<_> = sums.clone().into_iter().map(|x|x.pos).collect();
        //println!("{:?}", tmp);

        let new_candidates = [
            (next.pos.0, next.pos.1 + 1),
            (next.pos.0, next.pos.1 - 1),
            (next.pos.0 + 1, next.pos.1),
            (next.pos.0 - 1, next.pos.1)
        ];

        for candidate in new_candidates {
            if !risk.contains_key(&candidate) || explored.contains(&candidate){ continue; }
            let curr_risk = next.cost + risk[&candidate];
            if candidate == target_pos_p2 { 
                println!("Part2: {:?}", curr_risk);
                break 'bob; 
            }
            sums.push(Cost {pos: candidate, cost: curr_risk});
        }
        explored.insert(next.pos);
    }
}


