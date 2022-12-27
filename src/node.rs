use std::ops;
use std::cell::{Cell, RefCell};


#[derive(Debug, PartialEq, PartialOrd)]
pub struct Node {
    pub c: Option<String>,
    pub prob: f32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub filled: Cell<bool>,
    pub codes: RefCell<String>,
}

impl ops::Add<Node> for Node {
    type Output = Node;

    fn add(self, _rhs: Node) -> Node {
        Node {
            c: None,
            prob: self.prob + _rhs.prob,
            left: Some(Box::new(self)),
            right: Some(Box::new(_rhs)),
            filled: Cell::new(false),
            codes: RefCell::new(String::new()),
        }
    }
}

impl Node {
    pub fn new(c: Option<String>,
           prob: f32,
           left: Option<Box<Node>>, 
           right: Option<Box<Node>>) -> Node {
        Node {
            c: c,
            prob: prob,
            left: left,
            right: right,
            filled: Cell::new(true),
            codes: RefCell::new(String::new()),
        }
    }

    pub fn build(alphabets: &Vec<String>, 
                 probs: &Vec<f32>) -> Option<Box<Node>> {
        // panic if alphabets and probs are not equal
        assert_eq!(alphabets.len(), probs.len());
        assert!(alphabets.len() > 1);

        // Initialize nodes
        let mut init_nodes: Vec<Node> = Vec::new();
        
        for (c, p) in alphabets.iter().zip(probs) {
            let node = Node::new(
                             Some(c.to_string()), 
                             *p,
                             None,
                             None
                        );


            init_nodes.push(node);
        }

        loop {
            init_nodes.sort_by(|a, b| 
                b.prob.partial_cmp(&a.prob).unwrap()
            );

            let left_node = init_nodes.pop().unwrap();
            let right_node = init_nodes.pop().unwrap(); 

            // Add the probability

            let parent_node = left_node + right_node;

            if init_nodes.is_empty() {
                parent_node.update_lr();
                return Some(Box::new(parent_node));
            }
            init_nodes.push(parent_node);
        }
    }
}


impl Node {

    pub fn update_fill(&self, fill_status: bool) {
        self.filled.set(fill_status);
    }

    pub fn get_fill_status(&self) -> bool {
        self.filled.get()
    }

    pub fn update_lr(&self) {
       
        let lbinding = self.left.as_ref().unwrap();
        let rbinding = self.right.as_ref().unwrap();
        
        let mut lcodes = lbinding.codes.borrow_mut();
        let mut rcodes = rbinding.codes.borrow_mut();
    

        lcodes.push_str(&String::from("0"));
        rcodes.push_str(&String::from("1"));

    }

    pub fn is_base_node(&self) -> bool {
        match self.c.as_ref() {
            Some(x) => {
                return x != &String::from("");
            },
            _ => {
                return false;
            }
        }
    }

    pub fn downstream_codes(&self) {

        let lbind = self.left.as_ref();
        let rbind = self.right.as_ref();

        match (lbind, rbind) {
            (Some(x), Some(y)) => {

                let mut lcodes = x.codes.borrow_mut();
                let mut rcodes = y.codes.borrow_mut();

                let mut to_left = self.codes.borrow().clone();
                let mut to_right  = self.codes.borrow().clone();
        
                lcodes.push_str(&mut to_left);
                rcodes.push_str(&mut to_right);

                lcodes.push_str(&String::from("0"));
                rcodes.push_str(&String::from("1"));
            
            },
            (_, _) => println!("End of the line"),
        }
        
    }






    


}