use std::{collections::{HashMap}, io::BufRead};
use aoc2019::utils::read_buffer;
use itertools::Itertools;
use regex::Regex;


fn main(){
    let mut guard_sleeping_minutes: HashMap<i32, Vec<i32>> = HashMap::new();
    let guard_actions = get_guard_actions();
    let key_vector = guard_actions.keys().sorted();
    
    // for order_key in key_vector{
    //     println!("{:?}, {}", order_key, guard_actions[order_key]);
    // }
    // return;

    let mut current_guard = 0;
    let mut last_sleep_time = 0;

    for key in key_vector{
        if guard_actions[key] == -1 { 
            last_sleep_time = key.3;
        }
        else if guard_actions[key] == -2 {
            let mut minutes_passed: Vec<i32> = (last_sleep_time..key.3).collect();
            let current_entry = guard_sleeping_minutes.entry(current_guard).or_insert(Vec::new());
            current_entry.append(&mut minutes_passed);
        }

        else {
            current_guard = guard_actions[key]
        }
    }


    //Part 1
    let max_guard = guard_sleeping_minutes.iter()
    .max_by(|a, b| a.1.len().cmp(&b.1.len()))
    .map(|(k, _v)| k).unwrap();
    
    let sleeping_freq= make_freq(&guard_sleeping_minutes[max_guard]);

    let max_min = sleeping_freq.iter()
    .max_by(|a, b| a.1.cmp(&b.1))
    .map(|(k, _v)| k).unwrap();

    println!("Part1: {}", max_guard*max_min);


    //Part 2
    let freq_min = guard_sleeping_minutes.iter().map(|(x, y)|(x, make_freq(y)));
    let most_asleep_days = freq_min.map(|(x, y)| (x, y.into_iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap()));
    let guard = most_asleep_days.max_by(|a, b| a.1.1.cmp(&b.1.1)).unwrap();

    println!("Part2: {:?}", guard.0*guard.1.0);

}

fn make_freq(vector: &Vec<i32>) -> HashMap<i32, usize>{
    let mut freq: HashMap<i32, usize> = HashMap::new();
    for x in vector {
        *freq.entry(*x).or_default() += 1;
    }
    return freq;
}

fn get_guard_actions() -> HashMap<(i32, i32, i32, i32), i32>{
    let guard_text = read_buffer("input/day4.txt").lines();
    let re_date = Regex::new(r"\[1518\-(?P<mm>\d+)-(?P<dd>\d+) (?P<h>\d+):(?P<m>\d+)\]").unwrap();
    let re_shif_start = Regex::new(r"Guard #(?P<id>\d+) begins shift").unwrap();
    let re_fall_asleep = Regex::new(r"falls asleep").unwrap();
    let re_wakes_up = Regex::new(r"wakes up").unwrap();
    
    let mut guard_schedule = HashMap::new();

    for line in guard_text{
        let text = line.unwrap();
        let date_cap = re_date.captures(text.as_str()).unwrap();
        let date: i32 = date_cap["dd"].parse().unwrap();
        let month: i32 = date_cap["mm"].parse().unwrap();
        let hour: i32 = date_cap["h"].parse().unwrap();
        let min: i32 = date_cap["m"].parse().unwrap();

        let mut id = 0;
        let key=(month, date, hour, min);

        if re_shif_start.is_match(text.as_str()){
            let id_cap = re_shif_start.captures(text.as_str()).unwrap();
            id = id_cap["id"].parse().unwrap();
        }

        else if re_fall_asleep.is_match(text.as_str()){ id = -1 }
        else if re_wakes_up.is_match(text.as_str()){ id = -2 }

        guard_schedule.insert(key, id);
    }

    return guard_schedule;
}


