use json::JsonValue;


pub fn run(input: &str) -> String {
    let pairs = input.split("\n\n").collect::<Vec<&str>>();
    let mut packages = vec![];
    for pair in pairs.into_iter() {
        let (left,right) = pair.split_once("\n").map(|f| 
            (json::parse(f.0).unwrap(), json::parse(f.1).unwrap())).unwrap();
        packages.push(left);
        packages.push(right);
    }
    packages.push(json::parse("[[2]]").unwrap());
    packages.push(json::parse("[[6]]").unwrap());
    packages.sort_by(| a,b| check_arrays_in_order(&mut b.clone(),&mut a.clone()).cmp(&0));
    let first_index = packages.iter().position(|f| f == &json::parse("[[2]]").unwrap()).unwrap();
    let second_index = packages.iter().position(|f| f == &json::parse("[[6]]").unwrap()).unwrap();
    for (i,package) in packages.iter().enumerate() {
        println!("{}: {}", i, package);
    }
    println!("First: {}, Second: {}", first_index, second_index);
    ((first_index+1)*(second_index+1)).to_string()
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