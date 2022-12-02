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
                let decrypted = x.replace("X", "A").replace("Y", "B").replace("Z", "C");
                match decrypted.as_str() {
                    "A A" => round = 4,
                    "A B" => round = 8,
                    "A C" => round = 3,
                    "B A" => round = 1,
                    "B B" => round = 5,
                    "B C" => round = 9,
                    "C A" => round = 7,
                    "C B" => round = 2,
                    "C C" => round = 6,
                    _ => panic!("Invalid score")
                }
                score += round;
                println!("round: {decrypted} score: {round} / {score} ")
            },
            Err(_) => panic!("invalid line"),
        }
    }
    score += 1;
    Ok(())
}