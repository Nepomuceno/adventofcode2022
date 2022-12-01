use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() -> std::io::Result<()> {
    let file = File::open("day1_input.txt")
                                .expect("File exists");
    let  buf_reader = BufReader::new(file);
    let mut elfs_calories: Vec<i32> = vec![];
    let mut current = 0;
    for line in buf_reader.lines() {
        match line {
            Ok(x) => {
                if x.len() > 0 {
                    let num: i32 = x.trim().parse().expect("Please give a number");
                    current += num;
                } else {
                    elfs_calories.push(current);
                    current = 0;
                }
            },
            _ => {}
        }
    }
    elfs_calories.push(current);
    elfs_calories.sort();
    elfs_calories.reverse();
    let mut total = 0 ;
    for el in elfs_calories[..3].to_vec() {
        total += el;
        println!("{}",el) 
    }
    println!("Total: {total}");
    Ok(())
}