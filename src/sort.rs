#[cfg(test)]
mod benchmarks;

#[cfg(test)]
mod tests;

mod test_utils;

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