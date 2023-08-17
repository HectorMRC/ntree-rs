//! Definition and implementation of the different strategies to traverse a n-ary tree.

use crate::Node;

mod traverse;
pub use traverse::*;

mod traverse_mut;
pub use traverse_mut::*;

mod traverse_owned;
pub use traverse_owned::*;

mod macros;

/// Asynchronous marker.
pub struct Asynchronous;

/// Synchronous marker.
pub struct Synchronous;

pub trait Order {
    /// Determines if the value of the [Node] must be evaluated in the given iteration.
    fn evaluate_self<T>(node: &Node<T>, it: usize) -> bool;
    /// Determines which child of the [Node] is the one to continue with.
    fn continue_with<T>(node: &Node<T>, it: usize) -> Option<usize>;
}

/// Implements the [Order] trait for the pre-order traversal.
///
/// In the preorder traversal, the value of a [Node] is evaluated before moving forward
/// forward with its children.
pub struct Preorder;

impl Order for Preorder {
    fn evaluate_self<T>(_: &Node<T>, it: usize) -> bool {
        it == 0
    }

    fn continue_with<T>(_: &Node<T>, it: usize) -> Option<usize> {
        Some(it)
    }
}

/// Implements the [Order] trait for the post-order traversal.
///
/// In the postorder traversal, the value of a [Node] is evaluated once all its children
/// have been processed.
pub struct Postorder;
impl Order for Postorder {
    fn evaluate_self<T>(node: &Node<T>, it: usize) -> bool {
        it == node.children.len()
    }

    fn continue_with<T>(_: &Node<T>, it: usize) -> Option<usize> {
        Some(it)
    }
}

impl<'a, T> Node<T> {
    /// Returns a synchronous instance of [Traverse] for the given reference of node.
    pub fn traverse(&'a self) -> Traverse<'a, T, Synchronous> {
        self.into()
    }

    /// Returns a synchronous instance of [TraverseMut] for the given mutable reference of node.
    pub fn traverse_mut(&'a mut self) -> TraverseMut<'a, T, Synchronous> {
        self.into()
    }

    /// Returns a synchronous instance of [TraverseOwned] owning the given instance of node.
    pub fn into_traverse(self) -> TraverseOwned<T, Synchronous> {
        self.into()
    }
}
