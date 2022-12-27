use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::BTreeMap;

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
pub struct HuffmanGenerator {
    pub tree: Option<Box<Node>>,
    pub code_map: RefCell<BTreeMap<String, HuffmanCode>>
}

impl HuffmanGenerator {

    pub fn build(alphabets: &Vec<String>, 
        probs: &Vec<f32>) -> HuffmanGenerator {
       
        let tree = Node::build(alphabets, probs);

        HuffmanGenerator {
             tree: tree, 
             code_map: RefCell::new(BTreeMap::new())
        }
    }

    fn update_code_list(&self, huffman_code: HuffmanCode) {
        let mut code_list = self.code_map.borrow_mut();
        code_list.insert(huffman_code.alphabet.clone(), huffman_code);
    }

    fn update_child(&self, node_binding: &Option<Box<Node>>) -> bool {
        let node: &Node = node_binding.as_ref().unwrap();

        if node.get_fill_status() {
            if node.is_base_node() {
                let alphabet = node.c.borrow().clone().unwrap();
                let codes = node.codes.borrow().clone();
                let prob = node.prob.clone();

                let huffman_code = HuffmanCode::new(
                                            alphabet, 
                                            codes, 
                                            prob
                                        );

                self.update_code_list(huffman_code);
            }
            return true;
        }
        
        node.downstream_codes();
        return false;
    }

    pub fn propagate_codes(&self) {

        let mut queue = vec![&self.tree];

        while !queue.is_empty() {

            let last = queue.last().unwrap().as_ref().unwrap();

            if last.get_fill_status() {
                queue.pop();
                continue;
            }
            else {

                let lnode_binding = &last.as_ref().left;
                let rnode_binding = &last.as_ref().right;

                let lnode_filled = self.update_child(lnode_binding);
                let rnode_filled = self.update_child(rnode_binding);

                if !lnode_filled {
                    queue.push(lnode_binding);
                }

                if !rnode_filled {
                    queue.push(rnode_binding);
                }

                if lnode_filled && rnode_filled {
                    last.update_fill(true);
                }
            }
        }

    }
}