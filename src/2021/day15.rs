use std::{io::BufRead, collections::{HashMap, HashSet}, ops::Rem};
use std::cmp::Ordering;
use aoc2019::utils::read_buffer;

const starting_pos: (i32, i32) = (0, 0);
const target_pos: (i32, i32) = (100*5-1, 100*5-1);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
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
    let mut sums = HashMap::<(i32, i32), u32>::new();
    let mut sorted_keys = Vec::<(i32, i32)>::new();
    let mut next = starting_pos;
    sums.insert(starting_pos, 0);
    explored.insert((0, 0));

    'bob: loop {
        let new_candidates = [
            (next.0, next.1 + 1),
            (next.0, next.1 - 1),
            (next.0 + 1, next.1),
            (next.0 - 1, next.1)
        ];

        for candidate in new_candidates {
            if !risk.contains_key(&candidate) || explored.contains(&candidate){ continue; }
            if candidate == target_pos { 
                sums.insert(candidate, sums[&next] + risk[&candidate]);
                break 'bob; 
            }

            let curr_risk = sums[&next] + risk[&candidate];
            add_candidate(&mut sorted_keys, &mut sums, candidate, curr_risk);
        }
        sums.remove(&next);

        next = sums.keys().min_by(|a, b| sums[a].cmp(&sums[b])).unwrap().clone();
        explored.insert(next);
    }

    println!("Part2: {:?}", &sums[&target_pos]);
}

fn add_candidate(sorted: &mut Vec<(i32, i32)>, dict: &mut HashMap<(i32, i32), u32>, candidate: (i32, i32), val: u32){
    if dict.contains_key(&candidate){
        *dict.entry(candidate).or_default() = u32::min(dict[&candidate], val)
    }
    else { dict.insert(candidate, val); }
}
