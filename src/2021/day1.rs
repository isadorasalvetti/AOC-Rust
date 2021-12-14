use aoc2019::utils::read_nums;
use itertools::Itertools;

fn main() {
    let mut going_deeper = 0;
    let nums: Vec<i32> = read_nums("input/2021/day1.txt").collect();
    for (a, b) in nums.iter().tuple_windows(){
        if b > a { going_deeper += 1; }
    }

    println!("Part1: {}", going_deeper);

    let mut going_deeper = 0;
    let summed_nums = nums.iter().tuple_windows::<(_, _, _)>().map(|x| x.0+x.1+x.2);
    for (a, b) in summed_nums.tuple_windows(){
        if b > a { going_deeper += 1; }
    }

    println!("Part2: {}", going_deeper);

}
