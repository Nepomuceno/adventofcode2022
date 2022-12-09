use std::{collections::VecDeque, cell::RefCell, rc::Rc};

use regex::Regex;
struct File {
    size: i32,
    name: String
}
struct Dir {
    name: String,
    children: VecDeque<Rc<RefCell<Dir>>>,
    files: VecDeque<File>,
    parent: Option<Rc<RefCell<Dir>>>,
}
impl Dir {
    pub fn new(name: String, parent:  Option<Rc<RefCell<Dir>>>) -> Dir {
        return Dir {
          name: name,
          children: VecDeque::new(),
          files: VecDeque::new(),
          parent: parent,
        };
    }
    pub fn print(&self) {
        println!("--------");
        println!("{}",self.name);
        for file in self.files.iter() {
            println!("{} {}", file.name, file.size)
        }
        for child in self.children.iter() {
            child.borrow().print();
        }
    }
}


pub fn run(input: &str) -> String {
    let lines = input.lines().collect::<VecDeque<_>>();
    let root = Rc::new(RefCell::new(Dir::new("/".to_string(), None)));
    let mut current = Rc::clone(&root);

    let re_cd = Regex::new(r"^\$ cd (\w*)$").unwrap();
    let re_ls = Regex::new(r"^\$ ls$").unwrap();
    let cd_up = Regex::new(r"^\$ cd \.\.$").unwrap();
    let file_regex = Regex::new(r"^(\d*) ([\w\.]*)$").unwrap();


    for i in 1..lines.len() {
        let line = lines[i];
        match line {
            _ if re_cd.is_match(line) => {
                let captures = re_cd.captures(line).unwrap();
                let name = captures[1].to_string();
                let child = Rc::new(RefCell::new(Dir::new(name, Some(current.clone()))));
                current.borrow_mut().children.push_back(Rc::clone(&child));
                current = child;
            },
            _ if cd_up.is_match(line) => {
                let current_clone = Rc::clone(&current);
                current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
            },
            _ if re_ls.is_match(line) => {
                let mut j = i+1;
                while j < lines.len() && !lines[j].starts_with("$")  {
                    if file_regex.is_match(lines[j]) {
                        let captures = file_regex.captures(lines[j]).unwrap();
                        let size = captures[1].parse::<i32>().unwrap();
                        let name = captures[2].to_string();
                        current.borrow_mut().files.push_back(File {size: size, name: name});
                    }
                    j += 1;
                }
                // println!("{}: {}", root.borrow().name, root.borrow().files.len());
            },
            _ => {
                println!("skip line: {}", line);
            }
        }
    }
    
    root.borrow().print();


    let stacks = Rc::new(RefCell::new(Vec::new()));
    let stack_clone = Rc::clone(&stacks);
    calculate_size(root, stack_clone);
    let mut content = stacks.borrow_mut();
    content.sort();
    let needed_to_free:i32 = 70000000 - content.last().unwrap() - 30000000;
    for size in content.iter() {
        if *size + needed_to_free > 0 {
            return (*size).to_string()
        }
    } 
    return 0.to_string();
}

fn calculate_size(dir: Rc<RefCell<Dir>>,  sizes: Rc<RefCell<Vec<i32>>>) -> i32 {
    let mut sum = 0; 
    for file in dir.borrow().files.iter() {
        sum += file.size
    }
    for child in dir.borrow().children.iter() {
        sum += calculate_size(Rc::clone(child), Rc::clone(&sizes));
    }
    sizes.borrow_mut().push(sum);
    return sum
}
