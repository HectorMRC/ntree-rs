//! Node definition

use std::{cell::RefCell, sync::Arc};

/// Stores a value and its relation with others.
#[derive(Debug, PartialEq)]
pub struct Node<T> {
    value: T,
    children: Vec<Arc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            children: vec![],
        }
    }

    /// Returns a immutable reference to  node's value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns a mutable reference to node's value.
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Returns a immutable reference to an slice of all node's children.
    pub fn children(&self) -> &[Arc<RefCell<Node<T>>>] {
        &self.children
    }

    /// Returns a mutable reference to an slice of all node's children.
    pub fn children_mut(&mut self) -> &mut [Arc<RefCell<Node<T>>>] {
        &mut self.children
    }

    /// Adds a new child to the node.
    pub fn add_child(&mut self, child: Node<T>) -> usize {
        self.children.push(Arc::new(RefCell::new(child)));
        self.children.len() - 1
    }

    /// Removes the children located at the given index and returns it, if any.
    pub fn remove_child(&mut self, index: usize) -> Option<Arc<RefCell<Node<T>>>> {
        (index < self.children.len()).then_some(self.children.remove(index))
    }

    /// Returns the total of direct descendants (children) the node has.
    pub fn children_len(&self) -> usize {
        self.children.len()
    }

    /// Returns the number of descendants the node has. This method return 0 if,
    /// and only if, the node has no children.
    pub fn size(&self) -> usize {
        self.children
            .iter()
            .fold(self.children.len(), |len, node| -> usize {
                len.saturating_add(node.borrow().size())
            })
    }
}

impl<T> From<Node<T>> for Arc<RefCell<Node<T>>> {
    fn from(val: Node<T>) -> Self {
        Arc::new(RefCell::new(val))
    }
}
