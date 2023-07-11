#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct LinkedList {
    pub head: Option<Rc<RefCell<Node>>>,
    pub tail: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
struct Node {
    pub val: i32,
    pub prev: Option<Rc<RefCell<Node>>>,
    pub next: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList { head: None, tail: None }
    }

    fn init_first(&mut self, node_rc_refcell: Rc<RefCell<Node>>) {
        self.head = Some(node_rc_refcell.clone());
        self.tail = Some(node_rc_refcell.clone());
    }

    fn add_to_tail(&mut self, new_node: Node) {
        let node_rc_refcell = Rc::new(RefCell::new(new_node));

        match &self.tail {
            Some(tail) => {
                node_rc_refcell.clone().borrow_mut().prev = Some(tail.clone());
                tail.clone().borrow_mut().next = Some(node_rc_refcell.clone());
                self.tail = Some(node_rc_refcell.clone());
            }
            None => self.init_first(node_rc_refcell)
        }
    }

    fn add_to_head(&mut self, new_node: Node) {
        let node_rc_refcell = Rc::new(RefCell::new(new_node));

        match &self.head {
            Some(head) => {
                node_rc_refcell.clone().borrow_mut().next = Some(head.clone());
                head.clone().borrow_mut().prev = Some(node_rc_refcell.clone());
                self.head = Some(node_rc_refcell.clone());
            }
            None => self.init_first(node_rc_refcell)
        }
    }
}

fn main() {
    let mut linked_list = LinkedList::new();

    let node1 = Node { val: 21, prev: None, next: None };
    linked_list.add_to_tail(node1);

    let node2 = Node { val: 22, prev: None, next: None };
    linked_list.add_to_head(node2);

    let node3 = Node { val: 23, prev: None, next: None };
    linked_list.add_to_tail(node3);

    let mut node = linked_list.head;
    while let Some(node_rc_refcell) = node {
        println!("{:?}", node_rc_refcell.clone().borrow().val);
        node = node_rc_refcell.clone().borrow().next.clone();
    }
}
