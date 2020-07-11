use super::top_down::TopDownSplayTree;

mod zig;
mod zag;
mod zig_zig;

fn default_start_tree() -> TopDownSplayTree<u32> {
    let splay_tree : TopDownSplayTree<u32> = Default::default();

    return splay_tree;
}