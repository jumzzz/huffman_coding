use huffman_coding::node::Node;
use huffman_coding::huffman::HuffmanGenerator;


fn main() {
    let alphabets = vec![
        "a".to_string(),
        "b".to_string(), 
        "c".to_string(), 
        "d".to_string(), 
        "e".to_string()
    ];
    
    let probs = vec![
        0.25, 
        0.25, 
        0.2, 
        0.15, 
        0.15
    ];

    // let tree_node = Node::build(&alphabets, &probs).unwrap();
    
    let b_search = HuffmanGenerator::build(&alphabets, &probs);
    b_search.propagate_codes();

}


