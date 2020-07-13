use super::{SplayTree, SplayNode};

use super::visit::Visit;

use std::fmt::Debug;
use std::mem::{swap};

#[derive(Default)]
#[derive(Debug)]
pub struct TopDownSplayTree<K: Ord + 'static> {
    root: Option<Box<SplayNode<K>>>
}

// impl<K: Ord + 'static + Default> TopDownSplayTree<K> {
//     pub fn init_from_root(root: Box<SplayNode<K>>) -> Self {
//         let mut tree : TopDownSplayTree::<K> = Default::default();

//         tree.root = Some(root);

//         tree
//     }
// }

impl<K: Ord + 'static + Debug> SplayTree<K> for TopDownSplayTree<K> {
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

    fn splay<'a>(&mut self, key: K) {
        // If the tree is empty, there's nothing to splay
        if self.root.is_none() {
            return;
        }

        // The key is already at root, no need to go further
        if key == self.root.as_ref().unwrap().key {
            return;
        }

        let mut left_tree : Option<Box<SplayNode<K>>> = None; 
        let mut right_tree : Option<Box<SplayNode<K>>> = None;

        let mut x : Box<SplayNode<K>> = self.root.take().unwrap();

        let mut left_anchor : &mut Option<Box<SplayNode<K>>> = &mut left_tree; 
        let mut right_anchor : &mut Option<Box<SplayNode<K>>> = &mut right_tree;

        // let x: &mut Box<SplayNode<K>> = &mut T;
        
        if key <= x.key {
            // Left
            if x.left.is_some() {
                let mut y = x.left.take().unwrap();

                // Node containing key is the left child of the current node
                // Zig
                if key == y.key {
                    let old_anchor = right_anchor.get_or_insert(x);
                    right_anchor = &mut old_anchor.left;

                    x = y;
                } else {
                    if key <= y.key {
                        // Left
                        // Zig zig
                        let z = y.left.take().unwrap();

                        let old_y_right_child = y.right.take();
                        x.left = old_y_right_child;
                        y.right.replace(x);

                        let old_anchor = right_anchor.get_or_insert(y);
                        right_anchor = &mut old_anchor.left;

                        x = z;
                    } else {
                        // Right
                        // Zig zag
                        let z = y.right.take().unwrap();

                        let old_right_anchor = right_anchor.get_or_insert(x);
                        right_anchor = &mut old_right_anchor.left;

                        let old_left_anchor = left_anchor.get_or_insert(y);
                        left_anchor = &mut old_left_anchor.right;

                        x = z;
                    }
                }
            } 

            // Key is not present in the tree, we splay the last node we were on
        } else {
            // Right 
            if x.right.is_some() {
                let mut y = x.right.take().unwrap();

                // Node containing key is the right child of the current node
                // Zag
                if key == y.key {
                    let old_anchor = left_anchor.get_or_insert(x);
                    left_anchor = &mut old_anchor.right;

                    x = y;
                } else {
                    if key > y.key {
                        // Right
                        // Zag zag
                        let z = y.right.take().unwrap();

                        let old_y_left_child = y.left.take();
                        x.right = old_y_left_child;
                        y.left.replace(x);

                        let old_anchor = left_anchor.get_or_insert(y);
                        left_anchor = &mut old_anchor.right;

                        x = z;
                    } else {
                        // Left
                        // Zag zig
                        let z = y.left.take().unwrap();

                        let old_left_anchor = left_anchor.get_or_insert(x);
                        left_anchor = &mut old_left_anchor.right;

                        let old_right_anchor = right_anchor.get_or_insert(y);
                        right_anchor = &mut old_right_anchor.left;

                        x = z;
                    }
                }
            } 

            // Key is not present in the tree, we splay the last node we were on
        } 

        // Assembling the trees
        let mut left_child = x.left.take();
        let mut right_child = x.right.take();

        swap(left_anchor, &mut left_child);
        swap(right_anchor, &mut right_child);

        x.left = left_tree;
        x.right = right_tree;

        self.root = Some(x);
    }
}

impl<K: Ord + Debug + 'static> Visit for TopDownSplayTree<K> {
    fn in_order_visit(&self) -> String {
        if self.root.is_none() {
            return String::from("Empty");
        }

        let mut output = String::new();

        output += &format!("Root: {:?}\n", self.root.as_ref().unwrap().key).as_str();

        output += &self.root.as_ref().unwrap().in_order_visit();

        output += "\n";

        output
    }

    fn pre_order_visit(&self) -> String  {
        if self.root.is_none() {
            return String::from("Empty");
        }

        let mut output = String::new();

        output += &format!("Root: {:?}\n", self.root.as_ref().unwrap().key).as_str();

        output += &self.root.as_ref().unwrap().pre_order_visit();

        output += "\n";

        output
    }

    fn post_order_visit(&self) -> String {
        if self.root.is_none() {
            return String::from("Empty");
        }

        let mut output = String::new();

        output += &format!("Root: {:?}\n", self.root.as_ref().unwrap().key).as_str();

        output += &self.root.as_ref().unwrap().post_order_visit();

        output += "\n";

        output
    }
}