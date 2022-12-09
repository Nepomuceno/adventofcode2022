use std::{collections::{HashSet}};

pub fn is_touching_tail(head: (i32,i32), tail: (i32,i32)) -> bool {
    if tail.0 >= head.0 - 1 && tail.0 <= head.0 + 1 {
        if tail.1 >= head.1 - 1 && tail.1 <= head.1 + 1 {
            return true;
        }
    }
    return false;
}

pub fn run(input: &str) -> String {
    let mut tail_visit: HashSet<(i32,i32)> = HashSet::new();
    let mut tail: (i32,i32) = (0,0);
    let mut head: (i32,i32) = (0,0);
    let lines = input.lines();
    for line in lines {
        println!("{}", line);
        let content = line.split_once(" ").unwrap();
        let moves = content.1.parse::<u32>().unwrap();
        for _ in 0..moves {
            match content.0 {
                "R" =>  { 
                    head.0 += 1;
                    if !is_touching_tail(head, tail) {
                        tail.0 += 1;
                        tail.1 = head.1;
                    }
                }
                "L" => {
                    head.0 -= 1;
                    if !is_touching_tail(head, tail) {
                        tail.0 -= 1;
                        tail.1 = head.1;
                    }
                },
                "U" => {
                    head.1 += 1;
                    if !is_touching_tail(head, tail) {
                        tail.1 += 1;
                        tail.0 = head.0;
                    }
                },
                "D" => {
                    head.1 -= 1;
                    if !is_touching_tail(head, tail) {
                        tail.1 -= 1;
                        tail.0 = head.0;
                    }
                },
                _ => panic!("Unknown direction"),
            }
            println!("head: {:?}, tail: {:?}", head, tail);
            let tail_clone = (tail.0, tail.1);
            tail_visit.insert(tail_clone);
        
        }
    }
    println!("tail_visit: {:?}", tail_visit);
    return tail_visit.len().to_string();
}

