use huffman_coding::node::Node;

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

    let tree_node = Node::build(&alphabets, &probs).unwrap();
    
    println!("propagating on left");
    tree_node.left.as_ref().unwrap().downstream_codes();
    // tree_node.left.as_ref().unwrap().update_lr();

    println!("propagating on right");
    tree_node.right.as_ref().unwrap().downstream_codes();
    // tree_node.right.as_ref().unwrap().update_lr();
    
    //println!("{:?}", tree_node);

}


