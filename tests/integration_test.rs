
#[cfg(test)]
mod tests {
    use huffman_coding::huffman::HuffmanGenerator;

    #[test]
    fn test_uniform() {
        let alphabets = vec![
            "a".to_string(),
            "b".to_string(), 
            "c".to_string(), 
            "d".to_string(), 
            "e".to_string(), 
            "f".to_string(), 
            "g".to_string(), 
            "h".to_string(),
        ];
    
        let probs = vec![
            0.25, 
            0.25, 
            0.25, 
            0.25, 
            0.25,
            0.25, 
            0.25, 
            0.25,

        ];

        let huffman_gen = HuffmanGenerator::build(&alphabets, &probs);
        huffman_gen.propagate_codes();

        let mut c_lens = Vec::new();

        println!("\nTest #1: Huffman Code for Uniform Distribution (8 codes).");
        for (k,v) in huffman_gen.code_map.borrow().iter() {
            println!("key = {}, code = {}", &k, &v.code);
            c_lens.push(v.code.len());
        }

        let n_samples = c_lens.len();
        let total_sum: usize = c_lens.iter().sum();
        let average: usize = total_sum / n_samples + total_sum % n_samples;

        println!("\nn_samples = {}", n_samples);
        println!("total_sum = {}", total_sum);
        println!("average = {}", average);
        println!("c_lens[0] = {}", c_lens[0]);

        assert_eq!(average, c_lens[0]);
    }
    
    #[test]
    fn test_source_coding_theorem() {

        let alphabets = vec![
            "a".to_string(),
            "b".to_string(), 
            "c".to_string(), 
            "d".to_string(), 
            "e".to_string(), 
        ];
    
        let probs = vec![
            0.5, 
            0.30, 
            0.10, 
            0.05, 
            0.05,
        ];

        let huffman_gen = HuffmanGenerator::build(&alphabets, &probs);
        huffman_gen.propagate_codes();

        let mut code_lengths = Vec::new();
        let mut probs = Vec::new();

        println!("\nTest #2: Testing if the code satisfies Shannon's Source Coding Theorem.");
        for (k,v) in huffman_gen.code_map.borrow().iter() {
            println!("key = {}, code = {}", &k, &v.code);

            let code_length = v.code.len() as f32;
            code_lengths.push(code_length);
            probs.push(v.prob);
        }

        let log_probs = probs.iter().map(|v| -v.log2());
        
        let entropy: f32 = probs.iter()
                                .zip(log_probs)
                                .map(|(a,b)| a * b)
                                .sum();
        
        let expected_length: f32 = probs.iter()
                                        .zip(code_lengths.iter())
                                        .map(|(a,b)| a * b)
                                        .sum();

        println!("\nH(X) = {}", entropy);
        println!("L(C,X) = {}", expected_length);
        println!("H(X) + 1 = {}\n", entropy + 1_f32);

        let v0 = entropy <= expected_length;
        let v1 = expected_length < entropy + 1_f32;

        assert!(v0 && v1);


    }


    /// # Maximum Difference between expected length 
    /// The difference between the expected length *L* and the entropy
    /// H can be no bigger than max(p_max, 0.086). (Gallager, 1978)
    #[test]
    fn test_max_limit() {
        // (Gallager, 1978). The Difference
        let probs: Vec<f32> = [1.0 / 11.0 ; 11].to_vec();
        
        let alphabets = vec![
            "a".to_string(),
            "b".to_string(), 
            "c".to_string(), 
            "d".to_string(), 
            "e".to_string(),
            "f".to_string(),
            "g".to_string(),
            "h".to_string(),
            "i".to_string(),
            "j".to_string(),
            "k".to_string(), 
        ];
        

        let huffman_gen = HuffmanGenerator::build(&alphabets, &probs);
        huffman_gen.propagate_codes();

        let mut code_lengths = Vec::new();
        let mut probs = Vec::new();

        println!("\nTest #3: Testing max limit.");
        for (k,v) in huffman_gen.code_map.borrow().iter() {
            println!("key = {}, code = {}", &k, &v.code);

            let code_length = v.code.len() as f32;
            code_lengths.push(code_length);
            probs.push(v.prob);
        }

        let log_probs = probs.iter().map(|v| -v.log2());
        
        let entropy: f32 = probs.iter()
                                .zip(log_probs)
                                .map(|(a,b)| a * b)
                                .sum();
        
        let expected_length: f32 = probs.iter()
                                        .zip(code_lengths.iter())
                                        .map(|(a,b)| a * b)
                                        .sum();

        let max_diff = expected_length - entropy;
        let epsilon = (max_diff - 0.086).abs(); 
        println!("\nH(X) = {}", entropy);
        println!("L(C,X) = {}", expected_length);
        println!("H(X) + 1 = {}", entropy + 1_f32);

        println!("L(C,X) - H(X) = {}", max_diff);
        println!("epsilon = {} \n", (max_diff - 0.086).abs());

        assert!(epsilon < 0.0001);
        

        // assert_ep;
    }


}