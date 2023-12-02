// Add, Sort, Remove, Traverse?

use std::{cell::{Cell, RefCell, Ref}, pin, borrow::BorrowMut, ops::{Deref, DerefMut}};

#[derive(Debug)]
struct Body<T: Clone> { value: Option<T>, next: Option<Box<Body<T>>> }
struct LinkedList<T: Clone> { body: Body<T>, len: Cell<usize> }

impl<T: Clone> Body<T> {
    fn new(head_value: T, last_value: T) -> Self { 
        Body { 
            value: Some(head_value), 
            next: Some(Box::new(Body { value: Some(last_value), next: None }))
        }
    }

    fn push(&mut self, value: T) {
        let mut node = &mut self.next;
        while let Some(next) = node {
            let predict = &next.next;
            if predict.is_none() {
                let prev_val = next.value.clone();
                next.value = Some(value);
                next.next = Some(Box::new( Body { value: prev_val, next: None } ));
                break;
            }
            node = &mut next.next;
        }

    }
}

impl<T: Clone> LinkedList<T> {
    fn new(start: T, end: T) -> Self {
        LinkedList { body: Body::new(start, end), len: Cell::new(2) }
    }

    fn len(&self) -> usize {
        return self.len.get()
    }
}

fn main() {}

#[cfg(test)]
mod cases {
    use crate::*;

    #[test]
    fn has_tail() {
        let mut build = Body::new(5, 1);
        build.push(4);
        build.push(7);
        build.push(4);
        build.push(7);
        build.push(4);
        build.push(7);
        build.push(4);
        build.push(7);
        
        println!("{:?}", build);
    }


}