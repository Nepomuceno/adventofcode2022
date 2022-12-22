use std::{time::Instant};

const PRINT_ENABLED: bool = true;
use std::mem;


#[derive(Clone)]
struct Node {
    elem: (usize, i64),
    next: Link,
    prev: Link,
}

type Link = Option<Box<Node>>;

pub struct CircularDoublyLinkedList {
    head: Link,
    size: usize,
}

impl CircularDoublyLinkedList {
    pub fn new() -> Self {
        CircularDoublyLinkedList { head: None, size: 0 }
    }

    pub fn push_front(&mut self, elem: (usize, i64)) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
            prev: None,
        });

        let head = match self.head.take() {
            Some(mut old_head) => {
                old_head.prev = Some(new_node);
                old_head
            }
            None => new_node.clone(),
        };

        //self.head = Some(new_node);
        self.head.as_mut().unwrap().next = Some(head);
        self.size += 1;
    }

    pub fn swap_left(&mut self, node: &mut Node) {
        let mut prev = match node.prev.take() {
            Some(mut prev) => {
                node.prev = prev.prev.take();
                prev
            }
            None => return,
        };

        let next = node.next.take();

        node.prev.as_mut().unwrap().next = next;
        node.next = prev.next.take();

        prev.prev = Some(Box::new(node.clone()));
        prev.next = Some(Box::new(node.clone()));

        mem::swap(node, &mut prev);
    }

    pub fn swap_right(&mut self, node: &mut Node) {
        let mut next = match node.next.take() {
            Some(mut next) => {
                node.next = next.next.take();
                next
            }
            None => return,
        };

        let prev = node.prev.take();

        node.prev = next.prev.take();
        node.next = next.next.take();

        next.prev = Some(Box::new(node.clone()));
        next.next = Some(Box::new(node.clone()));

        mem::swap(node, &mut next);
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn mixing(&mut self, values: Vec<usize>) {
        for value in values {
            let mut current_node = self.head.as_mut().unwrap();
    
            while current_node.elem.0 != value {
                current_node = current_node.next.as_mut().unwrap();
            }
    
            if current_node.elem.1 < 0 {
                for _ in 0..current_node.elem.1.abs() as usize {
                    // self.swap_left(current_node);
                }
            } else {
                for _ in 0..current_node.elem.1 as usize {
                    // self.swap_right(current_node);
                }
            }
        }
    }
    
}


pub fn run(input: &str) -> String {
    let start = Instant::now();
    let mut array = input.lines().map(|x| x.parse::<i64>().unwrap()).enumerate().collect::<Vec<_>>();
    let mut list = CircularDoublyLinkedList::new();
    for (i, value) in array.iter().rev() {
        list.push_front((*i, *value));
    }
    list.mixing(array.iter().map(|x| x.0).collect::<Vec<_>>());
    let mut current_node = list.head.take();
    let mut array = Vec::new();
    while let Some(mut node) = current_node {
        array.push(node.elem);
        current_node = node.next.take();
        // Break out of the loop once we've inserted all the nodes
        if node.next.is_none() {
            break;
        }
    }
    println!("{:?}", array.iter().map(|x| x.1).collect::<Vec<_>>());
    //mix(array);

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
