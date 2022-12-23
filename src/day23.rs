use core::time;
use std::{time::{Instant, Duration}, collections::{HashMap, HashSet, VecDeque}, thread};
use termion::{color, style,cursor};


const PRINT_ENABLED: bool = false;

fn print_map(map: &HashSet<(i32,i32)>) {
    if !PRINT_ENABLED {
        return;
    }
    print!("{}{}", termion::clear::All, cursor::Goto(1, 1));
    println!("------------------------------------- Map -------------------------------------");
    let min_y = map.iter().map(|(_,y)| y).min().unwrap();
    let max_y = map.iter().map(|(_,y)| y).max().unwrap();
    let min_x = map.iter().map(|(x,_)| x).min().unwrap();
    let max_x = map.iter().map(|(x,_)| x).max().unwrap();
    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            print!("{}", style::Reset);
            if map.contains(&(x,y)) {
                print!("{}{}#", color::Fg(color::Green), style::Bold);
            } else {
                print!("{}{}.", color::Fg(color::White), style::Faint);
            }
        }
        println!("{}{}",color::Fg(color::Reset) ,  style::Reset);
    }
    println!("-------------------------------------------------------------------------------");
    println!("{}{}",color::Fg(color::Reset) ,  style::Reset);
    thread::sleep(Duration::from_millis(100));
}

fn get_free_map_spaces(map: &HashSet<(i32,i32)>) -> usize {
    let mut count = 0;
    let min_y = map.iter().map(|(_,y)| y).min().unwrap();
    let max_y = map.iter().map(|(_,y)| y).max().unwrap();
    let min_x = map.iter().map(|(x,_)| x).min().unwrap();
    let max_x = map.iter().map(|(x,_)| x).max().unwrap();
    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            if !map.contains(&(x,y)) {
                count += 1;
            }
        }
    }
    return count;
}

fn move_north (map: &HashSet<(i32,i32)>, elf: (i32,i32)) -> Option<(i32,i32)> {
    if !map.contains(&(elf.0, elf.1-1)) && !map.contains(&(elf.0-1, elf.1-1)) && !map.contains(&(elf.0+1, elf.1-1)) {
        return Some((elf.0, elf.1-1));
    }
    return None;
}
fn move_south (map: &HashSet<(i32,i32)>, elf: (i32,i32)) -> Option<(i32,i32)> {
    if !map.contains(&(elf.0+1, elf.1+1)) && !map.contains(&(elf.0, elf.1+1)) && !map.contains(&(elf.0-1, elf.1+1)) {
        return Some((elf.0, elf.1+1));
    }
    return None;
}
fn move_east (map: &HashSet<(i32,i32)>, elf: (i32,i32)) -> Option<(i32,i32)> {
    if !map.contains(&(elf.0+1, elf.1)) && !map.contains(&(elf.0+1, elf.1-1)) && !map.contains(&(elf.0+1, elf.1+1)) {
        return Some((elf.0+1, elf.1));
    }
    return None;
}
fn move_west (map: &HashSet<(i32,i32)>, elf: (i32,i32)) -> Option<(i32,i32)> {
    if !map.contains(&(elf.0-1, elf.1)) && !map.contains(&(elf.0-1, elf.1-1)) && !map.contains(&(elf.0-1, elf.1+1)) {
        return Some((elf.0-1, elf.1));
    }
    return None;
}

fn is_anyone_around(map: &HashSet<(i32,i32)>, elf: (i32,i32)) -> bool {
    return map.contains(&(elf.0, elf.1-1)) || 
           map.contains(&(elf.0+1, elf.1)) || 
           map.contains(&(elf.0-1, elf.1)) || 
           map.contains(&(elf.0, elf.1+1)) ||
           map.contains(&(elf.0+1, elf.1-1)) ||
           map.contains(&(elf.0-1, elf.1-1)) ||
           map.contains(&(elf.0+1, elf.1+1)) ||
           map.contains(&(elf.0-1, elf.1+1));
    }

fn plan_moves(map: &HashSet<(i32,i32)>,plans: VecDeque<fn(&HashSet<(i32,i32)>,(i32,i32)) -> Option<(i32,i32)>>) -> HashMap<(i32,i32), (i32,i32)> {
    let mut planned_moves = HashMap::new();
    for elf in map.iter() {
        if !is_anyone_around(map, *elf) {
            continue;
        }
        for plan in plans.iter() {
            if let Some(target) = plan(map, *elf) {
                planned_moves.insert(*elf, target);
                break;
            }
        }
    }
    return planned_moves;
}

fn filter_moves(planned_moves: &mut HashMap<(i32,i32), (i32,i32)>) {
    let mut to_remove = Vec::new();
    for (elf, target) in planned_moves.iter() {
        for (other_elf, other_target) in planned_moves.iter() {
            if elf != other_elf && target == other_target {
                to_remove.push(*elf);
            }
        }
    }
    for elf in to_remove {
        planned_moves.remove(&elf);
    }
}

fn apply_moves(map: &HashSet<(i32,i32)>, planned_moves: &HashMap<(i32,i32), (i32,i32)>) -> HashSet<(i32,i32)> {
    let mut new_map = HashSet::new();
    for (elf, target) in planned_moves {
        new_map.insert(*target);
    }
    for elf in map {
        if !planned_moves.contains_key(elf) {
            new_map.insert(*elf);
        }
    }
    return new_map;
}

pub fn run(input: &str) -> String {
    let mut map = HashSet::new();
    let mut plans:VecDeque<fn(&HashSet<(i32,i32)>,(i32,i32)) -> Option<(i32,i32)>> = VecDeque::new();
    plans.push_front(move_east);
    plans.push_front(move_west);
    plans.push_front(move_south);
    plans.push_front(move_north);
    for (y, line ) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((x as i32,y as i32));
            }
        }
    };
    print_map(&map);

    let mut number_of_rounds = 0;
    loop {
        number_of_rounds += 1;
        let mut planned_moves = plan_moves(&map, plans.clone());
        filter_moves(&mut planned_moves);
        if planned_moves.len() == 0 {
            break;
        }
        map = apply_moves(&map, &planned_moves);
        let top = plans.pop_front().unwrap();
        plans.push_back(top);
        print_map(&map);
        if number_of_rounds % 100 == 0 {
            println!("Number of rounds: {}", number_of_rounds);
        }
    }
    let count = get_free_map_spaces(&map);
    println!("Free Spaces: {}", count);
    return number_of_rounds.to_string();

}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string(format!("data/day23.txt"))
        .expect("Something went wrong reading the file");
        assert_eq!("6032", run(&file));
    }
}
