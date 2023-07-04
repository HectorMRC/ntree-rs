//! Definition and implementation of the different strategies to traverse a n-ary tree.

use crate::Node;
use std::marker::PhantomData;

#[cfg(feature = "async")]
mod r#async;
#[cfg(feature = "async")]
pub use r#async::*;

mod sync;
pub use sync::*;

/// Implements the traverse algorithms for an immutable reference of a [`Node`].
pub struct Traverse<'a, T, S> {
    node: &'a Node<T>,
    strategy: PhantomData<S>,
}

/// Implements the traverse algorithms for a mutable reference of a [`Node`].
pub struct TraverseMut<'a, T, S> {
    node: &'a mut Node<T>,
    strategy: PhantomData<S>,
}
