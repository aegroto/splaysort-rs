use rand;
use rand::Rng;

mod splay;

use splay::bottom_up::BottomUpSplayTree;
use splay::SplayTree;

use splay::visit::Visit;

fn main() {
    let mut splay_tree : BottomUpSplayTree<u32> = Default::default();

    let mut rng = rand::thread_rng();

    splay_tree.in_order_visit();

    for _ in 1..100 {
        let k : u32 = rng.gen_range(0, 200);

        splay_tree.insert(k);
    }

    println!("# Pre order visit");
    splay_tree.pre_order_visit();

    println!("# In order visit");
    splay_tree.in_order_visit();

    println!("# Post order visit");
    splay_tree.post_order_visit();
}
