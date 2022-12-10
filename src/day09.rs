use std::{collections::{VecDeque, HashSet}};

pub fn find_move (target: (i32,i32), chasing: (i32,i32)) -> (i32,i32) {
    let result = (chasing.0 - target.0, chasing.1 - target.1);
    if !(result.0.abs() > 1 || result.1.abs() > 1) {
        return (0,0);
    } 
    match result {
        (2,0) => (-1,0),
        (2,1) => (-1,-1),
        (2,-1) => (-1,1),
        (2,2) => (-1,-1),
        (0,-2) => (0,1),
        (1,-2) => (-1,1),
        (-1,-2) => (1,1),
        (-2,-2) => (1,1),
        (-2,0) => (1,0),
        (-2,1) => (1,-1),
        (-2,-1) => (1,1),
        (0,2) => (0,-1),
        (1,2) => (-1,-1),
        (-1,2) => (1,-1),
        (-2,2) => (1,-1),
        (2, -2) => (-1,1),
        _ => panic!("Unknown move: {:?}", result),
    }
}

pub fn print_grid(grid: VecDeque<(i32,i32)>) {
    for i in (0..25).rev() {
        for j in 0..25 {
            let mut found = false;
            'intern: for k in 0..grid.len() {
                if grid[k].1 == i && grid[k].0 == j {
                    if k == 0 {
                        print!("H ");
                    } else {
                        print!("{k} ");
                    }
                    found = true;
                    break 'intern;
                }
            }
            if !found {
                if j == 11 && i == 5 {
                    print!("s ");
                } else {
                    print!(". ");
                }
            }
        }
        println!("");
    }
}

pub fn run(input: &str) -> String {
    let mut not_9_visit: HashSet<(i32,i32)> = HashSet::new();
    let mut rope: VecDeque<(i32,i32)> = [
        (11,5),
        (11,5),(11,5),(11,5),
        (11,5),(11,5),(11,5),
        (11,5),(11,5),(11,5),
    ].iter().cloned().collect();
    let lines = input.lines();
    for line in lines {
        println!("== {} ==", line);
        let content = line.split_once(" ").unwrap();
        let moves = content.1.parse::<u32>().unwrap();
        for _ in 0..moves {
            match content.0 {
                "R" =>  { 
                    rope[0].0 += 1;
                }
                "L" => {
                    rope[0].0 -= 1;
                },
                "U" => {
                    rope[0].1 += 1;
                },
                "D" => {
                    rope[0].1 -= 1;
                },
                _ => panic!("Unknown direction"),
            }
            for i in 1..10 {
                let current_move = find_move(rope[i-1], rope[i]);
                rope[i].0 += current_move.0;
                rope[i].1 += current_move.1;
            }
            print_grid(rope.clone());
            println!("----------");  
            let tail_clone = (rope[9].0, rope[9].1);
            not_9_visit.insert(tail_clone);   
                 
        }
        print_grid(rope.clone());
        println!("----------");  
        println!("positions: {:?}\n", rope);

    }
    println!("tail_visit: {:?}", not_9_visit);
    return not_9_visit.len().to_string();
}

