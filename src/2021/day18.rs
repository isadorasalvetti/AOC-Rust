use core::num;
use std::{io::BufRead};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{delimited, separated_pair},
    IResult,
};
use aoc2019::utils::read_buffer;

#[derive(Debug)]
enum Number {
    Sing(u64),
    Pair(Box<Number>, Box<Number>),
}

// [[[3,9],[7,2]],[[8,4],[[5,6],0]]]
fn main(){
    let mut nums: Vec<Number> = Vec::new();

    for line in read_buffer("input/2021/day18mock.txt").lines() {
        let line_ = line.unwrap();
        let parsed = parse_number(line_.as_str()).unwrap().1;
        //println!("{:?}", &parsed);
        nums.push(parsed);
    }

    nums.reverse();
    let mut total= nums.pop().unwrap();
    while nums.len() > 0 {
        total = Number::Pair(Box::new(total), Box::new(nums.pop().unwrap()));
        loop {
            if !reduce(&mut total) {break;};
        }
    }
    println!("{:?}", &total);
}

fn get_nestings<'a>(num: &'a Number, depth_counter: &usize) -> Option<&'a Number> {
    match num {  // type of *v is Value
        Number::Pair(a, b) => {
            if depth_counter == &4 {return Some(num);}
            let a_counter = depth_counter.clone() + 1;
            let b_counter = depth_counter.clone() + 1;

            let a_nested = get_nestings(a, &a_counter);
            if a_nested.is_some() {return a_nested;}
            let b_nested = get_nestings(b, &b_counter);
            if b_nested.is_some() {return b_nested;}
        }
        Number::Sing(_) => {}
    }
    return None;
}

fn reduce(num: &mut Number) -> bool {
    // Explosion
    let left_most_nest = get_nestings(num, &0);
    let left: Option<&mut Number> = None;
    let right: Option<&mut Number> = None;
    if left_most_nest.is_some(){
        for p in num{

        }
    }

    // Split

    return false;
}

fn parse_number(input: &str) -> IResult<&str, Number> {
    alt((
        map(nom::character::complete::u64, Number::Sing),
        map(
            delimited(
                tag("["),
                separated_pair(parse_number, tag(","), parse_number),
                tag("]"),
            ),
            |(l, r)| Number::Pair(Box::new(l), Box::new(r)),
        ),
    ))(input)
}
