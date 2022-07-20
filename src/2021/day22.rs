use std::{io::BufRead, collections::{HashSet}};
use aoc2019::utils::read_buffer;

type Point = (i32, i32, i32);
type Cube = ((i32, i32), (i32, i32), (i32, i32));
type Entry = (bool, Cube);

fn main() {
    let input = read_buffer("input/2021/day22mock.txt").lines();
    let mut operations = Vec::<Entry>::new();

    for line in input {
        let line_str = line.unwrap();
        
        let split: Vec<&str> = line_str.split(" ").collect();

        let on_off;
        match split[0] {
            "on" => on_off = true,
            "off" => on_off = false,
            _ => panic!()
        }

        let mut min_max = Vec::<(i32, i32)>::new();
        let axis: Vec<&str> = split[1].split(",").collect();
        for a in axis { 
            let nums: Vec<&str> = a[2..].split("..").collect();
            let min: i32 = nums[0].parse().unwrap();
            let max: i32 = nums[1].parse().unwrap();
            min_max.push((min, max))
        }

        operations.push((on_off, (min_max[0], min_max[1], min_max[2])));
    }

    p2(&operations);
    //p1(&operations);

    //let test = cube_contains(&((-1, 1), (-1, 1), (-1, 1)), &((-2, 0), (-2, 0), (-2, 0))).unwrap();
    //println!("{:?}, {}", test, test.len());

    //let test1 = cube_contains(&((-1, 1), (-1, 1), (-1, 1)), &((-1, 0), (-1, 0), (-1, 0))).unwrap();
    //println!("{:?}, {}", test1, test1.len());

    //let test2 = cube_contains(&((-1, 1), (-1, 1), (-1, 1)), &((2, 3), (2, 3), (2, 3)));
    //println!("{:?}", test2);
}

fn p2(operations: &Vec<Entry>){
    let mut cubes_of_interest = HashSet::<Cube>::new();
    for entry in operations{
        cubes_of_interest.insert(entry.1);
    }
    'e: loop {
        let clone = cubes_of_interest.clone();
        for cube1 in &clone {
            for cube2 in &clone {
            let split_cubes = cube_contains(cube1, cube2);
            if split_cubes != None {
                cubes_of_interest.remove(&cube1);
                cubes_of_interest.remove(&cube2);

                cubes_of_interest.extend(split_cubes.unwrap());
                continue 'e;
                }
            }
        }
        break;
    }

    println!("{:?}", cubes_of_interest);
}

fn cube_contains(c1: &Cube, c2: &Cube) -> Option<HashSet<Cube>>{
    if c1 == c2 {return None}
    if c1.0.1 > c2.0.0 && c1.0.0 < c2.0.1 {
        if c1.1.1 > c2.1.0 && c1.1.0 < c2.1.1 {
            if c1.2.1 > c2.2.0 && c1.2.0 < c2.2.1 {
                let intersection = [(c1.0.0.max(c2.0.0), c1.0.1.min(c2.0.1)), (c1.1.0.max(c2.1.0), c1.1.1.min(c2.1.1)), (c1.2.0.max(c2.2.0), c1.2.1.min(c2.2.1))];
                let mut new_cubes = HashSet::<Cube>::new(); 

                for cube in [c1, c2].iter(){
                    let mut new_axis = [[(0, 0), (0, 0), (0, 0)]; 3];
                    for (i, axis) in [cube.0, cube.1, cube.2].iter().enumerate(){
                        let center;
                        if intersection[i].0 == axis.0 { 
                            center = intersection[i].1; 
                            new_axis[i] = [(axis.0, center), (center, axis.1), (0, 0)];
                        }
                        else if intersection[i].1 == axis.1 { 
                            center = intersection[i].0; 
                            new_axis[i] = [(axis.0, center), (center, axis.1), (0, 0)];
                        }
                        else {
                            let mut sp = [intersection[i].1, intersection[i].0, axis.0, axis.1];
                            sp.sort();
                            new_axis[i] = [(sp[0], sp[1]), (sp[1], sp[2]), (sp[2], sp[3])];
                        }
                    }

                    for x in new_axis[0] {
                        for y in new_axis[1] {
                            for z in new_axis[2] {
                                if x == (0, 0) || y == (0, 0) || z == (0, 0) {continue;}
                                new_cubes.insert((x, y, z));
                            }
                        }
                    }

                }
                return Some(new_cubes);
            }
        }
    }

    return None;



}


fn p1(operations: &Vec<Entry>){
    let mut lit_points = HashSet::<Point>::new();
    for (run, operation) in operations.iter().enumerate(){
        let lit = operation.0;
        let x = operation.1.0;
        let y = operation.1.1;
        let z = operation.1.2;

        if i32::abs(operation.1.0.0) > 50 {break;}
        for x in x.0..=x.1 {   
            for y in y.0..=y.1 {
                for z in z.0..=z.1 {
                    if lit {lit_points.insert((x, y, z));}
                    else {lit_points.remove(&(x, y, z));}
                }
            }
        }
    }

    println!("Part1: {}", lit_points.len());

}

