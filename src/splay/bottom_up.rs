use super::{SplayTree, SplayNode};

use super::visit::Visit;

use std::fmt::Debug;

#[derive(Default)]
pub struct BottomUpSplayTree<K: Ord + 'static>
{
    root: Option<Box<SplayNode<K>>>
}

impl<K: Ord + 'static> SplayTree<K> for BottomUpSplayTree<K> {
    fn insert(&mut self, key: K) {
        let node : SplayNode<K>  = SplayNode {
            key: key,

            left : None,
            right: None
        };

        if self.root.is_none() {
            self.root = Some(Box::new(node));
            return;
        }

        let mut p : &mut Option<Box<SplayNode<K>>> = &mut self.root;

        loop {
            let mut current_node = p.as_mut().unwrap();

            if node.key <= current_node.key {
                if current_node.left.is_some() {
                    p = &mut current_node.left
                } else {
                    current_node.left = Some(Box::new(node));
                    break;
                }
            } else {
                if current_node.right.is_some() {
                    p = &mut current_node.right
                } else {
                    current_node.right = Some(Box::new(node));
                    break;
                }
            }

        }
    }

    fn search(&self, _key: K) {

    }

    fn delete(&mut self, _key: K) {

    }

    fn splay(&mut self, _key: K) {

    }
}

impl<K: Ord + Debug + 'static> Visit for BottomUpSplayTree<K> {
    fn in_order_visit(&self) {
        if self.root.is_none() {
            println!("Empty");
            return;
        }

        println!("Root: {:?}", self.root.as_ref().unwrap().key);

        self.root.as_ref().unwrap().in_order_visit();
        print!("\n")
    }

    fn pre_order_visit(&self) {
        if self.root.is_none() {
            println!("Empty");
            return;
        }

        println!("Root: {:?}", self.root.as_ref().unwrap().key);

        self.root.as_ref().unwrap().pre_order_visit();
        print!("\n")
    }

    fn post_order_visit(&self) {
        if self.root.is_none() {
            println!("Empty");
            return;
        }

        println!("Root: {:?}", self.root.as_ref().unwrap().key);

        self.root.as_ref().unwrap().post_order_visit();
        print!("\n")
    }
}