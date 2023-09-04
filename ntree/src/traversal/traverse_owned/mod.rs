//! Traversal algorithms for an owned instance of [Node].

#[cfg(feature = "async")]
mod r#async;
#[cfg(feature = "async")]
pub use r#async::*;

mod sync;
pub use sync::*;

use crate::{Asynchronous, Node, Synchronous};
use std::{marker::PhantomData, ops::Not};

/// Implements the traverse algorithms for an owned instance of [`Node`].
pub struct TraverseOwned<T, S> {
    node: Node<T>,
    strategy: PhantomData<S>,
}

impl<T, S> From<Node<T>> for TraverseOwned<T, S> {
    fn from(node: Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }
}

impl<T> From<TraverseOwned<T, Asynchronous>> for TraverseOwned<T, Synchronous> {
    fn from(value: TraverseOwned<T, Asynchronous>) -> Self {
        TraverseOwned::new(value.node)
    }
}

impl<T> From<TraverseOwned<T, Synchronous>> for TraverseOwned<T, Asynchronous>
where
    T: Sync + Send,
{
    fn from(value: TraverseOwned<T, Synchronous>) -> Self {
        TraverseOwned::new_async(value.node)
    }
}

impl<T, S> TraverseOwned<T, S> {
    pub fn node(&self) -> &Node<T> {
        &self.node
    }

    pub fn node_mut(&mut self) -> &mut Node<T> {
        &mut self.node
    }

    pub fn take(self) -> Node<T> {
        self.node
    }

    /// Returns the `pre-order` traversal entity for the tree.
    pub fn pre(self) -> InPreOwned<T, S> {
        InPreOwned {
            next: vec![self.node],
            strategy: PhantomData,
        }
    }

    /// Returns the `post-order` traversal entity for the tree.
    pub fn post(self) -> InPostOwned<T, S> {
        InPostOwned {
            next: vec![self.node],
            strategy: PhantomData,
        }
    }
}

/// Represents the `pre-order` traversal.
pub struct InPreOwned<T, S> {
    next: Vec<Node<T>>,
    strategy: PhantomData<S>,
}

impl<T, S> Iterator for InPreOwned<T, S> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next.pop()?;
        self.next.extend(current.children.into_iter().rev());
        Some(current.value)
    }
}

/// Represents the `post-order` traversal.
pub struct InPostOwned<T, S> {
    next: Vec<Node<T>>,
    strategy: PhantomData<S>,
}

impl<T, S> Iterator for InPostOwned<T, S> {
    type Item = Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut parent = self.next.pop()?;

        if let Some(next_child) = parent
            .children
            .is_empty()
            .not()
            .then(|| parent.children.drain(0..1))
            .and_then(|mut drain| drain.next())
        {
            self.next.push(parent);
            self.next.push(next_child);
            return self.next();
        }

        Some(parent)
    }
}

/// Implements both traversals at once.
pub struct PrePostOwned<T, R, F, S> {
    node: Node<T>,
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
        root.into_traverse().pre().for_each(|n| result.push(n));

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
        root.into_traverse()
            .post()
            .for_each(|n| result.push(n.value));

        assert_eq!(result, vec![40, 50, 60, 20, 70, 80, 30, 10]);
    }
}
