use std::{cell::RefCell, rc::Rc};

struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>
}

pub struct TransactionLog {
    head: Option<Node>,
    tail: Option<Node>,
    pub length: u64
}