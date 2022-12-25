use std::collections::VecDeque;
use std::{time::Instant};

const PRINT_ENABLED: bool = true;



fn mixing(vec: &mut VecDeque<(usize, i64)>) {
    for x in 0..vec.len() {
        // Find the element with the value x
        let index = vec.iter().position(|(val, _)| *val == x).unwrap();
        let (val, offset) = vec.remove(index).unwrap();

        let vec_len = vec.len() as i64;
        let mut new_index = index as i64 + offset;
        new_index = new_index.rem_euclid(vec_len);
        if new_index < 0 {
            // If the new index is negative, add the vector length to wrap it around
            new_index += vec_len;
        }
        // Insert the element at the new position
        vec.insert(new_index as usize, (val, offset));
        if PRINT_ENABLED {
            println!("{} \t {} \t {}", x, new_index, offset);
            println!("state {:?}", vec.iter().map(|x| x.1).collect::<Vec<_>>());
        }
    }
}

pub fn run(input: &str) -> String {
    let start = Instant::now();
    let mut list = VecDeque::new();
    let array = input.lines().map(|x| x.parse::<i64>().unwrap()).enumerate().collect::<Vec<_>>();
    for (i, value) in array {
        list.push_back((i, value));
    }
    mixing(&mut list);
    let position_of_zero = list.iter().position(|x| x.1 == 0).unwrap();
    let k1_after_zero = (position_of_zero + 1000) % list.len();
    let k2_after_zero = (position_of_zero + 2000) % list.len();
    let k3_after_zero = (position_of_zero + 3000) % list.len();
    println!("0: {} \t k1: {} \t k2: {} \t k3: {}", position_of_zero, k1_after_zero, k2_after_zero, k3_after_zero);
    let result = list[k1_after_zero].1 + list[k2_after_zero].1 + list[k3_after_zero].1;

    if PRINT_ENABLED {
        println!("Elapsed: {:?}", start.elapsed());
    }
    result.to_string()
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
