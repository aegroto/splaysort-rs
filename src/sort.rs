#[cfg(test)]
mod benchmarks;

#[cfg(test)]
mod tests;

mod test_utils;

use std::time::{Instant, Duration};

use std::cell::RefCell;

use std::collections::BinaryHeap;

use crate::splay::sort::SplaySorter;

use crate::splay::top_down::TopDownSplayTree;

pub fn run_vecsort(input : &mut Vec::<u32>) {
    input.sort();
}

pub fn run_vecsort_unstable(input : &mut Vec::<u32>) {
    input.sort_unstable();
}

pub fn run_splaysort(splay_tree: &mut TopDownSplayTree<u32>, recipient: &mut Vec::<u32>) {
    while let Some(x) = splay_tree.extract_min() {
        recipient.push(x);
    }
}

pub fn run_heapsort(heap: BinaryHeap::<u32>) -> Vec::<u32> {
    heap.into_sorted_vec()
}

pub fn run_experiments() {
    let samples_count = 32;

    println!("### Running small size experiments...");
    let small_size = 1024;

    run_splaysort_experiments(small_size, samples_count);
    run_late_splaysort_experiments(small_size, samples_count);
    run_heapsort_experiments(small_size, samples_count);
    run_vecsort_experiments(small_size, samples_count);
    run_vecsort_unstable_experiments(small_size, samples_count);

    println!("### Running medium size experiments...");
    let medium_size = 1024 * 32;

    run_splaysort_experiments(medium_size, samples_count);
    run_late_splaysort_experiments(medium_size, samples_count);
    run_heapsort_experiments(medium_size, samples_count);
    run_vecsort_experiments(medium_size, samples_count);
    run_vecsort_unstable_experiments(medium_size, samples_count);

    println!("### Running large size experiments...");
    let large_size = 1024 * 512;

    run_splaysort_experiments(large_size, samples_count);
    run_late_splaysort_experiments(large_size, samples_count);
    run_heapsort_experiments(large_size, samples_count);
    run_vecsort_experiments(large_size, samples_count);
    run_vecsort_unstable_experiments(large_size, samples_count);
}

fn run_splaysort_experiments(n: usize, iterations: u32) {
    let splay_tree : TopDownSplayTree<u32> = Default::default();
    let ordered_elements = Vec::new();

    let splay_tree_ref = RefCell::new(splay_tree);
    let ordered_elements_ref = RefCell::new(ordered_elements);

    run_experiments_on(
    || {
        splay_tree_ref.replace(test_utils::generate_splay_tree(n));
        ordered_elements_ref.replace(Vec::<u32>::new());
    },
    || run_splaysort(&mut splay_tree_ref.borrow_mut(), &mut ordered_elements_ref.borrow_mut()),
    iterations, "SplaySort");
}

fn run_late_splaysort_experiments(n: usize, iterations: u32) {
    let splay_tree : TopDownSplayTree<u32> = Default::default();
    let ordered_elements = Vec::new();

    let splay_tree_ref = RefCell::new(splay_tree);
    let ordered_elements_ref = RefCell::new(ordered_elements);

    run_experiments_on(
    || {
        splay_tree_ref.replace(test_utils::generate_unbalanced_splay_tree(n));
        ordered_elements_ref.replace(Vec::<u32>::new());
    },
    || run_splaysort(&mut splay_tree_ref.borrow_mut(), &mut ordered_elements_ref.borrow_mut()),
    iterations, "Late SplaySort");
}

fn run_heapsort_experiments(n: usize, iterations: u32) {
    let heap_ref = RefCell::new(BinaryHeap::<u32>::new());

    run_experiments_on(
    || {
        let input = test_utils::generate_uniform_input(n);
        let mut heap = BinaryHeap::<u32>::new();

        test_utils::fill_binary_heap(&mut heap, input);

        heap_ref.replace(heap);
    },
    || {
        let heap = heap_ref.replace(BinaryHeap::<u32>::new());
        let _ = run_heapsort(heap);
    },
    iterations, "heap.into_sorted_vec()");
}

fn run_vecsort_experiments(n: usize, iterations: u32) {
    let input = Vec::new();

    let input_ref = RefCell::new(input);

    run_experiments_on(
    || {
        input_ref.replace(test_utils::generate_uniform_input(n));
    },
    || run_vecsort(&mut input_ref.borrow_mut()),
    iterations, "vec.sort()");
}

fn run_vecsort_unstable_experiments(n: usize, iterations: u32) {
    let input = Vec::new();

    let input_ref = RefCell::new(input);

    run_experiments_on(
    || {
        input_ref.replace(test_utils::generate_uniform_input(n));
    },
    || run_vecsort_unstable(&mut input_ref.borrow_mut()),
    iterations, "vec.unstable_sort()");
}

fn run_experiments_on<T, S: FnMut() -> T, F: FnMut() -> T>(mut setup: S, mut execution: F, iterations: u32, header: &str) {
    let mut setup_total = Duration::new(0, 0);
    let mut execution_total = Duration::new(0, 0);

    for _ in 0..iterations {
        let setup_start = Instant::now();

        setup();

        setup_total += setup_start.elapsed();

        let execution_start = Instant::now();

        execution();

        execution_total += execution_start.elapsed();
    }

    println!("{} mean setup time: {:?} µs", header, (setup_total / iterations).as_micros());
    println!("{} mean execution time: {:?} µs", header, (execution_total / iterations).as_micros());
}

