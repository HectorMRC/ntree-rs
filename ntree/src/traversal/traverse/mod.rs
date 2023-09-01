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
    pub(crate) node: &'a Node<T>,
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

    /// Returns the pre-order traversal entity for the tree.
    pub fn pre<R, F>(self, pre: F) -> WithPre<'a, T, R, F, S>
    where
        F: FnMut(&Node<T>, &R) -> R,
    {
        WithPre {
            node: self.node,
            pre,
            r: PhantomData,
            strategy: PhantomData,
        }
    }

    /// Returns the post-order traversal entity for the tree.
    pub fn post<R, F>(self, post: F) -> WithPost<'a, T, R, F, S>
    where
        F: FnMut(&Node<T>, &[R]) -> R,
    {
        WithPost {
            node: self.node,
            post,
            r: PhantomData,
            strategy: PhantomData,
        }
    }
}

/// Represents the pre-order traversal.
pub struct WithPre<'a, T, R, F, S>
where
    F: FnMut(&Node<T>, &R) -> R,
{
    node: &'a Node<T>,
    pre: F,
    r: PhantomData<R>,
    strategy: PhantomData<S>,
}

impl<'a, T, R, F, S> WithPre<'a, T, R, F, S>
where
    F: FnMut(&Node<T>, &R) -> R,
{
    pub fn post<U, P>(self, post: P) -> WithPrePost<'a, T, R, U, F, P, S>
    where
        P: FnMut(&Node<T>, &[U]) -> U,
    {
        WithPrePost {
            node: self.node,
            pre: self.pre,
            post,
            r: PhantomData,
            u: PhantomData,
            strategy: PhantomData,
        }
    }
}

/// Represents the post-order traversal.
pub struct WithPost<'a, T, R, F, S>
where
    F: FnMut(&Node<T>, &[R]) -> R,
{
    node: &'a Node<T>,
    post: F,
    r: PhantomData<R>,
    strategy: PhantomData<S>,
}

impl<'a, T, R, F, S> WithPost<'a, T, R, F, S>
where
    F: FnMut(&Node<T>, &[R]) -> R,
{
    pub fn pre<U, P>(self, pre: P) -> WithPrePost<'a, T, U, R, P, F, S>
    where
        P: FnMut(&Node<T>, &U) -> U,
    {
        WithPrePost {
            node: self.node,
            pre,
            post: self.post,
            r: PhantomData,
            u: PhantomData,
            strategy: PhantomData,
        }
    }
}

/// Represents a combination of both pre and post traversals.
pub struct WithPrePost<'a, T, R, U, F1, F2, S>
where
    F1: FnMut(&Node<T>, &R) -> R,
    F2: FnMut(&Node<T>, &[U]) -> U,
{
    node: &'a Node<T>,
    pre: F1,
    post: F2,
    r: PhantomData<R>,
    u: PhantomData<U>,
    strategy: PhantomData<S>,
}
