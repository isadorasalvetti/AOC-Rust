use std::{io::BufRead, collections::{HashMap}};
use aoc2019::utils::read_buffer;

fn main(){
    let mut pairs: HashMap<(char, char), char> = HashMap::new();

    //let protein_str = "NNCB";
    let protein_str = "VNVVKSNNFPBBBVSCVBBC";
    for line in read_buffer("input/2021/day14.txt").lines() {
        let line_ = line.unwrap();
        let (pair, ch) = line_.split_once(" -> ").unwrap();

        let mut pi = pair.chars();
        let pair_ = (pi.next().unwrap(), pi.next().unwrap());

        pairs.insert(pair_, ch.chars().next().unwrap());
    }

    p1(&pairs, &protein_str);
    p2(&pairs, &protein_str);

}

fn p2(pairs: &HashMap<(char, char), char>, protein_str: &str) {
    let trailing_char: char;
    let mut protein_pairs_counter = HashMap::<(char, char), usize>::new();
    let (mut p1_iterator, mut p2_iterator) = (protein_str.clone().chars(), protein_str.clone().chars());
    p2_iterator.next();
    
    loop {
        let p1 = p1_iterator.next().unwrap();
        let p2 = p2_iterator.next().unwrap_or('0');
        if p2 == '0' { 
            trailing_char = p1; 
            break; 
        }
        *protein_pairs_counter.entry((p1, p2)).or_default() += 1;            
    }
    
    for _ in 0..40 {
        let mut new_protein = HashMap::<(char, char), usize>::new();
        for (pair, ammount) in protein_pairs_counter.clone() {
            let inb = pairs[&pair];
            *new_protein.entry((pair.0, inb)).or_default() += ammount;
            *new_protein.entry((inb, pair.1)).or_default() += ammount;
        }
        protein_pairs_counter = new_protein;
        //println!("{:?}", protein_pairs_counter);
    }

    let mut counter: HashMap<char, usize> = HashMap::new();
    for ((p1, _), x) in protein_pairs_counter {
        *counter.entry(p1).or_default() += x;
    }
    *counter.entry(trailing_char).or_default() += 1;

    let (_, max) = counter.iter().max_by_key(|(_, v)| *v).unwrap();
    let (_, min) = counter.iter().min_by_key(|(_, v)| *v).unwrap();

    println!("Part2: {}", max-min);

}


fn p1(pairs: &HashMap<(char, char), char>, protein_str: &str) {
    let mut protein: Vec<char> = protein_str.chars().collect();
    for _ in 0..10 {
        let (mut p1_iterator, mut p2_iterator) = (protein.clone().into_iter(), protein.clone().into_iter());
        p2_iterator.next();

        protein.clear();
        loop {
            let p1 = p1_iterator.next().unwrap();
            let p2 = p2_iterator.next().unwrap_or('0');
            if p2 == '0' { protein.push(p1); break; }

            if pairs.contains_key(&(p1, p2)){
                protein.push(p1);
                protein.push(pairs[&(p1, p2)]);
            }
        }
    }

    let mut counter: HashMap<char, usize> = HashMap::new();
    for x in protein {
        *counter.entry(x).or_default() += 1;
    }

    let (_, max) = counter.iter().max_by_key(|(_, v)| *v).unwrap();
    let (_, min) = counter.iter().min_by_key(|(_, v)| *v).unwrap();

    println!("Part1: {}", max-min);
}