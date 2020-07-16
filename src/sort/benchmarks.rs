extern crate test;

use test::Bencher;

use crate::splay::top_down::TopDownSplayTree;

use super::*;

use super::test_utils::*;

const SMALL_SIZE: usize = 1024;

fn bench_splaysort(b: &mut Bencher, n: usize) {
    // Setup
    let mut splay_tree : TopDownSplayTree<u32> = generate_splay_tree(n);

    let mut ordered_elements = Vec::<u32>::new();

    // Bench 
    b.iter(|| run_splaysort(&mut splay_tree, &mut ordered_elements));
}

fn bench_vecsort(b: &mut Bencher, n: usize) {
    // Setup
    let mut input = generate_input(n);

    // Exercise
    b.iter(|| run_vecsort(&mut input));
}

fn bench_vecsort_unstable(b: &mut Bencher, n: usize) {
    // Setup
    let mut input = generate_input(n);

    // Bench 
    b.iter(|| run_vecsort_unstable(&mut input));
}

#[bench]
fn splaysort_small(b: &mut Bencher) {
    bench_splaysort(b, SMALL_SIZE);
}

#[bench]
fn vecsort_small(b: &mut Bencher) {
    bench_vecsort(b, SMALL_SIZE);
}

#[bench]
fn vecsort_unstable_small(b: &mut Bencher) {
    bench_vecsort_unstable(b, SMALL_SIZE);
}