//! Traversal algorithms for an immutable reference of [Node].

#[cfg(feature = "async")]
mod r#async;
#[cfg(feature = "async")]
pub use r#async::*;

mod sync;
pub use sync::*;

use crate::Node;
use std::marker::PhantomData;

/// Implements the traverse algorithms for an immutable reference of a [`Node`].
pub struct Traverse<'a, T, S> {
    node: &'a Node<T>,
    strategy: PhantomData<S>,
}

impl<'a, T, S> From<&'a Node<T>> for Traverse<'a, T, S> {
    fn from(node: &'a Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }
}

impl<'a, T, S> Traverse<'a, T, S> {
    pub fn node(&self) -> &Node<T> {
        self.node
    }

    /// Returns the `pre-order` traversal entity for the tree.
    pub fn pre(self) -> InPre<'a, T, S> {
        InPre {
            node: self.node,
            next: vec![self.node],
            strategy: PhantomData,
        }
    }

    /// Returns the `post-order` traversal entity for the tree.
    pub fn post(self) -> InPost<'a, T, S> {
        InPost {
            node: self.node,
            next: vec![(self.node, 0)],
            strategy: PhantomData,
        }
    }
}

/// Represents the `pre-order` traversal.
pub struct InPre<'a, T, S> {
    node: &'a Node<T>,
    next: Vec<&'a Node<T>>,
    strategy: PhantomData<S>,
}

impl<'a, T, S> Iterator for InPre<'a, T, S> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next.pop()?;
        self.next.extend(current.children.iter().rev());
        Some(current)
    }
}

impl<'a, T, S> InPre<'a, T, S> {
    pub fn iter(self) -> impl Iterator<Item = &'a Node<T>> {
        self
    }
}

/// Represents the `post-order` traversal.
pub struct InPost<'a, T, S> {
    node: &'a Node<T>,
    next: Vec<(&'a Node<T>, usize)>,
    strategy: PhantomData<S>,
}

impl<'a, T, S> Iterator for InPost<'a, T, S> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (parent, next_child) = *self.next.last()?;
        if let Some(next_child) = parent.children.get(next_child) {
            self.next.last_mut()?.1 += 1;
            self.next.push((next_child, 0));
            return self.next();
        }

        self.next.pop().map(|(parent, _)| parent)
    }
}

impl<'a, T, S> InPost<'a, T, S> {
    pub fn iter(self) -> impl Iterator<Item = &'a Node<T>> {
        self
    }
}

/// Implements both traversals at once.
pub struct PrePost<'a, T, R, F, S> {
    node: &'a Node<T>,
    pre: F,
    r: PhantomData<R>,
    strategy: PhantomData<S>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node;

    #[test]
    fn test_pre_order_traversal() {
        let root = node!(
            10,
            node!(20, node!(40), node!(50), node!(60)),
            node!(30, node!(70), node!(80))
        );

        let mut result = Vec::new();
        root.traverse()
            .pre()
            .iter()
            .for_each(|n| result.push(n.value));

        assert_eq!(result, vec![10, 20, 40, 50, 60, 30, 70, 80]);
    }

    #[test]
    fn test_post_order_traversal() {
        let root = node!(
            10,
            node!(20, node!(40), node!(50), node!(60)),
            node!(30, node!(70), node!(80))
        );

        let mut result = Vec::new();
        root.traverse()
            .post()
            .iter()
            .for_each(|n| result.push(n.value));

        assert_eq!(result, vec![40, 50, 60, 20, 70, 80, 30, 10]);
    }
}
