extern crate test;

use test::Bencher;

use crate::splay::top_down::TopDownSplayTree;

use super::*;

use super::test_utils::*;

// WARNING: Those benchmarks consider input initialization due to Rust's benchmark system limit
// Run the crate for sorting-only experimental results

const SMALL_SIZE: usize = 1024;
const MEDIUM_SIZE: usize = 1024 * 128;

fn bench_splaysort(b: &mut Bencher, n: usize) {
    b.iter(|| {
        let mut splay_tree : TopDownSplayTree<u32> = generate_splay_tree(n);
        let mut ordered_elements = Vec::<u32>::new();

        run_splaysort(&mut splay_tree, &mut ordered_elements)
    });
}

fn bench_vecsort(b: &mut Bencher, n: usize) {
    b.iter(|| {
        let input = generate_uniform_input(n);
        run_vecsort(&mut input.clone())
    });
}

fn bench_vecsort_unstable(b: &mut Bencher, n: usize) {
    b.iter(|| {
        let input = generate_uniform_input(n);
        run_vecsort_unstable(&mut input.clone())
    });
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

#[bench]
fn splaysort_medium(b: &mut Bencher) {
    bench_splaysort(b, MEDIUM_SIZE);
}

#[bench]
fn vecsort_medium(b: &mut Bencher) {
    bench_vecsort(b, MEDIUM_SIZE);
}

#[bench]
fn vecsort_unstable_medium(b: &mut Bencher) {
    bench_vecsort_unstable(b, MEDIUM_SIZE);
}