extern crate test;

use test::Bencher;

use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::splay::SplayTree;
use crate::splay::top_down::TopDownSplayTree;
use crate::splay::sort::SplaySorter;

const TEST_SEED : u64 = 0;

fn generate_input(n: usize) -> Vec::<u32> {
    let mut input = Vec::<u32>::new();

    let mut rng = StdRng::seed_from_u64(TEST_SEED);

    for _ in 0..n {
        let k : u32 = rng.gen_range(u32::MIN, u32::MAX);

        input.push(k);
    }

    input
}

fn splaysort_bench(n: usize) {
    let mut splay_tree : TopDownSplayTree<u32> = Default::default();

    let input = generate_input(n);

    input.into_iter().for_each(|x| splay_tree.splay_insert(x));

    let mut ordered_elements = Vec::<u32>::new();

    while let Some(x) = splay_tree.extract_min() {
        ordered_elements.push(x);
    }
}

#[bench]
fn splaysort_medium(b: &mut Bencher) {
    b.iter(|| splaysort_bench(1000));
}