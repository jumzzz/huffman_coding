# Huffman Coding
Implementation of Huffman Coding in Rust

## Implemented Parts
- Given a Vector of Arbitrary Characters with corresponding Probabilities, generate the appropriate Huffman Code
- Testing against Source Coding Theorem
- Testing against Uniform Distribution of 8 Samples
- Testing on the maximum difference between Expected Length L(C,X) and Entropy H(X) 

## Test

To run the test simply

```
cargo test -- --nocapture --test-threads=1
```

### Test Cases

Here are the following test case

1. Uniform Distribution with 8 Samples
2. Shannon Source Coding Theorem
3. Testing on the maximum difference between Expected Length `L(C,X)` and Entropy `H(X)`. 

## Reference
-  *Information Theory, Inference and Learning Algorithms (2005) by McKay D.J.C. Chapter 05 - Symbol Codes*
