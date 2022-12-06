use std::{collections::VecDeque};


pub fn run(input: &str) -> String {
    let mut index = 0;
    let mut content:VecDeque<char> = VecDeque::new(); 
    for input_char in input.chars() {
       index += 1;
       let mut temp: VecDeque<char> = VecDeque::new();
       while content.len() > 0 {
           let comp_char = content.pop_front().unwrap();
           if comp_char == input_char {
                temp = VecDeque::new()
           } else {
                temp.push_back(comp_char)
           }
       }
       temp.push_back(input_char);
       if temp.len() == 4 {
         break;
       } 
       content = temp;
    }
    return index.to_string();
}