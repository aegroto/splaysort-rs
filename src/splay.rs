pub mod bottom_up;

trait SplayTree<K: Ord + 'static, V: 'static> {
    fn insert(&mut self, key: K, value: V);
    fn search(&self, key: K);
    fn delete(&mut self, key: K);

    fn splay(&mut self, key: K);
}

struct SplayNode<K: Ord + 'static, V> {
    pub key: K,
    pub value: V,

    // pub left: Option<&SplayNode<K, V>>
}