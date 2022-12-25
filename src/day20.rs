use std::collections::VecDeque;
use std::{time::Instant};

const PRINT_ENABLED: bool = true;
use std::mem;

fn mixing(list: &mut VecDeque<(usize,i64)>) -> VecDeque<(usize,i64)> {
    for i in 0..list.len() {
        // get the index of i in the list
        let index = list.iter().position(|x| x.0 == i).unwrap();
        let mut movements = list[index].1 % list.len() as i64;
        if movements > 0 {
            let mut temp = list[index].clone();
            for _ in 0..movements {
                let next = list[index+1].clone();
                mem::swap(&mut temp, &mut list[index+1]);
                temp = next;
            }
        } else if movements < 0 {
            movements = movements * -1;
            let mut temp = list[index].clone();
            for _ in 0..movements {
                let next = list[index-1].clone();
                mem::swap(&mut temp, &mut list[index-1]);
                temp = next;
            }
        }
        println!("{:?}", list);
    }
    list.clone()
} 

pub fn run(input: &str) -> String {
    let start = Instant::now();
    let mut list = VecDeque::new();
    let array = input.lines().map(|x| x.parse::<i64>().unwrap()).enumerate().collect::<Vec<_>>();
    for (i, value) in array {
        list.push_back((i, value));
    }
    mixing(&mut list);
    println!("{:?}", list);
    
    if PRINT_ENABLED {
        println!("Elapsed: {:?}", start.elapsed());
    }
    0.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string(format!("data/day20.txt"))
        .expect("Something went wrong reading the file");
        assert_eq!("33", run(&file));
    }
}
