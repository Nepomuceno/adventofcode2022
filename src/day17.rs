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

const PRINT_ENABLED: bool = false;

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

fn print_cave(positions_ocupied: &Vec<(usize,usize)>, cave_length: usize, top: usize, rock: &Vec<(usize,usize)>) {
    if !PRINT_ENABLED {
        return;
    }
    let start = 0.max(top-20);
    for i in (start..=top+4).rev() {
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

fn filter_positions(positions_ocupied: &HashSet<(usize,usize)>, cave_length: usize) -> Vec<(usize,usize)> {
    let positions = (0..cave_length)
                    .map(|x| positions_ocupied.iter()
                                    .filter(|y| y.0 == x)
                                    .max_by(|x,y| x.1.cmp(&y.1)).unwrap()).collect::<Vec<&(usize,usize)>>();
    let min_y = positions.iter().min_by(|x,y| x.1.cmp(&y.1)).unwrap();
    positions.iter().map(|x| (x.0, x.1 - min_y.1)).collect()
}

pub fn run(input: &str) -> String {
    let wind_pattern = input.chars().collect::<Vec<char>>();
    let mut positions_ocupied = HashSet::new();
    let mut visited:HashMap<(Vec<(usize, usize)>, usize, usize), (usize, usize)> = HashMap::new();
    let cave_length:usize = 7;
    let num_rocks = 1_000_000_000_000;
    for i in 0..cave_length { 
        positions_ocupied.insert((i,0));
    }
    
    let mut top = 0;
    let mut wind_start = 0;
    let mut broken_cycles = false;
    let mut i = 0;
    loop {
        let start_pos = (2, top+4);
        let mut rock = generate_rock(start_pos, i%5);
        if i == num_rocks-1 {
            break;
        }

        
        // print_cave(&positions_ocupied, cave_length, top, &rock);
        loop {
            let temp_rock = 
            move_rock(rock.clone(), wind_pattern[wind_start%wind_pattern.len()], cave_length-1);
            if !check_collision(&temp_rock, &positions_ocupied) {
                //println!("pushing rock {}",wind_pattern[wind_start%wind_pattern.len()]);
                rock = temp_rock;
            }
            wind_start = wind_start + 1;
            let temp_rock = 
            move_rock(rock.clone(), 'v', cave_length-1);
            if check_collision(&temp_rock, &positions_ocupied) {
                break;
            }
            rock = temp_rock;
        }
        if broken_cycles {
            let position_to_print = positions_ocupied.clone().iter().map(|x| -> (usize,usize){*x}).collect::<Vec<(usize,usize)>>();
            print_cave(&position_to_print, cave_length, top, &rock);
        }
        for rock_part in rock.iter() {
            if rock_part.1 > top {
                top = rock_part.1;
            }
            positions_ocupied.insert(*rock_part);
        }
        let position_pattern = filter_positions(&positions_ocupied, cave_length);
        let wind_index = wind_start%wind_pattern.len();
        let rock_index = i%5;
        let position_clone = position_pattern.clone();
        let key = (position_clone, wind_index, rock_index);
        if !broken_cycles && visited.contains_key(&key) {
            let position_to_print = positions_ocupied.clone().iter().map(|x| -> (usize,usize){*x}).collect::<Vec<(usize,usize)>>();
            print_cave(&position_to_print, cave_length, top, &rock);
            let cycle_length = i - visited[&key].0;
            let cycle_top = top - visited[&key].1;
            let cycles_left = (num_rocks - i)/cycle_length;
            i = i + cycles_left*cycle_length;
            let max_key_0 = key.0.iter().max_by(|x,y| x.1.cmp(&y.1)).unwrap().1;
            top = top + cycles_left*cycle_top;
            for i in key.0 { 
                positions_ocupied.insert((i.0, top + i.1 - max_key_0));
            }
            broken_cycles = true;
            continue;
        } else if !broken_cycles {
            visited.insert(key, (i,top));
            if broken_cycles {
                let top_pattern = position_pattern.iter().max_by(|x,y| x.1.cmp(&y.1)).unwrap().1;
                print_cave(&position_pattern, cave_length, top_pattern, &rock);        
            }
        }
        
        if i%1_000_000 == 0 {
            println!("rock {} top {}", i, top);
        }

        i = i + 1;
    }
    (top-4).to_string()
}

// > 1532163742700