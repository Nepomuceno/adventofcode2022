use std::env::{args};
use std::fs;
use chrono::prelude::*;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;

type DayFunction = fn(&str) -> String;

const ADVENTDAYS: [DayFunction;24] = [
    day01::run,
    day02::run,
    day03::run,
    day04::run,
    day05::run,
    day06::run,
    day07::run,
    day08::run,
    day09::run,
    day10::run,
    day11::run,
    day12::run,
    day13::run,
    day14::run,
    day15::run,
    day16::run,
    day17::run,
    day18::run,
    day19::run,
    day20::run,
    day21::run,
    day22::run,
    day07::run,
    day08::run,
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
