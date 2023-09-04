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
impl<'a, T, S> TraverseMut<'a, T, S> {
    pub fn node(&self) -> &Node<T> {
        self.node
    }

    pub fn node_mut(&mut self) -> &mut Node<T> {
        self.node
    }

    /// Returns the `pre-order` traversal entity for the tree.
    pub fn pre(self) -> InPreMut<'a, T, S> {
        InPreMut {
            node: self.node,
            strategy: PhantomData,
        }
    }

    /// Returns the `post-order` traversal entity for the tree.
    pub fn post(self) -> InPostMut<'a, T, S> {
        InPostMut {
            node: self.node,
            strategy: PhantomData,
        }
    }
}

/// Represents the `pre-order` traversal.
pub struct InPreMut<'a, T, S> {
    node: &'a mut Node<T>,
    strategy: PhantomData<S>,
}

/// Represents the `post-order` traversal.
pub struct InPostMut<'a, T, S> {
    node: &'a mut Node<T>,
    strategy: PhantomData<S>,
}

/// Implements both traversals at once.
pub struct PrePostMut<'a, T, R, F, S> {
    node: &'a mut Node<T>,
    pre: F,
    r: PhantomData<R>,
    strategy: PhantomData<S>,
}
