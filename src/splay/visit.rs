use super::SplayNode;

use std::fmt::Debug;

pub trait Visit {
    fn in_order_visit(&self);
    fn pre_order_visit(&self);
    fn post_order_visit(&self);
}

impl<K: Ord + Debug + 'static> Visit for SplayNode<K> {
    fn in_order_visit(&self) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().in_order_visit();
        }

        print!("{:?} ", self.key);

        if self.right.is_some() {
            self.right.as_ref().unwrap().in_order_visit();
        }
    }

    fn pre_order_visit(&self) {
        print!("{:?} ", self.key);

        if self.left.is_some() {
            self.left.as_ref().unwrap().pre_order_visit();
        }

        if self.right.is_some() {
            self.right.as_ref().unwrap().pre_order_visit();
        }
    }

    fn post_order_visit(&self) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().post_order_visit();
        }

        if self.right.is_some() {
            self.right.as_ref().unwrap().post_order_visit();
        }

        print!("{:?} ", self.key);
    }
}
