//! Definition and implementation of the different strategies to traverse a n-ary tree.

use crate::Node;

mod traverse;
pub use traverse::*;

mod traverse_mut;
pub use traverse_mut::*;

mod traverse_owned;
pub use traverse_owned::*;

/// Asynchronous marker.
pub struct Asynchronous;

/// Synchronous marker.
pub struct Synchronous;

pub trait Order {
    fn traverse<F1: FnMut(), F2: FnMut()>(root_fn: F1, children_fn: F2);
}

pub struct Preorder;
impl Order for Preorder {
    fn traverse<F1: FnMut(), F2: FnMut()>(mut root_fn: F1, mut children_fn: F2) {
        root_fn();
        children_fn();
    }
}

pub struct Postorder;
impl Order for Postorder {
    fn traverse<F1: FnMut(), F2: FnMut()>(mut root_fn: F1, mut children_fn: F2) {
        children_fn();
        root_fn();
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
