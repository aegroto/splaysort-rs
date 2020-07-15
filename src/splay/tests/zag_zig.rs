use crate::{SplayTree};

use super::default_start_tree;

#[test]
fn test_zag_zig() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(1);
    splay_tree.insert(5);
    splay_tree.insert(3);

    // Exercise
    splay_tree.splay(&3);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 3, left: Some(SplayNode { key: 1, left: None, right: None }), right: Some(SplayNode { key: 5, left: None, right: None }) }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

#[test]
fn test_zag_zig_with_right_sibling() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(1);
    splay_tree.insert(5);
    splay_tree.insert(3);

    splay_tree.insert(7);

    // Exercise
    splay_tree.splay(&3);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 3, left: Some(SplayNode { key: 1, left: None, right: None }), right: Some(SplayNode { key: 5, left: None, right: Some(SplayNode { key: 7, left: None, right: None }) }) }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

#[test]
fn test_zag_zig_with_left_uncle() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(1);
    splay_tree.insert(5);
    splay_tree.insert(3);

    splay_tree.insert(0);

    // Exercise
    splay_tree.splay(&3);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 3, left: Some(SplayNode { key: 1, left: Some(SplayNode { key: 0, left: None, right: None }), right: None }), right: Some(SplayNode { key: 5, left: None, right: None }) }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

#[test]
fn test_zag_zig_with_both_children() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(1);
    splay_tree.insert(5);
    splay_tree.insert(3);

    splay_tree.insert(2);
    splay_tree.insert(4);

    // Exercise
    splay_tree.splay(&3);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 3, left: Some(SplayNode { key: 1, left: None, right: Some(SplayNode { key: 2, left: None, right: None }) }), right: Some(SplayNode { key: 5, left: Some(SplayNode { key: 4, left: None, right: None }), right: None }) }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

#[test]
fn test_zag_zig_with_all_children_and_relatives() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(1);
    splay_tree.insert(5);
    splay_tree.insert(3);

    splay_tree.insert(2);
    splay_tree.insert(4);

    splay_tree.insert(0);
    splay_tree.insert(7);

    // Exercise
    splay_tree.splay(&3);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 3, left: Some(SplayNode { key: 1, left: Some(SplayNode { key: 0, left: None, right: None }), right: Some(SplayNode { key: 2, left: None, right: None }) }), right: Some(SplayNode { key: 5, left: Some(SplayNode { key: 4, left: None, right: None }), right: Some(SplayNode { key: 7, left: None, right: None }) }) }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

#[test]
fn test_zag_zig_with_missing_key() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(1);
    splay_tree.insert(5);
    splay_tree.insert(3);

    // Exercise
    splay_tree.splay(&4);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 3, left: Some(SplayNode { key: 1, left: None, right: None }), right: Some(SplayNode { key: 5, left: None, right: None }) }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

