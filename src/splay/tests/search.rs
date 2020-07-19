use crate::splay::SplayTree;

use super::default_start_tree;

#[test]
fn test_search_in_empty() {
    // Setup
    let splay_tree = default_start_tree();

    // Exercise
    let result = splay_tree.search(0);

    // Verification
    assert_eq!(result, None);
}

#[test]
fn test_search_after_one_insert_with_existing_key() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(0);

    // Exercise
    let result = splay_tree.search(0);

    // Verification
    assert_eq!(result, Some(&0));
}

#[test]
fn test_search_after_one_insert_with_non_existing_key() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(0);

    // Exercise
    let result = splay_tree.search(2);

    // Verification
    assert_eq!(result, None);
}

#[test]
fn test_search_after_two_insert_with_existing_key() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(0);
    splay_tree.insert(1);

    // Exercise
    let result = splay_tree.search(1);

    // Verification
    assert_eq!(result, Some(&1));
}

#[test]
fn test_search_after_two_insert_with_non_existing_key() {
    // Setup
    let mut splay_tree = default_start_tree();

    splay_tree.insert(0);
    splay_tree.insert(1);

    // Exercise
    let result = splay_tree.search(2);

    // Verification
    assert_eq!(result, None);
}