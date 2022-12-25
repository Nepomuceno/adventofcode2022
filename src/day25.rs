use core::time;
use std::{time::{Instant, Duration}, collections::{HashMap, HashSet, VecDeque}, thread};
use termion::{color, style,cursor};


const PRINT_ENABLED: bool = true;

fn covert_to_snafu(input: i64) -> String {
    let mut current = input;
    let mut snafu = "".to_string();
    while current > 0 {
        let remainder = current % 5;
        current = current / 5;
        match remainder {
            0 => { snafu = "0".to_string() + &snafu; },
            1 => { snafu = "1".to_string() + &snafu; },
            2 => { snafu = "2".to_string() + &snafu; },
            3 => { 
                    snafu = "=".to_string() + &snafu;
                    current = current + 1;
                 },
            4 => { 
                    snafu = "-".to_string() + &snafu;
                    current = current + 1; 
                 },
            _ => { panic!("Invalid remainder"); }
        }
    }
    snafu
}

pub fn run(input: &str) -> String {
    let mut total_file = 0;
    for line in input.lines() {
        let mut total:i64 = 0;
        let line_chars = line.chars().collect::<Vec<char>>();
        for i in 0..line.len() {
            let m:i64 = 5_i64.pow((line.len()-i-1) as u32);
            match line_chars[i] {
                '0' => { total = total + (0 * m); },
                '1' => { total = total + (1 * m); },
                '2' => { total = total + (2 * m); },
                '-' => { total = total + (-1 * m); },
                '=' => { total = total + (-2 * m); },
                _ => { panic!("Invalid character"); }
            }
        }
        total_file = total_file + total;
    }
    let result = covert_to_snafu(total_file);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string(format!("data/day25.txt"))
        .expect("Something went wrong reading the file");
        assert_eq!("6032", run(&file));
    }
}
