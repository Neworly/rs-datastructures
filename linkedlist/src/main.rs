// Add, Sort, Remove, Traverse?

use std::{cell::{Cell, RefCell, Ref}, pin, borrow::BorrowMut, ops::{Deref, DerefMut}, fmt::{Display, Debug}, f32::NAN};

#[derive(Debug)] struct Body<T: Clone + Debug> { value: Option<T>, next: Option<Box<Body<T>>> }
#[derive(Debug)] struct LinkedList<T: Clone + Debug> { body: Body<T>, len: Cell<usize> }

impl<T: Clone + Debug> Body<T> {
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

    fn remove(&mut self, pos: usize) -> Option<T> {
        let mut current_at = 1;
        let mut node = &mut self.next;
        while let Some(curr) = node {
            if current_at >= pos {                
                let hold = &mut curr.next;
                if let Some(next) = hold {
                    let next_value = next.value.clone();
                    next.value = curr.value.clone();
                    curr.value = next_value;
                    let predict = &next.next;
                    if predict.is_none() {
                        curr.next = None;
                        break;
                    }
                }
                break;
            }
            node = &mut curr.next;
            current_at += 1;
        }
        
        return None;
    }

    fn pop(&mut self, last_pos: usize) -> Option<T> {
        return self.remove(last_pos)
    }    
}

impl<T: Clone + Debug> LinkedList<T> {
    fn new(start: T, end: T) -> Self {
        LinkedList { body: Body::new(start, end), len: Cell::new(2) }
    }

    fn push(&mut self, value: T) {
        self.body.push(value);
        self.len.set(self.len() + 1);
    }

    fn pop(&mut self) -> Option<T> {
        self.len.set(self.len() - 1);
        return self.body.pop(self.len() - 1);
    }

    fn out(&self) {
        let mut node = &self.body.next;
        print!("[{:?}]->", self.body.value);
        let mut prev_val = None;
        while let Some(next) = node {
            print!("[{:?}]->", next.value);
            prev_val = next.value.clone();
            node = &next.next;
        }
        // println!("[{:?}]->END", prev_val);
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
        let mut build = LinkedList::new(String::from("Start"), String::from("End"));
        build.push(String::from("Middle"));
        build.push(String::from("Middle"));
        build.push(String::from("Middle"));
        build.push(String::from("Middle"));
        build.push(String::from("Middle"));
        build.push(String::from("Middle"));
        build.push(String::from("Middle"));

        let v = build.pop();

        build.out();
    }
}