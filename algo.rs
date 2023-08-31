use std::{collections::HashSet, println};

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let nums1_set: HashSet<i32> = nums1.into_iter().collect::<HashSet<i32>>();
    let nums2_set: HashSet<i32> = nums2.into_iter().collect::<HashSet<i32>>();

    let result1: Vec<i32> = nums1_set
        .iter()
        .filter(|item| !nums2_set.contains(item))
        .cloned()
        .collect::<Vec<i32>>();
    let result2: Vec<i32> = nums2_set
        .iter()
        .filter(|item| !nums1_set.contains(item))
        .cloned()
        .collect::<Vec<i32>>();

    vec![result1, result2]
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    // - digits.splice(lastIndex, lastItem + 1)
    // - return digits
    let mut result = digits;
    let last_index = result.len();
    let last_item = result[last_index - 1];
    let addition = if last_item == 9 {
        vec![1, 0]
    } else {
        vec![last_item + 1]
    };
    result.splice(last_index..last_index + 1, addition);

    return result;
}

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> LinkedList<T> {
    fn new<U>() -> LinkedList<U> {
        LinkedList {
            head: None,
            length: 0,
        }
    }
    fn size(&self) -> usize {
        self.length
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn prepend(&mut self, data: T) {
        let mut new_node = Some(Box::new(Node::new(data)));
        if let Some(existing_head) = self.head.take() {
          new_node.as_mut().unwrap().next = Some(existing_head);
        }
        self.head = new_node;
    }
    fn append(&mut self, data: T) {
        let new_node = Some(Box::new(Node::new(data)));
        match self.head {
            Some(ref mut current) => {
                let mut inner_current = current;
                while inner_current.next.is_some() {
                    inner_current = inner_current.next.as_mut().unwrap();
                }
                inner_current.next = new_node;
            }
            None => {
                self.head = new_node;
            }
        };
        self.length += 1
    }
}

fn main() {
    let mut linked_list: LinkedList<i32> = LinkedList::<i32>::new();
    linked_list.append(50);
    linked_list.append(23);
    linked_list.append(23);

    println!("Linked list ======> {:?}", linked_list)
}
