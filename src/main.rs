use std::env::{args};
use std::fs;
use chrono::prelude::*;

mod day01;
mod day02;
mod day03;
mod day04;

type DayFunction = fn(&str) -> String;

const ADVENTDAYS: [DayFunction;4] = [
    day01::run,
    day02::run,
    day03::run,
    day04::run,
];

fn main() {
    
    let args: Vec<String>= args().collect();
    let mut day: u32 = Utc::now().day();
    
    if args.len() == 2 {
        day = args[1].parse().expect("Please give a number");
    }
    // Get the current date and time
    println!("Day: {day:02}");
    // Read a file for that day
    let file = fs::read_to_string(format!("data/day{day:02}.txt"))
        .expect("Something went wrong reading the file");
    // Run the function for that day
    let result = ADVENTDAYS[(day-1) as usize](&file);
    println!("Result: {}", result);
    
}
