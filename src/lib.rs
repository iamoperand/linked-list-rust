use std::{fmt::Display, mem};

pub struct List<T> {
    head: Option<Node<T>>,
}

pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    data: T,
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
            current_node = node.next.as_ref().map(|next_node| &**next_node);
        }

        count
    }

    pub fn print(&self) {
        let mut current_node = self.head.as_ref();
        while let Some(node) = current_node {
            print!("{} ", node.data);
            current_node = node.next.as_ref().map(|next_node| &**next_node);
        }

        println!();
    }

    pub fn push(&mut self, element: T) {
        match self.head {
            None => self.head = Some(Node::new(element)),
            Some(ref mut head) => head.push(element),
        };
    }

    pub fn pop(&mut self) {
        match self.head {
            None => panic!("can't pop out of nothing!"),
            Some(ref mut head) => head.pop(),
        };
    }

    fn get_nth_node_mut(&mut self, n: usize) -> Option<&mut Node<T>> {
        let mut nth_node = self.head.as_mut();
        for _ in 0..n {
            nth_node = match nth_node {
                None => return None,
                Some(node) => node.next.as_mut().map(|node| &mut **node),
            }
        }
        nth_node
    }

    fn add_at_beginning(&mut self, element: T) {
        let mut new_node = Node::new(element);

        match self.head {
            None => self.head = Some(new_node),
            Some(_) => {
                let current_head = self.head.take().unwrap();
                new_node.next = Some(Box::new(current_head));
                self.head = Some(new_node);
            }
        }
    }

    pub fn add_at_index(&mut self, element: T, index: usize) {
        if index == 0 {
            return self.add_at_beginning(element);
        }

        let length = self.len();
        if index >= length {
            panic!("index out of bounds");
        }

        let mut prev_to_index_node = self.get_nth_node_mut(index - 1).unwrap();
        let chain_node = mem::replace(&mut prev_to_index_node.next, None);

        let mut new_node = Box::new(Node::new(element));
        new_node.next = chain_node;

        prev_to_index_node.next = Some(new_node);
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
            Some(ref mut target) => mem::replace(&mut target.next, None),
            None => panic!("out of range"),
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

    fn pop(&mut self) {
        match self.next {
            None => {}
            Some(ref mut next_node) => match next_node.next {
                None => {}
                Some(_) => return next_node.pop(),
            },
        }

        self.next = None;
    }
}
