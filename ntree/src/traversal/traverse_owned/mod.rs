//! Traversal algorithms for an owned instance of [Node].

#[cfg(feature = "async")]
mod r#async;
#[cfg(feature = "async")]
pub use r#async::*;

mod sync;
pub use sync::*;

use crate::{Asynchronous, Node, Synchronous};
use std::marker::PhantomData;

/// Implements the traverse algorithms for an owned instance of [`Node`].
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
}
