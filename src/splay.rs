pub mod bottom_up;
pub mod top_down;

pub mod visit;

pub trait SplayTree<K: Ord + 'static> {
    fn insert(&mut self, key: K);
    fn search(&self, key: K);
    fn delete(&mut self, key: K);

    fn splay(&mut self, key: K);
}

struct SplayNode<K: Ord + 'static> {
    pub key: K,

    pub left: Option<Box<SplayNode<K>>>,
    pub right: Option<Box<SplayNode<K>>>
}