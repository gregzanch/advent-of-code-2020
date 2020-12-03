pub mod node;
use crate::bst::node::{Node, NodeOption};
use std::cell::RefCell;
use std::rc::Rc;

pub struct BinarySearchTree {
    root: NodeOption,
    pub count: u32,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            count: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn new_node_option(&self, value: i32) -> NodeOption {
        Some(Rc::new(RefCell::new(Node::new(value))))
    }

    pub fn insert(&mut self, value: i32) {
        if self.is_empty() {
            self.root = self.new_node_option(value);
            self.count = 1;
            return;
        }

        let should_insert_right;

        // set the current node to a copy of the root
        let mut current_node = Rc::clone(self.root.as_ref().unwrap());

        loop {
            let next_node: Rc<RefCell<Node>>;

            if value < current_node.borrow().value {
                match &current_node.borrow().left {
                    Some(left_node) => next_node = Rc::clone(left_node),
                    _ => {
                        should_insert_right = false;
                        break;
                    }
                }
            } else {
                match &current_node.borrow().right {
                    Some(right_node) => next_node = Rc::clone(right_node),
                    _ => {
                        should_insert_right = true;
                        break;
                    }
                }
            }
            current_node = next_node;
        }

        if should_insert_right {
            current_node.borrow_mut().right = self.new_node_option(value);
        } else {
            current_node.borrow_mut().left = self.new_node_option(value);
        }

        self.count += 1;
    }

    pub fn sort(&self, arr: &mut Vec<i32>) {
        self.in_order(&self.root, arr);
    }

    fn in_order(&self, root: &NodeOption, arr: &mut Vec<i32>) {
        if let Some(node) = root {
            self.in_order(&node.borrow().left, arr);
            arr.push(node.borrow().value);
            self.in_order(&node.borrow().right, arr);
        }
    }

    pub fn find_closest(&self, value: i32) -> Result<i32, bool> {
        if self.is_empty() {
            return Err(false);
        }

        let mut current_node = Rc::clone(self.root.as_ref().unwrap());

        loop {
            let next_node;
            // if queried value is less than the current value
            if value == current_node.borrow().value {
                return Ok(value);
            } else if value < current_node.borrow().value {
                match &current_node.borrow().left {
                    Some(left_node) => next_node = Rc::clone(left_node),
                    _ => return Ok(current_node.borrow().value),
                }
            } else {
                match &current_node.borrow().right {
                    Some(right_node) => next_node = Rc::clone(right_node),
                    _ => return Ok(current_node.borrow().value),
                }
            }
            current_node = next_node;
        }
    }

    pub fn reset(&mut self) {
        self.root.take();
        self.count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_a_node() {
        let mut arr = vec![];
        let mut bs_tree = BinarySearchTree::new();

        bs_tree.insert(10);
        bs_tree.insert(20);
        bs_tree.insert(3);
        bs_tree.insert(7);
        assert_eq!(bs_tree.count, 4);

        bs_tree.sort(&mut arr);
        assert_eq!(arr, vec![3, 7, 10, 20]);
    }

    #[test]
    fn test_find() {
        let mut bs_tree = BinarySearchTree::new();

        bs_tree.insert(8);
        bs_tree.insert(20);
        bs_tree.insert(3);
        bs_tree.insert(25);
        bs_tree.insert(7);

        let a = bs_tree.find_closest(5).expect("no elements");
        let c = bs_tree.find_closest(99).expect("no elements");

        assert_eq!(a, 7);
        assert_eq!(c, 25);
    }
}
