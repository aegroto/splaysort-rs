#[cfg(test)]
mod benchmarks;

#[cfg(test)]
mod tests;

mod test_utils;

use std::time::{Instant, Duration};

use std::cell::RefCell;

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

pub fn run_experiments() {
    println!("Running small size experiments...");

    run_splaysort_experiments(1024 * 32, 32);
    run_vecsort_experiments(1024 * 32, 32);
    run_vecsort_unstable_experiments(1024 * 32, 32);
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

fn run_vecsort_experiments(n: usize, iterations: u32) {
    let input = Vec::new();

    let input_ref = RefCell::new(input);

    run_experiments_on(
    || {
        input_ref.replace(test_utils::generate_input(n));
    },
    || run_vecsort(&mut input_ref.borrow_mut()),
    iterations, "vec.sort()");
}

fn run_vecsort_unstable_experiments(n: usize, iterations: u32) {
    let input = Vec::new();

    let input_ref = RefCell::new(input);

    run_experiments_on(
    || {
        input_ref.replace(test_utils::generate_input(n));
    },
    || run_vecsort_unstable(&mut input_ref.borrow_mut()),
    iterations, "vec.unstable_sort()");
}

fn run_experiments_on<T, S: FnMut() -> T, F: FnMut() -> T>(mut setup: S, mut execution: F, iterations: u32, header: &str) {
    let mut total = Duration::new(0, 0);

    for _ in 0..iterations {
        setup();

        let start = Instant::now();

        execution();

        total += start.elapsed();
    }

    let mean_execution_time = total / iterations;

    println!("{} mean execution time: {:#?}", header, mean_execution_time);
}

