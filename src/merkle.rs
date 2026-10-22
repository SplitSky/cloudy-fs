use std::Box;

struct MerkelNode {
    parent: Option<Box<&MerkelNode>>,
    hash: String,
}

// use blake2 for hashing algorithm for completeness or FNv-1a for completeness:

// use box for implementing tree with smart pointers for heap
