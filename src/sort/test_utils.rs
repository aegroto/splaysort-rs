use std::fmt::Debug;

use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::splay::top_down::TopDownSplayTree;
use crate::splay::SplayTree;

#[allow(dead_code)]
const TEST_SEED : u64 = 0;

#[allow(dead_code)]
pub fn generate_input(n: usize) -> Vec::<u32> {
    let mut input = Vec::<u32>::new();

    let mut rng = StdRng::seed_from_u64(TEST_SEED);

    for _ in 0..n {
        let k : u32 = rng.gen_range(0, 100);

        input.push(k);
    }

    input
}

#[allow(dead_code)]
pub fn assert_sort<K: Ord + Debug>(vec: Vec<K>, n: usize) {
    assert_eq!(vec.len(), n);

    for i in 1..vec.len() {
        match vec[i] < vec[i-1] {
            true => panic!("Elements at indices {} and {} are not ordered correctly ({:?} < {:?})", i, i-1, vec[i], vec[i-1]),
            _ => ()
        }
    }
}

#[allow(dead_code)]
pub fn generate_splay_tree(n: usize) -> TopDownSplayTree::<u32> {
    let mut splay_tree : TopDownSplayTree<u32> = Default::default();

    let input = generate_input(n);

    input.into_iter().for_each(|x| splay_tree.splay_insert(x));

    splay_tree
}

