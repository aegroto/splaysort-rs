use crate::{SplayTree};
use crate::splay::top_down::TopDownSplayTree;

use super::default_start_tree;

#[test]
fn test_zig_zig() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(5);
    splay_tree.insert(3);
    splay_tree.insert(1);

    // Exercise
    splay_tree.splay(1);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 1, left: None, right: Some(SplayNode { key: 3, left: None, right: Some(SplayNode { key: 5, left: None, right: None }) }) }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

#[test]
fn test_zig_zig_with_children() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(10);
    splay_tree.insert(12);

    splay_tree.insert(5);
    splay_tree.insert(7);
    splay_tree.insert(2);

    splay_tree.insert(1);
    splay_tree.insert(3);

    println!("{:?}", splay_tree);

    // Exercise
    splay_tree.splay(2);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 2, left: Some(SplayNode { key: 1, left: None, right: None }), right: Some(SplayNode { key: 5, left: Some(SplayNode { key: 3, left: None, right: None }), right: Some(SplayNode { key: 10, left: Some(SplayNode { key: 7, left: None, right: None }), right: Some(SplayNode { key: 12, left: None, right: None }) }) }) }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}