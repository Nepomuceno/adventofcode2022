use std::{time::Instant};

const PRINT_ENABLED: bool = true;

#[derive(Clone)]
struct Link{
    value: i32,
    right: Option<Box<Link>>,
    left: Option<Box<Link>>,
}

impl Link {
    fn new(value: i32) -> Link {
        Link {
            value,
            right: None,
            left: None,
        }
    }
}


fn mix(array: Vec<Box<Link>>) {
    for link in array {
        match link.clone().value {
            _ if link.clone().value == 0 => {
                continue;
            }
            _ if link.clone().value < 0 => {
                for _ in 0..=-link.value {
                    //link.clone().left.unwrap().right = link.clone().right;
                    //link.clone().right = link.clone().left;
                    //link.clone().left = link.clone().left.unwrap().left;
                    //link.clone().right.unwrap().left = Some(link);
                }
            }
            _ => {
                for _ in 0..link.value {
                    //link.clone().right.unwrap().left = link.clone().left;
                    //link.clone().left = link.clone().right;
                    //link.clone().right = link.clone().right.unwrap().right;
                    //link.clone().left.unwrap().right = Some(link);
                }
            }
        }
    }
}

pub fn run(input: &str) -> String {
    let start = Instant::now();
    let mut array = input.lines().map(|x| x.parse::<i32>().unwrap()).map(|x| Box::new(Link::new(x))).collect::<Vec<Box<Link>>>();
    array[0].left = Some(array[array.len()-1].clone());
    array[array.len()-1].clone().right = Some(array[0].clone());
    for i in 0..array.len()-1 {
        array[i].right = Some(array[i+1].clone());
        array[i+1].left = Some(array[i].clone());
    }
    for i in 0..array.len() {
        println!("{}: {} {} {}", i, array[i].value, array[i].clone().left.unwrap().value, array[i].clone().right.unwrap().value);
    }
    //mix(array);

    if PRINT_ENABLED {
        println!("Elapsed: {:?}", start.elapsed());
    }
    let index_of_0 = array.iter().position(|x| x.value == 0).unwrap();
    println!("index of 0: {}", index_of_0);
    let mut sum = 0;
    let mut current = array[index_of_0].clone().right.unwrap();
    for i in 1..=3000 {
        if i % 1000 == 0 {
            println!("{}: {}", i, current.value);
            sum += current.value;
        }
        current = current.right.unwrap();
    }
    sum.to_string()
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
