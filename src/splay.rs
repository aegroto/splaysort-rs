pub mod top_down;

pub mod visit;

mod tests;

use std::fmt::Debug;

pub trait SplayTree<K: Ord + 'static> {
    fn insert(&mut self, key: K);
    fn search(&self, key: K);
    fn delete(&mut self, key: K);

    fn splay(&mut self, key: &K);

    fn splay_insert(&mut self, key: K);
}

#[derive(Debug)]
struct SplayNode<K: Ord + 'static> {
    pub key: K,

    pub left: Option<Box<SplayNode<K>>>,
    pub right: Option<Box<SplayNode<K>>>
}

impl<K: Ord + 'static> SplayNode<K> {
    pub fn is_leaf(&self) -> bool {
        return self.left.is_none() && self.right.is_none();
    }
}