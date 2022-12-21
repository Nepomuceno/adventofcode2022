use std::{time::Instant, collections::{HashSet, HashMap}};

use regex::Regex;



const PRINT_ENABLED: bool = true;


pub fn run(input: &str) -> String {
    let start = Instant::now();
    let operation_pattern = Regex::new(r"(\w+): (\w+) ([\+/\-\*]) (\w+)").unwrap();
    let rolved_pattern = Regex::new(r"(\w+): (\d+)").unwrap();
    let mut unsolved_monkeys = HashMap::new();
    let mut solved_monkeys = HashMap::new();
    for line in input.lines() {
        println!("Line: {}", line);
        if operation_pattern.is_match(line) {
            let matches = operation_pattern.captures(line).unwrap();
            unsolved_monkeys.insert(matches[1].to_string(), (matches[2].to_string(), matches[3].to_string(), matches[4].to_string()));
        } else  {
            let matches = rolved_pattern.captures(line).unwrap();
            solved_monkeys.insert(matches[1].to_string(), matches[2].to_string().parse::<i64>().unwrap());
        }
    }
    while !solved_monkeys.contains_key("root") {
        for (monkey, (left, op, right)) in unsolved_monkeys.clone().iter() {
            if solved_monkeys.contains_key(left) && solved_monkeys.contains_key(right) {
                let left = solved_monkeys[left];
                let right = solved_monkeys[right];
                let result = match op.as_str() {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    _ => panic!("Unknown operation"),
                };
                solved_monkeys.insert(monkey.to_string(), result);
                unsolved_monkeys.remove(monkey);
            }
        }
        println!("Solved: {:?}", solved_monkeys.len());
        println!("Unsolved: {:?}", unsolved_monkeys.len());
    }

    if PRINT_ENABLED {
        println!("Elapsed: {:?}", start.elapsed());
    }
    solved_monkeys["root"].to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string(format!("data/day21.txt"))
        .expect("Something went wrong reading the file");
        assert_eq!("152", run(&file));
    }
}
