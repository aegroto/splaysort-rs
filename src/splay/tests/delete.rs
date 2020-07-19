use crate::splay::SplayTree;

use super::default_start_tree;

#[test]
fn test_delete_in_empty() {
    // Setup
    let mut splay_tree = default_start_tree();

    // Exercise
    splay_tree.delete(0);

    // Verification
    let expected_tree = "TopDownSplayTree { root: None }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

#[test]
fn test_delete_after_one_insert_with_existent_key() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(1);

    // Exercise
    splay_tree.delete(1);

    // Verification
    let expected_tree = "TopDownSplayTree { root: None }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

#[test]
fn test_delete_after_one_insert_with_non_existent_key() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(1);

    // Exercise
    splay_tree.delete(0);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 1, left: None, right: None }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

#[test]
fn test_delete_after_two_insert_with_existent_key() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(1);
    splay_tree.insert(2);

    // Exercise
    splay_tree.delete(2);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 1, left: None, right: None }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

#[test]
fn test_delete_after_two_insert_with_non_existent_key() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(1);
    splay_tree.insert(2);

    // Exercise
    splay_tree.delete(0);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 1, left: None, right: Some(SplayNode { key: 2, left: None, right: None }) }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}