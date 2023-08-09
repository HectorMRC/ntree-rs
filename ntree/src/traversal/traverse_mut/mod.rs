//! Traversal algorithms for a mutable reference of [Node].

#[cfg(feature = "async")]
mod r#async;
#[cfg(feature = "async")]
pub use r#async::*;

mod sync;
pub use sync::*;

use crate::{Asynchronous, Node, Synchronous};
use std::marker::PhantomData;

/// Implements the traverse algorithms for a mutable reference of a [`Node`].
pub struct TraverseMut<'a, T, S> {
    node: &'a mut Node<T>,
    strategy: PhantomData<S>,
}

impl<'a, T, S> From<&'a mut Node<T>> for TraverseMut<'a, T, S> {
    fn from(node: &'a mut Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }
}

impl<'a, T, S> TraverseMut<'a, T, S> {
    pub fn node(&'a self) -> &'a Node<T> {
        self.node
    }

    pub fn node_mut(&'a mut self) -> &'a mut Node<T> {
        self.node
    }
}

impl<'a, T> From<TraverseMut<'a, T, Asynchronous>> for TraverseMut<'a, T, Synchronous> {
    fn from(value: TraverseMut<'a, T, Asynchronous>) -> Self {
        TraverseMut::new(value.node)
    }
}

impl<'a, T> From<TraverseMut<'a, T, Synchronous>> for TraverseMut<'a, T, Asynchronous>
where
    T: Sync + Send,
{
    fn from(value: TraverseMut<'a, T, Synchronous>) -> Self {
        TraverseMut::new_async(value.node)
    }
}
