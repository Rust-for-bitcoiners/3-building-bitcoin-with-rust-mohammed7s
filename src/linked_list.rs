#![allow(unused)]

/* This module will be taught in the class */

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None} 
    }

    fn push(&mut self, val:T) {
        let new_node = Box::new(Node {
            val, 
            next: self.head.take(),
        });
        self.head = Some(new_node); 
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next; 
            node.val
        })

    }
}