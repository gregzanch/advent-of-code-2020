use std::cell::RefCell;
use std::option::Option;
use std::rc::Rc;

// Rc<RefCell<T>> allows T to have multiple owners and can mutate
pub type NodeOption = Option<Rc<RefCell<Node>>>;

#[derive(PartialEq, Debug)]
pub struct Node {
    pub value: i32,
    pub left: NodeOption,
    pub right: NodeOption,
}

#[allow(dead_code)]
impl Node {
    pub fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        let node = Node::new(5);
        assert_eq!(
            node,
            Node {
                value: 5,
                left: None,
                right: None
            }
        );
    }
}
