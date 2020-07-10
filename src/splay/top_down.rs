use super::{SplayTree, SplayNode};

use super::visit::Visit;

use std::fmt::Debug;
use std::mem::{replace, swap};

use std::borrow::{Borrow};

#[derive(Default)]
pub struct TopDownSplayTree<K: Ord + 'static> {
    root: Option<Box<SplayNode<K>>>
}

// impl<K: Ord + 'static> TopDownSplayTree<K> {
//     fn splay_from(&mut self, root: Box<SplayNode<K>>, key: K) {

//     }
// }

impl<K: Ord + 'static> SplayTree<K> for TopDownSplayTree<K> {
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

        let mut L : Option<Box<SplayNode<K>>> = None; 
        let mut R : Option<Box<SplayNode<K>>> = None;
        let mut T : Box<SplayNode<K>> = self.root.take().unwrap();

        let mut L_anchor : &mut Option<Box<SplayNode<K>>> = &mut L; 
        let mut R_anchor : &mut Option<Box<SplayNode<K>>> = &mut R;

        let mut parent_node: &mut Box<SplayNode<K>> = &mut T;
        
        if key < parent_node.key {
            // Left
            if parent_node.left.is_some() {
                let x = parent_node.left.take().unwrap();

                // Node containing key is an immediate child of the current node
                // Zig
                if key == x.key {
                    let old_anchor = R_anchor.get_or_insert(T);
                    R_anchor = &mut old_anchor.left;

                    T = x;
                }
            } 

            // Key is not present in the tree, we splay the last node we were on
        } else {
            // Right 
            if parent_node.right.is_some() {
                let x = parent_node.right.take().unwrap();

                // Node containing key is an immediate child of the current node
                // Zig
                if key == x.key {
                    let old_anchor = L_anchor.get_or_insert(T);
                    L_anchor = &mut old_anchor.right;

                    T = x;
                }
            } 

            // Key is not present in the tree, we splay the last node we were on
        } 

        // Assembling the trees
        let mut A = T.left.take();
        let mut B = T.right.take();

        swap(L_anchor, &mut A);
        swap(R_anchor, &mut B);

        T.left = L;
        T.right = R;

        self.root = Some(T);
    }
}

impl<K: Ord + Debug + 'static> Visit for TopDownSplayTree<K> {
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