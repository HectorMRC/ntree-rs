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
    pub node: &'a Node<T>,
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
    pub fn node(&self) -> &'a Node<T> {
        self.node
    }
}
