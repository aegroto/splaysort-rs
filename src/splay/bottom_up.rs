use super::{SplayTree, SplayNode};

#[derive(Default)]
pub struct BottomUpSplayTree<K: Ord + 'static, V: 'static>
{
    root: Option<Box<SplayNode<K, V>>>
}

impl<K: Ord + 'static, V> SplayTree<K, V> for BottomUpSplayTree<K, V> {
    fn insert(&mut self, key: K, value: V) {
        let node : SplayNode<K, V>  = SplayNode {
            key: key,
            value: value
        };

        if self.root.is_none() {
            self.root = Some(Box::new(node));
            return;
        }
    }

    fn search(&self, key: K) {

    }

    fn delete(&mut self, key: K) {

    }

    fn splay(&mut self, key: K) {

    }
}