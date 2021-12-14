use std::collections::HashSet;
use aoc2019::utils::read_nums;

fn main() {
    p1();
    p2();
}

fn p2(){
    let input:Vec<i32> = read_nums("input/day1.txt").collect();
    let mut i = input.iter().cycle();
    let mut freqs: HashSet<i32> = HashSet::new();
    let mut freq = 0;

    while !freqs.contains(&freq) {
        freqs.insert(freq);
        freq += i.next().unwrap();
    };

    println!("P1: {}", freq)

}

fn p1() {
    let mut freq = 0;
    for i in read_nums("input/day1.txt"){
        freq += i;
    };

    println!("P2: {}", freq)
}
