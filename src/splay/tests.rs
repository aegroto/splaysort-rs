use super::{SplayTree};
use super::top_down::TopDownSplayTree;

fn default_start_tree() -> TopDownSplayTree<u32> {
    let splay_tree : TopDownSplayTree<u32> = Default::default();

    return splay_tree;
}

#[test]
fn test_zig() {
    // Setup
    let mut splay_tree = default_start_tree();

    // Exercise
    splay_tree.insert(5);
    splay_tree.insert(3);

    splay_tree.splay(3);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 3, left: None, right: Some(SplayNode { key: 5, left: None, right: None }) }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

#[test]
fn test_zag() {
    // Setup
    let mut splay_tree = default_start_tree();

    // Exercise
    splay_tree.insert(3);
    splay_tree.insert(5);

    splay_tree.splay(5);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 5, left: Some(SplayNode { key: 3, left: None, right: None }), right: None }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

#[test]
fn test_zig_with_children() {
    // Setup
    let mut splay_tree = default_start_tree();

    // Exercise
    splay_tree.insert(5);
    splay_tree.insert(2);
    splay_tree.insert(1);
    splay_tree.insert(3);

    splay_tree.splay(2);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 2, left: Some(SplayNode { key: 1, left: None, right: None }), right: Some(SplayNode { key: 5, left: Some(SplayNode { key: 3, left: None, right: None }), right: None }) }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

#[test]
fn test_zag_with_children() {
    // Setup
    let mut splay_tree = default_start_tree();

    // Exercise
    splay_tree.insert(3);
    splay_tree.insert(5);
    splay_tree.insert(4);
    splay_tree.insert(7);

    splay_tree.splay(5);

    // Verification
    let expected_tree = "TopDownSplayTree { root: Some(SplayNode { key: 5, left: Some(SplayNode { key: 3, left: None, right: Some(SplayNode { key: 4, left: None, right: None }) }), right: Some(SplayNode { key: 7, left: None, right: None }) }) }";

    assert_eq!(format!("{:?}", splay_tree), expected_tree);
}

