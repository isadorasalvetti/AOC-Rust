use itertools::enumerate;

const FISH: [i32; 300] = [1,1,5,2,1,1,5,5,3,1,1,1,1,1,1,3,4,5,2,1,2,1,1,1,1,1,1,1,1,3,1,1,5,4,5,1,5,3,1,3,2,1,1,1,1,2,4,1,5,1,1,1,4,4,1,1,1,1,1,1,3,4,5,1,1,2,1,1,5,1,1,4,1,4,4,2,4,4,2,2,1,2,3,1,1,2,5,3,1,1,1,4,1,2,2,1,4,1,1,2,5,1,3,2,5,2,5,1,1,1,5,3,1,3,1,5,3,3,4,1,1,4,4,1,3,3,2,5,5,1,1,1,1,3,1,5,2,1,3,5,1,4,3,1,3,1,1,3,1,1,1,1,1,1,5,1,1,5,5,2,1,5,1,4,1,1,5,1,1,1,5,5,5,1,4,5,1,3,1,2,5,1,1,1,5,1,1,4,1,1,2,3,1,3,4,1,2,1,4,3,1,2,4,1,5,1,1,1,1,1,3,4,1,1,5,1,1,3,1,1,2,1,3,1,2,1,1,3,3,4,5,3,5,1,1,1,1,1,1,1,1,1,5,4,1,5,1,3,1,1,2,5,1,1,4,1,1,4,4,3,1,2,1,2,4,4,4,1,2,1,3,2,4,4,1,1,1,1,4,1,1,1,1,1,4,1,5,4,1,5,4,1,1,2,5,5,1,1,1,5];
const FISH_: [i32; 5] = [3,4,3,1,2];


fn main(){
    dumb_way();
    fast_way();
}

fn fast_way(){
    let mut fish_counter= [0; 9];
    for i in FISH {
        fish_counter[i as usize] += 1;
    }

    for day in 0..256 {
        let day_0of9 = day % 9;
        let day_7of9 = (day+7) % 9;

        fish_counter[day_7of9] += fish_counter[day_0of9];

    }
    println!("Part2: {}", fish_counter.iter().sum::<i64>())
}

fn dumb_way(){
    let mut current_fish = FISH.to_vec();

    for _day in 0..80 {
        let day_fish = current_fish.len();
        for (i, fish) in enumerate(current_fish.clone()){
            if fish == 0 {
                current_fish.push(8);
                current_fish[i] = 6;
            }
            else {
                current_fish[i] -= 1;
            }
        }
        println!("Part1: {}", current_fish.len()-day_fish);
    }
    println!("Part1: {}", current_fish.len())
}