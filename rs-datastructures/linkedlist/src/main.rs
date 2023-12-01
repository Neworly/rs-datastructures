use std::{rc::Rc, cell::{RefCell, Ref}, borrow::BorrowMut, ops::{Deref, Bound}, fmt::Display};

#[derive(Debug)] enum Node<V: Display> { Body(V), Empty }

#[derive(Debug)] struct LinkedList<V: Display> { body: Rc<RefCell<Vec<Node<V>>>>, len: usize }

impl<V: Display> LinkedList<V> {
    
    fn initial_value(value: V) -> Self {
        LinkedList { body: Rc::new(RefCell::new(vec![ Node::Body(value), Node::Empty ])), len: 1 }
    }

    fn insert(&mut self, value: Node<V>) {
        self.body.deref().borrow_mut().insert(self.len, value);
        self.len += 1;
    }

    fn remove_at(&mut self, idx: usize) -> Node<V> {

        if idx == 0 || idx == self.len {
            println!("You cannot remove either the head nor empty tail.");
            return Node::Empty;
        }

        if idx > self.len {
            println!("Indexing outside bounds limit.");
            return Node::Empty;
        }
        
        self.len -= 1;
        return self.body.deref().borrow_mut().remove(idx);
    }

    fn remove(&mut self) -> Node<V> {
        return self.remove_at(1);
    }

    fn add_node(&mut self, value: V) {
        self.insert(Node::Body(value))
    }    


    fn display(&self) {
        self.body.deref().borrow()
            .iter()
            .for_each(|x| {
                if let Node::Empty = x {
                    println!("Empty");
                } else if let Node::Body(value) = x {
                    print!("{}->", value)
            }
        }) 
    }

}

fn main() {
    let mut nodes = LinkedList::initial_value(1);    

    
    for i in 1 .. 10 {
        nodes.add_node(i);
    }


    nodes.remove();
    nodes.remove();
    nodes.remove();
    nodes.remove();
    nodes.remove();

    nodes.display();

}