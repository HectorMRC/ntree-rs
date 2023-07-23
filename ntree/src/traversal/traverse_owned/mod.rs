//! Traversal algorithms for an owned instance of [Node].

// #[cfg(feature = "async")]
// mod r#async;
// #[cfg(feature = "async")]
// pub use r#async::*;

mod sync;
pub use sync::*;

use crate::Node;
use std::marker::PhantomData;

/// Implements the traverse algorithms for an owned instance of a [`Node`].
pub struct TraverseOwned<T, S> {
    node: Node<T>,
    strategy: PhantomData<S>,
}

impl<'a, T, S> From<Node<T>> for TraverseOwned<T, S> {
    fn from(node: Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }
}

impl<T, S> TraverseOwned<T, S> {
    pub fn node(&self) -> &Node<T> {
        &self.node
    }

    pub fn node_mut(&mut self) -> &mut Node<T> {
        &mut self.node
    }

    pub fn take_node(self) -> Node<T> {
        self.node
    }
}
