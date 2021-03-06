pub mod top_down;

pub mod visit;
pub mod sort;

#[cfg(test)]
mod tests;

use std::fmt::Debug;

pub trait BinaryTree<K: Ord + 'static> {
    fn binary_insert(&mut self, key: K);
    fn binary_search(&self, key: K) -> Option<&K>;
    fn binary_delete(&mut self, key: K);
}

pub trait SplayTree<K: Ord + 'static> {
    fn splay(&mut self, key: &K);

    fn splay_insert(&mut self, key: K);
    fn splay_search(&mut self, key: K) -> Option<&K>;
    fn splay_delete(&mut self, key: K);
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