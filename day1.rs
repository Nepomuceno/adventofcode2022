use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() -> std::io::Result<()> {
    let file = File::open("day1_input.txt")
                                .expect("File exists");
    let  buf_reader = BufReader::new(file);
    let mut max  = 0;
    let mut current = 0;
    for line in buf_reader.lines() {
        match line {
            Ok(x) => {
                if x.len() > 0 {
                    let num: i32 = x.trim().parse().expect("Please give a number");
                    current += num;
                } else {
                    if current >= max {
                        max = current;
                    }
                    current = 0;
                }
            },
            _ => {}
        }
    }
    if current > max {
        max = current;
    }
    println!("Max: {max}");
    Ok(())
}