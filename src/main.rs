
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,                              
    next: Option<Rc<RefCell<Node>>>,         
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,        
}

trait List{
    fn new()->Self;
    fn add(&mut self, value: i32);
    fn delete(&mut self,value:i32);
    fn print(&self);
    fn len(&self)->usize;
}

impl List for LinkedList{
    fn new()->Self {
        LinkedList {
            head: None,
        }
    }

    fn add(&mut self, value: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
        }));

        match &self.head {
            Some(node) => {
                let mut current = node.clone();
                loop {
                    let next = current.borrow().next.clone();
                    match next {
                        Some(next_node) => {
                            current = next_node;
                        }
                        None => {
                            break;
                        }
                    }
                }
                current.borrow_mut().next = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }
    }

    fn delete(&mut self,value:i32) {
        let mut current = self.head.clone();
        let mut prev: Option<Rc<RefCell<Node>>> = None;
        while let Some(node) = current {
            if node.borrow().value == value {
                match &prev {
                    Some(prev_node) => {
                        prev_node.borrow_mut().next = node.borrow().next.clone();
                    }
                    None => {
                        self.head = node.borrow().next.clone();
                    }
                }
                break;
            }
            prev = Some(node.clone());
            current = node.borrow().next.clone();
        }
    }

    fn print(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            println!("{}", node.borrow().value);
            current = node.borrow().next.clone();
        }
    }

    fn len(&self)->usize {
        let mut current = self.head.clone();
        let mut count = 0;
        while let Some(node) = current {
            count += 1;
            current = node.borrow().next.clone();
        }
        count
    }
}



fn main(){
    let mut list = LinkedList::new();
    list.add(1);
    list.add(2);
    list.add(3);
    list.add(4);
    list.add(5);
    list.print();
    list.delete(3);
    list.print();
    println!("size: {}",list.len());
}
