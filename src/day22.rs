use std::{time::{Instant, Duration}, collections::{HashMap}, thread};
use termion::{color, style,cursor};


const MAP_SIZE: usize = 4;

const PRINT_ENABLED: bool = false;

fn print_map(map: &HashMap<(usize,usize),char>, visited: &HashMap<(usize,usize),char>, current_position: &(usize,usize,char)) {
    print!("{}{}", termion::clear::All, cursor::Goto(1, 1));
    let max_x = map.keys().map(|(x,_)| *x).max().unwrap();
    let max_y = map.keys().map(|(_,y)| *y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let mut c = map.get(&(x,y)).unwrap_or(&' ');
            if *c == '.' {
                 c = visited.get(&(x,y)).unwrap_or(&'.');
            }
            if current_position.0 == x && current_position.1 == y {
                match current_position.2 {
                    '>' => c = &'→',
                    '<' => c = &'←',
                    '^' => c = &'↑',
                    'v' => c = &'↓',
                    _ => panic!("Unknown direction"),
                }
            }
            match c {
                '#' => print!("{}#", color::Fg(color::Red)),
                '.' => print!("{}.", color::Fg(color::Rgb(30,30,30))),
                '>' | '<' | '^' | 'v' => print!("{}{}{}", color::Fg(color::Rgb(80,80,80)), style::Reset, c),
                '→' | '←' | '↑' | '↓' => print!("{}{}{}", color::Fg(color::Green), style::Bold, c),
                _ => {print!(" ")},
            }
            print!("{}{}", color::Fg(color::Reset), style::Reset);
        }
        println!();
    }
    println!();
}

fn move_in_map(map: &HashMap<(usize,usize),char>, visited: &mut HashMap<(usize,usize),char>, current_position: &mut (usize,usize,char), move_by: usize)
 -> (HashMap<(usize,usize),char>, (usize,usize,char)) {
    if PRINT_ENABLED {
        println!("Current Position: {:?} Move by {}", current_position, move_by);
    }
    let mut new_x = current_position.0;
    let mut new_y = current_position.1;
    let d = current_position.2;
    let max_x = 
    map.keys()
       .filter(|(_,y)| y == &current_position.1)
       .map(|(x,_)| *x).max().unwrap();
    let min_x = 
       map.keys()
          .filter(|(_,y)| y == &current_position.1)
          .map(|(x,_)| *x).min().unwrap();
    let max_y =
         map.keys()
             .filter(|(x,_)| x == &current_position.0)
             .map(|(_,y)| *y).max().unwrap();
    let min_y = 
         map.keys()
             .filter(|(x,_)| x == &current_position.0)
             .map(|(_,y)| *y).min().unwrap();
    for _ in 0..move_by {
        match d {
            '>' => {
                new_x += 1;
                if new_x > max_x {
                    new_x = min_x;
                }
            },
            '<' => {
                if new_x == 0 {
                    new_x = max_x;
                } else {
                    new_x -= 1;
                    if new_x < min_x {
                        new_x = max_x;
                    }
                } 
                
            },
            '^' => {
                if new_y == 0{
                    new_y = max_y;
                } else {
                    new_y -= 1;
                    if new_y < min_y {
                        new_y = max_y;
                    }
                }
            },
            'v' => {
                new_y += 1;
                if new_y > max_y {
                    new_y = min_y;
                }
            },
            _ => panic!("Unknown direction"),
        }
        let e = map.get(&(new_x,new_y)).unwrap();
        if e == &'.' {
            current_position.0 = new_x;
            current_position.1 = new_y;
            visited.insert((new_x,new_y),d);
        }
        if e == &'#' {
            break;
        }
    }
    if PRINT_ENABLED {
        print_map(map,visited,current_position);
        thread::sleep(Duration::from_millis(100));
        println!("New Position: {:?}", current_position);
        //println!("Visited: {:?}", visited);
    }
    (visited.clone(), *current_position)
}

pub fn run(input: &str) -> String {
    let start = Instant::now();

    let (map_string, instructions_string) = input.split_once("\n\n").unwrap();
    let mut map = HashMap::new();
    for (y,line) in map_string.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != ' ' {
                map.insert((x,y),c);
            }
        }
    }
    //start position it is the first line and the first . chart in the line
    let start_x = map.iter()
                                                    .filter(|x| x.0.1 == 0)
                                                    .filter(|x| *x.1 == '.')
                                                    .map(|x| x.0.0)
                                                    .min().unwrap();
    let mut current_position = (start_x, 0 as usize, '>');
    let mut visited = HashMap::new();

    print_map(&map,&visited,&current_position);
    
    let mut move_by:String = "0".to_string();
    for c in instructions_string.chars() {
        println!("Current Position: {:?}", current_position);
        match c {
            'L' => {
                (visited, current_position) = move_in_map(&map, &mut visited, &mut current_position, move_by.parse::<usize>().unwrap());
                move_by = "0".to_string();
                match current_position.2 {
                    '>' => current_position.2 = '^',
                    '<' => current_position.2 = 'v',
                    '^' => current_position.2 = '<',
                    'v' => current_position.2 = '>',
                    _ => panic!("Unknown direction"),
                };
            },
            'R' => {
                (visited, current_position) = move_in_map(&map, &mut visited, &mut current_position, move_by.parse::<usize>().unwrap());
                move_by = "0".to_string();
                match current_position.2 {
                    '>' => current_position.2 = 'v',
                    '<' => current_position.2 = '^',
                    '^' => current_position.2 = '>',
                    'v' => current_position.2 = '<',
                    _ => panic!("Unknown direction"),
                };
            },
            _ => {
                move_by = format!("{}{}", move_by, c);
            },
        }
    }
    (visited, current_position) = move_in_map(&map, &mut visited, &mut current_position, move_by.parse::<usize>().unwrap());
    
    print_map(&map, &visited, &current_position);
    println!("Elapsed: {:?}", start.elapsed());
    println!("Visited: {}", visited.len());
    println!("Current position: {:?}", current_position);
    let mut sum;
    match current_position.2 {
        '>' => {
           sum = 0;
        },
        '<' => {
            sum = 2;
        },
        '^' => {
            sum = 3;
        },
        'v' => {
            sum = 1;
        },
        _ => panic!("Unknown direction"),
    }
    sum += (current_position.0 + 1) * 4;
    sum += (current_position.1 + 1) * 1000;
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string(format!("data/day22.txt"))
        .expect("Something went wrong reading the file");
        assert_eq!("6032", run(&file));
    }
}
