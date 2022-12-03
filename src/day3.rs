use std::fs::File;
use std::io::{BufReader, BufRead};

fn get_letter_in_common(a : String, b: String) -> char {
    for l in a.chars() {
        if b.contains(l){
            return l
        }
    }
    return '0';
}
fn main() -> std::io::Result<()> {
    let file = File::open("data/day3_input.txt")
                                .expect("File exists");
    let buf_reader = BufReader::new(file);
    let letters = buf_reader.lines().map(|file| {
        let file_str = file.expect("line not found");
        let compartments = file_str.split_at(file_str.len()/2);
        let letter = get_letter_in_common(compartments.0.to_string(),compartments.1.to_string());
        if letter.is_ascii_uppercase() {
            letter as u32 - 38
        } else {
            letter as u32 - 96
        }
    });
    let result: u32 = letters.sum();
    println!("{}", result);
    
    Ok(())
}