use std::borrow::Borrow;
use std::cell::{Cell, RefCell};

use crate::node::Node;

#[derive(Debug)]
pub struct HuffmanCode {
    pub alphabet: String,
    pub code: String,
    pub prob: f32,
}

impl HuffmanCode {
    pub fn new(alphabet: String, code: String, prob: f32) -> HuffmanCode {
        HuffmanCode {
            alphabet,
            code,
            prob,
        }
    }
}

#[derive(Debug)]
pub struct BSearch {
    pub tree: Node,
    pub encoded_list: Vec<HuffmanCode>,
}



impl BSearch {

    pub fn build(alphabets: &Vec<String>, 
        probs: &Vec<f32>) -> BSearch {
       
        let tree = Node::build(alphabets, probs).unwrap();

        BSearch { tree: tree }

    }

    pub fn propagate_codes(&self) {

        let curr_ref = &self.tree;

        let mut queue = vec![curr_ref];

        while !queue.is_empty() {
            let last = queue.last().unwrap();

            if last.get_fill_status() {
                queue.pop();
                continue;
            }
            else {

            }

            let lnode_binding = last.left.as_ref().unwrap();
            let rnode_binding = last.right.as_ref().unwrap();

            let lnode : &Node = lnode_binding.borrow();
            let rnode : &Node = rnode_binding.borrow();

            queue.push(lnode);
            queue.push(rnode);

        }
    }
}