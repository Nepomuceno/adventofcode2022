pub fn run(input: &str) -> String {
    let mut elfs_calories: Vec<i32> = vec![];
    let mut current = 0;
    for line in input.lines() {
        if line.len() > 0 {
            let num: i32 = line.trim().parse().expect("Please give a number");
            current += num;
        } else {
            elfs_calories.push(current);
            current = 0;
        }
    }
    elfs_calories.push(current);
    elfs_calories.sort();
    elfs_calories.reverse();
    let mut total = 0 ;
    for el in elfs_calories[..3].to_vec() {
        total += el;
    }
    total.to_string()
}
