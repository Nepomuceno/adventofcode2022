use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() -> std::io::Result<()> {
    let file = File::open("data/day2_input.txt")
                                .expect("File exists");
    let buf_reader = BufReader::new(file);
    let mut score = 0;
    for line in buf_reader.lines() {
        let mut round = 0;
        match line {
            Ok(x) => {
                match x.as_str() {
                    "A X" => round = 3,
                    "A Y" => round = 4,
                    "A Z" => round = 8,
                    "B X" => round = 1,
                    "B Y" => round = 5,
                    "B Z" => round = 9,
                    "C X" => round = 2,
                    "C Y" => round = 6,
                    "C Z" => round = 7,
                    _ => panic!("Invalid score")
                }
                score += round;
                println!("round: {x} score: {round} / {score} ")
            },
            Err(_) => panic!("invalid line"),
        }
    }
    Ok(())
}