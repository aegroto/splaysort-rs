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

fn test_late_splaysort(n: usize) {
    // Setup
    let mut splay_tree : TopDownSplayTree<u32> = generate_unbalanced_splay_tree(n);

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

fn test_heapsort(n: usize) {
    // Setup
    let input = test_utils::generate_uniform_input(n);
    let mut heap = BinaryHeap::<u32>::new();

    test_utils::fill_binary_heap(&mut heap, input);

    // Exercise
    let sorted_vec = run_heapsort(heap);

    // Verify
    assert_sort(sorted_vec, n);
}

fn test_vecsort(n: usize) {
    // Setup
    let mut input = generate_uniform_input(n);

    // Exercise
    run_vecsort(&mut input);

    // Verify
    assert_sort(input, n);
}

fn test_vecsort_unstable(n: usize) {
    // Setup
    let mut input = generate_uniform_input(n);

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
fn test_late_splaysort_small() {
    test_late_splaysort(SMALL_SIZE);
}

#[test]
fn test_heapsort_small() {
    test_heapsort(SMALL_SIZE);
}

#[test]
fn test_vecsort_small() {
    test_vecsort(SMALL_SIZE);
}

#[test]
fn test_vecsort_unstable_small() {
    test_vecsort_unstable(SMALL_SIZE);
}