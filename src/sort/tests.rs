use crate::splay::top_down::TopDownSplayTree;

use super::*;

use super::test_utils::*;

use std::time::{SystemTime};

const SMALL_SIZE: usize = 1024 * 128;

fn test_splaysort(n: usize) {
    // Setup
    let mut splay_tree : TopDownSplayTree<u32> = generate_splay_tree(n);

    let mut ordered_elements = Vec::<u32>::new();

    // Exercise 
    let start = SystemTime::now();

    run_splaysort(&mut splay_tree, &mut ordered_elements);

    let end = SystemTime::now();

    let time = end.duration_since(start);

    println!("{:?}", time);

    // Verify
    assert_sort(ordered_elements, n);
}

fn test_vecsort(n: usize) {
    // Setup
    let mut input = generate_input(n);

    // Exercise
    run_vecsort(&mut input);

    // Verify
    assert_sort(input, n);
}

fn test_vecsort_unstable(n: usize) {
    // Setup
    let mut input = generate_input(n);

    // Exercise
    run_vecsort_unstable(&mut input);

    // Verify
    assert_sort(input, n);
}

#[test]
fn test_splaysort_small() {
    test_splaysort(SMALL_SIZE);
}


#[test]
fn test_vecsort_small() {
    test_vecsort(SMALL_SIZE);
}

#[test]
fn test_vecsort_unstable_small() {
    test_vecsort_unstable(SMALL_SIZE);
}