//! Iterator implementations

use crate::node::Node;
use std::{cell::RefCell, sync::Arc};

/// Implements a pre-order iterator of mutable references of [`Node`]
pub struct PreOrderMutIterator<T> {
    root: Arc<RefCell<Node<T>>>,
    current: Option<Box<PreOrderMutIterator<T>>>,
    next: usize,
}

impl<T> PreOrderMutIterator<T> {
    pub fn new(node: Arc<RefCell<Node<T>>>) -> Self {
        Self {
            root: node,
            current: None,
            next: 0,
        }
    }
}

impl<T> Iterator for PreOrderMutIterator<T> {
    type Item = Arc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == 0 {
            // pre-order traversal visits the parent before moving forward with children
            self.next += 1;
            return Some(self.root.clone());
        }

        if self.current.is_none() {
            // load next bunch of iterations
            self.current = self
                .root
                .borrow_mut()
                .children_mut()
                .get(self.next - 1)
                .map(|child| PreOrderMutIterator::new(child.clone()))
                .map(Into::into);
        }

        self.current.as_mut()?.next().or_else(|| {
            self.next += 1;
            self.current = None;
            self.next()
        })
    }
}

/// Implements a post-order iterator of mutable references of [`Node`]
pub struct PostOrderMutIterator<T> {
    root: Arc<RefCell<Node<T>>>,
    current: Option<Box<PostOrderMutIterator<T>>>,
    next: usize,
}

impl<T> PostOrderMutIterator<T> {
    pub fn new(node: Arc<RefCell<Node<T>>>) -> Self {
        Self {
            root: node,
            current: None,
            next: 0,
        }
    }
}

impl<T> Iterator for PostOrderMutIterator<T> {
    type Item = Arc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == self.root.borrow().children_len() {
            // post-order traversal visits the parent after traversing children
            self.next += 1;
            self.current = None;
            return Some(self.root.clone());
        }

        if self.current.is_none() {
            // load next bunch of iterations
            self.current = self
                .root
                .borrow_mut()
                .children_mut()
                .get(self.next)
                .map(|child| PostOrderMutIterator::new(child.clone()))
                .map(Into::into);
        }

        self.current.as_mut()?.next().or_else(|| {
            self.next += 1;
            self.current = None;
            self.next()
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, sync::Arc};

    use crate::{
        iterator::{PostOrderMutIterator, PreOrderMutIterator},
        node::Node,
    };

    #[test]
    fn pre_order_traversal() {
        let mut root = Node::<usize>::new(0);
        let child_1 = Node::<usize>::new(1);
        let mut child_2 = Node::<usize>::new(2);
        let child_3 = Node::<usize>::new(3);

        assert_eq!(root.add_child(child_1), 0);
        assert_eq!(child_2.add_child(child_3), 0);
        assert_eq!(root.add_child(child_2), 1);

        let got = PreOrderMutIterator::new(root.into())
            .map(|node| *node.borrow().value())
            .collect::<Vec<usize>>();

        assert_eq!(got, vec![0, 1, 2, 3]);
    }

    #[test]
    fn post_order_traversal() {
        let mut root = Node::<usize>::new(0);
        let child_1 = Node::<usize>::new(1);
        let mut child_2 = Node::<usize>::new(2);
        let child_3 = Node::<usize>::new(3);

        assert_eq!(root.add_child(child_1), 0);
        assert_eq!(child_2.add_child(child_3), 0);
        assert_eq!(root.add_child(child_2), 1);

        let got = PostOrderMutIterator::new(root.into())
            .map(|node| *node.borrow().value())
            .collect::<Vec<usize>>();

        assert_eq!(got, vec![1, 3, 2, 0]);
    }

    #[test]
    fn mutate_on_traversal() {
        let root: Arc<RefCell<Node<usize>>> = Node::<usize>::new(0).into();
        PreOrderMutIterator::new(root.clone()).for_each(|node_ref| {
            let mut node = node_ref.borrow_mut();
            let value = *node.value();

            if value < 3 {
                node.add_child(Node::new(value + 1));
            }
        });

        let got = PostOrderMutIterator::new(root)
            .map(|node| *node.borrow().value())
            .collect::<Vec<usize>>();

        assert_eq!(got, vec![3, 2, 1, 0]);
    }
}
