use std::{collections::{VecDeque}};
use json::JsonValue;


pub fn run(input: &str) -> String {
    let mut in_order = 0;
    let pairs = input.split("\n\n").collect::<Vec<&str>>();
    for (i,pair) in pairs.into_iter().enumerate() {
        let (left,right) = pair.split_once("\n").map(|f| 
            (json::parse(f.0).unwrap(), json::parse(f.1).unwrap())).unwrap();
        println!("---- 1 ----");
        let result_1 = check_arrays_in_order_1(&mut left.clone(), &mut right.clone());
        println!("---- 2 ----");
        let result = check_arrays_in_order(&mut left.clone(), &mut right.clone());
        if  result == 1 {
            in_order += i + 1;
            println!("[ORDER] {}: {} , {} ", i, left, right)
        } else {
            println!("[NOT] {}: {} , {}, ", i, left, right)
        }
        if result_1 != result {
            println!("{}: {} , {} , {} , {}", i, left, right, result_1, result)
        }
        //println!("{}: {} in order", i, in_order);
    }
    in_order.to_string()
}

fn check_arrays_in_order_1(left: &mut JsonValue, right: &mut JsonValue) -> i32 {
    println!("Compare: {} |--| {}", left, right);
    
    if left.len() == 0 {
        return 1;
    }
    if right.len() == 0 {
        return -1;
    }
    loop {
        if left.len() == 0 {
            return 1;
        }
        if right.len() == 0 {
            return -1;
        }
        let mut left_num = left.array_remove(0);
        let mut right_num = right.array_remove(0);
        println!("Partial: {} , {}", &left_num, &right_num);
        if right_num.is_number() && left_num.is_number() {
            if left_num.as_i32().unwrap() < right_num.as_i32().unwrap() {
                return 1;
            } else if left_num.as_i32().unwrap() > right_num.as_i32().unwrap() {
                return -1;
            }
        } else if right_num.is_array() && left_num.is_array() {
            return check_arrays_in_order_1(&mut left_num, &mut right_num);
        } else if right_num.is_array() && left_num.is_number() {
            let content = format!("[{}]", left_num.as_i32().unwrap());
            let mut content_value = json::parse(&content).unwrap();
            if check_arrays_in_order_1(&mut content_value, &mut right_num) < 0{
                return -1;
            }
        } else if right_num.is_number() && left_num.is_array() {
            let content = format!("[{}]", right_num.as_i32().unwrap());
            let mut content_value = json::parse(&content).unwrap();
            if check_arrays_in_order_1(&mut left_num, &mut content_value) < 0 {
                return -1;
            }
        }
    }
}

fn check_arrays_in_order(left: &mut JsonValue, right: &mut JsonValue) -> i32 {
    println!("Compare: {} |--| {}", left, right);
    if left.is_number() && right.is_number() {
        if left.as_i32().unwrap() < right.as_i32().unwrap() {
            return 1;
        } else if left.as_i32().unwrap() > right.as_i32().unwrap() {
            return -1;
        } else {
            return 0;
        }
    }
    if left.is_array() && right.is_number() {
        let content = format!("[{}]", right.as_i32().unwrap());
        let mut content_value = json::parse(&content).unwrap();
        return check_arrays_in_order(left, &mut content_value);
    }
    if left.is_number() && right.is_array() {
        let content = format!("[{}]", left.as_i32().unwrap());
        let mut content_value = json::parse(&content).unwrap();
        return check_arrays_in_order(&mut content_value, right);
    }
    if left.is_array() && right.is_array() {
        if left.len() == 0 {
            if right.len() == 0 {
                return 0;
            }
            return 1;
        }
        if right.len() == 0 {
            return -1;
        }
        let temp = check_arrays_in_order(&mut left[0], &mut right[0]);
        if temp == 0 {
            left.array_remove(0);
            right.array_remove(0);
            return check_arrays_in_order(left, right);
        } else {
            return temp;
        }
    }
    panic!("Not implemented: {} , {}", left, right)
}