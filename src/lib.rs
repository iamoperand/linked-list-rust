use std::{fmt::Display, mem};

pub struct List<T> {
    head: Option<Node<T>>,
}

pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    data: T,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_ref(),
        }
    }
}

impl<T> List<T>
where
    T: Display,
{
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;

        let mut current_node = self.head.as_ref();
        while let Some(node) = current_node {
            count += 1;
            current_node = node.next.as_deref();
        }

        count
    }

    pub fn print(&self, prefix: &str) {
        let mut current_node = self.head.as_ref();

        print!("{}\n", prefix);
        while let Some(node) = current_node {
            print!("{} ", node.data);
            current_node = node.next.as_deref();
        }
        println!("\n");
    }

    pub fn push(&mut self, element: T) {
        match self.head {
            None => self.head = Some(Node::new(element)),
            Some(ref mut head) => head.push(element),
        };
    }

    fn get_nth_node_mut(&mut self, n: usize) -> Option<&mut Node<T>> {
        let mut nth_node = self.head.as_mut();
        for _ in 0..n {
            nth_node = match nth_node {
                None => return None,
                Some(node) => node.next.as_deref_mut(),
            }
        }
        nth_node
    }

    fn remove_at_beginning(&mut self) {
        self.head
            .take()
            .map(|head| self.head = head.next.map(|node| *node));
    }

    pub fn remove_at_index(&mut self, index: usize) {
        let length = self.len();
        if index >= length {
            panic!("index out of bounds");
        }

        if index == 0 {
            return self.remove_at_beginning();
        }

        let mut prev_to_target_node = self.get_nth_node_mut(index - 1).unwrap();

        prev_to_target_node.next = match prev_to_target_node.next {
            None => panic!("out of range"),
            Some(ref mut target) => mem::replace(&mut target.next, None),
        };
    }
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            next: None,
            data: value,
        }
    }

    fn push(&mut self, element: T) {
        match self.next {
            None => {}
            Some(ref mut next) => return next.push(element),
        }

        self.next = Some(Box::new(Node::new(element)));
    }
}
