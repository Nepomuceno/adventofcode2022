use core::time;
use std::{time::{Instant, Duration}, collections::{HashMap, HashSet, VecDeque}, thread};
use termion::{color, style,cursor};


const PRINT_ENABLED: bool = true;

fn print_map(max_x: i32, max_y: i32, elf_positions: &HashSet<(i32,i32)>, walls: &HashSet<(i32,i32)>, blizzards: &Vec<(i32,i32,char)>) {
    if !PRINT_ENABLED {
        return;
    }
    print!("{}", cursor::Goto(1,1));
    for y in 0..max_y {
        for x in 0..max_x {
            if elf_positions.contains(&(x,y)) {
                print!("{}", color::Fg(color::Green));
                print!("E");
                print!("{}", style::Reset);
            } else if walls.contains(&(x,y)) {
                print!("{}", color::Fg(color::Red));
                print!("#");
                print!("{}", style::Reset);
            } else if blizzards.iter().any(|(bx,by,_)| *bx == x && *by == y) {
                print!("{}", color::Fg(color::Blue));
                print!("o");
                print!("{}", style::Reset);
            } else {
                print!(" ");
            }
        }
        println!();
    }
    thread::sleep(Duration::from_millis(40));
}

fn multiply_elves(elf_positions: &HashSet<(i32,i32)>, max_x: i32, max_y: i32) -> HashSet<(i32,i32)> {
    let mut new_elf_positions = HashSet::new();
    for (x,y) in elf_positions {
        new_elf_positions.insert((*x,*y));
        if *x > 0 {
            new_elf_positions.insert((*x-1,*y));
        }
        if *x < max_x - 1 {
            new_elf_positions.insert((*x+1,*y));
        }
        if *y > 0 {
            new_elf_positions.insert((*x,*y-1));
        }
        if *y < max_y - 1 {
            new_elf_positions.insert((*x,*y+1));
        }
    }
    new_elf_positions
}

fn move_blizzards(blizzards: &Vec<(i32,i32,char)>, max_x: i32, max_y: i32) -> Vec<(i32,i32,char)> {
    let mut new_blizzards = vec![];
    for (x,y,c) in blizzards {
        let (mut new_x, mut new_y) = match c {
            '>' => (x+1,*y),
            '<' => (x-1,*y),
            '^' => (*x,y-1),
            'v' => (*x,y+1),
            _ => panic!("Unknown blizzard direction"),
        };
        if new_x == max_x - 1 {
            new_x = 1;
        }
        if new_x == 0 {
            new_x = max_x - 2;
        }
        if new_y == max_y - 1 {
            new_y = 1;
        }
        if new_y == 0 {
            new_y = max_y - 2;
        }
        new_blizzards.push((new_x,new_y,*c));
    }
    new_blizzards
}

pub fn run(input: &str) -> String {
    let mut blizzards = vec![];
    let mut walls = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '>' || c == '<' || c == '^' || c == 'v' {
                blizzards.push((x as i32,y as i32,c));
            }
            if c == '#' {
                walls.insert((x as i32,y as i32));
            }
        }
    }
    let map_x_length = input.lines().next().unwrap().len() as i32;
    let map_y_length = input.lines().count() as i32;

    let mut elf_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut start_position = (1,0);
    elf_positions.insert(start_position);
    let mut goal_position = (map_x_length-2, map_y_length-1);
    let mut rounds = 0;
    loop {
        rounds += 1;
        if elf_positions.len() == 0 {
            panic!("No elves left");
        }
        elf_positions = multiply_elves(&elf_positions, map_x_length, map_y_length);
        blizzards = move_blizzards(&blizzards, map_x_length, map_y_length);
        elf_positions = elf_positions.difference(&blizzards.iter().map(|(x,y,_)| (*x,*y)).collect()).cloned().collect();
        elf_positions = elf_positions.difference(&walls).cloned().collect();
        print_map(map_x_length, map_y_length, &elf_positions, &walls, &blizzards);
        if elf_positions.contains(&goal_position) {
            break;
        }
    }
    rounds.to_string()
    
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string(format!("data/day24.txt"))
        .expect("Something went wrong reading the file");
        assert_eq!("6032", run(&file));
    }
}
