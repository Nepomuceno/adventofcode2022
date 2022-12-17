use std::collections::{HashMap, HashSet};



fn rock_1(x:(usize,usize)) -> Vec<(usize,usize)>{
    return vec![
        (x.0, x.1),
        (x.0 + 1, x.1),
        (x.0 + 2, x.1),
        (x.0 + 3, x.1),
    ];
}

fn rock_2(x:(usize,usize)) -> Vec<(usize,usize)>{
    return vec![
        (x.0, x.1 + 1),
        (x.0 + 1, x.1),
        (x.0 + 1, x.1 + 1),
        (x.0 + 1, x.1 + 2),
        (x.0 + 2, x.1 + 1),
    ];
}

fn rock_3(x:(usize,usize)) -> Vec<(usize,usize)>{
    return vec![
        (x.0, x.1),
        (x.0 + 1, x.1),
        (x.0 + 2, x.1),
        (x.0 + 2, x.1 + 1),
        (x.0 + 2, x.1 + 2),
    ];
}

fn rock_4(x:(usize,usize)) -> Vec<(usize,usize)>{
    return vec![
        (x.0, x.1 + 0),
        (x.0, x.1 + 1),
        (x.0, x.1 + 2),
        (x.0, x.1 + 3),
    ];
}

fn rock_5(x:(usize,usize)) -> Vec<(usize,usize)>{
    return vec![
        (x.0, x.1),
        (x.0, x.1 + 1),
        (x.0 + 1, x.1),
        (x.0 + 1, x.1 + 1),
    ];
}

fn generate_rock(x:(usize,usize), rock_type: usize) -> Vec<(usize,usize)>{
    match rock_type {
        0 => rock_1(x),
        1 => rock_2(x),
        2 => rock_3(x),
        3 => rock_4(x),
        4 => rock_5(x),
        _ => vec![]
    }
}

const print_enabled: bool = false;

fn move_rock(mut rock: Vec<(usize,usize)>, direction: char, cave_size: usize) -> Vec<(usize,usize)> {
    match direction {
        '<' => {
            if !rock.iter().any(|x| x.0 == 0) {
                for rock_part in rock.iter_mut()  {
                    rock_part.0 = rock_part.0 - 1;
                }
            }
        },
        '>' => {
            if !rock.iter().any(|x| x.0 == cave_size) {
                for rock_part in rock.iter_mut() {
                    rock_part.0 = rock_part.0 + 1;
                }
            }
        },
        'v' => {
            if !rock.iter().any(|x| x.1 == 0) {
                for rock_part in rock.iter_mut()  {
                    rock_part.1 = rock_part.1 - 1;
                }
            }
        },
        _ => {}
    };
    rock
}

fn check_collision(rock: &Vec<(usize,usize)>, positions_ocupied: &HashSet<(usize,usize)>) -> bool {
    for rock_part in rock.iter() {
        if positions_ocupied.contains(rock_part) {
            return true;
        }
    }
    false
}

fn print_cave(positions_ocupied: &HashSet<(usize,usize)>, cave_length: usize, top: usize, rock: &Vec<(usize,usize)>) {
    if !print_enabled {
        return;
    }
    for i in (0..=top+4).rev() {
        print!("|");
        for j in 0..cave_length {
            if positions_ocupied.contains(&(j,i)) {
                print!("#");
            } else if rock.iter().any(|x| x.0 == j && x.1 == i) {
                print!("@");
            } else {
                print!(".");
            }
        }
        print!("|");
        println!("");
    }
    println!("----------------");
}

pub fn run(input: &str) -> String {
    let wind_pattern = input.chars().collect::<Vec<char>>();
    let mut positions_ocupied = HashSet::new();
    let cave_length:usize = 7;
    let num_rocks = 2022;
    for i in 0..cave_length { 
        positions_ocupied.insert((i,0));
    }
    
    let mut top = 0;
    let mut wind_start = 0;
    for i in 0..num_rocks {
        let start_pos = (2, top+4);
        let mut rock = generate_rock(start_pos, i%5);
        print_cave(&positions_ocupied, cave_length, top, &rock);
        loop {
            let temp_rock = 
            move_rock(rock.clone(), wind_pattern[wind_start%wind_pattern.len()], cave_length-1);
            if !check_collision(&temp_rock, &positions_ocupied) {
                println!("pushing rock {}",wind_pattern[wind_start%wind_pattern.len()]);
                rock = temp_rock;
            }
            wind_start = wind_start + 1;
            print_cave(&positions_ocupied, cave_length, top, &rock);
            let temp_rock = 
            move_rock(rock.clone(), 'v', cave_length-1);
            if check_collision(&temp_rock, &positions_ocupied) {
                break;
            }
            println!("pushing rock v");
            rock = temp_rock;
            print_cave(&positions_ocupied, cave_length, top, &rock);
        }
        for rock_part in rock.iter() {
            if rock_part.1 > top {
                top = rock_part.1;
            }
            positions_ocupied.insert(*rock_part);
        }
    }
    let mut pos: Vec<&(usize,usize)> = positions_ocupied.iter().collect();
    pos.sort_by(|a,b| a.1.cmp(&b.1));
    
    top.to_string()
}