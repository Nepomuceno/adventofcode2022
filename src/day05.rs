use std::collections::VecDeque;
use regex::Regex;



pub fn run(input: &str) -> String {
    let mut containers: VecDeque<VecDeque<String>> =  VecDeque::new();
    let mut lines = input.lines().collect::<VecDeque<_>>();
    let number_of_stacks = (lines[0].len()/4)+1;
    for _ in 0..number_of_stacks {
        containers.push_front(VecDeque::new())
    }
    loop {
        let content = lines.pop_front().unwrap();
        let mut rest = content;
        let mut current;
        let mut i = 0;
        loop {
            if rest.len() < 4 {
                let result = rest.trim().replace('[', "").replace(']', "");
                if result.len() > 0 {
                    containers[i].push_back(result);
                }
                break;
            }
            (current, rest) = rest.split_at(4);
            let result = current.trim().replace('[', "").replace(']', "");
            if result == "1" {
                break;
            }
            if result.len() > 0 {
                containers[i].push_back(result);
            }
            i += 1;
        }
        if content.len() == 0 {
            break;
        }
    }

    println!("{containers:?}");
    let re = Regex::new(r"^move (\d*) from (\d*) to (\d*)$").unwrap();
    for line in lines {
        let captures = re.captures(line).unwrap();
        let number = captures[1].parse::<usize>().unwrap();
        let from = captures[2].parse::<usize>().unwrap() - 1;
        let to = captures[3].parse::<usize>().unwrap() - 1;
        let mut transvfer_vec:VecDeque<String> = VecDeque::new();
        for _ in 0..number {
            let container = containers[from].pop_front().unwrap();
            transvfer_vec.push_back(container)
        }
        for _ in 0..number {
            let container = transvfer_vec.pop_back().unwrap();
            containers[to].push_front(container)
        }

        println!("{containers:?}");
    }
    let mut result = "".to_owned();
    for container in containers {
        result.push_str(container[0].as_str())
    }
    return result;
}