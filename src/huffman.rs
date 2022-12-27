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
    pub tree: Option<Box<Node>>,
}

impl BSearch {

    pub fn build(alphabets: &Vec<String>, 
        probs: &Vec<f32>) -> BSearch {
       
        let tree = Node::build(alphabets, probs);

        BSearch { tree: tree }
    }

    pub fn propagate_codes(&self) {


        let mut queue = vec![&self.tree];

        while !queue.is_empty() {

            let last_idx = queue.len() - 1;
            let last = queue[last_idx].as_ref().unwrap();
            // let last = (*queue.last()).unwrap();

            println!("last_idx = {}", last_idx);

            if last.get_fill_status() {
                queue.pop();
                continue;
            }
            else {

                let lnode_binding = &last.as_ref().left;
                let rnode_binding = &last.as_ref().right;

                let lnode : &Node = lnode_binding.as_ref().unwrap();
                let rnode : &Node = rnode_binding.as_ref().unwrap();

                let mut lnode_filled = false;
                let mut rnode_filled = false;

                if lnode.get_fill_status() {
                    println!("lnode.codes = {:?}", lnode.codes);
                    println!("lnode.c = {:?}", lnode.c);
                    println!("lnode.prob = {:?}", lnode.prob);
                    lnode_filled = true;
                
                }
                else {
                    lnode.downstream_codes();
                    queue.push(lnode_binding);

                }

                println!("");

                if rnode.get_fill_status() {
                    println!("rnode.codes = {:?}", rnode.codes);
                    println!("rnode.c = {:?}", rnode.c);
                    println!("rnode.prob = {:?}", rnode.prob);
                    rnode_filled = true;
                }
                else {
                    rnode.downstream_codes();
                    queue.push(rnode_binding);

                }

                if lnode_filled && rnode_filled {
                    last.update_fill(true);
                }
            }
        }
    }
}