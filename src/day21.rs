use std::{time::Instant, collections::{HashMap}};

use regex::Regex;



const PRINT_ENABLED: bool = true;

#[derive(Debug)]
enum Dynamic {
    String(String),
    Int(i64),
    Operation(Box<Operation>),
}

#[derive(Debug)]
struct Operation {
    left: Dynamic,
    right: Dynamic,
    operation: String,
}


pub fn run(input: &str) -> String {
    let start = Instant::now();
    let operation_pattern = Regex::new(r"(\w+): (\w+) ([\+/\-\*]) (\w+)").unwrap();
    let rolved_pattern = Regex::new(r"(\w+): (\d+)").unwrap();
    let mut unsolved_monkeys = HashMap::new();
    let mut solved_monkeys: HashMap<String, i64> = HashMap::new();
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
    solved_monkeys.remove(&"humn".to_string());
    unsolved_monkeys.remove(&"humn".to_string());
    let root = unsolved_monkeys.remove(&"root".to_string()).unwrap();
    println!("Unsolved: {:?}", unsolved_monkeys.len());
    println!("Solved: {:?}", solved_monkeys.len());
    while unsolved_monkeys.len() > 0 {
        let mut one_solved = false;
        for (key, value) in unsolved_monkeys.clone().iter() {
            if solved_monkeys.contains_key(&value.0) && solved_monkeys.contains_key(&value.2) {
                let left = solved_monkeys.get(&value.0).unwrap();
                let right = solved_monkeys.get(&value.2).unwrap();
                let result = match value.1.as_str() {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    _ => panic!("Unknown operation"),
                };
                solved_monkeys.insert(key.to_string(), result);
                unsolved_monkeys.remove(key);
                one_solved = true;
            }
        }
        if !one_solved {
            break;
        }
    }
    println!("Unsolved: {:?}", unsolved_monkeys.len());
    println!("Solved: {:?}", solved_monkeys.len());

    if PRINT_ENABLED {
        println!("Elapsed: {:?}", start.elapsed());
    }
    let mut equality = Operation {
        left: Dynamic::String(root.0.to_string()),
        right: Dynamic::String(root.2.to_string()),
        operation: "==".to_string(),
    };
    loop {
        equality.left = expand(equality.left, &solved_monkeys, &unsolved_monkeys);
        equality.right = expand(equality.right, &solved_monkeys, &unsolved_monkeys);
        // check if equality contains humn
        if check_if_contain_node(&equality.left, "humn") || check_if_contain_node(&equality.right, "humn") {
            break;
        }
    }
    print_operation(&Dynamic::Operation(Box::new(equality)), &solved_monkeys);
    "".to_string()
}

fn print_operation(operation: &Dynamic, solved_monkeys: &HashMap<String,i64>) {
    match operation {
        Dynamic::String(string) => {
            if solved_monkeys.contains_key(string) {
                print!("{}", solved_monkeys.get(string).unwrap());
            } else {
                print!("{}", string);
            }
        },
        Dynamic::Int(int) => {
            print!("{}", int);
        },
        Dynamic::Operation(operation) => {
            print!("(");
            print_operation(&operation.left, solved_monkeys);
            print!(" {} ", operation.operation);
            print_operation(&operation.right, solved_monkeys);
            print!(")");
        }
    }
}
    
fn check_if_contain_node(dynamic: &Dynamic, node: &str) -> bool {
    match dynamic {
        Dynamic::String(string) => {
            if string == node {
                true
            } else {
                false
            }
        },
        Dynamic::Int(_) => {
            false
        },
        Dynamic::Operation(operation) => {
            check_if_contain_node(&operation.left, node) || check_if_contain_node(&operation.right, node)
        }
    }
}

fn expand(dynamic: Dynamic, solved_monkeys: &HashMap<String,i64>, unsolved_monkeys: &HashMap<String, (String,String,String)>) -> Dynamic {
    match dynamic {
        Dynamic::String(string) => {
            if solved_monkeys.contains_key(&string) {
                Dynamic::Int(*solved_monkeys.get(&string).unwrap())
            } else {
                if string == "humn" {
                    Dynamic::String(string.to_string())
                } else {
                    let monkey = unsolved_monkeys.get(&string).unwrap();
                    let operation = Operation {
                        left: Dynamic::String(monkey.0.to_string()),
                        right: Dynamic::String(monkey.2.to_string()),
                        operation: monkey.1.to_string(),
                    };
                    println!("Operation: {:?}", operation);
                    Dynamic::Operation(Box::new(operation))
                }
            }
        },
        Dynamic::Int(int) => {
            Dynamic::Int(int)
        },
        Dynamic::Operation(operation) => {
            Dynamic::Operation(Box::new(Operation {
                left: expand(operation.left, solved_monkeys, unsolved_monkeys),
                right: expand(operation.right, solved_monkeys, unsolved_monkeys),
                operation: operation.operation.to_string(),
            }))
        }
    }
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
