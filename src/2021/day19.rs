use std::{io::BufRead, collections::{HashSet, HashMap}};
use aoc2019::utils::read_buffer;
use itertools::Itertools;

type Point = (i32, i32, i32);
const points_per_scan: usize = 25;

fn main(){
    let mut sensors: Vec<Vec<Point>> = Vec::new();

    let mut points: Vec<Point> = Vec::new();
    for line in read_buffer("input/2021/day19mock.txt").lines() {
        let line_ = line.unwrap();
        if line_.len() < 5 { continue; }
        if line_.chars().nth(4).unwrap() == 's' {
            if points.len() > 1 {sensors.push(points.clone())}
            points = Vec::new();
            continue;
        }
        let nums_str: Vec<&str> = line_.as_str().split(',').collect();
        let parsed: Point = (nums_str[0].parse().unwrap(), nums_str[1].parse().unwrap(), nums_str[2].parse().unwrap());
        points.push(parsed);
    }
    sensors.push(points.clone());

    let mut dists = Vec::<Vec<Vec<HashSet<i32>>>>::new();
    for sensor in &sensors{
        let mut s_dists = Vec::<Vec<HashSet<i32>>>::new();
        for point_a in sensor{
            let mut p_dists = Vec::<HashSet<i32>>::new();
            for point_b in sensor{
                if point_a != point_b {
                    let axis_dist = HashSet::from([(point_a.0-point_b.0).abs(), (point_a.1-point_b.1).abs(), (point_a.2-point_b.2).abs()]);
                    p_dists.push(axis_dist);
                }
            }
            s_dists.push(p_dists);
        }
        dists.push(s_dists);
    }

    //println!("{:?}", dists);

    let mut overlaps = HashMap::<(usize, usize), Vec<Point>>::new();

    for ((a, sensor_a), (b, sensor_b)) in dists.iter().enumerate().tuple_combinations() {
        for (i, point_a) in sensor_a.into_iter().enumerate(){
            let mut similarities = 0;
            for point_b in sensor_b{
                for dist in point_a{
                    if point_b.contains(dist){
                        similarities += 1;
                    }
                }
            }
            if similarities > 1 {
                overlaps.entry((a, b)).or_insert(Vec::<Point>::new()).push(sensors[a][i]);
                //overlaps.entry((b, a)).or_insert(Vec::<Point>::new()).push(sensors[b][i]);
            }
        }
    }

    println!("{:?}", &overlaps);

    let mut all_duplicates = vec![HashSet::<Point>::new(); sensors.len()];

    for ((a, _), value) in overlaps{
        for val in value { 
            all_duplicates[a].insert(val); 
        }
    }

    let mut count = 0;
    for a in &all_duplicates{
        count += points_per_scan - a.len();
    }

    let only_duplicates = all_duplicates.into_iter().reduce(|a, b| a.union(&b).copied().collect()).unwrap();

    println!("Part1: {}, {}", &count, only_duplicates.len());

}  
