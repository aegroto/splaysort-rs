use super::SplayNode;

use std::fmt::Debug;

pub trait Visit {
    fn in_order_visit(&self) -> String;
    fn pre_order_visit(&self) -> String;
    fn post_order_visit(&self) -> String;
}

impl<K: Ord + Debug + 'static> Visit for SplayNode<K> {
    fn in_order_visit(&self) -> String {
        let mut output = String::new();

        if self.left.is_some() {
            output += &self.left.as_ref().unwrap().in_order_visit();
        }

        output += &format!("{:?} ", self.key).as_str();

        if self.right.is_some() {
            output += &self.right.as_ref().unwrap().in_order_visit();
        }
        
        output
    }

    fn pre_order_visit(&self) -> String {
        let mut output = String::new();

        output += &format!("{:?}", self.key).as_str();

        if self.left.is_some() {
            output += &self.left.as_ref().unwrap().in_order_visit();
        }

        if self.right.is_some() {
            output += &self.right.as_ref().unwrap().in_order_visit();
        }
        
        output
    }

    fn post_order_visit(&self) -> String {
        let mut output = String::new();

        if self.left.is_some() {
            output += &self.left.as_ref().unwrap().in_order_visit();
        }

        if self.right.is_some() {
            output += &self.right.as_ref().unwrap().in_order_visit();
        }

        output += &format!("{:?}", self.key).as_str();
        
        output
    }
}
