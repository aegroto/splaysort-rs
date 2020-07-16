mod splay;
mod utils;

use rand::Rng;

use splay::top_down::TopDownSplayTree;

use splay::SplayTree;
use splay::sort::SplaySorter;

use utils::assert_sort;

// use splay::visit::Visit;

fn main() {
    let mut splay_tree : TopDownSplayTree<u32> = Default::default();

    // splay_tree.insert(5);
    // splay_tree.insert(3);

    // print!("{}", splay_tree.in_order_visit());

    // splay_tree.splay(3);

    // print!("{}", splay_tree.in_order_visit());

    let mut rng = rand::thread_rng();

    for _ in 0..100000 {
        let k : u32 = rng.gen_range(0, 200);

        splay_tree.splay_insert(k);
    }

    let mut ordered_elements = Vec::<u32>::new();

    while let Some(x) = splay_tree.extract_min() {
        ordered_elements.push(x);
    }

    // println!("{:?}", ordered_elements);

    assert_sort(ordered_elements);

    // println!("# In order visit");
    // println!("{}", splay_tree.in_order_visit());
}
