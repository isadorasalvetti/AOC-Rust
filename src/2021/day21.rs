
use std::collections::HashMap;

const P1_START: i32 = 1;
const P2_START: i32 = 10;

const WIN_POINTS: u64 = 21;

fn main(){
    let mut board_status = HashMap::<((u64, u64), (u64, u64)), u64>::new();
    board_status.insert(((P1_START as u64, 0), (P2_START as u64, 0)), 1);

    let mut p1_wins = 0;
    let mut p2_wins = 0;
    let die_results = get_die_possibilities();

    while board_status.len() != 0 {
        let mut new_board_status = HashMap::<((u64, u64), (u64, u64)), u64>::new();
        for (((pos1, points1), (pos2, points2)), board_ammount) in &board_status {
            for (result1, rolls1) in die_results.iter() {
                let new_pos1 = (pos1-1 + result1) % 10 + 1;
                let new_points1 = points1 + new_pos1;
                if new_points1 >= WIN_POINTS {
                    p1_wins += rolls1*board_ammount;
                }
                else {
                    *new_board_status.entry(((new_pos1, new_points1), (*pos2, *points2))).or_insert(0) += rolls1*board_ammount;
                }
            }
        }

        board_status = new_board_status;
        new_board_status = HashMap::<((u64, u64), (u64, u64)), u64>::new();
        for (((pos1, points1), (pos2, points2)), board_ammount) in &board_status {
            for (result2, rolls2) in die_results.iter() {
                let new_pos2 = (pos2-1 + result2) % 10 + 1;
                let new_points2 = points2 + new_pos2;
                if new_points2 >= WIN_POINTS {
                    p2_wins += rolls2*board_ammount;
                }
                else {
                    *new_board_status.entry(((*pos1, *points1), (new_pos2, new_points2))).or_insert(0) += rolls2*board_ammount;
                }
            }
        }
        board_status = new_board_status;
        println!("Wins: {:?}, {:?}; Boards: {}",  p1_wins, p2_wins, board_status.len());
        
    }
    println!("Wins: {:?}, {:?}", p1_wins, p2_wins);

}

fn get_die_possibilities() -> HashMap<u64, u64>{
    let mut results = HashMap::<u64, u64>::new();
    for x in 1..=3 {
        for y in 1..=3 {
            for z in 1..=3 {
                *results.entry(x+y+z).or_insert(0) += 1;
            }
        }
    }
    return results;
}

fn p1(){
    let mut dice = 0;
    let mut rolls = 0;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut p1_position = P1_START;
    let mut p2_position = P2_START;

    while p1_score < 999 && p2_score < 999 {
        p1_position += dice_roll(&mut dice, &mut rolls);
        if p1_score > 999 {break;}
        p2_position += dice_roll(&mut dice, &mut rolls);
        p1_position = (p1_position-1) % 10 +1;
        p2_position = (p2_position-1) % 10 +1;

        p1_score += p1_position;
        p2_score += p2_position;
    }

    println!("Part1: {} - {}, {}, {}", rolls*i32::min(p2_score, p1_score), rolls, p1_score, p2_score)
}

fn dice_roll(dice: &mut i32, rolls: &mut i32) -> i32{
    let mut tot = 0;
    *dice += 1;
    if *dice > 1000 { *dice = 1 } 
    tot += *dice;

    *dice += 1;
    if *dice > 1000 { *dice = 1 } 
    tot += *dice;

    *dice += 1;
    if *dice > 1000 { *dice = 1 } 
    tot += *dice;
    

    *rolls += 3;
    
    return tot;
}