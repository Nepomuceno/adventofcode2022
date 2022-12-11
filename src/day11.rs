use std::{collections::{VecDeque}};

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Operation {
    Multiply,
    Add,
    Doubles,
    Squares,
}
#[derive(Debug)]
#[derive(Clone)]
struct Monkey {
    name: String,
    test: u32,
    true_index: i32,
    false_index: i32,
    operation: Operation,
    operand: u32,
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
                lines[2].split(" ").last().unwrap().parse::<u32>().unwrap()
            } else {
                0
            };
        let test = lines[3].replace("Test: divisible by ", "").trim().parse::<u32>().unwrap();
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
    fn get_items(content: &str, monkeys: &VecDeque<Monkey>) -> VecDeque<VecDeque<u32>> {
        let lines = content.lines().collect::<VecDeque<&str>>();
        lines[1]
            .split_once(":").unwrap().1.split(",")
            .map(
                |x| {
                    let number = x.trim().parse::<u32>().unwrap();
                    let mut result = VecDeque::new();
                    for monkey in monkeys {
                        result.push_back(number % monkey.test);
                    }
                    result
                }
            )
            .collect::<VecDeque<VecDeque<u32>>>()
    }

        
}

pub fn run(input: &str) -> String {
    let monkeys_input = input.split("\n\n").collect::<Vec<&str>>();
    let monkeys = monkeys_input.iter().map(|x| Monkey::new(x)).collect::<VecDeque<Monkey>>();
    let mut monkey_items = VecDeque::new();
    let mut monkeys_busines = VecDeque::new();
    for monkey_input in monkeys_input {
        monkey_items.push_back(Monkey::get_items(monkey_input, &monkeys));
        monkeys_busines.push_back(0u32)
    }
    println!("{:?}", monkey_items);
    println!("{:?}", monkeys);
    for i in 0..10000 {
        for j in 0..monkeys.len() {
            while monkey_items[j].len() > 0 {
                monkeys_busines[j] += 1;
                let mut item = monkey_items[j].pop_front().unwrap();
                for k in 0..monkeys.len() {
                    if monkeys[j].operation == Operation::Multiply {
                        item[k] = (item[k] * monkeys[j].operand) % monkeys[k].test;
                    } else if monkeys[j].operation == Operation::Add {
                        item[k] = (item[k] + monkeys[j].operand) % monkeys[k].test;
                    } else if monkeys[j].operation == Operation::Doubles {
                        item[k] = (item[k] * 2) % monkeys[k].test; 
                    } else {
                        item[k] = (item[k] * item[k]) % monkeys[k].test;
                    }
                }
                if item[j] == 0 {
                    monkey_items[monkeys[j].true_index as usize].push_back(item);
                } else {
                    monkey_items[monkeys[j].false_index as usize].push_back(item);
                }

            }
        }
        if i % 1000 == 0 {
            println!("===== {} =====", i);
            println!("{:?}\n", monkey_items);
            println!("{:?}\n", monkeys_busines);
        }
    }
    println!("{:?}",monkeys_busines);
    monkeys_busines.make_contiguous().sort_by(|a, b| b.cmp(a));
    (monkeys_busines[0] as u64 * monkeys_busines[1] as u64).to_string()
}

