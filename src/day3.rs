use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};


fn get_letter_in_common(a : String, b: String, c: String) -> char {
    for l in a.chars() {
        if b.contains(l) && c.contains(l){
            return l
        }
    }
    return '0';
}
fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
fn main() -> std::io::Result<()> {
    let file_lines = lines_from_file("data/day3_input.txt")
                                .expect("File exists");
    let elves_group = file_lines.chunks(3);
    let letters = elves_group.map(|group| {
        let letter = get_letter_in_common(
            group.get(0).expect("Not Found").to_owned(),
            group.get(1).expect("Not Found").to_owned(),
            group.get(2).expect("Not found").to_owned());
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