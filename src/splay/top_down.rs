use super::{SplayTree, SplayNode};

use super::visit::Visit;

use super::sort::SplaySorter;

use std::fmt::Debug;
use std::mem::{swap};
use std::cmp::Ordering;

#[derive(Default)]
#[derive(Debug)]
pub struct TopDownSplayTree<K: Ord + 'static> {
    root: Option<Box<SplayNode<K>>>
}

impl<K: Ord + 'static + Debug> TopDownSplayTree<K> {
    fn zig(x : Box<SplayNode<K>>, y : Box<SplayNode<K>>, right_anchor: &mut Option<Box<SplayNode<K>>>) -> (Box<SplayNode<K>>, &mut Option<Box<SplayNode<K>>>) {
        let old_right_anchor = right_anchor.get_or_insert(x);

        (y, &mut old_right_anchor.left)
    }

    fn zig_zig(mut x : Box<SplayNode<K>>, mut y : Box<SplayNode<K>>, right_anchor: &mut Option<Box<SplayNode<K>>>) -> (Box<SplayNode<K>>, &mut Option<Box<SplayNode<K>>>) {
        let z = y.left.take().unwrap();

        let old_y_right_child = y.right.take();
        x.left = old_y_right_child;
        y.right.replace(x);

        let old_anchor = right_anchor.get_or_insert(y);

        (z, &mut old_anchor.left)
    }

    fn zig_zag<'a>(x : Box<SplayNode<K>>, mut y : Box<SplayNode<K>>, left_anchor: &'a mut Option<Box<SplayNode<K>>>, right_anchor: &'a mut Option<Box<SplayNode<K>>>) -> (Box<SplayNode<K>>, &'a mut Option<Box<SplayNode<K>>>, &'a mut Option<Box<SplayNode<K>>>) {
        let z = y.right.take().unwrap();

        let old_left_anchor = left_anchor.get_or_insert(y);
        let old_right_anchor = right_anchor.get_or_insert(x);

        (z, &mut old_left_anchor.right, &mut old_right_anchor.left)
    }

    fn zag(x : Box<SplayNode<K>>, y : Box<SplayNode<K>>, left_anchor: &mut Option<Box<SplayNode<K>>>) -> (Box<SplayNode<K>>, &mut Option<Box<SplayNode<K>>>) {
        let old_left_anchor = left_anchor.get_or_insert(x);

        (y, &mut old_left_anchor.right)
    }

    fn zag_zag(mut x : Box<SplayNode<K>>, mut y : Box<SplayNode<K>>, left_anchor: &mut Option<Box<SplayNode<K>>>) -> (Box<SplayNode<K>>, &mut Option<Box<SplayNode<K>>>) {
        let z = y.right.take().unwrap();

        let old_y_left_child = y.left.take();
        x.right = old_y_left_child;
        y.left.replace(x);

        let old_anchor = left_anchor.get_or_insert(y);

        (z, &mut old_anchor.right)
    }

    fn zag_zig<'a>(x : Box<SplayNode<K>>, mut y : Box<SplayNode<K>>, left_anchor: &'a mut Option<Box<SplayNode<K>>>, right_anchor: &'a mut Option<Box<SplayNode<K>>>) -> (Box<SplayNode<K>>, &'a mut Option<Box<SplayNode<K>>>, &'a mut Option<Box<SplayNode<K>>>) {
        let z = y.left.take().unwrap();

        let old_left_anchor = left_anchor.get_or_insert(x);
        let old_right_anchor = right_anchor.get_or_insert(y);

        (z, &mut old_left_anchor.right, &mut old_right_anchor.left)
    }

    fn get_node_containing_as_mut<'a>(&'a mut self, key: K) -> &'a mut Option<Box<SplayNode<K>>> {
        let mut x : &'a mut Option<Box<SplayNode<K>>> = &mut self.root;
        
        let result : &'a mut Option<Box<SplayNode<K>>> = loop {
            if x.is_none() || x.as_ref().unwrap().key == key {
                break x;
            }
            
            let current_node = x.as_mut().unwrap();

            if key < current_node.key {
                x = &mut current_node.left;
            } else {
                x = &mut current_node.right;
            }
        };

        result
    }

    fn get_node_containing(&self, key: K) -> &Option<Box<SplayNode<K>>> {
        let mut x : &Option<Box<SplayNode<K>>> = &self.root;
        
        let result : &Option<Box<SplayNode<K>>> = loop {
            if x.is_none() {
                break x;
            }
            
            let current_node = x.as_ref().unwrap();

            match key.cmp(&current_node.key) {
                Ordering::Equal => break x,
                Ordering::Less => x = &current_node.left,
                Ordering::Greater => x = &current_node.right,
            }
        };

        result
    }
}

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

    fn splay_insert(&mut self, key: K) {
        let mut node : SplayNode<K>  = SplayNode {
            key: key,

            left : None,
            right: None
        };

        if self.root.is_none() {
            self.root.replace(Box::new(node));
            return;
        }

        self.splay(&node.key);

        let mut current_root = self.root.take().unwrap();

        if node.key <= current_root.key {
            node.left = current_root.left.take();
            node.right.replace(current_root);

            self.root = Some(Box::new(node));
        } else {
            node.right = current_root.right.take();
            node.left.replace(current_root);

            self.root = Some(Box::new(node));
        }
    }

    fn search(&self, key: K) -> Option<&K> {
        match self.get_node_containing(key) {
            Some(node) => Some(&node.key),
            None => None
        }
    }

    fn delete(&mut self, key: K) {
        self.get_node_containing_as_mut(key).take();
    }

    fn splay(&mut self, key: &K) {
        // If the tree is empty, there's nothing to splay
        if self.root.is_none() {
            return;
        }

        // The key is already at root, no need to go further
        if key == &self.root.as_ref().unwrap().key {
            return;
        }

        let mut left_tree : Option<Box<SplayNode<K>>> = None; 
        let mut right_tree : Option<Box<SplayNode<K>>> = None;

        let mut x : Box<SplayNode<K>> = self.root.take().unwrap();

        let mut left_anchor : &mut Option<Box<SplayNode<K>>> = &mut left_tree; 
        let mut right_anchor : &mut Option<Box<SplayNode<K>>> = &mut right_tree;

        loop {
            if key <= &x.key {
                // Left
                if x.left.is_some() {
                    let y = x.left.take().unwrap();

                    if key < &y.key && y.left.is_some() {
                        // println!("Zig Zig");
                        let (new_x, new_right_anchor) = TopDownSplayTree::<K>::zig_zig(x, y, right_anchor);

                        x = new_x;
                        right_anchor = new_right_anchor; 
                    } else if key > &y.key && y.right.is_some() {
                        // println!("Zig Zag");
                        let (new_x, new_left_anchor, new_right_anchor) = TopDownSplayTree::<K>::zig_zag(x, y, left_anchor, right_anchor);

                        x = new_x;
                        left_anchor = new_left_anchor; 
                        right_anchor = new_right_anchor; 
                    } else {
                        // println!("Zig");

                        let (new_x, new_right_anchor) = TopDownSplayTree::<K>::zig(x, y, right_anchor);
                        
                        x = new_x;
                        right_anchor = new_right_anchor; 
                    }
                } else {
                    break;
                }
            } else {
                // Right 
                if x.right.is_some() {
                    let y = x.right.take().unwrap();

                    if key > &y.key && y.right.is_some() {
                        // println!("Zag Zag");
                        let (new_x, new_left_anchor) = TopDownSplayTree::<K>::zag_zag(x, y, left_anchor);

                        x = new_x;
                        left_anchor = new_left_anchor; 
                    } else if key < &y.key  && y.left.is_some() {
                        // println!("Zag Zig");

                        let (new_x, new_left_anchor, new_right_anchor) = TopDownSplayTree::<K>::zag_zig(x, y, left_anchor, right_anchor);

                        x = new_x;
                        left_anchor = new_left_anchor; 
                        right_anchor = new_right_anchor;
                    } else {
                        // println!("Zag");

                        let (new_x, new_left_anchor) = TopDownSplayTree::<K>::zag(x, y, left_anchor);
                        
                        x = new_x;
                        left_anchor = new_left_anchor; 
                    }
                } else {
                    break;
                }
            } 

            if key == &x.key || x.is_leaf() {
                break;
            }
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

impl<K: Ord + 'static + Debug> SplaySorter<K> for TopDownSplayTree<K> {
    fn splay_min(&mut self) {
        // If the tree is empty, there's nothing to splay
        if self.root.is_none() {
            return;
        }

        // Root already contains the minimum key
        if self.root.as_ref().unwrap().left.is_none() {
            return;
        }

        let mut right_tree : Option<Box<SplayNode<K>>> = None;

        let mut x : Box<SplayNode<K>> = self.root.take().unwrap();

        let mut right_anchor : &mut Option<Box<SplayNode<K>>> = &mut right_tree;

        loop {
            if x.left.is_none() {
                break;
            } 

            let y = x.left.take().unwrap();

            if y.left.is_some() {
                let (new_x, new_right_anchor) = TopDownSplayTree::<K>::zig_zig(x, y, right_anchor);

                x = new_x;
                right_anchor = new_right_anchor; 
            } else {
                let (new_x, new_right_anchor) = TopDownSplayTree::<K>::zig(x, y, right_anchor);
                
                x = new_x;
                right_anchor = new_right_anchor; 
            } 
        }

        let mut right_child = x.right.take();

        swap(right_anchor, &mut right_child);

        x.right = right_tree;

        self.root = Some(x);
    }

    fn extract_min(&mut self) -> Option<K> {
        self.splay_min();

        match self.root.take() {
            None => None,
            Some(root) => {
                self.root = root.right;
                Some(root.key)
            }
        }
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