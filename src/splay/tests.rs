use super::top_down::TopDownSplayTree;

mod zig;
mod zag;
mod zig_zig;
mod zag_zag;
mod zig_zag;
mod zag_zig;

mod search;
mod delete;

mod splay_insert;

mod extract_min;

fn default_start_tree() -> TopDownSplayTree<u32> {
    let splay_tree : TopDownSplayTree<u32> = Default::default();

    return splay_tree;
}