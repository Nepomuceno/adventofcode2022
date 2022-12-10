use std::{collections::{VecDeque}};


pub fn run(input: &str) -> String {
    let mut total_stregh = 0;
    let lines = input.lines();
    let mut cycles = 0;
    let mut value = 1;
    let mut strengts = VecDeque::new();

    for line in lines {
        let command = line.split(" ").collect::<VecDeque<&str>>();
        cycles += 1;
        strengts.push_back(value * cycles);
        if command.len() == 2 {
            cycles += 1;
            strengts.push_back(value * cycles);
            value += command[1].trim().parse::<i32>().unwrap();
        }
    }
    let mut mod_20 = true;
    let mut i = 0;
    for j in 0..strengts.len() {
        i += 1;
        if mod_20 && i == 20 {
            total_stregh += strengts[j];
            mod_20 = false;
            println!("{} {} {}", i, strengts[j], j);
            i = 0;
        } else if i == 40 { 
            total_stregh += strengts[j];
            println!("{} {} {}", i, strengts[j], j);
            i = 0;
        }
        
    }
    return total_stregh.to_string();
}

