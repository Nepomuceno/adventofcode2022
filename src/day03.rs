fn get_letter_in_common(a : &str, b: &str, c: &str) -> char {
    for l in a.chars() {
        if b.contains(l) && c.contains(l){
            return l
        }
    }
    return '0';
}

pub fn run(input: &str) -> String {
    let lines = &input.lines().collect::<Vec<&str>>(); 
    let elves_group = lines.chunks(3);
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
    result.to_string()
}