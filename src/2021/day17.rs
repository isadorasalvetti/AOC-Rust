use std::{collections::{HashMap, HashSet}};

const XMIN: i32 = 34;
const XMAX: i32 = 67;

const YMIN: i32 = -215;
const YMAX: i32 = -186;

fn main(){
    p1();
    p2();
}

fn p1(){
    let mut traveled = 0;
    let mut last_increase = YMAX.abs()-1;
    
    while last_increase > 0 {
        traveled += last_increase;
        last_increase -= 1;
    }

    println!("Part1: {}", traveled);
}

fn p2(){
    let valid_x_speed = get_x();
    let valid_y_speed = get_y();

    let mut all = HashSet::<(i32, i32)>::new();
    for (x, steps_x, accept_higher_step) in &valid_x_speed {
        for (y, steps_y) in &valid_y_speed {
            if !accept_higher_step{ if steps_x == steps_y { all.insert((*x, *y)); }}
            else {if steps_x <= steps_y { all.insert((*x, *y)); }}
        }
    }

    println!("Part2: {:?}", &all.len());

}

fn get_y() ->  Vec::<(i32, i32)> {
    let mut ret = Vec::<(i32, i32)>::new();
    for y in YMIN-1..=YMAX.abs() + 50 {
        let mut step_count = 0;
        let mut step_dist = y;
        let mut dist = 0;
        while dist > YMIN {
            step_count += 1;
            dist += step_dist;
            step_dist -= 1;
            
            if dist >= YMIN && dist <= YMAX {
                ret.push((y, step_count));
            }
        }
    }
    //println!("{:?}", ret);
    return ret;
}

fn get_x() -> Vec<(i32, i32, bool)> {
    let mut ret = Vec::<(i32, i32, bool)>::new();
    for x in 0..=XMAX {
        let mut step_count = 0;
        let mut step_dist = x;
        let mut dist = 0;
        while step_dist >= 0 {
            step_count += 1;
            dist += step_dist;
            step_dist -= 1;
            
            if dist >= XMIN && dist <= XMAX {
                ret.push((x, step_count, step_dist==0));
            }
            if dist > XMAX {break;}
        }
    }
    //println!("{:?}", ret);
    return ret;
}