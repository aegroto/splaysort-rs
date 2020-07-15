use crate::{SplayTree};

use super::default_start_tree;

#[test]
fn test_single_splay_insert() {
    // Setup
    let mut splay_tree = default_start_tree();

    // Exercise
    splay_tree.splay_insert(3);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 3, left: None, right: None }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

#[test]
fn test_double_splay_insert() {
    // Setup
    let mut splay_tree = default_start_tree();

    // Exercise
    splay_tree.splay_insert(3);
    splay_tree.splay_insert(5);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 5, left: Some(SplayNode { key: 3, left: None, right: None }), right: None }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}