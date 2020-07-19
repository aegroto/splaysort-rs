use crate::splay::{BinaryTree};
use crate::splay::sort::SplaySorter;

use super::default_start_tree;

#[test]
fn test_extract_min_in_empty() {
    // Setup
    let mut splay_tree = default_start_tree();

    // Exercise
    let min = splay_tree.extract_min();

    // Verification
    assert_eq!(min, None);
}

#[test]
fn test_extract_min_with_one_node() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.binary_insert(3);

    // Exercise
    let min = splay_tree.extract_min();
    let min_key = min.unwrap();

    // Verification
    assert_eq!(min_key, 3);
}

#[test]
fn test_extract_min_with_three_node() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.binary_insert(3);
    splay_tree.binary_insert(1);
    splay_tree.binary_insert(5);

    // Exercise
    let min = splay_tree.extract_min();
    let min_key = min.unwrap();

    // Verification
    assert_eq!(min_key, 1);
}