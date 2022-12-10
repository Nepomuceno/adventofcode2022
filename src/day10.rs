use std::{collections::{VecDeque}};

pub fn print_content (x: i32, cycles: i32) {
    if x - cycles > 0 || x - cycles < -2 {
        print!(" ");
    } else {
        print!("#");
    }
    if cycles % 40 == 0 {
        println!();
    }
}

pub fn run(input: &str) -> String {
    let lines = input.lines().collect::<VecDeque<&str>>();
    let mut cycles = 0;
    let mut x = 1;

    for i in 0..lines.len() {
        cycles += 1;
        let command = lines[i].split(" ").collect::<VecDeque<&str>>();
        print_content(x, cycles);
        if cycles == 40 {
            cycles = 0;
        }
        if command.len() == 2 {
            cycles += 1;
            print_content(x, cycles);
            if cycles == 40 {
                cycles = 0;
            }    
            x += command[1].trim().parse::<i32>().unwrap();
        }
    }
    return 0.to_string();
}

