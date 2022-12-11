use std::{collections::{VecDeque}};

#[derive(Debug)]
#[derive(PartialEq)]
enum Operation {
    Multiply,
    Add,
    Doubles,
    Squares,
}
#[derive(Debug)]
struct Monkey {
    name: String,
    test: i32,
    true_index: i32,
    false_index: i32,
    operation: Operation,
    operand: i32,
}

impl Monkey {
    fn new(content: &str) -> Monkey {
        let lines = content.lines().collect::<VecDeque<&str>>();
        let name = lines[0].split(" ").last().unwrap();
        let operation = 
            if lines[2].contains("old * old") { 
                Operation::Squares
            } else if lines[2].contains("old + old") { 
                Operation::Doubles
            } else if lines[2].contains("*") { 
                Operation::Multiply
            } else { 
                Operation::Add
            };
        let operand = 
            if operation == Operation::Multiply || operation == Operation::Add {
                lines[2].split(" ").last().unwrap().parse::<i32>().unwrap()
            } else {
                0
            };
        let test = lines[3].replace("Test: divisible by ", "").trim().parse::<i32>().unwrap();
        let true_index = lines[4].replace("If true: throw to monkey ", "").trim().parse::<i32>().unwrap();
        let false_index = lines[5].replace("If false: throw to monkey ", "").trim().parse::<i32>().unwrap();
        println!("{:?}", (name, test, true_index, false_index, &operation, operand));
        Monkey {
            name: name.to_string(),
            test: test,
            true_index: true_index,
            false_index: false_index,
            operation: operation,
            operand: operand,
        }
    }
    fn get_items(content: &str) -> VecDeque<i32> {
        let lines = content.lines().collect::<VecDeque<&str>>();
        lines[1]
            .split_once(":").unwrap().1.split(",")
            .map(
                |x| {
                    x.trim().parse::<i32>().unwrap()
                }
            )
            .collect::<VecDeque<i32>>()
    }

        
}

pub fn run(input: &str) -> String {
    let monkeys_input = input.split("\n\n").collect::<Vec<&str>>();
    let monkeys = monkeys_input.iter().map(|x| Monkey::new(x)).collect::<Vec<Monkey>>();
    let mut monkey_items = VecDeque::new();
    let mut monkeys_busines = VecDeque::new();
    for monkey_input in monkeys_input {
        monkey_items.push_back(Monkey::get_items(monkey_input));
        monkeys_busines.push_back(0u32)
    }
    println!("{:?}", monkey_items);
    println!("{:?}", monkeys);
    for i in 0..20 {
        for j in 0..monkeys.len() {
            let monkey = &monkeys[j];
            while monkey_items[j].len() > 0 {
                monkeys_busines[j] += 1;
                let item = monkey_items[j].pop_front().unwrap();
                let mut result = 
                    if monkey.operation == Operation::Multiply {
                        item * monkey.operand
                    } else if monkey.operation == Operation::Add {
                        item + monkey.operand
                    } else if monkey.operation == Operation::Squares {
                        item * item
                    } else {
                        item + item
                    };
                result = result / 3;
                if result % monkey.test == 0 {
                    monkey_items[monkey.true_index as usize].push_back(result);
                } else {
                    monkey_items[monkey.false_index as usize].push_back(result);
                }

            }
        }

        println!("===== {} =====", i);
        println!("{:?}\n", monkey_items);
        println!("{:?}\n", monkeys_busines);
    }
    format!("{:?}",monkeys_busines).to_string()
}

